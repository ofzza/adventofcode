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
mod day01;

/// Registers year runner
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  registry = demo::init(registry);
  registry = day01::init(registry);
  registry
}
