#![cfg_attr(feature = "jit", feature(plugin))]
#![cfg_attr(feature = "jit", plugin(dynasm))]

#[cfg(feature = "jit")]
extern crate dynasmrt;

pub mod result;
pub mod op_code;
pub mod state;
pub mod traits;

pub mod ast;
pub mod rle;
pub mod flat;
pub mod peephole;

#[cfg(feature = "jit")]
pub mod jit;

pub mod test_helpers;
