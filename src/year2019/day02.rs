//! 2019/02 puzzle
//! 
//! https://adventofcode.com/2019/day/2
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
      let input = PuzzleInput::Vector1D(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
      stats.update(
        Puzzle::new(2019, 2, 1, "test", input, implementation1, |c| (c.memory[0], Some(3500)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::Vector1D(vec![1, 0, 0, 0, 99]);
      stats.update(
        Puzzle::new(2019, 2, 1, "test", input, implementation1, |c| (c.memory[0], Some(2)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::Vector1D(vec![2, 3, 0, 3, 99]);
      stats.update(
        Puzzle::new(2019, 2, 1, "test", input, implementation1, |c| (c.memory[3], Some(6)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::Vector1D(vec![2, 4, 4, 5, 99, 0]);
      stats.update(
        Puzzle::new(2019, 2, 1, "test", input, implementation1, |c| (c.memory[5], Some(9801)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::Vector1D(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
      stats.update(
        Puzzle::new(2019, 2, 1, "test", input, implementation1, |c| (c.memory[0], Some(30)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = match &mut parse_1d::<i64>(load_input("./src/year2019/data/day02input.txt"), ",") {
        PuzzleInput::Vector1D(input) => {
          input[1] = 12;
          input[2] = 2;
          PuzzleInput::Vector1D(input.clone())
        },
        _ => panic!("This should never, ever happen!")
      };
      stats.update(
        Puzzle::new(2019, 2, 1, "solution", input, implementation1, |c| (c.memory[0], Some(3101878)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Run puzzle
  if (index == 0) || (index == 2) {
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<i64>(load_input("./src/year2019/data/day02input.txt"), ",");
      stats.update(
        Puzzle::new(2019, 2, 2, "solution", input, implementation2, |c| ((100 * c.memory[1] + c.memory[2]), Some(8444)))
          .run(false, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<i64, IntCode, i64>, verbose: bool) -> Result<IntCode, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(code) => {
      // Initialize IntCode
      let mut computer = IntCode::new(code.clone());
      // Run IntCode
      loop {
        match computer.next(verbose) {
          OpCodeResult::ExecutedWithoutOutput => {
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
          },
          _ => {
            // Unsupported result error
            return Err("IntCode execution produced an unexpected result!");
          }
        }
      }
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<i64, IntCode, i64>, verbose: bool) -> Result<IntCode, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(code) => {
      for i in 0..99 {
        for j in 0..99 {
          // Update memory
          let mut memory = code.clone();
          memory[1] = i;
          memory[2] = j;
          // Initialize IntCode
          let mut computer = IntCode::new(memory);
          // Run IntCode
          loop {
            match computer.next(verbose) {
              OpCodeResult::ExecutedWithoutOutput => continue,
              OpCodeResult::End => {
                if computer.memory[0] == 19690720 {
                  // Return as done
                  return Ok(computer);
                } else {
                  // Execution done - look for next candidate
                  break;
                }
              },
              OpCodeResult::Error(_) => {
                // Execution error
                return Err("IntCode exited with error!");
              },
              _ => {
                // Unsupported result error
                return Err("IntCode execution produced an unexpected result!")
              }
            }
          }
        }
      }
      // Return no matches found error
      return Err("Execution finished with no matches found!");
    },
    _ => panic!("This shouldn't ever happen!")
  }
}
