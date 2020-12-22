//! 2020/23 puzzle
//! 
//! https://adventofcode.com/2020/day/23
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
  if false && ((index == 0) || (index == 2)) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let mut input_values = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
      for i in 10..1000001 { input_values.push(i); }
      let input = PuzzleInput::ParamVector1D(10000000, input_values);
      stats.update(
        Puzzle::new(2020, 23, 2, "test", input, implementation2, |r| (r, Some(149245887792)))
          .run(false, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = match parse_1d::<usize>(load_input("./src/year2020/data/day23input.txt"), "\n") {
        PuzzleInput::Vector1D(mut input_values) => {
          for i in 10..1000001 { input_values.push(i); }
          PuzzleInput::ParamVector1D(10000000, input_values)
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
      let cups = play_game(cups, iterations.clone(), 3, verbose);
      // Rotate after "1" value
      let mut result: Vec<usize> = vec![];
      let first_index = cups.iter().position(|cup| cup.clone() == 1).unwrap();
      for i in 1..cups.len() {
        result.push(cups[(first_index + i) % cups.len()]);
      }

      // Return result
      Ok(result.iter().map(|cup| cup.to_string()).collect::<Vec<String>>().join(""))
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<usize, usize, usize>, verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::ParamVector1D(iterations, cups) => {
      // Play game
      let cups = play_game(cups, iterations.clone(), 3, verbose);
      // Rotate after "1" value
      let mut result: Vec<usize> = vec![];
      let first_index = cups.iter().position(|cup| cup.clone() == 1).unwrap();
      for i in 1..cups.len() {
        result.push(cups[(first_index + i) % cups.len()]);
      }

      // Find cup with value "1"
      let index = cups.iter().position(|cup| cup.clone() == 1).unwrap();

      // Return result
      return Ok(cups[index + 1] * cups[index + 2]);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

/// TODO: ...
fn play_game (cups: &Vec<usize>, iterations: usize, move_length: usize, verbose: bool) -> Vec<usize> {
  // Clone cups before playing
  let mut cups: Vec<usize> = cups.clone();
  let modulo = cups.len();

  // Prompt initial state
  if verbose {
    println!("  Initial state: {} -> {:?}", cups[0], cups);
  }

  // Play game
  let mut current_index: usize = 0;
  let mut current_value = cups[current_index];
  for iteration_index in 0..iterations {

    // Prompt move
    if verbose {
      println!("\n  > Move {}", iteration_index + 1);
      println!("    > Cups: {:?}", cups);
    }

    // Pick up cups
    let mut picked_up_indices: Vec<usize> = vec![];
    let mut picked_up_cups: Vec<usize> = vec![];
    for i in (current_index + 1)..(current_index + move_length + 1) {
      let picked_up_index = i % modulo;
      picked_up_indices.push(picked_up_index);
      picked_up_cups.push(cups[picked_up_index]);
    }

    // Prompt move
    if verbose {
      println!("    > Pick up: {:?}", picked_up_cups);
    }

    // Determine destination cup value
    let mut destination_value: usize = 0;
    for diff in 1..modulo {
      destination_value = (modulo + cups[current_index] - diff - 1) % modulo + 1;
      if !picked_up_cups.contains(&destination_value) {
        break;
      }
    }

    // Prompt move
    if verbose {
      println!("    > Destination: {}", destination_value);
    }

    // Remove picked up cups
    picked_up_indices.sort();
    for i in 0..picked_up_indices.len() {
      cups.remove(picked_up_indices[picked_up_indices.len() - i - 1]);
    }

    // Find destination cup index
    let destination_index: usize = (
      cups.iter()
        .position(|cup| cup.clone() == destination_value)
        .unwrap() + 1
    ) % modulo;

    // Place picked up cups
    for picked_index in 0..picked_up_cups.len() {
      cups.insert(destination_index + picked_index, picked_up_cups[picked_index]);
    }

    // Move to new current
    current_index = (cups.iter().position(|cup| cup.clone() == current_value).unwrap() + 1) % modulo;
    current_value = cups[current_index];

  }


  
  // Return cups 
  return cups;
}
