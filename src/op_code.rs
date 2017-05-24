#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpCode {
    Left,
    Right,
    Up,
    Down,
    In,
    Out,
    Begin,
    End,
}