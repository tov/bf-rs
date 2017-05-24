#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
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