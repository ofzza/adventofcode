//! 2019/01 puzzle
//! 
//! https://adventofcode.com/2019/day/1
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::inputs::*;
use crate::lib::puzzle::*;

/// Registers puzzles for the day
pub fn run (index: u32, key: &str, _verbose: bool, obfuscate: bool) -> PuzzleExecutionStatistics {

  // Initialize stats
  let mut stats = PuzzleExecutionStatistics{
    ..Default::default()
  };
  
  // Run puzzle
  if (index == 0) || (index == 1) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector1D(vec![12]);
      stats.update(
        Puzzle::new(2019, 1, 1, "test", input, implementation1, |r| (r, Some(2)))
          .run(false, obfuscate)
      );
      // Test
      let input = PuzzleInput::Vector1D(vec![14]);
      stats.update(
        Puzzle::new(2019, 1, 1, "test", input, implementation1, |r| (r, Some(2)))
          .run(false, obfuscate)
      );
      // Test
      let input = PuzzleInput::Vector1D(vec![1969]);
      stats.update(
        Puzzle::new(2019, 1, 1, "test", input, implementation1, |r| (r, Some(654)))
          .run(false, obfuscate)
      );
      // Test
      let input = PuzzleInput::Vector1D(vec![100756]);
      stats.update(
        Puzzle::new(2019, 1, 1, "test", input, implementation1, |r| (r, Some(33583)))
          .run(false, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<i32>(load_input("./src/year2019/data/day01input.txt"), "\n");
      stats.update(
        Puzzle::new(2019, 1, 1, "solution", input, implementation1, |r| (r, Some(3262358)))
          .run(false, obfuscate)
      );
    }
  }

  // Run puzzle
  if (index == 0) || (index == 2) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector1D(vec![14]);
      stats.update(
        Puzzle::new(2019, 1, 2, "test", input, implementation2, |r| (r, Some(2)))
          .run(false, obfuscate)
      );
      // Test
      let input = PuzzleInput::Vector1D(vec![1969]);
      stats.update(
        Puzzle::new(2019, 1, 2, "test", input, implementation2, |r| (r, Some(966)))
          .run(false, obfuscate)
      );
      // Test
      let input = PuzzleInput::Vector1D(vec![100756]);
      stats.update(
        Puzzle::new(2019, 1, 2, "test", input, implementation2, |r| (r, Some(50346)))
          .run(false, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<i32>(load_input("./src/year2019/data/day01input.txt"), "\n");
      stats.update(
        Puzzle::new(2019, 1, 2, "solution", input, implementation2, |r| (r, Some(4890696)))
          .run(false, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<i32, i32, i32>, _verbose: bool) -> Result<i32, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(parts) => {
      let mut fuel: i32 = 0;
      for module in parts.iter() {
        fuel += calculate_fuel_per_mass(*module);
      }
      return Ok(fuel);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<i32, i32, i32>, _verbose: bool) -> Result<i32, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(parts) => {
      let mut fuel: i32 = 0;
      for module in parts.iter() {
        fuel += calculate_fuel_per_total_mass(*module);
      }
      return Ok(fuel);
    },
    _ => panic!("This shouldn't ever happen!")
  }

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
