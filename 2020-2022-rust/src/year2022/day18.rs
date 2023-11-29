//! 2022 day 18 puzzle
//! 
//! https://adventofcode.com/2022/day/18
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2022::lib::voxel_space::VoxelSpace;

/// Parses input data
fn parse<'a>(data: &'a String) -> Vec<Vec<isize>> {
  Input::parse(data.as_str().trim(), "\n", |data| {
    Input::parse(data, ",", |x| {
      x.parse::<isize>().unwrap()
    })
  })
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 18,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize a voxel saoce
      let mut voxel_space = VoxelSpace::new(data);

      // Enumerae unique polygons
      let polygons = voxel_space.enumerate_polygons();

      // Return result
      String::from(format!("{:?}", polygons.len()))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 18,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize a voxel saoce
      let mut voxel_space = VoxelSpace::new(data);

      // Find fully enclosed voxels and fill them in
      let fully_enclosed_voxels = voxel_space.find_fully_enclosed_voxels();
      for fully_enclosed_voxel in fully_enclosed_voxels {
        voxel_space.voxels.push(fully_enclosed_voxel);
      }

      // Enumerae unique polygons
      let polygons = voxel_space.enumerate_polygons();
      
      // Return result
      String::from(format!("{:?}", polygons.len()))
    }

  );

  // Return registry ownership
  registry
}
