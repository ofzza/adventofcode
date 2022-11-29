//! 2022 day 01 puzzle
//! 
//! https://adventofcode.com/2022/day/1
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;

/// Parses input data
/// 
/// # Arguments
/// * data: Puzzle input data
/// 
/// # Returns
/// Vector of numbers
fn parse(data: &String) -> Vec<usize>{
  Input::parse(data.as_str().trim(), "\n", |x| {
    x.parse::<usize>().unwrap()
  })
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 1,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Find increasing values
      let mut sum: usize = 0;
      for n in data {        
        sum += n;
      }

      // Return result
      String::from(format!("{:?}", sum))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 1,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Find increasing values
      let mut prod: usize = 1;
      for n in data {
        prod *= n;
      }

      // Return result
      String::from(format!("{:?}", prod))
    }

  );

  // Return registry ownership
  registry
}
