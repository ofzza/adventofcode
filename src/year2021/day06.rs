//! 2021 day 06 puzzle
//! 
//! https://adventofcode.com/2021/day/6
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2021::lib::hatchery::*;

/// Parses input data
fn parse(data: &String) -> Vec<usize> {
  Input::parse(data.trim(), ",", |n| { n.parse::<usize>().unwrap() })
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 6,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize hatchery
      let mut hatchery = Hatchery::new(2, 7, 1);
      hatchery.populate(data);

      // Run model
      for _ in 0..80 {
        hatchery.tick();
      }

      // Return result
      String::from(format!("{:?}", hatchery.len()))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 6,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize hatchery
      let mut hatchery = Hatchery::new(2, 7, 1);
      hatchery.populate(data);

      // Run model
      for _ in 0..256 {
        hatchery.tick();
      }

      // Return result
      String::from(format!("{:?}", hatchery.len()))
    }

  );

  // Return registry ownership
  registry
}
