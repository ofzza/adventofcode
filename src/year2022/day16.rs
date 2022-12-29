//! 2022 day 16 puzzle
//! 
//! https://adventofcode.com/2022/day/16
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2022::lib::vulcano::Vulcano;

/// Parses input data
fn parse<'a>(data: &'a String) -> Vec<(&str, usize, Vec<&str>)> {
  Input::parse(data.as_str().trim(), "\n", |data| {
    let parsed: Vec<&str> = data.split(';').collect::<Vec<&str>>();
    let name = &parsed[0][6..8];
    let flow_rate = (&parsed[0].split('=').last().unwrap().parse::<usize>().unwrap()).clone();
    let connections = (&parsed[1][23..].trim().split(',').map(|c| c.trim().clone()).collect::<Vec<&str>>()).clone();
    (name, flow_rate, connections)
  })
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 16,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize a vulcano
      let mut vulcano = Vulcano::new(data);

      // Calculate maximum release starting from valve "AA" within 30 minutes
      let max = vulcano.calculate_max_release(vec!["AA"], 30);
      
      // Return result
      String::from(format!("{:?}", max))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 16,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize a vulcano
      let mut vulcano = Vulcano::new(data);

      // Calculate maximum release starting from valve "AA" within 30 minutes
      let max = vulcano.calculate_max_release(vec!["AA", "AA"], 26);
      
      // Return result
      String::from(format!("{:?}", max))
    }

  );

  // Return registry ownership
  registry
}
