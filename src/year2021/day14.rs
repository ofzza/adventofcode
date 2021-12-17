//! 2021 day 14 puzzle
//! 
//! https://adventofcode.com/2021/day/14
// -----------------------------------------------------------------------------

// Include dependencies
use std::collections::hash_map::*;
use crate::lib::puzzle::*;
use crate::lib::input::*;

/// Parses input data
fn parse(data: &String) -> (Vec<char>, HashMap<(char, char), char>) {
  let sections = Input::parse(data.trim(), "\n\n", |section| { section.trim() });
  let compound: Vec<char> = Input::parse(sections[0], "", |el| { el.chars().next().unwrap() });
  let mut reactions: HashMap<(char, char), char> = HashMap::new();
  let reaction_pairs = Input::parse(sections[1], "\n", |reaction| {
    let mut reaction = Input::parse(reaction.trim(), "->", |els| { els.trim().chars() } );
    ((reaction[0].next().unwrap(), reaction[0].next().unwrap()), reaction[1].next().unwrap())
  });
  for i in 0..reaction_pairs.len() {
    reactions.insert(reaction_pairs[i].0, reaction_pairs[i].1);
  }
  (compound, reactions)
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 14,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize count
      let mut count = init_count(&data.0);

      // Run 10 steps of reactions
      for _ in 0..10 {
        react(&mut count, &data.1);
      }

      // Count individual elements
      let count = count_elements(&count);

      // Calculate and return result
      String::from(format!("{:?}", count[count.len() - 1] - count[0]))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 14,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize count
      let mut count = init_count(&data.0);

      // Run 10 steps of reactions
      for _ in 0..40 {
        react(&mut count, &data.1);
      }

      // Count individual elements
      let count = count_elements(&count);

      // Calculate and return result
      String::from(format!("{:?}", count[count.len() - 1] - count[0]))
    }

  );

  // Return registry ownership
  registry
}

/// Initializes element pairs count hashmap
/// 
/// # Arguments
/// * compound: Original compound elements
/// 
/// # Returns
/// Hashmap of ocunts of all element pairs in the compound
fn init_count (compound: &Vec<char>) -> HashMap<(char, char), usize> {
  // Initialize count
  let mut count: HashMap<(char, char), usize> = HashMap::new();
  // Fill with initial state
  for i in 1..compound.len() {
    let key = (compound[i - 1], compound[i]);
    let value = match count.get(&key) {
      Some(value) => value + 1,
      None => 1
    };
    count.insert(key, value);
  }
  // Return count
  count
}

/// Mutates the current compound based on reaction rules and reflects the mutation in current element pair counts
/// 
/// # Arguments
/// * count:      Hashmap of ocunts of all element pairs in the compound
/// * reactions:  Reaction rules for every element pair
fn react (count: &mut HashMap<(char, char), usize>, reactions: &HashMap<(char, char), char>) {
  // Clone previous counts and clear counts
  let previous = count.clone();
  count.clear();
  // For each pair, compose and count reacted elemnt pairs
  let pairs: Vec<&(char, char)> = previous.keys().collect::<Vec<&(char, char)>>();
  let mut inserts: Vec<((char, char), usize)> = Vec::with_capacity(pairs.len());
  for i in 0..pairs.len() {
    let value = previous.get(&pairs[i]).unwrap().clone();
    let reactant: char = reactions.get(&pairs[i]).unwrap().clone();
    let first: (char, char) = (pairs[i].0, reactant);
    let first_previous_count = match count.get(&first) {
      Some(value) => value,
      None => &0usize
    };
    inserts.push((first, first_previous_count + value));
    count.insert(first, first_previous_count + value);
    let second: (char, char) = (reactant, pairs[i].1);
    let second_previous_count = match count.get(&second) {
      Some(value) => value,
      None => &0usize
    };
    inserts.push((second, second_previous_count + value));
    count.insert(second, second_previous_count + value);
  }
}

/// Counts individual elements from element pair counts
/// 
/// # Arguments
/// * count: Hashmap of ocunts of all element pairs in the compound
/// 
/// # Returns
/// Hashmap of individual element counts
fn count_elements (count: &HashMap<(char, char), usize>) -> Vec<usize> {
  // Initialize elements' count
  let mut els: HashMap<char, usize> = HashMap::new();
  // Cound individual elements in element pairs count hashmap
  let pairs: Vec<&(char, char)> = count.keys().collect::<Vec<&(char, char)>>();
  for i in 0..pairs.len() {
    // Count first element in a pair
    let first_el = pairs[i].0;
    let mut first_value: usize = 0;
    first_value += match els.get(&first_el) {
      Some(value) => value,
      None => &0usize
    };
    first_value += match count.get(&pairs[i]) {
      Some(value) => value,
      None => &0usize
    };
    els.insert(first_el, first_value);
    // Count second element in a pair
    let second_el = pairs[i].1;
    let mut second_value: usize = 0;
    second_value += match els.get(&second_el) {
      Some(value) => value,
      None => &0usize
    };
    second_value += match count.get(&pairs[i]) {
      Some(value) => value,
      None => &0usize
    };
    els.insert(second_el, second_value);
  }
  // Convert element counts into a sorted array
  let mut els: Vec<usize> = els.values().map(|v| (v.clone() as f64 / 2.0).ceil() as usize).collect::<Vec<usize>>();
  els.sort();
  els
}
