// Advent of code (adventofcode.com)
// https://adventofcode.com/
// -----------------------------------------------------------------------------

// Import child modules
pub mod lib;
mod year2019;
mod year2020;

// Include dependencies
use std::env;
use lib::puzzle::*;
use lib::console::*;

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
  let mut year: u32       = 0;
  let mut day:u32         = 0;
  let mut index: u32      = 0;
  let mut key: String     = String::default();
  let mut verbose: bool   = false;
  let mut obfuscate: bool = false;
  
  for i in 0..args.len() {
    // Get year argument
    if (args[i] == "--year") && (args.len() > i) {
      year = args[i + 1].parse::<u32>().expect("Failed parsing '--year' parameter - expecting a positive, whole number!");
    }
    // Get day argument
    if (args[i] == "--day") && (args.len() > i) {
      day = args[i + 1].parse::<u32>().expect("Failed parsing '--day' parameter - expecting a positive, whole number!");
    }
    // Get index argument
    if (args[i] == "--index") && (args.len() > i) {
      index = args[i + 1].parse::<u32>().expect("Failed parsing '--index' parameter - expecting a positive, whole number!");
    }
    // Get key argument
    if (args[i] == "--key") && (args.len() > i) {
      key = args[i + 1].clone();
    }
    // Get verbose argument
    if args[i] == "--verbose" {
      verbose = true;
    }
    // Get obfuscate argument
    if args[i] == "--obfuscate" {
      obfuscate = true;
    }
  }

  // Run all years' puzzles
  let mut stats = PuzzleExecutionStatistics{
    ..Default::default()
  };
  if (year == 0) || (year == 2019) {
    stats.update(year2019::run(day, index, key.as_str(), verbose, obfuscate));
  }
  if (year == 0) || (year == 2020) {
    stats.update(year2020::run(day, index, key.as_str(), verbose, obfuscate));
  }

  // Output collected stats
  println!();
  println!("{}{}---------------------------------------------------{}",
    CONSOLE_TITLE_BG, CONSOLE_TITLE_FG, CONSOLE_RESET);
  
  if stats.successful_count == stats.total_count {
    println!("{}{}STATISTICS [SUCCESS]:{}",
    CONSOLE_SUCCESS_BG, CONSOLE_SUCCESS_FG, CONSOLE_RESET);
  } else if stats.failed_count != 0 {
    println!("{}{}STATISTICS [FAIL]:{}",
    CONSOLE_FAIL_BG, CONSOLE_FAIL_FG, CONSOLE_RESET);
  } else {
    println!("{}{}STATISTICS:{}",
    CONSOLE_RESULT_BG, CONSOLE_RESULT_FG, CONSOLE_RESET);
  }

  println!("{}{}Successful   puzzles: {}/{}{}",
    CONSOLE_SUBTITLE_BG, CONSOLE_SUBTITLE_FG,
    stats.successful_count, stats.total_count,
    CONSOLE_RESET);
  println!("{}{}Failed       puzzles: {}/{}{}",
    CONSOLE_SUBTITLE_BG, CONSOLE_SUBTITLE_FG,
    stats.failed_count, stats.total_count,
    CONSOLE_RESET);
  println!("{}{}Undetermined puzzles: {}/{}{}",
    CONSOLE_SUBTITLE_BG, CONSOLE_SUBTITLE_FG,
    stats.total_count - (stats.successful_count + stats.failed_count), stats.total_count,
    CONSOLE_RESET);
  println!("{}{}Total execution time: {} sec{}",
    CONSOLE_SUBTITLE_BG, CONSOLE_SUBTITLE_FG,
    stats.execution_time,
    CONSOLE_RESET);

}
