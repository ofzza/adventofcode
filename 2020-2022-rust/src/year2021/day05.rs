//! 2021 day 05 puzzle
//! 
//! https://adventofcode.com/2021/day/5
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2021::lib::geometry_plain::*;

/// Parses input data
fn parse(data: &String) -> Vec<Vec<Vec<isize>>> {
  Input::parse(data.trim(), "\n", |line| {
    Input::parse(line, "->", |point| {
      Input::parse(point.trim(), ",", |n| n.parse::<isize>().unwrap())
    })
  })
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 5,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize a geometry plain
      let mut plain = GeometryPlain::new(data.len());
      // Add lines to the plain
      for i in 0..data.len() {
        plain.add_line(data[i][0][0], data[i][0][1], data[i][1][0], data[i][1][1]);
      }

      // Find intersections
      let intersections = plain.find_intersections(false);

      // Return result
      String::from(format!("{:?}", intersections.len()))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 5,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize a geometry plain
      let mut plain = GeometryPlain::new(data.len());
      // Add lines to the plain
      for i in 0..data.len() {
        plain.add_line(data[i][0][0], data[i][0][1], data[i][1][0], data[i][1][1]);
      }

      // Find intersections
      let intersections = plain.find_intersections(true);

      // Return result
      String::from(format!("{:?}", intersections.len()))
    }

  );

  // Return registry ownership
  registry
}
