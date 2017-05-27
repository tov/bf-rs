use dynasmrt::x64::Assembler;
use dynasmrt::{DynasmApi, DynasmLabelApi};

use super::*;
use common::Count;
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

fn compile_sequence(asm: &mut Assembler, program: &[peephole::Statement], checked: bool) {
    for instruction in program {
        compile_instruction(asm, instruction, checked);
    }
}

fn compile_instruction(asm: &mut Assembler, instruction: &peephole::Statement, checked: bool) {
    use peephole::Statement::*;
    use common::Instruction::*;

    match *instruction {
        Instr(Right(count)) => {
            dynasm!(asm
                ;; load_pos_offset(asm, count, checked)
                ; add pointer, rax
            );
        }

        Instr(Left(count)) => {
            dynasm!(asm
                ;; load_neg_offset(asm, count, checked)
                ; sub pointer, rax
            );
        }

        Instr(Change(count)) => {
            dynasm!(asm
                ; add [pointer], BYTE count as i8
            );
        }

        Instr(In) => {
            dynasm!(asm
                ; mov rax, QWORD rts::RtsState::read as _
                ; mov rcx, rts
                ; sub rsp, BYTE 0x28
                ; call rax
                ; add rsp, BYTE 0x28
                ; mov [pointer], al
            );
        }

        Instr(Out) => {
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

        Instr(SetZero) => {
            dynasm!(asm
                ; mov BYTE [pointer], 0
            )
        }

        Instr(FindZeroRight(skip)) => {
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

        Instr(FindZeroLeft(skip)) => {
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

        Instr(OffsetAddRight(offset)) => {
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

        Instr(OffsetAddLeft(offset)) => {
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

        Instr(JumpZero(_)) | Instr(JumpNotZero(_)) =>
            panic!("unexpected jump instruction"),

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

#[inline]
fn load_offset(asm: &mut Assembler, offset: Count) {
    if offset as i32 as Count == offset {
        dynasm!(asm
            ; mov rax, DWORD offset as i32
        );
    } else {
        dynasm!(asm
            ; mov rax, QWORD offset as i64
        );
    }
}

#[inline]
fn load_pos_offset(asm: &mut Assembler, offset: Count, checked: bool) {
    load_offset(asm, offset);

    if checked {
        dynasm!(asm
            ; mov rcx, mem_limit
            ; sub rcx, pointer
            ; cmp rcx, rax
            ; jle ->overflow
        );
    }
}

#[inline]
fn load_neg_offset(asm: &mut Assembler, offset: Count, checked: bool) {
    load_offset(asm, offset);

    if checked {
        dynasm!(asm
            ; mov rcx, pointer
            ; sub rcx, mem_start
            ; cmp rcx, rax
            ; jl ->underflow
        );
    }
}

