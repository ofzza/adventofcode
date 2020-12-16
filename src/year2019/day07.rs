//! 2019/07 puzzle
//! 
//! https://adventofcode.com/2019/day/7
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
      let input = PuzzleInput::Vector1D(vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0]);
      stats.update(
        Puzzle::new(2019, 7, 1, "test", input, implementation1, |n| (n, Some(43210)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::Vector1D(vec![3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99, 0, 0]);
      stats.update(
        Puzzle::new(2019, 7, 1, "test", input, implementation1, |n| (n, Some(54321)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::Vector1D(vec![3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0]);
      stats.update(
        Puzzle::new(2019, 7, 1, "test", input, implementation1, |n| (n, Some(65210)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<i64>(load_input("./src/year2019/data/day07input.txt"), ",");
      stats.update(
        Puzzle::new(2019, 7, 1, "solution", input, implementation1, |n| (n, Some(34686)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Run puzzle
  if (index == 0) || (index == 2) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector1D(vec![3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28, 1005, 28, 6, 99, 0, 0, 5]);
      stats.update(
        Puzzle::new(2019, 7, 2, "test", input, implementation2, |n| (n, Some(139629729)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::Vector1D(vec![
        3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
        -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
        53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10
      ]);
      stats.update(
        Puzzle::new(2019, 7, 2, "test", input, implementation2, |n| (n, Some(18216)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<i64>(load_input("./src/year2019/data/day07input.txt"), ",");
      stats.update(
        Puzzle::new(2019, 7, 2, "solution", input, implementation2, |n| (n, Some(36384144)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<i64, i64, i64>, verbose: bool) -> Result<i64, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(code) => find_max_phase(code, 0, 5, false, verbose),
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<i64, i64, i64>, verbose: bool) -> Result<i64, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(code) => find_max_phase(code, 5, 10, true, verbose),
    _ => panic!("This shouldn't ever happen!")
  }
}

fn find_max_phase (code: &Vec<i64>, min_phase: i64, max_phase: i64, feedback: bool, verbose: bool) -> Result<i64, &str> {
  // Initialize max value
  let mut max_value = 0;
  // Find max value
  for i in min_phase..max_phase {
    for j in min_phase..max_phase {
      // Skip duplicate phases
      if j == i { continue; }
      for k in min_phase..max_phase {
        // Skip duplicate phases
        if k == i || k == j { continue; }
        for l in min_phase..max_phase {
          // Skip duplicate phases
          if l == i || l == j || l == k { continue; }
          for m in min_phase..max_phase {
            // Skip duplicate phases
            if m == i || m == j || m == k || m == l { continue; }
            // Compose phases for testing
            let phases = vec![i, j, k, l, m];
            // If verbose, output phase setting being tested
            if verbose {
              println!("Testing phases: {:?}", phases);
            }
            // Run for phase settings
            match run_cascade(code, phases, feedback, verbose) {
              Ok(value) => {
                if value > max_value {
                  max_value = value;
                }
              },
              _ => panic!("This shouldn't ever happen!")
            }
          }
        }
      }
    }
  }
  // Return max value
  return Ok(max_value);
}

fn run_cascade (code: &Vec<i64>, phases: Vec<i64>, feedback: bool, verbose: bool) -> Result<i64, &str> {
  // Initialize IntCode computers and inputs
  let mut computers: Vec<IntCode> = Vec::new();
  let mut input_stacks: Vec<Vec<i64>> = Vec::new();
  for i in 0..5 {
    computers.push(IntCode::new(code.clone()));
    input_stacks.push(vec![phases[i]]);
    if i == 0 {
      input_stacks[i].push(0);
    }
  }  
  // Run computers
  loop {
    for i in 0..5 {
      match computers[i].next(false) {
        OpCodeResult::ExecutedWithoutOutput => {
          // Proceed with execution
          continue;
        },
        OpCodeResult::ExecutionProducedOutput(output) => {
          // If verbose, output processing status
          if verbose {
            println!("  Computer {} outputted {}", i, output);
          }
          // Pipe output
          if i != 4 {
            // Pipe output to next computer's input stack
            input_stacks[i + 1].push(output);
            // Proceed with execution
            continue;
          } else if feedback {
            // Pipe output back to 1st computer
            input_stacks[0].push(output);
            // Proceed with execution
            continue;
          } else {
            // Proceed with execution
            continue;
          }
        },
        OpCodeResult::ExecutionRequestedInput => {
          // Check if input available
          if input_stacks[i].len() > 0 {
            // Set input and remove from input stack
            let value = input_stacks[i].remove(0);
            computers[i].input = Some(value);
            // If verbose, output processing status
            if verbose {
              println!("  Computer {} reads input {}", i, value);
            }
          }
          // Proceed with execution
          continue;
        },
        OpCodeResult::End => {
          // Pipe final computer's final output
          if i == 4 {
            match computers[i].output {
              Some(output) => {
                // If verbose, output processing status
                if verbose {
                  println!("  Computer {} ended with FINAL output {}", i, output);
                }
                // Return final output value
                return Ok(output);
              },
              _ => panic!("This shouldn't ever happen!")
            };
          } else {
            // If verbose, output processing status
            if verbose {
              println!("  Computer {} ended execution", i);
            }
          }
        },
        OpCodeResult::Error(_) => {
          // Execution error
          return Err("IntCode exited with error!");
        }
      }
    }
  };
}
