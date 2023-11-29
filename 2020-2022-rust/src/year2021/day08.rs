//! 2021 day 08 puzzle
//! 
//! https://adventofcode.com/2021/day/8
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2021::lib::segment_display::*;

/// Parses input data
fn parse(data: &String) -> Vec<(Vec<Vec<char>>, Vec<Vec<char>>)> {
  Input::parse(data.trim(), "\n", |line| {
    let parsed: Vec<&str> = line.trim().split('|').collect();
    let digits = Input::parse(parsed[0].trim(), " ", |code| {
      Input::parse(code.trim(), "", |signal| { signal.chars().next().unwrap() })
    });
    let value = Input::parse(parsed[1].trim(), " ", |code| {
      Input::parse(code.trim(), "", |signal| { signal.chars().next().unwrap() })
    });  
    (digits, value)
  })
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 8,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize segment display
      let mut display = SegmentDisplay::new();

      // Program and decode all lines
      let mut count = 0;
      for i in 0..data.len() {
        display.program(&data[i].0);
        let decoded = display.decode(&data[i].1);
        for j in 0..decoded.len() {
          if decoded[j] == 1 || decoded[j] == 4 || decoded[j] == 7 || decoded[j] == 8 {
            count += 1;
          }
        }
      }

      // Return result
      String::from(format!("{:?}", count))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 8,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize segment display
      let mut display = SegmentDisplay::new();

      // Program and decode all lines
      let mut sum = 0;
      for i in 0..data.len() {
        display.program(&data[i].0);
        let decoded = display.decode(&data[i].1);
        sum += decoded.iter().map(|n| { n.to_string() }).collect::<Vec<String>>().join("").parse::<usize>().unwrap();
      }

      // Return result
      String::from(format!("{:?}", sum))
    }

  );

  // Return registry ownership
  registry
}
