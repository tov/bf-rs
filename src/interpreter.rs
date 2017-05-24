use std::io::{Cursor, Read, Write, stdin, stdout};

use result::BfResult;
use state::State;

/// Program forms which can be interpreted.
pub trait Interpretable {
    /// Interprets a program against the given state.
    fn interpret_state<R: Read, W: Write>(
        &self, state: State, input: R, output: W) -> BfResult<()>;

    /// Interprets a program. If the given `state` is `None`, uses a new, empty state.
    fn interpret<R: Read, W: Write>(
        &self, state: Option<State>, input: R, output: W) -> BfResult<()>
    {
        let state = state.unwrap_or_else(|| State::new());
        self.interpret_state(state, input, output)
    }

    /// Interprets a program using stdin and stdout for input and output.
    fn interpret_stdin(&self, state: Option<State>) -> BfResult<()> {
        self.interpret(state, stdin(), stdout())
    }

    /// Interprets a program from memory, returning a vector of its output.
    fn interpret_memory(&self, state: Option<State>, input: &[u8]) -> BfResult<Vec<u8>> {
        let input = Cursor::new(input);
        let mut output = Cursor::new(Vec::new());

        self.interpret(state, input, &mut output)?;
        Ok(output.into_inner())
    }
}

