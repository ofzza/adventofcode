//! 2022 day 17 puzzle
//! 
//! https://adventofcode.com/2022/day/17
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2022::lib::tetris::Tetris;

/// Parses input data
fn parse<'a>(data: &'a String) -> Vec<char> {
  Input::parse(data.as_str().trim(), "", |c| {
    c.chars().nth(0).unwrap()
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
      let data = parse(&data);

      // Initialize a game of tetris
      let mut tetris = Tetris::new(data);
      for _ in 0..2022 {
        tetris.drop_next();
      }

      // Calculate stack height
      let height: u64 = tetris.field.len() as u64 - tetris.field_empty as u64 + tetris.field_cleared;

      // Return result
      String::from(format!("{:?}", height))
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
      let data = parse(&data);

      // Initialize a game of tetris
      let mut tetris = Tetris::new(data);
      for i in 0u64..1_000_000_000_000u64 {
        tetris.drop_next();
        if i % 100_000_000u64 == 0 {
          println!("{} / 1000000000000 = {}% -> height = {}", i, (10000f64 * 100f64 * (i as f64) / 1_000_000_000_000f64).floor() / 10000f64, tetris.field.len() as u64 - tetris.field_empty as u64 + tetris.field_cleared)
        }
      }

      // Calculate stack height
      let height: u64 = tetris.field.len() as u64 - tetris.field_empty as u64 + tetris.field_cleared;

      // Return result
      String::from(format!("{:?}", height))
    }

  );

  // Return registry ownership
  registry
}
