//! 2020/05 puzzle
//! 
//! https://adventofcode.com/2020/day/5
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
      let input = PuzzleInput::Vector1D(vec![String::from("FBFBBFFRLR")]);
      stats.update(
        Puzzle::new(2020, 5, 1, "test", input, implementation1, |r| (r.2, Some(357)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<String>(load_input("./src/year2020/data/day05input.txt"), "\n");
      stats.update(
        Puzzle::new(2020, 5, 1, "solution", input, implementation1, |r| (r.2, Some(915)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Run puzzle
  if (index == 0) || (index == 2) {
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<String>(load_input("./src/year2020/data/day05input.txt"), "\n");
      stats.update(
        Puzzle::new(2020, 5, 2, "solution", input, implementation2, |r| (r, Some(699)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<String, (usize, usize, usize) , usize>, verbose: bool) -> Result<(usize, usize, usize) , &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(codes) => {
      // Initialize max
      let mut max: (usize, usize, usize) = (0, 0, 0);
      // Decode all codes and get max
      for code in codes {
        let decoded: (usize, usize, usize) = decode_seat(code, verbose);
        if decoded.2 > max.2 {
          max = decoded;
        }
      }
      // Return max
      return Ok(max);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<String, usize, usize>, verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(codes) => {
      // Initialize IDs
      let mut ids: Vec<usize> = vec![];
      // Decode all codes
      for code in codes {
        let decoded: (usize, usize, usize) = decode_seat(code, verbose);
        ids.push(decoded.2);
      }
      // Sort IDs
      ids.sort();
      // If verbose, output IDs
      if verbose {
        println!("IDs: {:?}", ids);
      }
      // Find missing ID
      for i in 1..ids.len() {
        if ids[i] != ids[i-1] + 1 {
          return Ok(ids[i-1] + 1);
        }
      }
      // Couldn't find missing ID
      return Err("Couldn't find missing ID");
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

/// Decodes a seat code into a (row, column, id)
/// 
/// # Arguments
/// * `code`    - Seat code to decode from 
/// * `verbose` - Outputs executing output of the puzzle to the console
fn decode_seat (code: &String, verbose: bool) -> (usize, usize, usize) {
  // Parse seat row, column and ID
  let binary: String = code.replace('B', "1").replace('F', "0").replace('R', "1").replace('L', "0");
  let binary_row: String = String::from(&binary[..7]);
  let row = usize::from_str_radix(&binary_row, 2).expect("Failed parsing binary seat ID");
  let binary_column: String = String::from(&binary[7..]);
  let column = usize::from_str_radix(&binary_column, 2).expect("Failed parsing binary seat ID");
  let seat_id = row * 8 + column;

  // Prompt seat row, column and ID
  if verbose {
    println!("{} ({}) -> row {} ({}), column {} ({}) -> ID {}", code, binary, row, binary_row, column, binary_column, seat_id);
  }

  // Return seat row, column and ID
  return (row, column, seat_id);
}
