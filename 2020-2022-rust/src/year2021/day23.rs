//! 2021 day 23 puzzle
//! 
//! https://adventofcode.com/2021/day/23
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2021::lib::amphipods_burrow::*;

/// Parses input data
fn parse(data: &String) -> Vec<Vec<char>> {
  Input::parse(data.trim(), "\n", |line| {
    line.chars().collect()
  })
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 23,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize burrow
      let mut burrow = AmphipodsBurrow::new(vec![
        data[2][3], data[3][3],
        data[2][5], data[3][5],
        data[2][7], data[3][7],
        data[2][9], data[3][9]
      ]);

      // Organize a burrow
      let count = burrow.organize(false);

      // Calculate and return result
      String::from(format!("{:?}", count))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 23,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize burrow
      let mut burrow = AmphipodsBurrow::new(vec![
        data[2][3], 'D', 'D', data[3][3],
        data[2][5], 'C', 'B', data[3][5],
        data[2][7], 'B', 'A', data[3][7],
        data[2][9], 'A', 'C', data[3][9]
      ]);

      // Organize a burrow
      let count = burrow.organize(false);

      // Calculate and return result
      String::from(format!("{:?}", count))
    }

  );

  // Return registry ownership
  registry
}
