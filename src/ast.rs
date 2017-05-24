/// A BF program is a sequence of instructions.
pub type Program = Box<[Instruction]>;

/// A BF instruction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    Left,
    Right,
    Up,
    Down,
    In,
    Out,
    Loop(Program),
}

/// Takes a vector of instructions and makes them into a loop.
///
/// Equivalent to
///
/// ```ignore
/// Instruction::Loop(instructions.into_boxed_slice())
/// ```
pub fn make_loop(instructions: Vec<Instruction>) -> Instruction {
    Instruction::Loop(instructions.into_boxed_slice())
}