//! 2021 day 01 puzzle
//! 
//! https://adventofcode.com/2021
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 1,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = Input::parse(data.as_str().trim(), "\n", |x| {
        x.parse::<usize>().unwrap()
      });

      // Find increasing values
      let mut increases: usize = 0;
      for i in 1usize..(data.len()) {
        if data[i] > data[i - 1] {
          increases += 1;
        }
      }

      // Return result
      String::from(format!("{:?}", increases))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 1,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = Input::parse(data.as_str().trim(), "\n", |x| {
        x.parse::<usize>().unwrap()
      });

      // Find increasing values
      let mut increases: usize = 0;
      for i in 3usize..(data.len()) {
        if (data[i] + data[i - 1] + data[i - 2]) > data[i - 1] + data[i - 2] + data[i - 3] {
          increases += 1;
        }
      }

      // Return result
      String::from(format!("{:?}", increases))
    }

  );

  // Return registry ownership
  registry
}
