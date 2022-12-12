//! 2022 day 12 puzzle
//! 
//! https://adventofcode.com/2022/day/12
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2022::lib::hill_climb::*;

/// Parses input data
fn parse<'a>(data: &'a String) -> Vec<Vec<&str>> {
  Input::parse(data.as_str().trim(), "\n", |data| {
    Input::parse(data, "", |x| x)
  })
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 12,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize hill climb heightmap
      let mut hillclimb = HillClimb::new(
        data,
        |_, from_height, to_height| from_height + 1 >= to_height,
        |hillclimb, coords, _| coords[0] == hillclimb.end[0] && coords[1] == hillclimb.end[1]
      );

      // Calculate distances from start point
      let start = &hillclimb.start.clone();
      let distances = hillclimb.calculate_distances(start);

      // Get end point index and distance
      let end = &hillclimb.end.clone();
      let end_index = hillclimb.heightmap.coords_to_index(end).unwrap();
      let distance = distances[end_index];

      // Return result
      String::from(format!("{:?}", distance))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 12,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize hill climb heightmap
      let mut hillclimb = HillClimb::new(
        data,
        |_, from_height, to_height| to_height + 1 >= from_height,
        |_, _, height| height == 'a' as usize - 96
      );

      // Calculate distances from start point
      let end = &hillclimb.end.clone();
      let distances = hillclimb.calculate_distances(end);

      // Find max found distance
      let distance = distances.iter().filter(|x| x != &&usize::MAX).max().unwrap();

      // Return result
      String::from(format!("{:?}", distance))
    }

  );

  // Return registry ownership
  registry
}
