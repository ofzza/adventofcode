//! IntCode struct
//! 
//! Implements a propriatery Touring machine system as a iterable/generator
// -----------------------------------------------------------------------------

// Include dependencies
// use super::*;

/// IntCode struct
/// 
/// Implements a propriatery Touring machine system as a iterable/generator
/// 
/// TODO: more details ...
#[derive(Default)]
pub struct IntCode {
  pub ip: usize,
  pub memory: Vec<i32>,
  pub output: i32,
  pub flag_err: bool
}
