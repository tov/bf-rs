mod compiler;
mod rts;

use dynasmrt;

pub struct Executable {
    code: dynasmrt::ExecutableBuffer,
    start: dynasmrt::AssemblyOffset,
}

