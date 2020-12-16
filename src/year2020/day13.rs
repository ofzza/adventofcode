//! 2020/13 puzzle
//! 
//! https://adventofcode.com/2020/day/13
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
      let input = PuzzleInput::Vector1D(vec![
        String::from("939"),
        String::from("7,13,x,x,59,x,31,19")
      ]);
      stats.update(
        Puzzle::new(2020, 13, 1, "test", input, implementation1, |r| (r, Some(295)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d(load_input("./src/year2020/data/day13input.txt"), "\n");
      stats.update(
        Puzzle::new(2020, 13, 1, "solution", input, implementation1, |r| (r, Some(333)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Run puzzle
  if (index == 0) || (index == 2) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector1D(vec![
        String::from("0"),
        String::from("7,13,x,x,59,x,31,19")
      ]);
      stats.update(
        Puzzle::new(2020, 13, 2, "test", input, implementation2, |r| (r, Some(1068781)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::Vector1D(vec![
        String::from("0"),
        String::from("17,x,13,19")
      ]);
      stats.update(
        Puzzle::new(2020, 13, 2, "test", input, implementation2, |r| (r, Some(3417)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::Vector1D(vec![
        String::from("0"),
        String::from("67,7,59,61")
      ]);
      stats.update(
        Puzzle::new(2020, 13, 2, "test", input, implementation2, |r| (r, Some(754018)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::Vector1D(vec![
        String::from("0"),
        String::from("67,x,7,59,61")
      ]);
      stats.update(
        Puzzle::new(2020, 13, 2, "test", input, implementation2, |r| (r, Some(779210)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::Vector1D(vec![
        String::from("0"),
        String::from("67,7,x,59,61")
      ]);
      stats.update(
        Puzzle::new(2020, 13, 2, "test", input, implementation2, |r| (r, Some(1261476)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::Vector1D(vec![
        String::from("0"),
        String::from("1789,37,47,1889")
      ]);
      stats.update(
        Puzzle::new(2020, 13, 2, "test", input, implementation2, |r| (r, Some(1202161486)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d(load_input("./src/year2020/data/day13input.txt"), "\n");
      stats.update(
        Puzzle::new(2020, 13, 2, "solution", input, implementation2, |r| (r, Some(690123192779524)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<String, usize, usize>, _verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(input) => {
      // Parse input
      let parsed = parse_input(&input);
      let timestamp = parsed.0;
      let ids = parsed.1;

      // Find min wait
      let mut min_id = 0;
      let mut min_waittime = 0;
      for id in ids {
        if id != 0 {
          let wait_time = id  - (timestamp % id);
          if min_waittime == 0 || wait_time < min_waittime {
            min_id = id;
            min_waittime = wait_time;
          }
        }
      }

      // Return min waittime/id
      Ok(min_id * min_waittime)
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<String, usize, usize>, verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(input) => {
      // Parse input
      let parsed = parse_input(&input);
      let ids = parsed.1;

      // Find constraints
      let mut constraints: Vec<(usize, usize)> = vec![];
      for i in 0..ids.len() {
        if ids[i] != 0 {
          // Register constraint
          constraints.push((ids[i], (i * ids[i] - i) % ids[i]));
          // Prompt 
          if verbose {
            println!("T % {} = {}", ids[i], (i * ids[i] - i) % ids[i])
          }
        }
      }

      // Find number by remainders
      let mut seed: usize = 0;
      let mut step: usize = 1;
      for i in 0..constraints.len() {
        seed = find_value_by_remainders(seed, step, constraints[i].clone());
        step *= constraints[i].0;
      }

      Ok(seed)
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

/// Parse input strings as a timestamp and an vector iof IDs ("x" IDs will be parsed as 0)
/// 
/// # Arguments
/// * `input` - Inputs to parse 
fn parse_input (input: &Vec<String>) -> (usize, Vec<usize>) {
  let timestamp = input[0].trim().parse::<usize>().unwrap();
  let ids: Vec<usize> = input[1].split(',').map(|id| if id.clone() == "x" { 0 } else { id.parse::<usize>().unwrap() }).collect();
  return (timestamp, ids);
}

/// Finds a value, searching from `seed` in steps of `step` which matches the given constraint in that when
/// divided by `constraint.0` it has a remainder of `constraint.1`
/// 
/// # Arguments
/// * `seed`       - Starting value to check
/// * `step`       - How much to increment the testing value in each step
/// * `constraint` - constraint on the number being searched for
fn find_value_by_remainders (seed: usize, step: usize, constraint: (usize, usize)) -> usize {
  let mut test = seed;
  loop {
    // Validate constraints
    if test > 0 && test % constraint.0 == constraint.1 {
      return test;
    }
    // Test next
    test += step;
  }
}
