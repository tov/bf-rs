use std::os::raw::{c_char, c_int};

use llvm_alt::*;

use peephole;
//use rts;

pub type EntryFunction = extern fn(memory_size: u64) -> u64;

pub fn compile(program: &peephole::Program) {
    use peephole::Statement::*;
    use common::Instruction::*;

    let context = Context::new();
    let module = Module::new("bfi_module", &context);
    let main_function = module.add_function("bfi_main", Type::get::<EntryFunction>(&context));

    let memory_size = &main_function[0];

    let entry = main_function.append("entry");
    let builder = Builder::new(&context);
    builder.position_at_end(entry);

//    let memset_t = Type::get::<extern "C" fn(*const c_char, u8, u64, u32, bool) -> ()>(&context);
//    let memset = module.add_function("llvm.memset.p0i8.i64", memset_t);
//
//    let getchar_t = Type::get::<extern "C" fn() -> c_int>(&context);
//    let getchar = module.add_function("getchar", getchar_t);
//
//    let putchar_t = Type::get::<extern "C" fn(c_int) -> c_int>(&context);
//    let putchar = module.add_function("putchar", putchar_t);

//    let memory   = builder.build_array_alloca(Type::get::<u8>(&context), memory_size);
//    builder.build_call(memset, &[memory, 0u8.compile(&context), memory_size,
//                                 0u32.compile(&context), false.compile(&context)]);
//    let pointer  = builder.build_alloca(Type::get::<u64>(&context));
//    builder.build_store((0 as u64).compile(&context), pointer);

    for statement in program {
        match *statement {
//            Instr(Right(count)) => {
//                let old_pointer = builder.build_load(pointer);
//                let new_pointer = builder.build_add(old_pointer, count.compile(&context));
//                builder.build_store(new_pointer, pointer);
//            }
//            Instr(Left(count)) => {
//                let old_pointer = builder.build_load(pointer);
//                let new_pointer = builder.build_sub(old_pointer, count.compile(&context));
//                builder.build_store(new_pointer, pointer);
//            }
//            Instr(Add(count)) => {
//                let address = builder.build_gep(memory, &[builder.build_load(pointer)]);
//                let old_value = builder.build_load(address);
//                let new_value = builder.build_add(old_value, count.compile(&context));
//                builder.build_store(new_value, address);
//            }
//            Instr(In) => {
//                let int = builder.build_call(getchar, &[]);
//                let c   = builder.build_trunc(int, Type::get::<c_char>(&context));
//                let address = builder.build_gep(memory, &[builder.build_load(pointer)]);
//                builder.build_store(c, address);
//            }
//            Instr(Out) => {
//                let address = builder.build_gep(memory, &[builder.build_load(pointer)]);
//                let c = builder.build_load(address);
//                let int = builder.build_zext(c, Type::get::<c_int>(&context));
//                builder.build_call(putchar, &[int]);
//            }
            _ => unimplemented!(),
        }
    }

    builder.build_ret(0u64.compile(&context));

    module.verify().unwrap();
//    module.optimize(3, 0);

    println!("{:?}", module);

    let ee = JitEngine::new(&module, JitOptions {opt_level: 0}).unwrap();
    ee.with_function(main_function, |main: EntryFunction| {
        //         main(30_000);
    });
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


