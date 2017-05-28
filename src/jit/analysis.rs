use std::collections::HashMap;

use common::Count;
use peephole::{Statement, Program};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// The net movement of a loop.
enum LoopBalance {
    /// The exact movement of one iteration.
    Exact(isize),
    /// May move right but not left.
    RightOnly,
    /// May move left but not right.
    LeftOnly,
    /// Net movement may be either direction.
    Unknown,
}

impl LoopBalance {
    /// Is the loop body exactly balanced between right and left?
    fn is_balanced(self) -> bool {
        self == LoopBalance::Exact(0)
    }

    /// Does the loop move net right (if at all)?
    fn is_right_only(self) -> bool {
        use self::LoopBalance::*;

        match self {
            Exact(disp) => disp >= 0,
            RightOnly   => true,
            LeftOnly    => false,
            Unknown     => false,
        }
    }

    /// Does the loop move net left (if at all)?
    fn is_left_only(self) -> bool {
        use self::LoopBalance::*;

        match self {
            Exact(disp) => disp <= 0,
            RightOnly   => false,
            LeftOnly    => true,
            Unknown     => false,
        }
    }
}

/// An index to a loop.
///
/// This is represented as the address of the first instruction of the loop.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct LoopIndex(usize);

impl LoopIndex {
    /// Gets the loop index from a boxed loop.
    fn from_loop_body(body: &Box<[Statement]>) -> Self {
        LoopIndex(body.as_ptr() as usize)
    }
}

/// The abstract interpreter tracks an abstraction of the pointer position.
///
/// In particular, it tracks the minimum distances from each end of memory. This can be used to
/// prove some bounds checks unnecessary.
#[derive(Debug, Clone)]
pub struct AbstractInterpreter {
    /// The minimum distance from the bottom of memory.
    left_mark: usize,
    /// The minimum distance from the top of memory.
    right_mark: usize,
    /// The computed net movement for each loop.
    loop_balances: HashMap<LoopIndex, LoopBalance>,
}

impl AbstractInterpreter {
    /// Initialize the interpreter with the body of the program.
    ///
    /// The interpreter initially analyzes the program for loop balances.
    pub fn new(program: &Program) -> Self {
        let mut result = AbstractInterpreter {
            left_mark: 0,
            right_mark: 0,
            loop_balances: HashMap::new(),
        };

        result.analyze(program);

        result
    }

    fn analyze(&mut self, statements: &[Statement]) -> LoopBalance {
        use peephole::Statement::*;
        use common::Instruction::*;
        use self::LoopBalance::*;

        let mut net: LoopBalance = Exact(0);

        for statement in statements {
            match *statement {
                Instr(Right(count)) => net = match net {
                    Exact(disp)         => Exact(disp + count as isize),
                    RightOnly           => RightOnly,
                    _                   => Unknown,
                },

                Instr(Left(count)) => net = match net {
                    Exact(disp)         => Exact(disp - count as isize),
                    LeftOnly            => LeftOnly,
                    _                   => Unknown,
                },

                Instr(Add(_)) | Instr(In) | Instr(Out) => (),

                Instr(JumpZero(_)) | Instr(JumpNotZero(_)) =>
                    panic!("unexpected jump instruction"),

                Instr(SetZero) | Instr(OffsetAddRight(_)) | Instr(OffsetAddLeft(_)) => (),

                Instr(FindZeroRight(_)) =>
                    net = if net.is_right_only() {RightOnly} else {Unknown},

                Instr(FindZeroLeft(_)) =>
                    net = if net.is_left_only() {LeftOnly} else {Unknown},

                Loop(ref body) => {
                    let index = LoopIndex::from_loop_body(body);
                    let body  = self.analyze(body);

                    self.loop_balances.insert(index, body);

                    net = match net {
                        Exact(disp) if body.is_balanced()                   => Exact(disp),
                        _ if net.is_right_only() && body.is_right_only()    => RightOnly,
                        _ if net.is_left_only() && body.is_left_only()      => LeftOnly,
                        _                                                   => Unknown,
                    }
                }
            }
        }

        net
    }

    /// Moves the pointer the given distance to the left.
    ///
    /// Returns whether we can prove that this move will not underflow.
    pub fn left(&mut self, count: Count) -> bool {
        let count = count as usize;

        self.right_mark += count;
        if count <= self.left_mark {
            self.left_mark -= count;
            true
        } else {
            self.left_mark = 0;
            false
        }
    }

    /// Moves the pointer the given distance to the right.
    ///
    /// Returns whether we can prove that this move will not overflow.
    pub fn right(&mut self, count: Count) -> bool {
        let count = count as usize;

        self.left_mark += count;
        if count <= self.right_mark {
            self.right_mark -= count;
            true
        } else {
            self.right_mark = 0;
            false
        }
    }

    /// Resets the left mark.
    ///
    /// This is used when we may move an arbitrary distance to the left.
    pub fn reset_left(&mut self) {
        self.left_mark = 0;
    }

    /// Resets the right mark.
    ///
    /// This is used when we may move an arbitrary distance to the right.
    pub fn reset_right(&mut self) {
        self.right_mark = 0;
    }

    /// Resets both marks.
    ///
    /// This is used when we enter and leave a loop.
    pub fn reset(&mut self) {
        self.reset_left();
        self.reset_right();
    }
}
