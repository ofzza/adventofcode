//! 2019/01 puzzle
//! 
//! https://adventofcode.com/2019/day/1
// -----------------------------------------------------------------------------

// Include dependencies
use super::super::lib::puzzle::*;

/// Registers puzzles for the day
pub fn register (puzzles: &mut Vec<Puzzle>) {

  // Register daily puzzle #1 (test 01)
  puzzles.push(create_intvec_from_value(2019, 1, 1, String::from("test"), String::from("12"), String::from("2"), implementation1));
  // Register daily puzzle #1 (test 02)
  puzzles.push(create_intvec_from_value(2019, 1, 1, String::from("test"), String::from("14"), String::from("2"), implementation1));
  // Register daily puzzle #1 (test 03)
  puzzles.push(create_intvec_from_value(2019, 1, 1, String::from("test"), String::from("1969"), String::from("654"), implementation1));
  // Register daily puzzle #1 (test 04)
  puzzles.push(create_intvec_from_value(2019, 1, 1, String::from("test"), String::from("100756"), String::from("33583"), implementation1));
  // Register daily puzzle #1
  puzzles.push(create_intvec_from_path(2019, 1, 1, String::from("solution"), String::from("./src/year2019/data/day01input.txt"), String::from("3262358"), implementation1));

  // Register daily puzzle #2 (test 01)
  puzzles.push(create_intvec_from_value(2019, 1, 2, String::from("test"), String::from("14"), String::from("2"), implementation2));
  // Register daily puzzle #2 (test 02)
  puzzles.push(create_intvec_from_value(2019, 1, 2, String::from("test"), String::from("1969"), String::from("966"), implementation2));
  // Register daily puzzle #2 (test 03)
  puzzles.push(create_intvec_from_value(2019, 1, 2, String::from("test"), String::from("100756"), String::from("50346"), implementation2));
  // Register daily puzzle #2
  puzzles.push(create_intvec_from_path(2019, 1, 2, String::from("solution"), String::from("./src/year2019/data/day01input.txt"), String::from("4890696"), implementation2));
}

fn implementation1 (puzzle: &Puzzle, verbose: bool) -> String {
  let mut fuel: u32 = 0;
  for module in puzzle.input.value_int_vec.iter() {
    fuel += calculate_fuel_per_mass(*module);
  }
  if verbose == true { println!("I'm working here!!!"); }
  return String::from(fuel.to_string());
}

fn implementation2 (puzzle: &Puzzle, verbose: bool) -> String {
  let mut fuel: u32 = 0;
  for module in puzzle.input.value_int_vec.iter() {
    fuel += calculate_fuel_per_total_mass(*module);
  }
  if verbose == true { println!("I'm working here too!!!"); }
  return String::from(fuel.to_string());
}

fn calculate_fuel_per_total_mass (mass: u32) -> u32 {
  let mut fuel: u32 = 0;
  let mut fuel_additional: u32 = mass;
  loop {
    fuel_additional = calculate_fuel_per_mass(fuel_additional);
    fuel += &fuel_additional;
    if fuel_additional == 0 {
      return fuel;
    }
  }
}

fn calculate_fuel_per_mass (mass: u32) -> u32 {
  let fuel = mass / 3;
  return if fuel > 2 { fuel - 2 } else { 0 };
}
