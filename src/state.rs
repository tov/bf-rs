/// The default number of 8-bit memory cells, as used by
/// [`State::new`](struct.State.html#method.new).
pub const DEFAULT_CAPACITY: usize = 30_000;

/// The BF machine state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    memory: Box<[u8]>,
    pointer: usize,
}

impl State {
    /// Creates a new BF machine state with capacity
    /// [`DEFAULT_CAPACITY`].
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_CAPACITY)
    }

    /// Creates a new BF machine state.
    pub fn with_capacity(capacity: usize) -> Self {
        State {
            memory: vec![0; capacity].into_boxed_slice(),
            pointer: 0,
        }
    }

    /// Decrements the pointer.
    ///
    /// # Errors
    ///
    /// Panics if pointer is already 0.
    #[inline]
    pub fn left(&mut self) {
        if self.pointer == 0 {
            panic!("pointer underflow");
        } else {
            self.pointer -= 1;
        }
    }

    /// Increments the pointer.
    ///
    /// # Errors
    ///
    /// Panics if pointer would go past the end of the memory.
    #[inline]
    pub fn right(&mut self) {
        self.pointer += 1;
        if self.pointer == self.memory.len() {
            panic!("pointer overflow");
        }
    }

    /// Increments the byte at the pointer.
    ///
    /// Wraps around modulo 256.
    #[inline]
    pub fn up(&mut self) {
        let old = self.load();
        self.store(old.wrapping_add(1));
    }

    /// Decrements the byte at the pointer.
    ///
    /// Wraps around modulo 256.
    #[inline]
    pub fn down(&mut self) {
        let old = self.load();
        self.store(old.wrapping_sub(1));
    }

    /// Gets the value of the point at the pointer.
    #[inline]
    pub fn load(&self) -> u8 {
        self.memory[self.pointer]
    }

    /// Sets the value of the byte at the pointer.
    #[inline]
    pub fn store(&mut self, value: u8) {
        self.memory[self.pointer] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn right_moves_right() {
        let mut actual = make(&[0, 0, 0], 0);
        let expected = make(&[0, 0, 0], 1);

        actual.right();

        assert_eq!(actual, expected);
    }

    #[test]
    fn right_then_left_restores() {
        let mut actual = make(&[0, 0, 0], 0);
        let expected = make(&[0, 0, 0], 0);

        actual.right();
        actual.left();

        assert_eq!(actual, expected);
    }

    #[test]
    fn up_goes_to_1() {
        let mut actual = make(&[0, 0, 0], 0);
        actual.up();
        assert_eq!(actual, make(&[1, 0, 0], 0))
    }

    #[test]
    fn down_goes_to_255() {
        let mut actual = make(&[0, 0, 0], 0);
        actual.down();
        assert_eq!(actual, make(&[255, 0, 0], 0))
    }

    #[test]
    fn load_reads() {
        assert_eq!(make(&[0, 0, 0], 0).load(), 0);
        assert_eq!(make(&[1, 0, 0], 0).load(), 1);
        assert_eq!(make(&[1, 2, 0], 1).load(), 2);
    }

    #[test]
    fn store_writes() {
        let mut actual = make(&[0, 0, 0], 0);
        actual.store(5);
        assert_eq!(actual, make(&[5, 0, 0], 0));
        actual.right();
        actual.store(8);
        assert_eq!(actual, make(&[5, 8, 0], 1));
    }

    #[test]
    fn longer_sequence_of_actions() {
        let mut actual = make(&[0, 0, 0], 0);
        actual.up();
        assert_eq!(actual, make(&[1, 0, 0], 0));
        actual.up();
        assert_eq!(actual, make(&[2, 0, 0], 0));
        actual.right();
        assert_eq!(actual, make(&[2, 0, 0], 1));
        actual.down();
        assert_eq!(actual, make(&[2, 255, 0], 1));
        actual.down();
        assert_eq!(actual, make(&[2, 254, 0], 1));
        actual.right();
        assert_eq!(actual, make(&[2, 254, 0], 2));
        actual.store(77);
        assert_eq!(actual, make(&[2, 254, 77], 2));
    }

    #[test]
    fn right_to_right_edge_is_okay() {
        let mut actual = make(&[0, 0, 0], 0);
        actual.right();
        actual.right();
        assert_eq!(actual, make(&[0, 0, 0], 2));
    }

    #[test]
    #[should_panic]
    fn right_past_edge_is_error() {
        let mut actual = make(&[0, 0, 0], 0);
        actual.right();
        actual.right();
        actual.right();
    }

    #[test]
    #[should_panic]
    fn move_left_crashes() {
        let mut machine = make(&[0, 0, 0], 0);
        machine.left();
    }

    fn make(memory: &[u8], pointer: usize) -> State {
        State {
            memory: memory.to_vec().into_boxed_slice(),
            pointer: pointer,
        }
    }
}
