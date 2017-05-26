use dynasmrt::x64::Assembler;
use dynasmrt::DynasmApi;

use super::*;
use op_code::OpCode;
use rle_ast::{Program, Instruction};

dynasm!(asm
    ; .alias pointer, r12
    ; .alias mem_start, r13
    ; .alias mem_limit, r14
    ; .alias io_struct, r15
);

pub fn compile(program: &Program) -> Executable {
    let mut asm = dynasmrt::x64::Assembler::new();
    let start = asm.offset();

    dynasm!(asm
        ; push pointer
        ; push mem_start
        ; push mem_limit
        ; push io_struct
    );

    compile_sequence(&mut asm, program);

    dynasm!(asm
        ; pop io_struct
        ; pop mem_limit
        ; pop mem_start
        ; pop pointer
        ; ret
    );

    let code = asm.finalize().unwrap();

    Executable {
        code: code,
        start: start,
    }
}

pub fn compile_sequence(asm: &mut Assembler, program: &[Instruction]) {
//    for instruction in program {
//        compile_instruction(asm, instruction);
//    }
    dynasm!(asm
        ; mov rax, rdx
        ; add rax, rcx
    )
}

pub fn compile_instruction(asm: &mut Assembler, instruction: &Instruction) {
    use self::Instruction::*;

    match *instruction {
        Op((OpCode::Right, count)) => {
            dynasm!(asm
                ; mov rax, QWORD count as i64
                ; add pointer, rax
            );
        }

        Op((OpCode::Left, count)) => {
            dynasm!(asm
                ; mov rax, QWORD count as i64
                ; sub pointer, rax
            );
        }

        _ => unimplemented!(),
    }
}