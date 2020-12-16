//! 2019/11 puzzle
//! 
//! https://adventofcode.com/2019/day/11
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::inputs::*;
use crate::lib::puzzle::*;

/// Registers puzzles for the day
pub fn run (index: u32, key: &str, verbose: bool, obfuscate: bool) -> PuzzleExecutionStatistics {

  // Initialize stats
  let mut stats = PuzzleExecutionStatistics{
    ..Default::default()
  };

  // Run puzzle
  if (index == 0) || (index == 1) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector1D(vec![0]);
      stats.update(
        Puzzle::new(2019, 11, 1, "test", input, implementation1, |n| (n, None))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<i64>(load_input("./src/year2019/data/day11input.txt"), ",");
      stats.update(
        Puzzle::new(2019, 11, 1, "solution", input, implementation1, |n| (n, None))
          .run(verbose, obfuscate)
      );
    }
  }

  // Run puzzle
  if (index == 0) || (index == 2) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector1D(vec![0]);
      stats.update(
        Puzzle::new(2019, 11, 2, "test", input, implementation1, |n| (n, None))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<i64>(load_input("./src/year2019/data/day11input.txt"), ",");
      stats.update(
        Puzzle::new(2019, 11, 2, "solution", input, implementation2, |n| (n, None))
          .run(verbose, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<i64, i64, i64>, _verbose: bool) -> Result<i64, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(_code) => Ok(0),
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<i64, i64, i64>, _verbose: bool) -> Result<i64, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(_code) => Ok(0),
    _ => panic!("This shouldn't ever happen!")
  }
}
