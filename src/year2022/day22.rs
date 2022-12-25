//! 2022 day 22 puzzle
//! 
//! https://adventofcode.com/2022/day/22
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2022::lib::monkey_map::MonkeyMap;

/// Parses input data
fn parse<'a>(data: &'a String) -> (Vec<Vec<char>>, &str) {
  // Extract sections
  let sections = Input::parse(data.as_str().trim(), "\n\n", |data| data);
  // Parse 1st sectipn
  let section = Input::parse(sections[0].trim(), "\n", |data| {
    Input::parse(data, "", |x| x.chars().nth(0).unwrap())
  });
  // Return sections
  (section, sections[1].trim())

}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 22,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize map
      let mut map = MonkeyMap::new(data.0, data.1);

      // Return result
      String::from(format!("{:?}", 0))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 22,
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
