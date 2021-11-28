//! 2021 puzzles
//! 
//! https://adventofcode.com/2021
// -----------------------------------------------------------------------------

// Load child modules
pub mod lib;
pub use lib::*;

// Include dependencies
use crate::lib::puzzle::*;

// Import child modules
mod demo;

/// Registers year runner
pub fn init (registry: PuzzleRegistry) -> PuzzleRegistry {
  demo::init(registry)  
}
