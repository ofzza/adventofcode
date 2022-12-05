//! 2022 day 03 puzzle
//! 
//! https://adventofcode.com/2022/day/3
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;

/// Parses input data
fn parse(data: &String) -> Vec<Vec<u8>> {
  Input::parse(data.as_str().trim(), "\n", |data| {
    Input::parse(data, "", |data| {
      data.chars().nth(0).unwrap() as u8
    })
  })
}

fn data_to_rucksacks (data: Vec<Vec<u8>>) -> Vec<(Vec<u8>, Vec<u8>)> {
  // Initialize result
  let mut rucksacs: Vec<(Vec<u8>, Vec<u8>)> = vec![];
  // Split each rucksack into compartments
  for line in data {
    let midpoint = line.len() / 2;
    rucksacs.push((line[..midpoint].to_vec(), line[midpoint..].to_vec() ))
  }
  // Return result
  rucksacs
}

fn get_compartment_items_count_by_type (compartment: &Vec<u8>) -> [usize; 256] {
  // Search compartment
  let mut counts: [usize; 256] = [0; 256];
  for n in compartment {
    counts[n.clone() as usize] += 1;
  }
  // Return counts
  counts
}

fn find_duplicates_in_rucksack (compartments: &(Vec<u8>, Vec<u8>)) -> Vec<u8> {
  // Search first compartment
  let counts: ([usize; 256], [usize; 256]) = (
    get_compartment_items_count_by_type(&compartments.0),
    get_compartment_items_count_by_type(&compartments.1)
  );
  // Search and match duplicates betzween compartment
  let mut duplicates: Vec<u8> = vec![];
  for i in 0..256 {
    if counts.0[i] > 0 && counts.1[i] > 0 {
      duplicates.push(i as u8);
    }
  }
  // Return result
  duplicates
}

fn evaluate_item_priority(item: u8) -> u8 {
  // Uppercase: [65, 90]
  // Lowercase: [97, 122]
  if item >= 97 {
    1 + (item - 97)
  }
  else if item >= 65 {
    27 + (item - 65)
  } else {
    panic!("This can never happen!");
  }
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 3,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);
      let rucksacks = data_to_rucksacks(data);

      // Find and sum up duplicate priorities
      let mut sum: usize = 0;
      for rucksack in rucksacks {
        let duplicates = find_duplicates_in_rucksack(&rucksack);
        for item in duplicates {
          sum += evaluate_item_priority(item) as usize
        }
      }

      // Return result
      String::from(format!("{:?}", sum))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 3,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Process all rucksacks and sum up group tags
      let mut sum: usize = 0;
      for i in 0..(data.len() / 3) {
        // Group rucksacks into groups of 3
        let rucksacks = (
          get_compartment_items_count_by_type(&data[3 * i + 0]),
          get_compartment_items_count_by_type(&data[3 * i + 1]),
          get_compartment_items_count_by_type(&data[3 * i + 2])
        );
        // Find common item in group
        for i in 0..256 {
          if rucksacks.0[i] > 0 && rucksacks.1[i] > 0 && rucksacks.2[i] > 0 {
            sum += evaluate_item_priority(i as u8) as usize;
            break;
          }
        }
      }

      // Return result
      String::from(format!("{:?}", sum))
    }

  );

  // Return registry ownership
  registry
}
