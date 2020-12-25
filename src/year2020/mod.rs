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

// Include dependencies
use super::lib::puzzle::*;

/// Run all 2019 days's puzzles
pub fn run (day: u32, index: u32, key: &str, verbose: bool, obfuscate: bool) -> PuzzleExecutionStatistics {
  // Initialize stats
  let mut stats = PuzzleExecutionStatistics{
    ..Default::default()
  };
  // Run puzzles
  if (day == 0) || (day ==  1) { stats.update(day01::run(index, key, verbose, obfuscate)); }
  if (day == 0) || (day ==  2) { stats.update(day02::run(index, key, verbose, obfuscate)); }
  if (day == 0) || (day ==  3) { stats.update(day03::run(index, key, verbose, obfuscate)); }
  if (day == 0) || (day ==  4) { stats.update(day04::run(index, key, verbose, obfuscate)); }
  if (day == 0) || (day ==  5) { stats.update(day05::run(index, key, verbose, obfuscate)); }
  if (day == 0) || (day ==  6) { stats.update(day06::run(index, key, verbose, obfuscate)); }
  if (day == 0) || (day ==  7) { stats.update(day07::run(index, key, verbose, obfuscate)); }
  if (day == 0) || (day ==  8) { stats.update(day08::run(index, key, verbose, obfuscate)); }
  if (day == 0) || (day ==  9) { stats.update(day09::run(index, key, verbose, obfuscate)); }
  if (day == 0) || (day == 10) { stats.update(day10::run(index, key, verbose, obfuscate)); }
  if (day == 0) || (day == 11) { stats.update(day11::run(index, key, verbose, obfuscate)); }
  if (day == 0) || (day == 12) { stats.update(day12::run(index, key, verbose, obfuscate)); }
  if (day == 0) || (day == 13) { stats.update(day13::run(index, key, verbose, obfuscate)); }
  if (day == 0) || (day == 14) { stats.update(day14::run(index, key, verbose, obfuscate)); }
  if (day == 0) || (day == 15) { stats.update(day15::run(index, key, verbose, obfuscate)); }
  if (day == 0) || (day == 16) { stats.update(day16::run(index, key, verbose, obfuscate)); }
  if (day == 0) || (day == 17) { stats.update(day17::run(index, key, verbose, obfuscate)); }
  if (day == 0) || (day == 18) { stats.update(day18::run(index, key, verbose, obfuscate)); }
  if (day == 0) || (day == 19) { stats.update(day19::run(index, key, verbose, obfuscate)); }
  if (day == 0) || (day == 20) { stats.update(day20::run(index, key, verbose, obfuscate)); }
  if (day == 0) || (day == 21) { stats.update(day21::run(index, key, verbose, obfuscate)); }
  if (day == 0) || (day == 22) { stats.update(day22::run(index, key, verbose, obfuscate)); }
  if (day == 0) || (day == 23) { stats.update(day23::run(index, key, verbose, obfuscate)); }
  if (day == 0) || (day == 24) { stats.update(day24::run(index, key, verbose, obfuscate)); }
  if (day == 0) || (day == 25) { stats.update(day25::run(index, key, verbose, obfuscate)); }
  // Return composed stats
  return stats;
}
