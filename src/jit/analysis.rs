use std::collections::HashMap;

use common::Count;
use peephole::{Statement, Program};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// The net movement of a loop.
enum LoopBalance {
    /// May move right but not left.
    NoLeft,
    /// May move left but not right.
    NoRight,
    /// No net movement.
    Balanced,
    /// Net movement may be either direction.
    Unknown,
}

/// An index to a loop.
///
/// This is represented as the address of the first instruction of the loop.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct LoopIndex(usize);

impl LoopIndex {
    /// Gets the loop index from a boxed loop.
    fn from_loop_body(body: &Box<[Statement]>) -> Self {
        LoopIndex(&body[0] as *const Statement as usize)
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
        AbstractInterpreter {
            left_mark: 0,
            right_mark: 0,
            loop_balances: HashMap::new(),
        }
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
