#[derive(Debug, Eq, PartialEq)]
pub enum Ast {
    Left,
    Right,
    Up,
    Down,
    In,
    Out,
    Loop(Box<[Ast]>),
}