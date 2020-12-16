//! 2020/01 puzzle
//! 
//! https://adventofcode.com/2020/day/1
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
      let input = PuzzleInput::Vector1D(vec![1721, 979, 366, 299, 675, 1456]);
      stats.update(
        Puzzle::new(2020, 1, 1, "test", input, implementation1, |r| (r, Some(514579)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<u32>(load_input("./src/year2020/data/day01input.txt"), "\n");
      stats.update(
        Puzzle::new(2020, 1, 1, "solution", input, implementation1, |r| (r, Some(1018944)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Run puzzle
  if (index == 0) || (index == 2) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector1D(vec![1721, 979, 366, 299, 675, 1456]);
      stats.update(
        Puzzle::new(2020, 1, 2, "test", input, implementation2, |r| (r, Some(241861950)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<u32>(load_input("./src/year2020/data/day01input.txt"), "\n");
      stats.update(
        Puzzle::new(2020, 1, 2, "solution", input, implementation2, |r| (r, Some(8446464)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<u32, u32, u32>, verbose: bool) -> Result<u32, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(options) => {
      // (Pre)sort options
      let mut sorted_options = options.clone();
      sorted_options.sort();
      // Find sum == 2020
      match find_sum(2020, sorted_options, 0, 2) {
        Some(sum) => {
          // If verbose, output result
          if verbose {
            println!("  > {} + {} == 2020 ---> {} * {} = {}", sum[0], sum[1], sum[0], sum[1], sum[0] * sum[1]);
          }
          // Return result
          return Ok(sum[0] * sum[1])
        },
        None => Err("No 2 items sum up to 2020!")
      }
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<u32, u32, u32>, verbose: bool) -> Result<u32, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(options) => {
      // (Pre)sort options
      let mut sorted_options = options.clone();
      sorted_options.sort();
      // Find sum == 2020
      match find_sum(2020, sorted_options, 0, 3) {
        Some(sum) => {
          // If verbose, output result
          if verbose {
            println!("  > {} + {} + {} == 2020 ---> {} * {} * {} = {}", sum[0], sum[1], sum[2], sum[0], sum[1], sum[2], sum[0] * sum[1] * sum[2]);
          }
          // Return result
          return Ok(sum[0] * sum[1] * sum[2])
        },
        None => Err("No 3 items sum up to 2020!")
      }
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

/// Searches for a sum of N options matching a target value and returns a vector of matched options or None if no combination was found
/// 
/// # Arguments
///
/// * `target`         - Target value that the sum should match
/// * `sorted_options` - Optional numbers to use in the sum
/// * `sum`           - Already summed up value
/// * `n`              - Number of members in the sum
fn find_sum(target: u32, sorted_options: Vec<u32>, sum: u32, n: u32) -> Option<Vec<u32>> {
  // Check if target depth level
  if n == 1 {
    
    // Check for options matching sum
    for option in sorted_options {
      if sum + option == target {
        // Return matching option
        return Some(vec!(option));
      } else if sum + option > target {
        // Return no matching option (assuming input options were pre-sorted)
        return None
      }
    }

  } else if n > 1 {

    // Check for options matching sum recursivelly at higher depths
    for i in 0..(sorted_options.len() - 1) {
      let option = sorted_options[i];
      if sum + option < target {
        // Search recursivelly for next option in the sum (Only use not already covered options)
        match find_sum(target, sorted_options[i..].to_vec(), sum + option, n - 1) {
          Some(members) => {
            let mut result = vec![option];
            result.extend(members);
            return Some(result);
          },
          None => {}
        }
      } else {
        // Return no matching option (assuming input options were pre-sorted)
        return None
      }
    }

  }

  // Return none found
  return None;
}
