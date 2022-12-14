//! 2022 day 14 puzzle
//! 
//! https://adventofcode.com/2022/day/14
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2022::lib::regolith_reservoir::*;

/// Parses input data
fn parse<'a>(data: &'a String) -> Vec<Vec<(usize, usize)>> {
  Input::parse(data.as_str().trim(), "\n", |data| {
    Input::parse(data, "->", |data| {
      let parsed: Vec<&str> = data.trim().split(",").collect();
      (
        parsed[0].parse::<usize>().unwrap(),
        parsed[1].parse::<usize>().unwrap()
      )
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
      day: 14,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize reservoir
      let mut reservoir = RegolithReservoir::new((500,0), data, Option::None);      

      // Prompt initial state
      // reservoir._prompt();
      // println!();

      // Simulate reservoir
      loop {
        // Simulate nextstep
        reservoir.step();

        // Prompt reservior
        // reservoir._prompt();
        // println!();

        // Check if current grand of sand is "off the map"
        match reservoir.current {
          Option::Some(current) => if current.1 > reservoir.floor { break; },
          _ => ()
        }
      }

      // Prompt final state
      // reservoir._prompt();
      // println!();

      // Count grains of sand
      let count = reservoir.hash.values().filter(|value| !value.clone()).count();

      // Return result
      String::from(format!("{:?}", count))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 14,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize reservoir
      let mut reservoir = RegolithReservoir::new((500,0), data, Option::Some(2));      

      // Prompt initial state
      // reservoir._prompt();
      // println!();

      // Simulate reservoir
      while !reservoir.hash.contains_key(&(500,0)) {
        // Simulate nextstep
        reservoir.step();

        // Prompt reservior
        // reservoir._prompt();
        // println!();

        // Check if current grand of sand is "off the map"
        match reservoir.current {
          Option::Some(current) => if current.1 > reservoir.floor { break; },
          _ => ()
        }
      }

      // Prompt final state
      // reservoir._prompt();
      // println!();

      // Count grains of sand
      let count = reservoir.hash.values().filter(|value| !value.clone()).count();

      // Return result
      String::from(format!("{:?}", count))
    }

  );

  // Return registry ownership
  registry
}
