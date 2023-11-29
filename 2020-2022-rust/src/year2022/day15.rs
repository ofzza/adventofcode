//! 2022 day 15 puzzle
//! 
//! https://adventofcode.com/2022/day/15
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2022::lib::beacons::BeaconExclusionZone;

/// Parses input data
fn parse<'a>(data: &'a String) -> (bool, Vec<(Vec<isize>, Vec<isize>)>) {
  // Parse around additional input type rows
  let data = Input::parse(data.as_str().trim(), "\n\n", |data| data);
  // Parse and return data
  (
    data[0] == "solution",
    Input::parse(data[1].trim(), "\n", |data| {
      let points = data.split(':').collect::<Vec<&str>>();
      let sensor_coords = points[0].split(',').map(|p| p.split('=').last().unwrap().parse::<isize>().unwrap()).collect::<Vec<isize>>();
      let beacon_coords = points[1].split(',').map(|p| p.split('=').last().unwrap().parse::<isize>().unwrap()).collect::<Vec<isize>>();
      (vec![sensor_coords[0], sensor_coords[1]], vec![beacon_coords[0], beacon_coords[1]])
    })
  )
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 15,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let (is_solution, data) = parse(&data);

      // Set Y coordinate depending on test
      let y = if !is_solution { 10 } else { 2000000 };

      // Initialize beacon exclusion zone
      let mut zone = BeaconExclusionZone::new(data);
      // Detect bounds
      let bounds = zone.detect_bounds();

      // Scan for points of missing coverage
      let count = (bounds.0[1] - bounds.0[0] + 1) as isize - zone.scan_bounds_for_missing_coverage(
        // Bounds to search within - single row only
        (bounds.0, vec![y, y]),
        // Counts every point of no coverage
        |count, _| count + 1,
        // Start counting at 0
        0
      );

      // Count all sensors and beacons on the same y coordinate
      let (sensors, beacons) = zone.get_distinct_sensors_and_beacons();
      let sensors_count = sensors.iter().filter(|c| c[1] == y).count();
      let beacons_count = beacons.iter().filter(|c| c[1] == y).count();
      
      // Return result
      String::from(format!("{:?}", count - sensors_count as isize - beacons_count as isize))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 15,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let (is_solution, data) = parse(&data);

      // Set Y coordinate depending on test
      let search_range = if !is_solution { 20 } else { 4000000 };

      // Initialize beacon exclusion zone
      let mut zone = BeaconExclusionZone::new(data);

      // Scan for points of missing coverage
      let found = zone.scan_bounds_for_missing_coverage(
        // Bounds to search within - single row only
        (vec![0, search_range], vec![0, search_range]),
        // Counts every point of no coverage
        |found: Vec<Vec<isize>>, current| [found, vec![current]].concat(),
        // Start counting at 0
        vec![]
      );

      // Check only a single point found
      if found.len() != 1 {
        panic!("Only a single point of no coverage expected!");
      }
      
      // Return result
      String::from(format!("{:?}", 4000000 * found[0][0] + found[0][1]))
    }

  );

  // Return registry ownership
  registry
}
