// Advent of code (adventofcode.com)
// https://adventofcode.com/
// -----------------------------------------------------------------------------

// Include dependencies
use std::env;
pub mod lib;
use lib::puzzle::*;
mod year2019;

/// Program entry point
/// 
/// # Startup arguments
/// 
/// * `--year`    - Only executes puzzles marked with same year  (Set 0 or ommit for all years)
/// * `--day`     - Only executes puzzles marked with same day   (Set 0 or ommit for all years)
/// * `--index`   - Only executes puzzles marked with same index (Set 0 or ommit for all years)
/// * `--verbose` - Outputs executing output of the puzzle to the console
fn main() {

  // Get arguments
  let args: Vec<String> = env::args().collect();

  // Match arguments
  let mut year: u32     = 0;
  let mut day:u32       = 0;
  let mut index: u32    = 0;
  let mut key: String   = String::default();
  let mut verbose: bool = false;
  
  for i in 0..args.len() {
    // Get year argument
    if (args[i] == "--year") && (args.len() > i) {
      year = args[i + 1].parse::<u32>().expect("Failed parsing '--year' parameter - expecting a positive, while number!");
    }
    // Get day argument
    if (args[i] == "--day") && (args.len() > i) {
      day = args[i + 1].parse::<u32>().expect("Failed parsing '--day' parameter - expecting a positive, while number!");
    }
    // Get index argument
    if (args[i] == "--index") && (args.len() > i) {
      index = args[i + 1].parse::<u32>().expect("Failed parsing '--index' parameter - expecting a positive, while number!");
    }
    // Get key argument
    if (args[i] == "--key") && (args.len() > i) {
      key = args[i + 1].clone();
    }
    // Get verbose argument
    if args[i] == "--verbose" {
      verbose = true;
    }
  }

  // Register all puzzles
  let mut puzzles: Vec<Puzzle> = Vec::new();
  year2019::register(&mut puzzles);

  // Execute puzzles
  for puzzle in puzzles.iter_mut() {
    // Check if puzzle matches requested parameters
    let match_year  = (year  == 0)                 || (puzzle.year == year);
    let match_day   = (day   == 0)                 || (puzzle.day == day);
    let match_index = (index == 0)                 || (puzzle.index == index);
    let match_key   = (key   == String::default()) || (puzzle.key.eq(&key));
    if match_year && match_day && match_index && match_key {
      puzzle.run(verbose);
    }
  }

}
