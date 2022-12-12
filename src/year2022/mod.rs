//! 2022 puzzles
//! 
//! https://adventofcode.com/2022
// -----------------------------------------------------------------------------

// Load child modules
pub mod lib;
pub use lib::*;

// Include dependencies
use crate::lib::puzzle::*;

// Import child modules
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;

/// Registers year runner
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  registry = day01::init(registry);
  registry = day02::init(registry);
  registry = day03::init(registry);
  registry = day04::init(registry);
  registry = day05::init(registry);
  registry = day06::init(registry);
  registry = day07::init(registry);
  registry = day08::init(registry);
  registry = day09::init(registry);
  registry = day10::init(registry);
  registry = day11::init(registry);
  registry = day12::init(registry);
  registry = day13::init(registry);
  registry
}
