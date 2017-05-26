#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Command {
    Left,
    Right,
    Up,
    Down,
    In,
    Out,
    Begin,
    End,
}