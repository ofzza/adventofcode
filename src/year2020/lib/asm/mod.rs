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
  // Instruction pointer
  _ip: usize,
  // Instructions
  pub instructions: Vec<(String, Vec<isize>)>,
  // Registries
  pub registries: AssemblerInterpreterRegistries,
  // Loop prevention toggle
  _loop_prevention: bool,
  // Loop prevention memory
  _loop_prevention_memory: HashMap<usize, bool>,
}

/// Assembler interpreter struct
#[derive(Debug, Default, Clone)]
pub struct AssemblerInterpreterRegistries {
  // Accumulation registry
  pub acc: isize
}

