//! 2020/18 puzzle
//! 
//! https://adventofcode.com/2020/day/18
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::inputs::*;
use crate::lib::puzzle::*;
use super::mathish::Mathish;

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
      let input = PuzzleInput::Vector1D(vec![String::from("2 * 3 + (4 * 5)")]);
      stats.update(
        Puzzle::new(2020, 18, 1, "test", input, implementation1, |r| (r, Some(26)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::Vector1D(vec![String::from("5 + (8 * 3 + 9 + 3 * 4 * 3)")]);
      stats.update(
        Puzzle::new(2020, 18, 1, "test", input, implementation1, |r| (r, Some(437)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::Vector1D(vec![String::from("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")]);
      stats.update(
        Puzzle::new(2020, 18, 1, "test", input, implementation1, |r| (r, Some(12240)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::Vector1D(vec![String::from("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")]);
      stats.update(
        Puzzle::new(2020, 18, 1, "test", input, implementation1, |r| (r, Some(13632)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<String>(load_input("./src/year2020/data/day18input.txt"), "\n");
      stats.update(
        Puzzle::new(2020, 18, 1, "solution", input, implementation1, |r| (r, Some(5783053349377)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Run puzzle
  if (index == 0) || (index == 2) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector1D(vec![String::from("1 + (2 * 3) + (4 * (5 + 6))")]);
      stats.update(
        Puzzle::new(2020, 18, 2, "test", input, implementation2, |r| (r, Some(51)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::Vector1D(vec![String::from("2 * 3 + (4 * 5)")]);
      stats.update(
        Puzzle::new(2020, 18, 2, "test", input, implementation2, |r| (r, Some(46)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::Vector1D(vec![String::from("5 + (8 * 3 + 9 + 3 * 4 * 3)")]);
      stats.update(
        Puzzle::new(2020, 18, 2, "test", input, implementation2, |r| (r, Some(1445)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::Vector1D(vec![String::from("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")]);
      stats.update(
        Puzzle::new(2020, 18, 2, "test", input, implementation2, |r| (r, Some(669060)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::Vector1D(vec![String::from("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")]);
      stats.update(
        Puzzle::new(2020, 18, 2, "test", input, implementation2, |r| (r, Some(23340)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<String>(load_input("./src/year2020/data/day18input.txt"), "\n");
      stats.update(
        Puzzle::new(2020, 18, 2, "solution", input, implementation2, |r| (r, Some(74821486966872)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<String, usize, usize>, verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(expressions) => {
      // Evaluate expressions
      let mut results: Vec<usize> = vec![];
      for expression in expressions {
        // Prompt expression
        if verbose {
          println!("eval: {}", expression);
        }
        // Evaluate expression
        results.push(Mathish::eval(expression.clone(), vec![], verbose));
      }

      // Sum results
      let mut sum = 0;
      for result in results {
        sum += result;
      }

      // Return sum
      return Ok(sum);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<String, usize, usize>, verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(expressions) => {
      // Evaluate expressions
      let mut results: Vec<usize> = vec![];
      for expression in expressions {
        // Prompt expression
        if verbose {
          println!("eval: {}", expression);
        }
        // Evaluate expression
        results.push(Mathish::eval(expression.clone(), vec!['+', '*'], verbose));
      }

      // Sum results
      let mut sum = 0;
      for result in results {
        sum += result;
      }

      // Return sum
      return Ok(sum);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}
