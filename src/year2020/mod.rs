//! 2020 puzzles
//! 
//! https://adventofcode.com/2020
// -----------------------------------------------------------------------------

// (re)Export modules
pub mod lib;
pub use lib::*;

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

// Include dependencies
use super::lib::puzzle::*;

/// Run all 2019 days's puzzles
pub fn run (day: u32, index: u32, key: &str, verbose: bool) -> PuzzleExecutionStatitics {
  // Initialize stats
  let mut stats = PuzzleExecutionStatitics{
    ..Default::default()
  };
  // Run puzzles
  if (day == 0) || (day ==  1) { stats.update(day01::run(index, key, verbose)); }
  if (day == 0) || (day ==  2) { stats.update(day02::run(index, key, verbose)); }
  if (day == 0) || (day ==  3) { stats.update(day03::run(index, key, verbose)); }
  if (day == 0) || (day ==  4) { stats.update(day04::run(index, key, verbose)); }
  if (day == 0) || (day ==  5) { stats.update(day05::run(index, key, verbose)); }
  if (day == 0) || (day ==  6) { stats.update(day06::run(index, key, verbose)); }
  if (day == 0) || (day ==  7) { stats.update(day07::run(index, key, verbose)); }
  if (day == 0) || (day ==  8) { stats.update(day08::run(index, key, verbose)); }
  if (day == 0) || (day ==  9) { stats.update(day09::run(index, key, verbose)); }
  if (day == 0) || (day == 10) { stats.update(day10::run(index, key, verbose)); }
  if (day == 0) || (day == 11) { stats.update(day11::run(index, key, verbose)); }
  if (day == 0) || (day == 12) { stats.update(day12::run(index, key, verbose)); }
  // Return composed stats
  return stats;
}
