use std::io::{Cursor, Read, Write, stdin, stdout};

use common::BfResult;
use state::State;

/// Program forms which can be interpreted.
pub trait Interpretable {
    /// Interprets a program against the given state.
    fn interpret_state<R: Read, W: Write>(&self, state: State,
                                          input: R, output: W)
        -> BfResult<()>;

    /// Interprets a program. If the given `size` is `None`, the default memory size.
    fn interpret<R: Read, W: Write>(
        &self, size: Option<usize>, input: R, output: W) -> BfResult<()>
    {
        let state = size.map(State::with_capacity).unwrap_or_else(|| State::new());
        self.interpret_state(state, input, output)
    }

    /// Interprets a program using stdin and stdout for input and output.
    fn interpret_stdin(&self, size: Option<usize>) -> BfResult<()> {
        self.interpret(size, stdin(), stdout())
    }

    /// Interprets a program from memory, returning a vector of its output.
    fn interpret_memory(&self, size: Option<usize>, input: &[u8]) -> BfResult<Vec<u8>> {
        let input = Cursor::new(input);
        let mut output = Cursor::new(Vec::new());

        self.interpret(size, input, &mut output)?;
        Ok(output.into_inner())
    }
}

