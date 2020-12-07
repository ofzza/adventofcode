//! Assembler module
//! 
//! Implements an assembler interpreter
// -----------------------------------------------------------------------------

// Import child modules
mod constructor;
mod next_method;

// (re)Export child modules
pub use constructor::*;
pub use next_method::*;

// Import dependencies
use std::collections::hash_map::HashMap;

/// Assembler interpreter struct
#[derive(Debug, Default)]
pub struct AssemblerInterpreter {
  pub instructions: Vec<(String, isize, Option<isize>)>,
  _loop_prevention: bool,
  _instruction_last_state: HashMap<usize, bool>,
  _ip: usize,
  _acc: isize
}
