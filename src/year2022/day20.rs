//! 2022 day 20 puzzle
//! 
//! https://adventofcode.com/2022/day/20
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2022::lib::grove_positioning_system::GPS;

/// Parses input data
fn parse<'a>(data: &'a String) -> Vec<i64> {
  Input::parse(data.as_str().trim(), "\n", |x| {
    x.parse::<i64>().unwrap()
  })
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 20,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize GPS
      let mut gps = GPS::new(data);
      // Mix up the values
      let mixed = gps.mix();
      // Sum up the 1000th, 2000th and 3000th value
      let mut start = 0;
      for i in 0..mixed.len() {
        if mixed[i] == 0 { start = i; break; }
      }
      let sum = mixed[(start + 1000) % mixed.len()] + mixed[(start + 2000) % mixed.len()] + mixed[(start + 3000) % mixed.len()];

      // Return result
      String::from(format!("{:?}", sum))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 20,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let mut data = parse(&data);

      // Adjust the input
      data = data.iter().map(|n| n * 811589153).collect::<Vec<i64>>();

      // Initialize GPS
      let mut gps = GPS::new(data);
      // Mix up the values 10 times
      let mut mixed = vec![];
      for _ in 0..10 { mixed = gps.mix(); };
      // Sum up the 1000th, 2000th and 3000th value
      let mut start = 0;
      for i in 0..mixed.len() {
        if mixed[i] == 0 { start = i; break; }
      }
      let sum = mixed[(start + 1000) % mixed.len()] + mixed[(start + 2000) % mixed.len()] + mixed[(start + 3000) % mixed.len()];
      
      // Return result
      String::from(format!("{:?}", sum))
    }

  );

  // Return registry ownership
  registry
}
