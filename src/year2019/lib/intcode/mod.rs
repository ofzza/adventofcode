//! IntCode module
//! 
//! Implements an IntCode turing machine computer
// -----------------------------------------------------------------------------

// Import child modules
mod constructor;
mod next_method;

// (re)Export child modules
pub use constructor::*;
pub use next_method::*;

/// IntCode struct
/// 
/// Implements a propriatery Touring machine system as a iterable/generator
/// 
/// TODO: more details ...
#[derive(Default)]
pub struct IntCode {
  _ip: usize,
  pub memory: Vec<i32>,
  pub output: i32,
  pub flag_err: bool
}
