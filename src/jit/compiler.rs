use dynasmrt::x64::Assembler;
use dynasmrt::{DynasmApi, DynasmLabelApi};

use super::*;
use op_code::OpCode;
use rle_ast;

dynasm!(asm
    ; .alias pointer, r12
    ; .alias mem_start, r13
    ; .alias mem_limit, r14
    ; .alias rts, r15
);

pub fn compile(program: &rle_ast::Program) -> Program {
    let mut asm = dynasmrt::x64::Assembler::new();
    let start = asm.offset();

    dynasm!(asm
        ; push r12
        ; push r13
        ; push r14
        ; push r15
        ; mov pointer, rcx
        ; mov mem_start, rcx
        ; mov mem_limit, rcx
        ; add mem_limit, rdx
        ; mov rts, r8
    );

    compile_sequence(&mut asm, program);

    dynasm!(asm
        ; mov rax, 0
        ; jmp ->finish
        ; ->underflow:
        ; mov rax, 1
        ; jmp ->finish
        ; ->overflow:
        ; mov rax, 2
        ; ->finish:
        ; pop r15
        ; pop r14
        ; pop r13
        ; pop r12
        ; ret
    );

    let code = asm.finalize().unwrap();

    Program {
        code: code,
        start: start,
    }
}

pub fn compile_sequence(asm: &mut Assembler, program: &[rle_ast::Instruction]) {
    for instruction in program {
        compile_instruction(asm, instruction);
    }
}

pub fn compile_instruction(asm: &mut Assembler, instruction: &rle_ast::Instruction) {
    use rle_ast::Instruction::*;

    match *instruction {
        Op((OpCode::Right, count)) => {
            dynasm!(asm
                ; mov rax, QWORD count as i64
                ; mov rcx, mem_limit
                ; sub rcx, pointer
                ; cmp rcx, rax
                ; jle ->overflow
                ; add pointer, rax
            );
        }

        Op((OpCode::Left, count)) => {
            dynasm!(asm
                ; mov rax, QWORD count as i64
                ; mov rcx, pointer
                ; sub rcx, mem_start
                ; cmp rcx, rax
                ; jl ->underflow
                ; sub pointer, rax
            );
        }

        Op((OpCode::Up, count)) => {
            dynasm!(asm
                ; add [pointer], BYTE usize_to_i8(count)
            );
        }

        Op((OpCode::Down, count)) => {
            dynasm!(asm
                ; sub [pointer], BYTE usize_to_i8(count)
            );
        }

        Op((OpCode::In, count)) => {
            for _ in 0 .. count {
                dynasm!(asm
                    ; mov rax, QWORD rts::RtsState::read as _
                    ; mov rcx, rts
                    ; sub rsp, BYTE 0x28
                    ; call rax
                    ; add rsp, BYTE 0x28
                    ; mov [pointer], al
                );
            }
        }

        Op((OpCode::Out, count)) => {
            for _ in 0 .. count {
                dynasm!(asm
                    ; mov rax, QWORD rts::RtsState::write as _
                    ; mov rcx, rts
                    ; xor rdx, rdx
                    ; mov dl, [pointer]
                    ; sub rsp, BYTE 0x28
                    ; call rax
                    ; add rsp, BYTE 0x28
                );
            }
        }

        Op((OpCode::Begin, _)) | Op((OpCode::End, _)) => panic!("bad opcode"),

        Loop(ref body) => {
            let begin_label = asm.new_dynamic_label();
            let end_label   = asm.new_dynamic_label();

            dynasm!(asm
                ; cmp BYTE [pointer], 0
                ; jz =>end_label
                ; =>begin_label
            );

            compile_sequence(asm, body);

            dynasm!(asm
                ; cmp BYTE [pointer], 0
                ; jnz =>begin_label
                ; =>end_label
            );
        }
    }
}

fn usize_to_i8(n: usize) -> i8 {
    let mut n = n % 256;
    if n > 127 {n -= 256 };
    n as i8
}
