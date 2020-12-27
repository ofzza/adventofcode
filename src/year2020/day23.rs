//! 2020/23 puzzle
//! 
//! https://adventofcode.com/2020/day/23
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::inputs::*;
use crate::lib::puzzle::*;
use super::RingOfCups;

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
      let input = PuzzleInput::ParamVector1D(10, vec![3, 8, 9, 1, 2, 5, 4, 6, 7]);
      stats.update(
        Puzzle::new(2020, 23, 1, "test", input, implementation1, |r| (r, Some(String::from("92658374"))))
          .run(verbose, obfuscate)
      );
    }
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::ParamVector1D(100, vec![3, 8, 9, 1, 2, 5, 4, 6, 7]);
      stats.update(
        Puzzle::new(2020, 23, 1, "test", input, implementation1, |r| (r, Some(String::from("67384529"))))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = match parse_1d::<usize>(load_input("./src/year2020/data/day23input.txt"), "\n") {
        PuzzleInput::Vector1D(input) => PuzzleInput::ParamVector1D(100, input),
        _ => panic!("This should never, ever happen!")
      };
      stats.update(
        Puzzle::new(2020, 23, 1, "solution", input, implementation1, |r| (r, Some(String::from("75893264"))))
          .run(verbose, obfuscate)
      );
    }
  }

  // Run puzzle
  if (index == 0) || (index == 2) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::ParamsVector1D(vec![1000000, 10000000], vec![3, 8, 9, 1, 2, 5, 4, 6, 7]);
      stats.update(
        Puzzle::new(2020, 23, 2, "test", input, implementation2, |r| (r, Some(149245887792)))
          .run(false, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = match parse_1d::<usize>(load_input("./src/year2020/data/day23input.txt"), "\n") {
        PuzzleInput::Vector1D(input) => {
          PuzzleInput::ParamsVector1D(vec![1000000, 10000000], input)
        },
        _ => panic!("This should never, ever happen!")
      };
      stats.update(
        Puzzle::new(2020, 23, 2, "solution", input, implementation2, |r| (r, Some(38162588308)))
          .run(false, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<usize, String, String>, verbose: bool) -> Result<String, &str> {
  match &puzzle.input {
    PuzzleInput::ParamVector1D(iterations, cups) => {
      // Play game
      let ring = play_game(cups, cups.len(), iterations.clone(), 3, verbose);

      // Return result
      let mut current_value = 1;
      let mut cup_values: Vec<usize> = vec![];
      for _ in 0..(cups.len() - 1) {
        current_value = ring.cups_by_value[current_value].1;
        cup_values.push(current_value);
      }
      Ok(cup_values.iter().map(|cup| cup.to_string()).collect::<Vec<String>>().join(""))
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<usize, usize, usize>, verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::ParamsVector1D(params, cups) => {
      // Play game
      let cups = play_game(cups, params[0].clone(), params[1].clone(), 3, verbose);

      // Return result
      let value_a = cups.cups_by_value[1].1;
      let value_b = cups.cups_by_value[value_a].1;
      return Ok(value_a * value_b);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

/// Plays the game of cups
/// 
/// # Arguments
/// * `cups`               - Initial (partial) arrangement of cups
/// * `cups_expanded_size` - Final number of cups, achieved by adding cups with increasing values after the last specified one
/// * `iterations`         - Number of iterations the game will play for
/// * `move_length`        - Number of cups moved on each iteration
/// * `verbose`            - Outputs executing output of the puzzle to the console
fn play_game (cups: &Vec<usize>, cups_expanded_size: usize, iterations: usize, move_length: usize, verbose: bool) -> RingOfCups {
  // Instantiate cups
  let mut cups = RingOfCups::new(cups, cups_expanded_size);

  // Prompt initial state
  if verbose {
    println!("  Initial state: {} -> [{}]", cups.current_value, cups_to_string(&cups));
  }

  // Play game
  for iteration_index in 0..iterations {

    // Prompt move and pre-move ordering
    if verbose {
      println!("\n  > Move {}", iteration_index + 1);
      println!("    > Cups: [{}]", cups_to_string(&cups));
    }

    // Pick up cups
    let mut current_value = cups.current_value;
    let mut picked_up_values: Vec<usize> = vec![];
    for _ in 0..move_length {
      current_value = cups.cups_by_value[current_value].1;
      picked_up_values.push(current_value);
    }

    // Prompt picked up cups
    if verbose {
      println!("    > Pick up: {:?}", picked_up_values);
    }

    // Determine destination cup value
    let mut destination_value: usize = 0;
    for diff in 1..cups_expanded_size {
      destination_value = ((cups_expanded_size + cups.current_value - diff - 1) % cups_expanded_size) + 1;
      if !picked_up_values.contains(&destination_value) {
        break;
      }
    }

    // Prompt destination
    if verbose {
      println!("    > Destination: {}", destination_value);
    } 

    // Move picked up cups
    cups.move_cups(picked_up_values[0], picked_up_values.len(), destination_value);

    // Prompt post-move ordering
    if verbose {
      println!("    > Cups: [{}]", cups_to_string(&cups));
    }

    // Move to new current
    cups.current_value = cups.cups_by_value[cups.current_value].1;

  }
  
  // Return cups 
  return cups;
}

/// Composes a string representation of the ring of cups
/// 
/// Arguments
/// * `cups` - Cups to convert to a string representation
fn cups_to_string (cups: &RingOfCups) -> String {
  let mut current_value = cups.current_value;
  let mut cup_values: Vec<usize> = vec![];
  for _ in 0..cups.length {
    cup_values.push(current_value);
    current_value = cups.cups_by_value[current_value].1;
  }
  return format!("{}", cup_values.iter().map(|value| value.to_string()).collect::<Vec<String>>().join(", "));
}
