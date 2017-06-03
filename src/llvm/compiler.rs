use std::os::raw::c_int;
use std::mem;

use common::{BfResult, Error, Count};
use rts;
use state::DEFAULT_CAPACITY;
use peephole;

use super::wrapper::*;

/// Program forms that can be compiled and run via LLVM.
pub trait LlvmCompilable {
    /// Compile the given program into the peephole AST to prepare for LLVM compilation.
    fn with_peephole<F, R>(&self, k: F) -> R
        where F: FnOnce(&peephole::Program) -> R;

    /// JIT compile and run the given program via LLVM.
    fn llvm_run(&self, memory_size: Option<usize>) -> BfResult<()> {
        self.with_peephole(|ast| compile_and_run(ast, memory_size, false))
    }
}

/// State required for the LLVM compiler.
struct Compiler<'a> {
    /// The LLVM context
    context:        &'a Context,
    /// The main module
    module:         Module<'a>,
    /// A builder positioned at the current end of the program
    builder:        Builder<'a>,
    /// Label to jump to for pointer underflow
    underflow:      BasicBlock<'a>,
    /// Label to jump to for pointer overflow
    overflow:       BasicBlock<'a>,
    /// The native C `int` type, for `putchar` and `getchar`
    int_type:       Type<'a>,
    /// The size of memory, for bounds checks
    memory_size:    Value<'a>,
    /// The main function
    main_function:  Value<'a>,
    /// `getchar`
    read_function:  Value<'a>,
    /// `putchar`
    write_function: Value<'a>,
    /// The program’s memory (“tape”)
    memory:         Value<'a>,
    /// The current offset into memory
    pointer:        Value<'a>,
}

/// JIT compile and run the given program via LLVM.
pub fn compile_and_run(program: &peephole::Program, memory_size: Option<usize>, debug: bool)
                       -> BfResult<()> {
    let context = Context::new();

    let compiler = Compiler::prologue(&context, memory_size.unwrap_or(DEFAULT_CAPACITY) as u64);
    compiler.compile_block(program);
    compiler.epilogue();

    compiler.module.optimize(3, 0);

    if debug {
        compiler.module.dump();
        compiler.module.verify().unwrap();
    }

    // This panics if LLVM fails.
    let result = compiler.module.run_function(compiler.main_function).unwrap();

    match result {
        rts::OKAY       => Ok(()),
        rts::UNDERFLOW  => Err(Error::PointerUnderflow),
        rts::OVERFLOW   => Err(Error::PointerOverflow),
        _ => panic!("unrecognized error code"),
    }
}

impl<'a> Compiler<'a> {
    fn compile_block(&self, body: &[peephole::Statement]) {
        use peephole::Statement::*;
        use common::Instruction::*;

        let builder = self.builder;

        for statement in body {
            match *statement {
                Instr(Right(count)) => {
                    let new_pointer = self.load_pos_offset(count, "new_pointer");
                    builder.store(new_pointer, self.pointer);
                }

                Instr(Left(count)) => {
                    let new_pointer = self.load_neg_offset(count, "new_pointer");
                    builder.store(new_pointer, self.pointer);
                }

                Instr(Add(count)) => {
                    let count = Value::get_u8(self.context, count);
                    let old_value = self.load_data("old_val");
                    let new_value = builder.add(old_value, count, "new_val");
                    self.store_data(new_value);
                }

                Instr(In) => {
                    let result = builder.call(self.read_function, &[], "");
                    let result = builder.trunc(result, Type::get_i8(self.context), "");
                    self.store_data(result);
                }

                Instr(Out) => {
                    let argument = self.load_data("data");
                    let argument = builder.zext(argument, self.int_type, "");
                    builder.call(self.write_function, &[argument], "");
                }

                Instr(SetZero) => {
                    self.store_data(Value::get_u8(self.context, 0));
                }

                Instr(FindZeroRight(count)) => {
                    let instr = Loop(vec![Instr(Right(count))].into_boxed_slice());
                    self.compile_block(&[instr]);
                }

                Instr(FindZeroLeft(count)) => {
                    let instr = Loop(vec![Instr(Left(count))].into_boxed_slice());
                    self.compile_block(&[instr]);
                }

                Instr(OffsetAddRight(count)) => {
                    let do_it = self.main_function.append("do_it");
                    let after = self.main_function.append("after");

                    self.if_not0(do_it, after);

                    builder.position_at_end(do_it);
                    let pointer = self.load_pos_offset(count, "offset_ptr");
                    let to_add = self.load_data("to_add");
                    self.store_data(Value::get_u8(self.context, 0));
                    let add_to = self.load_data_at(pointer, "add_to");
                    let sum = builder.add(to_add, add_to, "sum");
                    self.store_data_at(pointer, sum);
                    builder.br(after);

                    builder.position_at_end(after);
                }

                Instr(OffsetAddLeft(count)) => {
                    let do_it = self.main_function.append("do_it");
                    let after = self.main_function.append("after");

                    self.if_not0(do_it, after);

                    builder.position_at_end(do_it);
                    let pointer = self.load_neg_offset(count, "offset_ptr");
                    let to_add = self.load_data("to_add");
                    self.store_data(Value::get_u8(self.context, 0));
                    let add_to = self.load_data_at(pointer, "add_to");
                    let sum = builder.add(to_add, add_to, "sum");
                    self.store_data_at(pointer, sum);
                    builder.br(after);

                    builder.position_at_end(after);
                }

                Instr(JumpZero(_)) | Instr(JumpNotZero(_)) =>
                    panic!("unexpected instruction"),

                Loop(ref body) => {
                    let header = self.main_function.append("loop_header");
                    let true_  = self.main_function.append("loop_body");
                    let false_ = self.main_function.append("after_loop");

                    builder.br(header);

                    builder.position_at_end(header);
                    self.if_not0(true_, false_);

                    builder.position_at_end(true_);
                    self.compile_block(body);
                    builder.br(header);

                    builder.position_at_end(false_);
                }
            }
        }
    }

    /// Set up compilation.
    fn prologue(context: &'a Context, memory_size: u64) -> Self {
        let module = Module::new(context, "bfi_module");

        // Some useful types
        let i64_type        = Type::get_i64(context);
        let i32_type        = Type::get_i32(context);
        let i8_type         = Type::get_i8(context);
        let bool_type       = Type::get_bool(context);
        let void_type       = Type::get_void(context);
        let char_ptr_type   = Type::get_pointer(i8_type);

        // getchar and putchar use C's int type, which varies in size.
        let int_type = if mem::size_of::<c_int>() == 4 {i32_type} else {i64_type};

        // The size of memory as an LLVM Value
        let memory_size = Value::get_u64(context, memory_size);

        // Create the main function, create an entry basic block, and position a builder at entry.
        let main_function_type = Type::get_function(&[], i64_type);
        let main_function  = module.add_function("bfi_main", main_function_type);
        let entry_bb = main_function.append("entry");
        let builder = Builder::new(context);
        builder.position_at_end(entry_bb);

        // All state for the compiler.
        let compiler = Compiler {
            context:        context,
            module:         module,
            builder:        builder,
            underflow:      main_function.append("underflow"),
            overflow:       main_function.append("overflow"),
            int_type:       int_type,
            memory_size:    memory_size,
            main_function:  main_function,
            pointer:        builder.alloca(i64_type, "pointer"),
            memory:         builder.array_alloca(i8_type, memory_size, "memory"),
            read_function: {
                let read_function_type = Type::get_function(&[], int_type);
                module.add_function("getchar", read_function_type)
            },
            write_function: {
                let write_function_type = Type::get_function(&[int_type], int_type);
                module.add_function("putchar", write_function_type)
            }
        };

        // Zero-initialize the memory
        let memset_type = Type::get_function(&[char_ptr_type, i8_type, i64_type, i32_type, bool_type],
                                             void_type);
        let memset = compiler.module.add_function("llvm.memset.p0i8.i64", memset_type);
        builder.call(memset,
                     &[compiler.memory,
                         Value::get_u8(context, 0),
                         compiler.memory_size,
                         Value::get_u32(context, 0),
                         Value::get_bool(context, false)],
                     "");

        // Start the data pointer at 0.
        builder.store(Value::get_u64(context, 0), compiler.pointer);

        compiler
    }

    /// Emit the returns for the successful path and both error paths.
    fn epilogue(&self) {
        self.builder.ret(Value::get_u64(self.context, rts::OKAY));

        self.builder.position_at_end(self.underflow);
        self.builder.ret(Value::get_u64(self.context, rts::UNDERFLOW));

        self.builder.position_at_end(self.overflow);
        self.builder.ret(Value::get_u64(self.context, rts::OVERFLOW));
    }

    /// Branch based on whether the byte at the data pointer is 0.
    fn if_not0(&self, true_: BasicBlock<'a>, false_: BasicBlock<'a>) {
        let byte = self.load_data("data");
        let zero = Value::get_u8(self.context, 0);
        let comparison = self.builder.cmp(LLVMIntPredicate::LLVMIntNE, byte, zero, "comparison");
        self.builder.cond_br(comparison, true_, false_);
    }

    /// Load the byte from the given index into memory.
    fn load_data_at(&self, index: Value<'a>, name: &str) -> Value<'a> {
        let address = self.builder.gep(self.memory, &[index], "data_ptr");
        self.builder.load(address, name)
    }

    /// Store the given value at the given index into memory.
    fn store_data_at(&self, index: Value<'a>, value: Value<'a>) {
        let address = self.builder.gep(self.memory, &[index], "data_ptr");
        self.builder.store(value, address);
    }

    /// Load the byte from the data pointer.
    fn load_data(&self, name: &str) -> Value<'a> {
        let pointer = self.builder.load(self.pointer, "");
        self.load_data_at(pointer, name)
    }

    /// Store the given value at the data pointer.
    fn store_data(&self, value: Value<'a>) {
        let pointer = self.builder.load(self.pointer, "");
        self.store_data_at(pointer, value);
    }

    /// Add the given offset to the data pointer, checking for overflow.
    fn load_pos_offset(&self, offset: Count, name: &str) -> Value<'a> {
        let success = self.main_function.append("right_success");
        let old_pointer = self.builder.load(self.pointer, "old_pointer");
        let allowed = self.builder.sub(self.memory_size, old_pointer, "room");
        let offset = Value::get_u64(self.context, offset as u64);
        let comparison = self.builder.cmp(LLVMIntPredicate::LLVMIntULT, offset, allowed, "allowed");
        self.builder.cond_br(comparison, success, self.overflow);
        self.builder.position_at_end(success);
        self.builder.add(old_pointer, offset, name)
    }

    /// Subtract the given offset from the data pointer, checking for underflow.
    fn load_neg_offset(&self, offset: Count, name: &str) -> Value<'a> {
        let success = self.main_function.append("left_success");
        let old_pointer = self.builder.load(self.pointer, "old_pointer");
        let offset = Value::get_u64(self.context, offset as u64);
        let comparison = self.builder.cmp(LLVMIntPredicate::LLVMIntULE, offset, old_pointer,
                                     "allowed");
        self.builder.cond_br(comparison, success, self.underflow);
        self.builder.position_at_end(success);
        self.builder.sub(old_pointer, offset, name)
    }
}

impl LlvmCompilable for peephole::Program {
    fn with_peephole<F, R>(&self, k: F) -> R
        where F: FnOnce(&peephole::Program) -> R
    {
        k(self)
    }
}

impl<T: peephole::PeepholeCompilable + ?Sized> LlvmCompilable for T {
    fn with_peephole<F, R>(&self, k: F) -> R
        where F: FnOnce(&peephole::Program) -> R
    {
        k(&self.peephole_compile())
    }
}
