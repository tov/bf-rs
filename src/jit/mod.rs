mod compiler;
mod rts;

pub use self::compiler::*;

use dynasmrt;

pub struct Program {
    code: dynasmrt::ExecutableBuffer,
    start: dynasmrt::AssemblyOffset,
}

