//! 2019 puzzles
//! 
//! https://adventofcode.com/2019
// -----------------------------------------------------------------------------

// (re)Export modules
pub mod lib;
pub use lib::*;

// Include dependencies
mod day01;
mod day02;
mod day03;
mod day04;

/// Run all 2019 days's puzzles
pub fn run (day: u32, index: u32, key: &str, verbose: bool) {
  if (day == 0) || (day == 1) { day01::run(index, key, verbose); }
  if (day == 0) || (day == 2) { day02::run(index, key, verbose); }
  if (day == 0) || (day == 3) { day03::run(index, key, verbose); }
  if (day == 0) || (day == 4) { day04::run(index, key, verbose); }
}
