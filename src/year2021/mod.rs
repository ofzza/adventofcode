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
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

/// Registers year runner
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  registry = demo::init(registry);
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
  registry = day14::init(registry);
  registry = day15::init(registry);
  registry = day16::init(registry);
  registry = day17::init(registry);
  registry = day18::init(registry);
  registry = day19::init(registry);
  registry = day20::init(registry);
  registry = day21::init(registry);
  registry = day22::init(registry);
  registry = day23::init(registry);
  registry = day24::init(registry);
  registry = day25::init(registry);
  registry
}
