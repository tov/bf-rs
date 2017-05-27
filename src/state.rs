//! The Brainfuck machine state.
//!
//! Useful for creating initial states for testing, and also the interface used by the
//! interpreters to access the state.

use std::default::Default;
use std::io::{Read, Write};
use std::mem;
use std::num::Wrapping;

use common::{BfResult, Error};

/// (`== 30_000`) The default number of 8-bit memory cells, as used by
/// [`State::new`](struct.State.html#method.new).
pub const DEFAULT_CAPACITY: usize = 30_000;

/// The Brainfuck machine state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    memory: Box<[Wrapping<u8>]>,
    pointer: usize,
}

impl State {
    /// Creates a new BF machine state with memory capacity
    /// [`DEFAULT_CAPACITY`](constant.DEFAULT_CAPACITY.html) (30_000).
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_CAPACITY)
    }

    /// Creates a new BF machine state with the given memory capacity.
    pub fn with_capacity(memory_size: usize) -> Self {
        State {
            memory: vec![Wrapping(0); memory_size].into_boxed_slice(),
            pointer: 0,
        }
    }

    /// Decrements/decreases the pointer.
    ///
    /// # Errors
    ///
    /// Return `Err` if pointer would go below 0.
    #[inline]
    pub fn left(&mut self, count: usize) -> BfResult<()> {
        self.pointer = self.neg_offset(count)?;
        Ok(())
    }

    /// Increments/increases the pointer.
    ///
    /// # Errors
    ///
    /// Return `Err` if pointer would go past the end of the memory.
    #[inline]
    pub fn right(&mut self, count: usize) -> BfResult<()> {
        self.pointer = self.pos_offset(count)?;
        Ok(())
    }

    #[inline]
    fn pos_offset(&self, offset: usize) -> BfResult<usize> {
        if self.pointer + offset < self.memory.len() {
            Ok(self.pointer + offset)
        } else {
            Err(Error::PointerOverflow)
        }
    }

    #[inline]
    fn neg_offset(&self, offset: usize) -> BfResult<usize> {
        if self.pointer >= offset {
            Ok(self.pointer - offset)
        } else {
            Err(Error::PointerUnderflow)
        }
    }

    /// Increments/increases the byte at the pointer.
    ///
    /// Wraps around modulo 256.
    #[inline]
    pub fn up(&mut self, count: u8) {
        self.memory[self.pointer] += Wrapping(count);
    }

    /// Decrements/decreases the byte at the pointer.
    ///
    /// Wraps around modulo 256.
    #[inline]
    pub fn down(&mut self, count: u8) {
        self.memory[self.pointer] -= Wrapping(count);
    }

    /// Gets the value of the byte at the pointer.
    #[inline]
    pub fn load(&self) -> u8 {
        self.memory[self.pointer].0
    }

    /// Sets the value of the byte at the pointer.
    #[inline]
    pub fn store(&mut self, value: u8) {
        self.memory[self.pointer] = Wrapping(value);
    }

    /// Adds the given value at the given positive offset from the pointer.
    #[inline]
    pub fn up_pos_offset(&mut self, offset: usize, value: u8) -> BfResult<()> {
        let address = self.pos_offset(offset)?;
        self.memory[address] += Wrapping(value);
        Ok(())
    }

    /// Adds the given value at the given negative offset from the pointer.
    #[inline]
    pub fn up_neg_offset(&mut self, offset: usize, value: u8) -> BfResult<()> {
        let address = self.neg_offset(offset)?;
        self.memory[address] += Wrapping(value);
        Ok(())
    }

    /// Reads from a `Read` into the byte at the pointer.
    #[inline]
    pub fn read<R: Read>(&mut self, input: &mut R) {
        let mut byte = [0];
        let _ = input.read_exact(&mut byte);
        self.store(byte[0]);
    }

    /// Writes to a `Write` from the byte at the pointer.
    #[inline]
    pub fn write<W: Write>(&self, output: &mut W) {
        let _ = output.write_all(&[self.load()]);
    }

    /// The memory capacity.
    pub fn capacity(&self) -> usize {
        self.memory.len()
    }

    /// Gets a mutable, raw pointer to the start of memory.
    ///
    /// This is used by the JIT RTS to pass the memory pointer to the generated code.
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        // Assumes that Wrapping<u8> == u8:
        unsafe { mem::transmute(self.memory.as_mut_ptr()) }
    }
}

impl Default for State {
    fn default() -> Self {
        State::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn right_moves_right() {
        let mut actual = make(&[0, 0, 0], 0);
        let expected = make(&[0, 0, 0], 1);

        actual.right(1).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn right_then_left_restores() {
        let mut actual = make(&[0, 0, 0], 0);
        let expected = make(&[0, 0, 0], 0);

        actual.right(1).unwrap();
        actual.left(1).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn up_goes_to_1() {
        let mut actual = make(&[0, 0, 0], 0);
        actual.up(1);
        assert_eq!(actual, make(&[1, 0, 0], 0))
    }

    #[test]
    fn down_goes_to_255() {
        let mut actual = make(&[0, 0, 0], 0);
        actual.down(1);
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
        actual.right(1).unwrap();
        actual.store(8);
        assert_eq!(actual, make(&[5, 8, 0], 1));
    }

    #[test]
    fn longer_sequence_of_actions() {
        let mut actual = make(&[0, 0, 0], 0);
        actual.up(1);
        assert_eq!(actual, make(&[1, 0, 0], 0));
        actual.up(1);
        assert_eq!(actual, make(&[2, 0, 0], 0));
        actual.right(1).unwrap();
        assert_eq!(actual, make(&[2, 0, 0], 1));
        actual.down(1);
        assert_eq!(actual, make(&[2, 255, 0], 1));
        actual.down(1);
        assert_eq!(actual, make(&[2, 254, 0], 1));
        actual.right(1).unwrap();
        assert_eq!(actual, make(&[2, 254, 0], 2));
        actual.store(77);
        assert_eq!(actual, make(&[2, 254, 77], 2));
    }

    #[test]
    fn right_to_right_edge_is_okay() {
        let mut actual = make(&[0, 0, 0], 0);
        actual.right(1).unwrap();
        actual.right(1).unwrap();
        assert_eq!(actual, make(&[0, 0, 0], 2));
    }

    #[test]
    #[should_panic]
    fn right_past_edge_is_error() {
        let mut actual = make(&[0, 0, 0], 0);
        actual.right(1).unwrap();
        actual.right(1).unwrap();
        actual.right(1).unwrap();
    }

    #[test]
    #[should_panic]
    fn move_left_is_error() {
        let mut machine = make(&[0, 0, 0], 0);
        machine.left(1).unwrap();
    }

    fn make(memory: &[u8], pointer: usize) -> State {
        State {
            memory: memory.iter().map(|&b| Wrapping(b)).collect::<Vec<_>>().into_boxed_slice(),
            pointer: pointer,
        }
    }
}
