//! Loop movement balance analysis.
//!
//! This analysis determines, for the body of a loop, whether its net movement is an exact
//! amount, an unknown amount in a given direction, or unknown altogether. This is used by the
//! bound checking analysis when it encounters loops.

use std::collections::HashMap;
use peephole::{Statement, Program};

/// The body of a loop is a boxed slice of `Statement`s.
pub type LoopBody = Box<[Statement]>;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// The net movement of a loop.
pub enum LoopBalance {
    /// The exact movement of one iteration.
    Exact(isize),
    /// May move right but not left.
    RightOnly,
    /// May move left but not right.
    LeftOnly,
    /// Net movement may be either direction.
    Unknown,
}

/// An index to a loop.
///
/// This is represented as the address of the first instruction of the loop.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct LoopIndex(usize);

/// The computed net movement for each loop.
#[derive(Debug)]
pub struct LoopBalanceMap(HashMap<LoopIndex, LoopBalance>);

impl LoopBalance {
    /// Is the loop body exactly balanced between right and left?
    pub fn is_balanced(self) -> bool {
        self == LoopBalance::Exact(0)
    }

    /// Does the loop move net right (if at all)?
    pub fn is_right_only(self) -> bool {
        use self::LoopBalance::*;

        match self {
            Exact(disp) => disp >= 0,
            RightOnly   => true,
            LeftOnly    => false,
            Unknown     => false,
        }
    }

    /// Does the loop move net left (if at all)?
    pub fn is_left_only(self) -> bool {
        use self::LoopBalance::*;

        match self {
            Exact(disp) => disp <= 0,
            RightOnly   => false,
            LeftOnly    => true,
            Unknown     => false,
        }
    }
}

impl LoopIndex {
    /// Gets the loop index from a boxed loop.
    fn from_loop_body(body: &LoopBody) -> Self {
        LoopIndex(body.as_ptr() as usize)
    }
}

impl LoopBalanceMap {
    /// Initializes the map for the given program.
    pub fn new(program: &Program) -> Self {
        let mut lbm = LoopBalanceMap(HashMap::new());

        for statement in program {
            match *statement {
                Statement::Instr(_) => (),
                Statement::Loop(ref body) => {
                    lbm.analyze_loop(body);
                }
            }
        }

        lbm
    }

    /// Gets the balance of the given loop body.
    pub fn get(&self, body: &LoopBody) -> LoopBalance {
        *self.0.get(&LoopIndex::from_loop_body(body)).unwrap_or(&LoopBalance::Unknown)
    }

    /// Performs the analysis for the given loop body and any sub-loops.
    fn analyze_loop(&mut self, body: &LoopBody) -> LoopBalance {
        use peephole::Statement::*;
        use common::Instruction::*;
        use self::LoopBalance::*;

        let mut net = Exact(0);

        for statement in &**body {
            match *statement {
                Instr(Right(count)) => net = match net {
                    Exact(disp) => Exact(disp + count as isize),
                    RightOnly   => RightOnly,
                    _           => Unknown,
                },

                Instr(Left(count)) => net = match net {
                    Exact(disp) => Exact(disp - count as isize),
                    LeftOnly    => LeftOnly,
                    _           => Unknown,
                },

                Instr(Add(_)) | Instr(In) | Instr(Out) => (),

                Instr(JumpZero(_)) | Instr(JumpNotZero(_)) =>
                    panic!("unexpected jump instruction"),

                Instr(SetZero) | Instr(OffsetAddRight(_)) | Instr(OffsetAddLeft(_)) => (),

                Instr(FindZeroRight(_)) =>
                    net = if net.is_right_only() { RightOnly } else { Unknown },

                Instr(FindZeroLeft(_)) =>
                    net = if net.is_left_only() { LeftOnly } else { Unknown },

                Loop(ref body) => {
                    let body = self.analyze_loop(body);

                    net = match net {
                        Exact(disp) if body.is_balanced()                   => Exact(disp),
                        _ if net.is_right_only() && body.is_right_only()    => RightOnly,
                        _ if net.is_left_only() && body.is_left_only()      => LeftOnly,
                        _                                                   => Unknown,
                    }
                }
            }
        }

        self.0.insert(LoopIndex::from_loop_body(body), net);

        net
    }
}
