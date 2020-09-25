//! 2019 puzzles
//! 
//! https://adventofcode.com/2019
// -----------------------------------------------------------------------------

// Include dependencies
use super::lib::puzzle::*;

// Import puzzle modules
mod day01;

/// Register all 2019 puzzles
pub fn register (puzzles: &mut Vec<Puzzle>) {
  day01::register(puzzles);
}

