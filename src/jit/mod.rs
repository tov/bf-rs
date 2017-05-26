mod compiler;
mod rts;

pub use self::compiler::*;

use dynasmrt;

pub struct Executable {
    code: dynasmrt::ExecutableBuffer,
    start: dynasmrt::AssemblyOffset,
}

