use common::Count;

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
}

impl AbstractInterpreter {
    /// In the initial state, we know nothing.
    pub fn new() -> Self {
        AbstractInterpreter {
            left_mark: 0,
            right_mark: 0,
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
