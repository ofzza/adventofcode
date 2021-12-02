//! 2021 day 02 puzzle
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
      day: 2,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = Input::parse(data.as_str().trim(), "\n", |x| {
        let x: Vec<&str> = x.split(" ").collect();
        (x[0], x[1].parse::<usize>().unwrap())
      });

      // Track position
      let mut x = 0;
      let mut z = 0;
      for i in 0..data.len() {
        if data[i].0 == "forward" {
          x += data[i].1;
        } else if data[i].0 == "down" {
          z += data[i].1;
        } else if data[i].0 == "up" {
          z -= data[i].1;
        }
      }

      // Return result
      String::from(format!("{:?}", x * z))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 2,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = Input::parse(data.as_str().trim(), "\n", |x| {
        let x: Vec<&str> = x.split(" ").collect();
        (x[0], x[1].parse::<usize>().unwrap())
      });

      // Track position
      let mut x = 0;
      let mut aim = 0;
      let mut z = 0;
      for i in 0..data.len() {
        if data[i].0 == "forward" {
          x += data[i].1;
          z += data[i].1 * aim;
        } else if data[i].0 == "down" {
          aim += data[i].1;
        } else if data[i].0 == "up" {
          aim -= data[i].1;
        }
      }

      // Return result
      String::from(format!("{:?}", x * z))
    }

  );

  // Return registry ownership
  registry
}
