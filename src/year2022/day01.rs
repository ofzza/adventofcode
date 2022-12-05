//! 2022 day 01 puzzle
//! 
//! https://adventofcode.com/2022/day/1
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;

/// Parses input data
fn parse(data: &String) -> Vec<Vec<usize>> {
  Input::parse(data.as_str().trim(), "\n\n", |data| {
    Input::parse(data, "\n", |x| { x.parse::<usize>().unwrap() })
  })
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 1,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Find max subset
      let mut max_sum: usize = 0;
      for i in 0..data.len() {        
        // Sum up subset
        let mut sum: usize = 0;
        for n in &data[i] {        
          sum += n;
        }
        // Compare tp max subset found
        if sum > max_sum {
          max_sum = sum;
        }
      }

      // Return result
      String::from(format!("{:?}", max_sum))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 1,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Find max 3 subsets
      let mut max_sums: Vec<usize> = vec![0, 0, 0];
      for i in 0..data.len() {        
        // Sum up subset
        let mut sum: usize = 0;
        for n in &data[i] {        
          sum += n;
        }
        // Compare to top 3 max subsets found
        for j in 0..max_sums.len() {
          if sum > max_sums[j] {
            // Insert max into cirrect sorted place
            max_sums.insert(j, sum);
            // Trim number of tracked max values
            max_sums = max_sums[0..3].to_vec();
            break;
          }
        }
      }
      // Sub up 3 max subsets
      let mut sum: usize = 0;
      for i in 0..3 {
        sum += max_sums[i];
      }

      // Return result
      String::from(format!("{:?}", sum))
    }

  );

  // Return registry ownership
  registry
}
