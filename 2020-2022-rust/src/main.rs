// Advent of code (adventofcode.com)
// https://adventofcode.com/
// -----------------------------------------------------------------------------

// Load shared libraries
#![allow(special_module_name)]
mod lib;

// Load puzzles
mod demo;
mod year;
mod year2021;
mod year2022;

// Include dependencies
use std::env;
use std::time::Instant;
use lib::vargs::*;
use lib::input::*;
use lib::puzzle::*;

/// Program entry point
/// 
/// # Can be run with startup arguments
/// * `--year`        - Only executes puzzles marked with same year  (Set 0 or omit for all years)
/// * `--day`         - Only executes puzzles marked with same day   (Set 0 or omit for all days)
/// * `--index`       - Only executes puzzles marked with same index (Set 0 or omit for all indices)
/// * `--tag`        - Only executes puzzles marked with same tag  (Leave empty or omit for all tags)
/// * `--input-file`  - Path to the input data file (Leave empty if you want to use explicit input value)
/// * `--input-value` - Explicit input data (Leave empty if you want to use data from the input file instead)
/// * `--expect`      - Expected result for the puzzle to output (Leave empty or omit for no expected result)
/// * `--verbose`     - If any output apart from he result should be displayed
/// * `--obfuscate`   - If the final result should be obfuscated
fn main() {
  
  // Get arguments
  let args = VArgs::new(env::args().collect());

  // Register puzzles
  let mut registry = PuzzleRegistry::new();
  // Register demo puzzle
  registry = demo::init(registry);
  // Register 2021 puzzles
  if args.puzzle.year == 0 || args.puzzle.year == 2021 {
    registry = year2021::init(registry);
  }
  // Register 2022 puzzles
  if args.puzzle.year == 0 || args.puzzle.year == 2022 {
    registry = year2022::init(registry);
  }

  // Run all puzzles just for fun
  for i in 0..registry.puzzles.values().len() {
    // Fetch puzzle
    let info = &registry.puzzles.keys().nth(i).unwrap();
    let f = registry.puzzles.values().nth(i).unwrap();
    // Check if puzzle matches startup criteria
    if args.puzzle.matches(info) {
      // Time function execution
      let start_input = Instant::now();
      // Load puzzle input
      let input = if !args.input_value.is_empty() {
          args.input_value.clone()
        }
        else if !args.input_file.is_empty() {
          let year  = format!("{:0>4}", info.year);
          let day   = format!("{:0>2}", info.day);
          let index = format!("{:0>2}", info.index);
          let tag   = info.tag.as_str();
          Input::read_file(
            &args.input_file
              .replace("[:year]",  year.to_string().as_str())
              .replace("[:day]",   day.to_string().as_str())
              .replace("[:index]", index.to_string().as_str())
              .replace("[:tag]",   tag)
          )
        }
        else {
          String::default()
        };
      // Run puzzle
      PuzzleRegistry::execute(info, start_input, f, input, &args);
    }
  }
}
