//! 2022 day 17 puzzle
//! 
//! https://adventofcode.com/2022/day/17
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;

/// Parses input data
fn parse<'a>(data: &'a String) -> Vec<&str> {
  Input::parse(data.as_str().trim(), "\n", |data| {
    data
  })
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 17,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let _data = parse(&data);

      // Return result
      String::from(format!("{:?}", 0))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 17,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let _data = parse(&data);
      
      // Return result
      String::from(format!("{:?}", 0))
    }

  );

  // Return registry ownership
  registry
}
