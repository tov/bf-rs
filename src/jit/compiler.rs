use dynasmrt::x64::Assembler;
use dynasmrt::{DynasmApi, DynasmLabelApi};

use super::*;
use super::analysis::AbstractInterpreter;
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
    let mut compiler = Compiler::new(checked);
    compiler.compile(program);
    compiler.into_program()
}

/// The compiler state.
struct Compiler {
    /// The underlying assembler.
    asm: Assembler,
    /// The offset of the starting instruction for the object function.
    start: dynasmrt::AssemblyOffset,
    /// Whether we are emitting bounds checks.
    checked: bool,
    /// Abstract interpreter for bounds checking analysis.
    interpreter: AbstractInterpreter,
}

impl Compiler {
    fn new(checked: bool) -> Self {
        let asm = Assembler::new();
        let start = asm.offset();

        let mut result = Compiler {
            asm: asm,
            start: start,
            checked: checked,
            interpreter: AbstractInterpreter::new(),
        };

        result.emit_prologue();

        result
    }

    fn into_program(mut self) -> Program {
        self.emit_epilogue();

        let code = self.asm.finalize().unwrap();

        Program {
            code: code,
            start: self.start,
        }
    }

    fn emit_prologue(&mut self) {
        dynasm!(self.asm
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
    }

    fn emit_epilogue(&mut self) {
        dynasm!(self.asm
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
    }

    fn compile(&mut self, program: &[peephole::Statement]) {
        for stm in program {
            self.compile_statement(stm);
        }
    }

    fn compile_statement(&mut self, stm: &peephole::Statement) {
        use peephole::Statement::*;
        use common::Instruction::*;

        match *stm {
            Instr(Right(count)) => {
                let proved = self.interpreter.right(count);

                dynasm!(self.asm
                    ;; self.load_pos_offset(count, proved)
                    ; add pointer, rax
                );
            }

            Instr(Left(count)) => {
                let proved = self.interpreter.left(count);

                dynasm!(self.asm
                    ;; self.load_neg_offset(count, proved)
                    ; sub pointer, rax
                );
            }

            Instr(Add(count)) => {
                dynasm!(self.asm
                    ; add [pointer], BYTE count as i8
                );
            }

            Instr(In) => {
                dynasm!(self.asm
                    ; mov rax, QWORD rts::RtsState::read as _
                    ; mov rcx, rts
                    ; sub rsp, BYTE 0x28
                    ; call rax
                    ; add rsp, BYTE 0x28
                    ; mov [pointer], al
                );
            }

            Instr(Out) => {
                dynasm!(self.asm
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
                dynasm!(self.asm
                    ; mov BYTE [pointer], 0
                )
            }

            Instr(FindZeroRight(skip)) => {
                self.interpreter.reset_right();

                dynasm!(self.asm
                    ; jmp >end_loop
                    ; begin_loop:
                    ;; self.load_pos_offset(skip, false)
                    ; add pointer, rax
                    ; end_loop:
                    ; cmp BYTE [pointer], 0
                    ; jnz <begin_loop
                )
            }

            Instr(FindZeroLeft(skip)) => {
                self.interpreter.reset_left();

                dynasm!(self.asm
                    ; jmp >end_loop
                    ; begin_loop:
                    ;; self.load_neg_offset(skip, false)
                    ; sub pointer, rax
                    ; end_loop:
                    ; cmp BYTE [pointer], 0
                    ; jnz <begin_loop
                )
            }

            Instr(OffsetAddRight(offset)) => {
                let proved = self.interpreter.right(offset);
                self.interpreter.left(offset);

                dynasm!(self.asm
                    ; cmp BYTE [pointer], 0
                    ; jz >skip
                    ;; self.load_pos_offset(offset, proved)
                    ; mov cl, BYTE [pointer]
                    ; mov BYTE [pointer], 0
                    ; add BYTE [pointer + rax], cl
                    ; skip:
                );
            }

            Instr(OffsetAddLeft(offset)) => {
                let proved = self.interpreter.left(offset);
                self.interpreter.right(offset);

                dynasm!(self.asm
                    ; cmp BYTE [pointer], 0
                    ; jz >skip
                    ;; self.load_neg_offset(offset, proved)
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
                let begin_label = self.asm.new_dynamic_label();
                let end_label   = self.asm.new_dynamic_label();

                self.interpreter.reset();

                dynasm!(self.asm
                    ; jmp =>end_label
                    ; =>begin_label
                    ;; self.compile(body)
                    ; =>end_label
                    ; cmp BYTE [pointer], 0
                    ; jnz =>begin_label
                );

                self.interpreter.reset();
            }
        }
    }

    #[inline]
    fn load_offset(&mut self, offset: Count) {
        if offset as i32 as Count == offset {
            dynasm!(self.asm
            ; mov rax, DWORD offset as i32
        );
        } else {
            dynasm!(self.asm
            ; mov rax, QWORD offset as i64
        );
        }
    }

    #[inline]
    fn load_pos_offset(&mut self, offset: Count, proved: bool) {
        self.load_offset(offset);

        if self.checked && !proved {
            dynasm!(self.asm
                ; mov rcx, mem_limit
                ; sub rcx, pointer
                ; cmp rcx, rax
                ; jle ->overflow
            );
        }
    }

    #[inline]
    fn load_neg_offset(&mut self, offset: Count, proved: bool) {
        self.load_offset(offset);

        if self.checked && !proved {
            dynasm!(self.asm
                ; mov rcx, pointer
                ; sub rcx, mem_start
                ; cmp rcx, rax
                ; jl ->underflow
            );
        }
    }
}

