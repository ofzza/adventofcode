//! 2019/01 puzzle
//! 
//! https://adventofcode.com/2019/day/1
// -----------------------------------------------------------------------------

// Include dependencies
use super::super::lib::inputs::*;
use super::super::lib::puzzle::*;

/// Registers puzzles for the day
pub fn run (index: u32, key: &str, _verbose: bool) {

  // Run 1st puzzle
  if (index == 0) || (index == 1) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = vec![12];
      create_vec(2019, 1, 1, "test", input, implementation1, |r| (r, Some(2)))
        .run(false);
      // Test
      let input = vec![14];
      create_vec(2019, 1, 1, "test", input, implementation1, |r| (r, Some(2)))
        .run(false);
      // Test
      let input = vec![1969];
      create_vec(2019, 1, 1, "test", input, implementation1, |r| (r, Some(654)))
        .run(false);
      // Test
      let input = vec![100756];
      create_vec(2019, 1, 1, "test", input, implementation1, |r| (r, Some(33583)))
        .run(false);
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = load_input::<i32>("./src/year2019/data/day01input.txt", '\n');
      create_vec(2019, 1, 1, "solution", input, implementation1, |r| (r, Some(3262358)))
        .run(false);
    }
  }

  // Run 2nd puzzle
  if (index == 0) || (index == 2) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = vec![14];
      create_vec(2019, 1, 2, "test", input, implementation2, |r| (r, Some(2)))
        .run(false);
      // Test
      let input = vec![1969];
      create_vec(2019, 1, 2, "test", input, implementation2, |r| (r, Some(966)))
        .run(false);
      // Test
      let input = vec![100756];
      create_vec(2019, 1, 2, "test", input, implementation2, |r| (r, Some(50346)))
        .run(false);
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = load_input::<i32>("./src/year2019/data/day01input.txt", '\n');
      create_vec(2019, 1, 2, "solution", input, implementation2, |r| (r, Some(4890696)))
        .run(false);
    }
  }
}

fn implementation1 (puzzle: &Puzzle<i32, i32, i32>, _verbose: bool) -> Result<i32, &str> {
  let mut fuel: i32 = 0;
  for module in puzzle.input.value_vec.iter() {
    fuel += calculate_fuel_per_mass(*module);
  }
  return Ok(fuel);
}

fn implementation2 (puzzle: &Puzzle<i32, i32, i32>, _verbose: bool) -> Result<i32, &str> {
  let mut fuel: i32 = 0;
  for module in puzzle.input.value_vec.iter() {
    fuel += calculate_fuel_per_total_mass(*module);
  }
  return Ok(fuel);
}

fn calculate_fuel_per_total_mass (mass: i32) -> i32 {
  let mut fuel: i32 = 0;
  let mut fuel_additional: i32 = mass;
  loop {
    fuel_additional = calculate_fuel_per_mass(fuel_additional);
    fuel += &fuel_additional;
    if fuel_additional == 0 {
      return fuel;
    }
  }
}

fn calculate_fuel_per_mass (mass: i32) -> i32 {
  let fuel = mass / 3;
  return if fuel > 2 { fuel - 2 } else { 0 };
}
