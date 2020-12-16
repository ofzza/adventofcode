//! 2019/05 puzzle
//! 
//! https://adventofcode.com/2019/day/5
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::inputs::*;
use crate::lib::puzzle::*;
use super::intcode::*;

/// Registers puzzles for the day
pub fn run (index: u32, key: &str, verbose: bool, obfuscate: bool) -> PuzzleExecutionStatistics {

  // Initialize stats
  let mut stats = PuzzleExecutionStatistics{
    ..Default::default()
  };
  
  // Run puzzle
  if (index == 0) || (index == 1) {
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = match parse_1d::<i64>(load_input("./src/year2019/data/day05input.txt"), ",") {
        PuzzleInput::Vector1D(code) => PuzzleInput::ParamVector1D(1, code),
        _ => panic!("This should never, ever happen!")
      };
      stats.update(
        Puzzle::new(2019, 5, 1, "solution", input, implementation, |c| (match c.output { Some(output) => output, None => 0 }, Some(7286649)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Run puzzle
  if (index == 0) || (index == 2) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::ParamVector1D(0, vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
      stats.update(
        Puzzle::new(2019, 5, 2, "test", input, implementation, |c| (match c.output { Some(output) => output, None => 0 }, Some(0)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamVector1D(8, vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
      stats.update(
        Puzzle::new(2019, 5, 2, "test", input, implementation, |c| (match c.output { Some(output) => output, None => 0 }, Some(1)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamVector1D(0, vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
      stats.update(
        Puzzle::new(2019, 5, 2, "test", input, implementation, |c| (match c.output { Some(output) => output, None => 0 }, Some(1)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamVector1D(8, vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
      stats.update(
        Puzzle::new(2019, 5, 2, "test", input, implementation, |c| (match c.output { Some(output) => output, None => 0 }, Some(0)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamVector1D(0, vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
      stats.update(
        Puzzle::new(2019, 5, 2, "test", input, implementation, |c| (match c.output { Some(output) => output, None => 0 }, Some(0)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamVector1D(8, vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
      stats.update(
        Puzzle::new(2019, 5, 2, "test", input, implementation, |c| (match c.output { Some(output) => output, None => 0 }, Some(1)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamVector1D(0, vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
      stats.update(
        Puzzle::new(2019, 5, 2, "test", input, implementation, |c| (match c.output { Some(output) => output, None => 0 }, Some(1)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamVector1D(8, vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
      stats.update(
        Puzzle::new(2019, 5, 2, "test", input, implementation, |c| (match c.output { Some(output) => output, None => 0 }, Some(0)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamVector1D(0, vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9]);
      stats.update(
        Puzzle::new(2019, 5, 2, "test", input, implementation, |c| (match c.output { Some(output) => output, None => 0 }, Some(0)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamVector1D(1, vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9]);
      stats.update(
        Puzzle::new(2019, 5, 2, "test", input, implementation, |c| (match c.output { Some(output) => output, None => 0 }, Some(1)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamVector1D(0, vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
      stats.update(
        Puzzle::new(2019, 5, 2, "test", input, implementation, |c| (match c.output { Some(output) => output, None => 0 }, Some(0)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamVector1D(1, vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
      stats.update(
        Puzzle::new(2019, 5, 2, "test", input, implementation, |c| (match c.output { Some(output) => output, None => 0 }, Some(1)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamVector1D(7, vec![3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
                                                    1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
                                                    999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99]);
      stats.update(
        Puzzle::new(2019, 5, 2, "test", input, implementation, |c| (match c.output { Some(output) => output, None => 0 }, Some(999)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamVector1D(8, vec![3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
                                                    1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
                                                    999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99]);
      stats.update(
        Puzzle::new(2019, 5, 2, "test", input, implementation, |c| (match c.output { Some(output) => output, None => 0 }, Some(1000)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamVector1D(9, vec![3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
                                                    1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
                                                    999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99]);
      stats.update(
        Puzzle::new(2019, 5, 2, "test", input, implementation, |c| (match c.output { Some(output) => output, None => 0 }, Some(1001)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = match parse_1d::<i64>(load_input("./src/year2019/data/day05input.txt"), ",") {
        PuzzleInput::Vector1D(code) => PuzzleInput::ParamVector1D(5, code),
        _ => panic!("This should never, ever happen!")
      };
      stats.update(
        Puzzle::new(2019, 5, 2, "solution", input, implementation, |c| (match c.output { Some(output) => output, None => 0 }, Some(15724522)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation (puzzle: &Puzzle<i64, IntCode, i64>, verbose: bool) -> Result<IntCode, &str> {
  match &puzzle.input {
    PuzzleInput::ParamVector1D(input, code) => {
      // Initialize IntCode
      let mut computer = IntCode::new(code.clone());
      // Run IntCode
      loop {
        match computer.next(verbose) {
          OpCodeResult::ExecutedWithoutOutput => {
            // Proceed with execution
            continue;
          },
          OpCodeResult::ExecutionProducedOutput(_output) => {
            // Proceed with execution
            continue;
          },
          OpCodeResult::ExecutionRequestedInput => {
            // Set input
            computer.input = Some(input.clone());
            // Proceed with execution
            continue;
          },
          OpCodeResult::End => {
            // Return result
            return Ok(computer)
          },
          OpCodeResult::Error(_) => {
            // Execution error
            return Err("IntCode exited with error!");
          }
        }
      }
    },
    _ => panic!("This should never, ever happen!")
  }
}
