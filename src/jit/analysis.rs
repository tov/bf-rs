use super::loop_balance::LoopBalanceMap;
use common::Count;
use peephole::{Statement, Program};

/// Interface for bounds checking analysis.
///
/// We use two implementations of this trait to specialize the compiler for checked versus
/// unchecked mode. The [impl for checked mode](struct.AbstractInterpreter.html) actually does the
/// analysis, whereas the [impl for unchecked mode](struct.NoAnalysis.html) is all no-ops.
pub trait BoundsAnalysis {
    /// Creates a new analyzer for the given program.
    fn new(program: &Program) -> Self;

    /// Moves the pointer the given distance to the left.
    ///
    /// Returns whether we can prove that this move will not underflow.
    fn move_left(&mut self, count: Count) -> bool;

    /// Moves the pointer the given distance to the right.
    ///
    /// Returns whether we can prove that this move will not overflow.
    fn move_right(&mut self, count: Count) -> bool;

    /// Checks whether the pointer can safely move the given distance to the left.
    fn check_left(&self, count: Count) -> bool;

    /// Checks whether the pointer can safely move the given distance to the right.
    fn check_right(&self, count: Count) -> bool;

    /// Resets the left mark.
    ///
    /// This is used when we may move an arbitrary distance to the left.
    fn reset_left(&mut self);

    /// Resets the right mark.
    ///
    /// This is used when we may move an arbitrary distance to the right.
    fn reset_right(&mut self);

    /// Updates the marks upon entering a loop.
    fn enter_loop(&mut self, body: &Box<[Statement]>);

    /// Updates the marks upon leaving a loop.
    fn leave_loop(&mut self);
}

/// Abstract interpreter that tracks an abstraction of the pointer position.
///
/// In particular, it tracks the minimum distances from each end of memory. This can be used to
/// prove some bounds checks unnecessary.
#[derive(Debug)]
pub struct AbstractInterpreter {
    /// The minimum distance from the bottom of memory.
    left_mark: usize,
    /// The minimum distance from the top of memory.
    right_mark: usize,
    /// The marks to restore when leaving a loop.
    loop_stack: Vec<(usize, usize)>,
    /// The computed net movement for each loop.
    loop_balances: LoopBalanceMap,
}

impl BoundsAnalysis for AbstractInterpreter {
    /// Initialize the interpreter with the body of the program.
    ///
    /// The interpreter initially analyzes the program for loop balances, but only if we're doing
    /// bounds checking in the first place. (There's no point in doing the analysis if we're not
    /// going to use it.)
    fn new(program: &Program) -> Self {
        AbstractInterpreter {
            left_mark: 0,
            right_mark: 0,
            loop_stack: Vec::new(),
            loop_balances: LoopBalanceMap::new(program),
        }
    }

    /// Moves the pointer the given distance to the left.
    ///
    /// Returns whether we can prove that this move will not underflow.
    fn move_left(&mut self, count: Count) -> bool {
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
    fn move_right(&mut self, count: Count) -> bool {
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

    fn check_left(&self, count: Count) -> bool {
        count <= self.left_mark
    }

    fn check_right(&self, count: Count) -> bool {
        count <= self.right_mark
    }

    /// Resets the left mark.
    ///
    /// This is used when we may move an arbitrary distance to the left.
    fn reset_left(&mut self) {
        self.left_mark = 0;
    }

    /// Resets the right mark.
    ///
    /// This is used when we may move an arbitrary distance to the right.
    fn reset_right(&mut self) {
        self.right_mark = 0;
    }

    /// Updates the marks upon entering a loop.
    fn enter_loop(&mut self, body: &Box<[Statement]>) {
        let balance = self.loop_balances.get(body);

        if balance.is_balanced() {
            // No change
        } else if balance.is_right_only() {
            self.reset_right();
        } else if balance.is_left_only() {
            self.reset_left();
        } else {
            self.reset_left();
            self.reset_right();
        }

        self.loop_stack.push((self.left_mark, self.right_mark));
    }

    /// Updates the marks upon leaving a loop.
    fn leave_loop(&mut self) {
        let (left_mark, right_mark) = self.loop_stack.pop()
            .expect("got exit_loop without matching enter_loop");
        self.left_mark = left_mark;
        self.right_mark = right_mark;
    }
}

/// No-op implementation of `BoundsAnalysis`.
///
/// Tracks no information, and returns `false` (not proved) for moves.
pub struct NoAnalysis;

impl BoundsAnalysis for NoAnalysis {
    fn new(_program: &Program) -> Self { NoAnalysis }
    fn move_left(&mut self, _count: Count) -> bool { false }
    fn move_right(&mut self, _count: Count) -> bool { false }
    fn check_left(&self, _count: Count) -> bool { false }
    fn check_right(&self, _count: Count) -> bool { false }
    fn reset_left(&mut self) { }
    fn reset_right(&mut self) { }
    fn enter_loop(&mut self, _body: &Box<[Statement]>) { }
    fn leave_loop(&mut self) { }
}
