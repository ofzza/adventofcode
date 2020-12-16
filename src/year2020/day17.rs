//! 2020/17 puzzle
//! 
//! https://adventofcode.com/2020/day/17
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::inputs::*;
use crate::lib::puzzle::*;
use super::ctube::ConwayTube;

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
      let input = PuzzleInput::ParamVector1D(String::from("6"), vec![
        String::from(".#."),
        String::from("..#"),
        String::from("###")
      ]);
      stats.update(
        Puzzle::new(2020, 17, 1, "test", input, implementation1, |r| (r, Some(112)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = match parse_1d::<String>(load_input("./src/year2020/data/day17input.txt"), "\n") {
        PuzzleInput::Vector1D(code) => PuzzleInput::ParamVector1D(String::from("6"), code),
        _ => panic!("This should never, ever happen!")
      };
      stats.update(
        Puzzle::new(2020, 17, 1, "solution", input, implementation1, |r| (r, Some(265)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Run puzzle
  if (index == 0) || (index == 2) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::ParamVector1D(String::from("6"), vec![
        String::from(".#."),
        String::from("..#"),
        String::from("###")
      ]);
      stats.update(
        Puzzle::new(2020, 17, 2, "test", input, implementation2, |r| (r, Some(848)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = match parse_1d::<String>(load_input("./src/year2020/data/day17input.txt"), "\n") {
        PuzzleInput::Vector1D(code) => PuzzleInput::ParamVector1D(String::from("6"), code),
        _ => panic!("This should never, ever happen!")
      };
      stats.update(
        Puzzle::new(2020, 17, 2, "solution", input, implementation2, |r| (r, Some(1936)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<String, usize, usize>, verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::ParamVector1D(count, input) => {
      // Initialize Conway Tube
      let mut ctube: ConwayTube = ConwayTube::new(input);
      // Iterate Conway tube for 6 initialization cycles
      for _ in 0..count.parse::<usize>().expect("Failed parsing iteration count!") {
        // Generate next state
        ctube.next(verbose, 3);
        // Prompt next state
        if verbose {
          println!("{}", ctube.to_string());
        }
      }
      // Return number of enabled cubes in the tube
      return Ok(
        ctube.cubes.values()
          .map(|state| state.clone())
          .filter(|state| state.clone())
          .collect::<Vec<bool>>()
          .len()
      );
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<String, usize, usize>, verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::ParamVector1D(count, input) => {
      // Initialize Conway Tube
      let mut ctube: ConwayTube = ConwayTube::new(input);
      // Iterate Conway tube for 6 initialization cycles
      for _ in 0..count.parse::<usize>().expect("Failed parsing iteration count!") {
        // Generate next state
        ctube.next(verbose, 4);
        // Prompt next state
        if verbose {
          println!("{}", ctube.to_string());
        }
      }
      // Return number of enabled cubes in the tube
      return Ok(
        ctube.cubes.values()
          .map(|state| state.clone())
          .filter(|state| state.clone())
          .collect::<Vec<bool>>()
          .len()
      );
    },
    _ => panic!("This shouldn't ever happen!")
  }
}
