use std::os::raw::c_int;
use std::mem;

use common::{BfResult, Error, Count};
use rts;
use state::DEFAULT_CAPACITY;
use peephole;

use super::wrapper::*;

struct Compiler<'a> {
    context:        &'a Context,
    builder:        Builder<'a>,
    overflow:       BasicBlock<'a>,
    underflow:      BasicBlock<'a>,
    int_type:       Type<'a>,
    memory_size:    Value<'a>,
    main_function:  Value<'a>,
    pointer:        Value<'a>,
    memory:         Value<'a>,
    read_function:  Value<'a>,
    write_function: Value<'a>,
}

pub fn compile_and_run(program: &peephole::Program, memory_size: Option<usize>) -> BfResult<()> {
    let memory_size = memory_size.unwrap_or(DEFAULT_CAPACITY);

    let context = Context::new();
    let module = Module::new(&context, "bfi_module");

    let i64_type        = Type::get_i64(&context);
    let i32_type        = Type::get_i32(&context);
    let i8_type         = Type::get_i8(&context);
    let bool_type       = Type::get_bool(&context);
    let void_type       = Type::get_void(&context);
    let char_ptr_type   = Type::get_pointer(i8_type);
    let int_type        = if mem::size_of::<c_int>() == 4 {i32_type} else {i64_type};

    let false_i1        = Value::get_bool(&context, false);
    let zero_u8         = Value::get_u8(&context, 0);
    let zero_u32        = Value::get_u32(&context, 0);
    let zero_u64        = Value::get_u64(&context, 0);
    let memory_size_u64 = Value::get_u64(&context, memory_size as u64);

    let read_function_type = Type::get_function(&[], int_type);
    let read_function = module.add_function("getchar", read_function_type);

    let write_function_type = Type::get_function(&[int_type], int_type);
    let write_function = module.add_function("putchar", write_function_type);

    let main_function_type = Type::get_function(&[], i64_type);
    let main_function  = module.add_function("bfi_main", main_function_type);

    let memset_type = Type::get_function(&[char_ptr_type, i8_type, i64_type, i32_type, bool_type],
                                         void_type);
    let memset = module.add_function("llvm.memset.p0i8.i64", memset_type);

    let entry_bb = main_function.append("entry");
    let underflow_bb = main_function.append("underflow");
    let overflow_bb = main_function.append("overflow");

    let builder = Builder::new(&context);

    builder.position_at_end(underflow_bb);
    builder.ret(Value::get_u64(&context, rts::UNDERFLOW));

    builder.position_at_end(overflow_bb);
    builder.ret(Value::get_u64(&context, rts::OVERFLOW));

    builder.position_at_end(entry_bb);
    let memory = builder.array_alloca(i8_type, memory_size_u64, "memory");
    builder.call(memset, &[memory, zero_u8, memory_size_u64, zero_u32, false_i1], "");
    let pointer = builder.alloca(i64_type, "pointer");
    builder.store(zero_u64, pointer);

    let compiler = Compiler {
        context: &context,
        builder: builder,
        overflow: overflow_bb,
        underflow: underflow_bb,
        int_type: int_type,
        memory_size: memory_size_u64,
        main_function: main_function,
        pointer: pointer,
        memory: memory,
        read_function: read_function,
        write_function: write_function,
    };

    compiler.compile_block(program);

    builder.ret(Value::get_u64(&context, rts::OKAY));

    module.optimize(3, 0);
    module.dump();
    module.verify().unwrap();

    let result = module.run_function(main_function).unwrap();

    match result {
        rts::OKAY => Ok(()),
        rts::UNDERFLOW => Err(Error::PointerUnderflow),
        rts::OVERFLOW => Err(Error::PointerOverflow),
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
                    let count = Value::get_u8(&self.context, count);
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
                    self.compile_block(&[Instr(Right(count))]);
                }

                Instr(FindZeroLeft(count)) => {
                    self.compile_block(&[Instr(Left(count))]);
                }

                Instr(OffsetAddRight(count)) => {
                    let do_it = self.main_function.append("do_it");
                    let after = self.main_function.append("after");

                    self.if_not0(do_it, after);

                    builder.position_at_end(do_it);
                    let pointer = self.load_pos_offset(count, "offset_ptr");
                    let to_add = self.load_data("to_add");
                    self.store_data(Value::get_u8(&self.context, 0));
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
                    self.store_data(Value::get_u8(&self.context, 0));
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

    fn if_not0(&self, true_: BasicBlock<'a>, false_: BasicBlock<'a>) {
        let byte = self.load_data("data");
        let zero = Value::get_u8(self.context, 0);
        let comparison = self.builder.cmp(LLVMIntPredicate::LLVMIntNE, byte, zero, "comparison");
        self.builder.cond_br(comparison, true_, false_);
    }

    fn load_data_at(&self, index: Value<'a>, name: &str) -> Value<'a> {
        let address = self.builder.gep(self.memory, &[index], "data_ptr");
        self.builder.load(address, name)
    }

    fn store_data_at(&self, index: Value<'a>, value: Value<'a>) {
        let address = self.builder.gep(self.memory, &[index], "data_ptr");
        self.builder.store(value, address);
    }

    fn load_data(&self, name: &str) -> Value<'a> {
        let pointer = self.builder.load(self.pointer, "");
        self.load_data_at(pointer, name)
    }

    fn store_data(&self, value: Value<'a>) {
        let pointer = self.builder.load(self.pointer, "");
        self.store_data_at(pointer, value);
    }

    fn load_pos_offset(&self, offset: Count, name: &str) -> Value<'a> {
        let success = self.main_function.append("right_success");
        let old_pointer = self.builder.load(self.pointer, "old_pointer");
        let allowed = self.builder.sub(self.memory_size, old_pointer, "room");
        let offset = Value::get_u64(&self.context, offset as u64);
        let comparison = self.builder.cmp(LLVMIntPredicate::LLVMIntULT, offset, allowed, "allowed");
        self.builder.cond_br(comparison, success, self.overflow);
        self.builder.position_at_end(success);
        self.builder.add(old_pointer, offset, name)
    }

    fn load_neg_offset(&self, offset: Count, name: &str) -> Value<'a> {
        let success = self.main_function.append("left_success");
        let old_pointer = self.builder.load(self.pointer, "old_pointer");
        let offset = Value::get_u64(&self.context, offset as u64);
        let comparison = self.builder.cmp(LLVMIntPredicate::LLVMIntULE, offset, old_pointer,
                                     "allowed");
        self.builder.cond_br(comparison, success, self.underflow);
        self.builder.position_at_end(success);
        self.builder.sub(old_pointer, offset, name)
    }
}
