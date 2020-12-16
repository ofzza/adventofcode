//! 2020/09 puzzle
//! 
//! https://adventofcode.com/2020/day/9
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
      let input = PuzzleInput::ParamVector1D(5, vec![35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576]);
      stats.update(
        Puzzle::new(2020, 9, 1, "test", input, implementation1, |r| (r, Some(127)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = match parse_1d::<usize>(load_input("./src/year2020/data/day09input.txt"), "\n") {
        PuzzleInput::Vector1D(code) => PuzzleInput::ParamVector1D(25, code),
        _ => panic!("This should never, ever happen!")
      };
      stats.update(
        Puzzle::new(2020, 9, 1, "solution", input, implementation1, |r| (r, Some(90433990)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Run puzzle
  if (index == 0) || (index == 2) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::ParamVector1D(5, vec![35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576]);
      stats.update(
        Puzzle::new(2020, 9, 2, "test", input, implementation2, |r| (r, Some(62)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = match parse_1d::<usize>(load_input("./src/year2020/data/day09input.txt"), "\n") {
        PuzzleInput::Vector1D(code) => PuzzleInput::ParamVector1D(25, code),
        _ => panic!("This should never, ever happen!")
      };
      stats.update(
        Puzzle::new(2020, 9, 2, "solution", input, implementation2, |r| (r, Some(11691646)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<usize, usize, usize>, verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::ParamVector1D(preamble_len, stream) => {
      // Find item braking checksum
      match checksum(preamble_len.clone(), stream, verbose) {
        Some(i) => { return Ok(stream[i]); },
        None => { return Err("Stream checksum completed with no errors - was expecting an error!"); }
      }
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<usize, usize, usize>, verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::ParamVector1D(preamble_len, stream) => {
      // Find item braking checksum
      match checksum(preamble_len.clone(), stream, verbose) {
        Some(i) => {
          // Find range of items matching broken item
          match find_range_sum(i, &stream, verbose) {
            Some(result) => {
              let mut range: Vec<usize> = result.clone();
              range.sort();
              let a: usize = range.first().unwrap().clone();
              let b: usize = range.last().unwrap().clone();
              return Ok(a + b);
            },
            None => { return Err("Failed finding matching range sum!"); }
          }
        },
        None => { return Err("Stream checksum completed with no errors - was expecting an error!"); }
      }
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

/// Check checksum of stream and returns index of first failing item, or None
/// 
/// # Arguments
/// * `preamble_len` - Length of range of previous items to look through
/// * `stream`       - Reference to full stream of values
/// * `verbose`      - Outputs executing output of the puzzle to the console
fn checksum (preamble_len: usize, stream: &Vec<usize>, verbose: bool) -> Option<usize> {
  for i in preamble_len..stream.len() {
    // Check checksum for item
    match checksum_item(i, preamble_len, stream) {
      Some((a, b)) => {
        // Prompt successful checksum of item
        if verbose {
          println!("{} [{}] == {} [{}] + {} [{}]", stream[i], i, stream[a], a, stream[b], b)
        }
      },
      None => {
        // Return failed checksum of item
        return Some(i);
      }
    }
  }
  // Return successful checksum
  return None;
}

/// Check single item checksum by finding 2 previous items that sum up to it and returns those 2 items, or None
/// 
/// # Arguments
/// * `index`        - Index of the item being checksummed
/// * `preamble_len` - Length of range of previous items to look through
/// * `stream`       - Reference to full stream of values
fn checksum_item (index: usize, preamble_len: usize, stream: &Vec<usize>) -> Option<(usize, usize)> {
  for i in (index - preamble_len)..index {
    for j in (index - preamble_len)..index {
       if i != j && stream[i] + stream[j] == stream[index] {
         return Some((i, j));
       }
    }
  }

  // Checksum succesful
  return None;
}

/// Finds a continuous range of items summing up to a value in the stream and returns those values, or None
/// 
/// # Arguments
/// * `index`        - Index of the item being summed to
/// * `stream`       - Reference to full stream of values
/// * `verbose`      - Outputs executing output of the puzzle to the console
fn find_range_sum (index: usize, stream: &Vec<usize>, verbose: bool) -> Option<Vec<usize>> {
  // Move range length ...
  for l in 2..stream.len() {
    // Move range ...
    for i in 0..(stream.len() - l) {
      // Initialize range
      let mut sum: usize  = 0;
      let mut sum_members: Vec<usize> = vec![];
      // Sum range
      for j in i..(i + l) {
        sum += stream[j];
        sum_members.push(stream[j]);
      }
      // Prompt matched range sum
      if verbose {
        println!("Sum (len {} offset {}) of {:?} == {} == {}[{}]", l, i, sum_members, sum, index, stream[index]);
      }
      // If sum matches, return match
      if sum == stream[index] {
        // Return matched sum
        return Some(sum_members);
      }
    }
  }

  // Return no range found
  return None;
}
