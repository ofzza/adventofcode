//! 2019/09 puzzle
//! 
//! https://adventofcode.com/2019/day/9
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
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::ParamsVector1D(vec![], vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]);
      stats.update(
        Puzzle::new(2019, 9, 1, "test", input, implementation, |outputs| (outputs, Some(vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99])))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamsVector1D(vec![], vec![1102,34915192,34915192,7,4,7,99,0]);
      stats.update(
        Puzzle::new(2019, 9, 1, "test", input, implementation, |outputs| (outputs, Some(vec![1219070632396864])))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamsVector1D(vec![], vec![104,1125899906842624,99]);
      stats.update(
        Puzzle::new(2019, 9, 1, "test", input, implementation, |outputs| (outputs, Some(vec![1125899906842624])))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = match parse_1d::<i64>(load_input("./src/year2019/data/day09input.txt"), ",") {
        PuzzleInput::Vector1D(code) => PuzzleInput::ParamsVector1D(vec![1], code),
        _ => panic!("This should never, ever happen!")
      };
      stats.update(
        Puzzle::new(2019, 9, 1, "solution", input, implementation, |outputs| (outputs, Some(vec![3601950151])))
          .run(verbose, obfuscate)
      );
    }
  }

  // Run puzzle
  if (index == 0) || (index == 2) {
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = match parse_1d::<i64>(load_input("./src/year2019/data/day09input.txt"), ",") {
        PuzzleInput::Vector1D(code) => PuzzleInput::ParamsVector1D(vec![2], code),
        _ => panic!("This should never, ever happen!")
      };
      stats.update(
        Puzzle::new(2019, 9, 2, "solution", input, implementation, |outputs| (outputs, Some(vec![64236])))
          .run(verbose, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation (puzzle: &Puzzle<i64, Vec<i64>, Vec<i64>>, verbose: bool) -> Result<Vec<i64>, &str> {
  match &puzzle.input {
    PuzzleInput::ParamsVector1D(inputs, code) => {
      // Initialize IntCode
      let mut computer = IntCode::new(code.clone());
      let mut inputs = inputs.clone();
      let mut outputs: Vec<i64> = Vec::new();
      // Run IntCode
      loop {
        match computer.next(verbose) {
          OpCodeResult::ExecutedWithoutOutput => {
            // Proceed with execution
            continue;
          },
          OpCodeResult::ExecutionProducedOutput(output) => {
            // Store output
            outputs.push(output);
            // Proceed with execution
            continue;
          },
          OpCodeResult::ExecutionRequestedInput => {
            // Set input
            computer.input = Some(inputs.remove(0));
            // Proceed with execution
            continue;
          },
          OpCodeResult::End => {
            // Return result
            return Ok(outputs)
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
