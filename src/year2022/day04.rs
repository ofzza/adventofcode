//! 2022 day 04 puzzle
//! 
//! https://adventofcode.com/2022/day/4
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;

/// Parses input data
fn parse(data: &String) -> Vec<Vec<Vec<usize>>> {
  Input::parse(data.as_str().trim(), "\n", |data| {
    Input::parse(data, ",", |data| {
      Input::parse(data, "-", |x| {
        x.parse::<usize>().unwrap()
      })
    })
  })
}

fn calculate_length (range: (usize, usize)) -> usize {
  range.1 - range.0 + 1
}

fn calculate_overlap (range_1: (usize, usize), range_2: (usize, usize)) -> usize {
  if ((range_2.0 >= range_1.0) && (range_2.0 <= range_1.1)) || ((range_1.0 >= range_2.0) && (range_1.0 <= range_2.1)) {
    range_1.1.min(range_2.1) - range_1.0.max(range_2.0) + 1
  } else {
    0
  }
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 4,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Count completely overlapping regions
      let mut count = 0;
      for pairs in data {
        let range_1_len = calculate_length((pairs[0][0], pairs[0][1]));        
        let range_2_len = calculate_length((pairs[1][0], pairs[1][1]));
        let ranges_overlap = calculate_overlap((pairs[0][0], pairs[0][1]), (pairs[1][0], pairs[1][1]));
        if ranges_overlap == range_1_len || ranges_overlap == range_2_len {
          count += 1;
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
      year: 2022,
      day: 4,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Count completely overlapping regions
      let mut count = 0;
      for pairs in data {
        let ranges_overlap = calculate_overlap((pairs[0][0], pairs[0][1]), (pairs[1][0], pairs[1][1]));
        if ranges_overlap > 0 {
          count += 1;
        }
      }
      
      // Return result
      String::from(format!("{:?}", count))
    }

  );

  // Return registry ownership
  registry
}
