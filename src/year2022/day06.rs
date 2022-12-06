//! 2022 day 06 puzzle
//! 
//! https://adventofcode.com/2022/day/6
// -----------------------------------------------------------------------------

// Include dependencies
use std::collections::HashSet;
use crate::lib::puzzle::*;
use crate::lib::input::*;

/// Parses input data
fn parse(data: &String) -> Vec<&str> {
  Input::parse(data.as_str().trim(), "", |data| data)
}

fn find_start_packet_position<'a> (data: Vec<&'a str>, packet_len: usize, hash: &mut HashSet<&'a str>) -> Option<usize> {
  // Initialize buffer
  let mut buffer: Vec<&str> = vec![""; packet_len];
  // Search for start packet
  for i in 0..data.len() {
    // Add to buffer
    buffer.push(data[i]);
    buffer.splice(0..1, []);
    // Check buffer for duplicates
    if i >= packet_len && !find_duplicates(&buffer, hash) {
      return Option::Some(i);
    }
  }
  // Return no packet found
  Option::None
}

fn find_duplicates<'a> (buffer: &Vec<&'a str>, hash: &mut HashSet<&'a str>) -> bool {
  // Clear hash
  hash.clear();
  // Search for duplicates
  for n in buffer {
    if hash.contains(n) { return true; }
    hash.insert(n);
  }
  false

  // // Search for duplicates (TODO: Write this better with less than O(NxLog(N))
  // for i in 0..(buffer.len() - 1) {
  //   for j in (i + 1)..buffer.len() {
  //     if buffer[i] == buffer[j] {
  //       return true
  //     }
  //   }
  // }
  // // No duplicates found
  // false
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 6,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize a global hash set for determining duplicates
      let mut hash: HashSet<&str> = HashSet::with_capacity(4);

      // Return result
      String::from(format!("{:?}", find_start_packet_position(data, 4, &mut hash).unwrap() + 1))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 6,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize a global hash set for determining duplicates
      let mut hash: HashSet<&str> = HashSet::with_capacity(14);

      // Return result
      String::from(format!("{:?}", find_start_packet_position(data, 14, &mut hash).unwrap() + 1))
    }

  );

  // Return registry ownership
  registry
}
