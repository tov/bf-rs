use dynasmrt::x64::Assembler;
use dynasmrt::{DynasmApi, DynasmLabelApi};

use super::*;
use peephole;

dynasm!(asm
    ; .alias pointer, r12
    ; .alias mem_start, r13
    ; .alias mem_limit, r14
    ; .alias rts, r15
);

/// Compiles peephole-optimized AST to x64 machine code.
///
/// Uses the `dynasmrt` assembler
pub fn compile(program: &peephole::Program, checked: bool) -> Program {
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

    compile_sequence(&mut asm, program, checked);

    dynasm!(asm
        ; mov rax, rts::OKAY as i32
        ; jmp ->finish

        ; ->underflow:
        ; mov rax, rts::UNDERFLOW as i32
        ; jmp ->finish

        ; ->overflow:
        ; mov rax, rts::OVERFLOW as i32

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

fn compile_sequence(asm: &mut Assembler, program: &[peephole::Instruction], checked: bool) {
    for instruction in program {
        compile_instruction(asm, instruction, checked);
    }
}

fn compile_instruction(asm: &mut Assembler, instruction: &peephole::Instruction, checked: bool) {
    use peephole::Instruction::*;

    match *instruction {
        Right(count) => {
            dynasm!(asm
                ;; load_pos_offset(asm, count, checked)
                ; add pointer, rax
            );
        }

        Left(count) => {
            dynasm!(asm
                ;; load_neg_offset(asm, count, checked)
                ; sub pointer, rax
            );
        }

        Change(count) => {
            dynasm!(asm
                ; add [pointer], BYTE u8_to_i8(count)
            );
        }

        In => {
            dynasm!(asm
                ; mov rax, QWORD rts::RtsState::read as _
                ; mov rcx, rts
                ; sub rsp, BYTE 0x28
                ; call rax
                ; add rsp, BYTE 0x28
                ; mov [pointer], al
            );
        }

        Out => {
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

        SetZero => {
            dynasm!(asm
                ; mov BYTE [pointer], 0
            )
        }

        FindZeroRight(skip) => {
            dynasm!(asm
                ; jmp >end_loop
                ; begin_loop:
                ;; load_pos_offset(asm, skip, checked)
                ; add pointer, rax
                ; end_loop:
                ; cmp BYTE [pointer], 0
                ; jnz <begin_loop
            )
        }

        FindZeroLeft(skip) => {
            dynasm!(asm
                ; jmp >end_loop
                ; begin_loop:
                ;; load_neg_offset(asm, skip, checked)
                ; sub pointer, rax
                ; end_loop:
                ; cmp BYTE [pointer], 0
                ; jnz <begin_loop
            )
        }

        OffsetAddRight(offset) => {
            dynasm!(asm
                ; cmp BYTE [pointer], 0
                ; jz >skip
                ;; load_pos_offset(asm, offset, checked)
                ; mov cl, BYTE [pointer]
                ; mov BYTE [pointer], 0
                ; add BYTE [pointer + rax], cl
                ; skip:
            );
        }

        OffsetAddLeft(offset) => {
            dynasm!(asm
                ; cmp BYTE [pointer], 0
                ; jz >skip
                ;; load_neg_offset(asm, offset, checked)
                ; mov cl, BYTE [pointer]
                ; mov BYTE [pointer], 0
                ; neg rax
                ; add BYTE [pointer + rax], cl
                ; skip:
            );
        }

        Loop(ref body) => {
            let begin_label = asm.new_dynamic_label();
            let end_label   = asm.new_dynamic_label();

            dynasm!(asm
                ; jmp =>end_label
                ; =>begin_label
            );

            compile_sequence(asm, body, checked);

            dynasm!(asm
                ; =>end_label
                ; cmp BYTE [pointer], 0
                ; jnz =>begin_label
            );
        }
    }
}

fn load_pos_offset(asm: &mut Assembler, offset: usize, checked: bool) {
    dynasm!(asm
        ; mov rax, QWORD offset as i64
    );

    if checked {
        dynasm!(asm
            ; mov rcx, mem_limit
            ; sub rcx, pointer
            ; cmp rcx, rax
            ; jle ->overflow
        );
    }
}

fn load_neg_offset(asm: &mut Assembler, offset: usize, checked: bool) {
    dynasm!(asm
        ; mov rax, QWORD offset as i64
    );

    if checked {
        dynasm!(asm
            ; mov rcx, pointer
            ; sub rcx, mem_start
            ; cmp rcx, rax
            ; jl ->underflow
        );
    }
}

fn u8_to_i8(n: u8) -> i8 {
    let mut n = n as isize % 256;
    if n > 127 { n -= 256 };
    n as i8
}
