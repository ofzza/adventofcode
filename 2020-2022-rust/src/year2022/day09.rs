//! 2022 day 09 puzzle
//! 
//! https://adventofcode.com/2022/day/9
// -----------------------------------------------------------------------------

// Include dependencies
use std::collections::hash_set::HashSet;
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2022::lib::rope::Rope;

/// Parses input data
fn parse(data: &String) -> Vec<(&str, usize)> {
  Input::parse(data.as_str().trim(), "\n", |data| {
    let parsed: Vec<&str> = data.split(" ").collect();
    (parsed[0], parsed[1].parse::<usize>().unwrap())
  })
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 9,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);
      
      // Initialize rope and rope tail position tracking
      let mut rope = Rope::new(2);
      let mut hash: HashSet<(isize, isize)> = HashSet::new();
      // Move rope
      for m in data {
        hash = rope.move_head(m.0, m.1, |rope, mut hash| {
          if !hash.contains(&rope.sections[1]) {
            hash.insert(rope.sections[1].clone());
          }
          hash
        }, hash);
      }

      // Return result
      String::from(format!("{:?}", hash.len()))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 9,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize rope and rope tail position tracking
      let mut rope = Rope::new(10);
      let mut hash: HashSet<(isize, isize)> = HashSet::new();
      // Move rope
      for m in data {
        hash = rope.move_head(m.0, m.1, |rope, mut hash| {
          if !hash.contains(&rope.sections[9]) {
            hash.insert(rope.sections[9].clone());
          }
          hash
        }, hash);
      }

      // Return result
      String::from(format!("{:?}", hash.len()))
    }

  );

  // Return registry ownership
  registry
}
