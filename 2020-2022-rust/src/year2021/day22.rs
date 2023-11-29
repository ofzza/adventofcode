//! 2021 day 22 puzzle
//! 
//! https://adventofcode.com/2021/day/22
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2021::lib::geometry_cuboids::*;

/// Parses input data
fn parse(data: &String) -> Vec<(bool, Vec<Vec<isize>>)> {
  Input::parse(data.trim(), "\n", |line| {
    // Parse values
    let parsed = Input::parse(line.trim(), " ", |x| x);
    let state_on: bool = parsed[0].trim() == "on";
    let coord_ranges: Vec<Vec<isize>> = Input::parse(parsed[1].trim(), ",", |range| {
      let parsed = Input::parse(range.trim(), "=", |x| x);
      Input::parse(parsed[1].trim(), "..", |n| {
        n.parse::<isize>().unwrap()
      })
    });
    // Compose parsed values
    let mut result: (bool, Vec<Vec<isize>>) = (state_on, Vec::with_capacity(coord_ranges.len()));
    for i in 0..coord_ranges[0].len() {
      let mut coord: Vec<isize> = Vec::with_capacity(coord_ranges[0].len());
      for j in 0..coord_ranges.len() {
        coord.push(coord_ranges[j][i]);
      }
      result.1.push(coord);
    }
    // Return composed result
    result
  })
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 22,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize cuboid space
      let mut space = GeometryCuboidSpace::new(3);

      // Add and remove cuboids from the input
      for i in 0..data.len() {

        // Instantiate the cuboid volume
        let cuboid = GeometryCuboid::new(
          &GeometryPoint { coords: data[i].1[0].clone() },
          &GeometryPoint { coords: data[i].1[1].clone() }
        );

        // Check if within bounds
        let mut out_of_bounds = false;
        for j in 0..data[i].1.len() {
          if out_of_bounds { break; }
          for k in 0..data[i].1[j].len() {
            if data[i].1[j][k] < -50 || data[i].1[j][k] > 50 {
              out_of_bounds = true;
              break;
            }
          }
        }
        if out_of_bounds { continue; }

        // Add cuboid volume
        if data[i].0 {
          space.add(&cuboid);
        }
        // Subtract cuboid volume
        else {
          space.subtract(&cuboid);
        }
      }

      // Calculate and return result
      String::from(format!("{:?}", space.get_volume()))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 22,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize cuboid space
      let mut space = GeometryCuboidSpace::new(3);

      // Add and remove cuboids from the input
      for i in 0..data.len() {

        // Instantiate the cuboid volume
        let cuboid = GeometryCuboid::new(
          &GeometryPoint { coords: data[i].1[0].clone() },
          &GeometryPoint { coords: data[i].1[1].clone() }
        );

        // Add cuboid volume
        if data[i].0 {
          space.add(&cuboid);
        }
        // Subtract cuboid volume
        else {
          space.subtract(&cuboid);
        }
      }

      // Calculate and return result
      String::from(format!("{:?}", space.get_volume()))
    }

  );

  // Return registry ownership
  registry
}
