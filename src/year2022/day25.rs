//! 2022 day 25 puzzle
//! 
//! https://adventofcode.com/2022/day/25
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2022::lib::snafu::SNAFU;

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
      day: 25,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Decode and sum all the numbers provided
      let mut sum: isize = 0;
      for num in data {
        sum += SNAFU::decode(num);
      }

      // Encode the sum
      let encoded = SNAFU::encode(sum);

      // Return result
      String::from(format!("{}", encoded))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 25,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |_| {
      // Return result
      String::from("Done!")
    }

  );

  // Return registry ownership
  registry
}
