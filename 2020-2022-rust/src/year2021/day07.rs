//! 2021 day 07 puzzle
//! 
//! https://adventofcode.com/2021/day/7
// -----------------------------------------------------------------------------

// Include dependencies
use std::collections::HashMap;
use crate::lib::puzzle::*;
use crate::lib::input::*;

/// Parses input data
fn parse(data: &String) -> Vec<usize> {
  Input::parse(data.trim(), ",", |n| { n.parse::<usize>().unwrap() })
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 7,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Find limits of the search space
      let min = data.iter().min().unwrap().clone();
      let max = data.iter().max().unwrap().clone();

      // Find target with minimal fuel consumption
      let mut cache: HashMap<usize, usize> = HashMap::new();
      for i in (min + 1)..max {
        let target_consumption = calculate_fuel(&data, i, |d| { d }, &mut cache);
        let prev_consumption = calculate_fuel(&data, i - 1, |d| { d }, &mut cache);
        let next_consumption = calculate_fuel(&data, i + 1, |d| { d }, &mut cache);
        if target_consumption < prev_consumption && target_consumption < next_consumption {
          return String::from(format!("{:?}", target_consumption));
        }
      }

      // Return result
      String::from(format!("{:?}", 0))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 7,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Find limits of the search space
      let min = data.iter().min().unwrap().clone();
      let max = data.iter().max().unwrap().clone();

      // Find target with minimal fuel consumption
      let mut cache: HashMap<usize, usize> = HashMap::new();
      for i in (min + 1)..max {
        let target_consumption = calculate_fuel(&data, i, |d| { ((1.0 + d as f64) * (0.5 * d as f64)) as usize }, &mut cache);
        let prev_consumption = calculate_fuel(&data, i - 1, |d| { ((1.0 + d as f64) * (0.5 * d as f64)) as usize }, &mut cache);
        let next_consumption = calculate_fuel(&data, i + 1, |d| { ((1.0 + d as f64) * (0.5 * d as f64)) as usize }, &mut cache);
        if target_consumption < prev_consumption && target_consumption < next_consumption {
          return String::from(format!("{:?}", target_consumption));
        }
      }

      // Return result
      String::from(format!("{:?}", 0))
    }

  );

  // Return registry ownership
  registry
}

/// Calculates total fuel expenditure to get all vehicles to the target position
/// 
/// # Arguments
/// * positions:  Initial positions of all vehicles
/// * target:     Target position for all vehicles
/// * fuel_fn:    Function determining fuel expenditure for a vehicle to move between 2 locations
/// * cache:lib   Hashmap instance used to cache already calculated fuel consumption
/// 
/// # Returns
/// Total fuel expenditure
fn calculate_fuel (positions: &Vec<usize>, target: usize, fuel_fn: fn(distance: usize) -> usize, cache: &mut HashMap<usize, usize>) -> usize {
  // Initialize total
  let mut total = 0;
  // Add up all vehicles
  for i in 0..positions.len() {
    let distance: usize = (positions[i] as isize - target as isize).abs() as usize;
    match cache.get(&distance) {
      Some(consumption) => total += consumption,
      None => {
        let consumption = fuel_fn(distance);
        cache.insert(distance, consumption);
        total += consumption;
      }
    }
  }
  // Return total
  total
}
