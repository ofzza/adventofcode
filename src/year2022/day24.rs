//! 2022 day 24 puzzle
//! 
//! https://adventofcode.com/2022/day/24
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2022::lib::blizzards::BlizzardBasin;

/// Parses input data
fn parse<'a>(data: &'a String) -> Vec<Vec<char>> {
  Input::parse(data.as_str().trim(), "\n", |data| {
    data.chars().collect::<Vec<char>>()
  })
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 24,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize blizzard basin
      let mut blizzards = BlizzardBasin::new(data);

      // Find quickest path
      let steps = blizzards.traverse(blizzards.position_start.clone(), blizzards.position_end.clone(), 0);      
      
      // Return result
      String::from(format!("{:?}", steps))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 24,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize blizzard basin
      let mut blizzards = BlizzardBasin::new(data);

      // Find quickest path
      let steps_there                      = blizzards.traverse(blizzards.position_start.clone(), blizzards.position_end.clone(), 0);      
      let steps_there_and_back             = blizzards.traverse(blizzards.position_end.clone(), blizzards.position_start.clone(), steps_there);      
      let steps_there_back_and_there_again = blizzards.traverse(blizzards.position_start.clone(), blizzards.position_end.clone(), steps_there_and_back);      
      
      // Return result
      // 819 => Too low!
      String::from(format!("{:?}", steps_there_back_and_there_again))
    }

  );

  // Return registry ownership
  registry
}
