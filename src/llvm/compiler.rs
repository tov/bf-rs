use std::io::Read;

use peephole;

use super::wrapper::*;

pub extern fn rts_write(c: u8) {
    print!("{}", c as char);
}

pub extern fn rts_read() -> u8 {
    let mut buf = [0u8];
    let _ = ::std::io::stdin().read_exact(&mut buf);
    buf[0]
}

pub fn compile(program: &peephole::Program) {
    use peephole::Statement::*;
    use common::Instruction::*;

    let context = Context::new();
    let module = Module::new(&context, "bfi_module");

    let i64_type      = Type::get_i64(&context);
    let i32_type      = Type::get_i32(&context);
    let i8_type       = Type::get_i8(&context);
    let bool_type     = Type::get_bool(&context);
    let void_type     = Type::get_void(&context);
    let char_ptr_type = Type::get_pointer(i8_type);

    let false_i1 = Value::get_bool(&context, false);
    let zero_u8  = Value::get_u8(&context, 0);
    let zero_u32 = Value::get_u32(&context, 0);
    let zero_u64 = Value::get_u64(&context, 0);

    let read_function_type = Type::get_pointer(Type::get_function(&[], i8_type));
    let write_function_type = Type::get_pointer(Type::get_function(&[i8_type], void_type));

    let main_function_type = Type::get_function(&[i64_type,
                                                  read_function_type,
                                                  write_function_type],
                                                i64_type);
    let main_function  = module.add_function("bfi_main", main_function_type);
    let memory_size    = main_function.get_fun_param(0);
    let read_function  = main_function.get_fun_param(1);
    let write_function = main_function.get_fun_param(2);

    let memset_type = Type::get_function(&[char_ptr_type, i8_type, i64_type, i32_type, bool_type],
                                         void_type);
    let memset = module.add_function("llvm.memset.p0i8.i64", memset_type);

    let entry_bb = main_function.append("entry");

    let builder = Builder::new(&context);
    builder.position_at_end(entry_bb);

    let memory = builder.array_alloca(i8_type, memory_size, "memory");
    builder.call(memset, &[memory, zero_u8, memory_size, zero_u32, false_i1], "");

    let pointer = builder.alloca(i64_type, "pointer");
    builder.store(zero_u64, pointer);

    for statement in program {
        match *statement {
            Instr(Right(count)) => {
                let old_pointer = builder.load(pointer, "");
                let offset = Value::get_u64(&context, count as u64);
                let new_pointer = builder.add(old_pointer, offset, "");
                builder.store(new_pointer, pointer);
            }

            Instr(Left(count)) => {
                let old_pointer = builder.load(pointer, "");
                let offset = Value::get_u64(&context, count as u64);
                let new_pointer = builder.sub(old_pointer, offset, "");
                builder.store(new_pointer, pointer);
            }

            Instr(Add(count)) => {
                let count     = Value::get_u8(&context, count);
                let address   = builder.gep(memory, &[builder.load(pointer, "")], "data_ptr");
                let old_value = builder.load(address, "old_val");
                let new_value = builder.add(old_value, count, "new_val");
                builder.store(new_value, address);
            }

            Instr(In) => {
                let result  = builder.call(read_function, &[], "");
                let address = builder.gep(memory, &[builder.load(pointer, "")], "data_ptr");
                builder.store(result, address);
            }

            Instr(Out) => {
                let address  = builder.gep(memory, &[builder.load(pointer, "")], "data_ptr");
                let argument = builder.load(address, "");
                builder.call(write_function, &[argument], "");
            }

            _ => {

            }
        }
    }

    builder.ret(zero_u64);

    module.optimize(3, 0);
    module.dump();
    module.verify().unwrap();

    println!("{:?}", module.run_function(main_function));
}

//pub struct Compiler<'a> {
//    function: &'a mut Function,
//    entry:    &'a BasicBlock,
//    builder:  CSemiBox<'a, Builder>,
//}

//impl<'a> Compiler<'a> {
//    fn new(context: &'a Context, module: &'a Module) -> Self {
//        Compiler {

//            entry: entry,
//            builder:  builder,
//        }
//    }
//}


