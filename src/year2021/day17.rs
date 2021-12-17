//! 2021 day 17 puzzle
//! 
//! https://adventofcode.com/2021/day/17
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2021::lib::probe::*;

/// Parses input data
fn parse(data: &String) -> ((isize, isize),(isize, isize)) {
  let parsed = Input::parse(data.trim(), ":", |line| line);
  let ranges = Input::parse(parsed[1].trim(), ",", |value| {
    let parsed = Input::parse(value.trim(), "=", |value| value);
    let limits = Input::parse(parsed[1].trim(), "..", |value| value.parse::<isize>().unwrap());
    (limits[0], limits[1])
  });
  (ranges[0], ranges[1])
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 17,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize probe
      let mut global_max_y: isize = 0;
      for vy in 0..(data.0.0.abs() + data.1.1.abs()) {
        for vx in 1..data.0.1 + 1 {
          if Probe::test_trajectory(&(vx, vy), &data) {
            let max_y = if vy < 0 { 0 } else { (vy + 1) * vy / 2 };
            if max_y > global_max_y {
              global_max_y = max_y;
            }
          }
        }
      }

      // Calculate and return result
      String::from(format!("{:?}", global_max_y))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 17,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize probe
      let mut global_hits: isize = 0;
      for vy in data.1.0..(data.0.0.abs() + data.1.1.abs()) {
        for vx in 1..data.0.1 + 1 {
          if Probe::test_trajectory(&(vx, vy), &data) {
            global_hits += 1;
          }
        }
      }

      // Calculate and return result
      String::from(format!("{:?}", global_hits))
    }

  );

  // Return registry ownership
  registry
}
