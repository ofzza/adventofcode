//! 2020/15 puzzle
//! 
//! https://adventofcode.com/2020/day/15
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::inputs::*;
use crate::lib::puzzle::*;
// use std::collections::HashMap;

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
      let input = PuzzleInput::ParamVector1D(2020, vec![0,3,6]);
      stats.update(
        Puzzle::new(2020, 15, 1, "test", input, implementation1, |r| (r, Some(436)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamVector1D(2020, vec![1,3,2]);
      stats.update(
        Puzzle::new(2020, 15, 1, "test", input, implementation1, |r| (r, Some(1)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamVector1D(2020, vec![2,1,3]);
      stats.update(
        Puzzle::new(2020, 15, 1, "test", input, implementation1, |r| (r, Some(10)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamVector1D(2020, vec![1,2,3]);
      stats.update(
        Puzzle::new(2020, 15, 1, "test", input, implementation1, |r| (r, Some(27)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamVector1D(2020, vec![2,3,1]);
      stats.update(
        Puzzle::new(2020, 15, 1, "test", input, implementation1, |r| (r, Some(78)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamVector1D(2020, vec![3,2,1]);
      stats.update(
        Puzzle::new(2020, 15, 1, "test", input, implementation1, |r| (r, Some(438)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamVector1D(2020, vec![3,1,2]);
      stats.update(
        Puzzle::new(2020, 15, 1, "test", input, implementation1, |r| (r, Some(1836)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = match parse_1d::<usize>(load_input("./src/year2020/data/day15input.txt"), ",") {
        PuzzleInput::Vector1D(code) => PuzzleInput::ParamVector1D(2020, code),
        _ => panic!("This should never, ever happen!")
      };
      stats.update(
        Puzzle::new(2020, 15, 1, "solution", input, implementation1, |r| (r, Some(1696)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Run puzzle
  if (index == 0) || (index == 2) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::ParamVector1D(30000000, vec![0,3,6]);
      stats.update(
        Puzzle::new(2020, 15, 2, "test", input, implementation2, |r| (r, Some(175594)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamVector1D(30000000, vec![1,3,2]);
      stats.update(
        Puzzle::new(2020, 15, 2, "test", input, implementation2, |r| (r, Some(2578)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamVector1D(30000000, vec![2,1,3]);
      stats.update(
        Puzzle::new(2020, 15, 2, "test", input, implementation2, |r| (r, Some(3544142)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamVector1D(30000000, vec![1,2,3]);
      stats.update(
        Puzzle::new(2020, 15, 2, "test", input, implementation2, |r| (r, Some(261214)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamVector1D(30000000, vec![2,3,1]);
      stats.update(
        Puzzle::new(2020, 15, 2, "test", input, implementation2, |r| (r, Some(6895259)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamVector1D(30000000, vec![3,2,1]);
      stats.update(
        Puzzle::new(2020, 15, 2, "test", input, implementation2, |r| (r, Some(18)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamVector1D(30000000, vec![3,1,2]);
      stats.update(
        Puzzle::new(2020, 15, 2, "test", input, implementation2, |r| (r, Some(362)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = match parse_1d::<usize>(load_input("./src/year2020/data/day15input.txt"), ",") {
        PuzzleInput::Vector1D(code) => PuzzleInput::ParamVector1D(30000000, code),
        _ => panic!("This should never, ever happen!")
      };
      stats.update(
        Puzzle::new(2020, 15, 2, "solution", input, implementation2, |r| (r, Some(37385)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<usize, usize, usize>, verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::ParamVector1D(index, numbers) => {
      let number = generate_numbers(index.clone(), numbers, verbose);
      return Ok(number);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<usize, usize, usize>, verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::ParamVector1D(index, numbers) => {
      let number = generate_numbers(index.clone(), numbers, verbose);
      return Ok(number);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

/// Continues generating numbers in a series until the required index and returns the number under that index
/// 
/// # Arguments
/// * `index`   - Index of the number being queried
/// * `numbers` - Seed numbers
/// * `verbose` - Outputs executing output of the puzzle to the console
fn generate_numbers (index: usize, numbers: &Vec<usize>, verbose: bool) -> usize {
    // Initialize last position index
    let mut global_index: Vec<isize> = vec![-1; index];
    let mut last_number: usize = 0;

    // Count numbers
    for i in 0..index.clone() {

      // Initialize this step's number
      let new_number;

      // Check if still reading seed numbers
      if i < numbers.len() {

        // Set new number
        new_number = numbers[i];
        // Prompt existing number
        if verbose {
          println!("{} -> SEED", new_number);
        }

      }
      
      // ... else, generate next number
      else if global_index[last_number] == -1 {

        // If number is seen for first time, new number is 0
        new_number = 0;
        // Prompt never before seen last number
        if verbose {
          println!("{} -> {} was never seen before", new_number, last_number);
        }

      }
        
      // ... or if last number was already read previously
      else {

        // If number was seen before, new number is the position diff between last seen position
        let previous_position = global_index[last_number] as usize;
        new_number = (i - 1) - previous_position;
        // Prompt previously seen last number
        if verbose {
          println!("{} -> {} was seen before at position {}", new_number, last_number, previous_position);
        }

      }

      // Store number's last position (with a step offset of one)
      if i > 0 {
        global_index[last_number] = (i - 1) as isize;
      }

      // Store number for next step
      last_number = new_number;
    }

    // Return last generated number
    return last_number;
}
