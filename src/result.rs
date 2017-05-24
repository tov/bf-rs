use std::fmt;

pub type BfResult<T> = Result<T, Error>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    UnmatchedBegin,
    UnmatchedEnd,
    PointerUnderflow,
    PointerOverflow,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        match *self {
            UnmatchedBegin => write!(f, "unmatched ‘[’"),
            UnmatchedEnd => write!(f, "unmatched ‘]’"),
            PointerUnderflow => write!(f, "pointer underflow"),
            PointerOverflow => write!(f, "pointer overflow"),
        }
    }
}