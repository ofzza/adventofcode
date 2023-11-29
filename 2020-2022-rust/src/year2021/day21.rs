//! 2021 day 21 puzzle
//! 
//! https://adventofcode.com/2021/day/21
// -----------------------------------------------------------------------------

// Include dependencies
use std::collections::hash_map::*;
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2021::numeric_sequence::*;

/// Parses input data
fn parse(data: &String) -> Vec<usize> {
  Input::parse(data.trim(), "\n", |line| {
    let parsed = Input::parse(line.trim(), ":", |n| { n });
    parsed[1].trim().parse::<usize>().unwrap()
  })
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 21,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize player sequences
      let first_position_sequence = NumericSequence::new::<usize>(&data[0], |i: usize, prev_position: &usize, starting_position: &usize| {
        let move_length = 3 * (6 * i + 0) + 1 + 2 + 3;        
        let position = (if i == 0 { starting_position } else { prev_position } + move_length - 1) % 10 + 1;
        (i % 10, position)
      });
      let second_position_sequence = NumericSequence::new::<usize>(&data[1], |i: usize, prev_position: &usize, starting_position: &usize| {
        let move_length = 3 * (6 * i) + 4 + 5 + 6;
        let position = (if i == 0 { starting_position } else { prev_position } + move_length - 1) % 10 + 1;
        (i % 10, position)
      });

      // Find winner
      let (first_winning_index, _first_winning_sum) = first_position_sequence.get_index_where_sum_more_than(999);
      let _first_winning_value = first_position_sequence.get_value_for_index(&first_winning_index);
      let second_value_on_first_win = second_position_sequence.get_sum_for_index(&first_winning_index);
      let (second_winning_index, _second_winning_sum) = second_position_sequence.get_index_where_sum_more_than(999);
      let _second_winning_value = second_position_sequence.get_value_for_index(&second_winning_index);
      let first_value_on_first_win = first_position_sequence.get_sum_for_index(&second_winning_index);

      // Find winning player
      let winning_score = if first_winning_index < second_winning_index {
        second_value_on_first_win * (first_winning_index * 6 + 3)
      } else {
        first_value_on_first_win* (second_winning_index * 6 + 6)
      };

      // Calculate and return result
      String::from(format!("{:?}", winning_score))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 21,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize stats
      let mut first_wins: u128 = 0;
      let mut second_wins: u128 = 0;

      // Generate all possible 3 dice throw cums
      let mut dice_sums: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
      for i in 1..4 {
        for j in 1..4 {
          for k in 1..4 {
            dice_sums[i + j + k] += 1;
          }
        }
      }

      // Initialize hash of positions and scores
      let mut hash: HashMap<((usize, usize), (usize, usize)), u128> = HashMap::new();
      hash.insert(((data[0], 0), (data[1], 0)), 1);

      // Play game until all permutations have reached a winning state
      while hash.len() > 0 {

        // Player #1 move from every recorded previous step state
        let states: Vec<(&((usize, usize), (usize, usize)), &u128)> = hash.iter().collect();
        let mut first_updated_hash: HashMap<((usize, usize), (usize, usize)), u128> = HashMap::new();
        for i in 0..states.len() {
          
          // Get state position and score
          let permutations = states[i].1;
          let position = states[i].0.0.0;
          let score = states[i].0.0.1;

          // Move every possible dice thrown distance
          for distance in 0..dice_sums.len() {
            if dice_sums[distance] > 0 {
              // Calculate destination
              let updated_position: usize = (position + distance - 1) % 10 + 1;
              let updated_score: usize = score + updated_position;
              // Calculate way to get to this stte
              let permutations_updated: u128 = permutations * dice_sums[distance] as u128;
              // If player has won, store number of permutations into global count
              if updated_score >= 21 {
                first_wins += permutations_updated;
              }
              // ... else store state for next step
              else {
                let updated_state = ((updated_position, updated_score), states[i].0.1.clone());
                let existing_solutions_option = first_updated_hash.get(&updated_state);
                let existing_solutions: u128 = match existing_solutions_option { Some(n) => n.clone(), _=> 0 };
                first_updated_hash.insert(updated_state, existing_solutions + permutations_updated);
              }
            }
          }
        }

        // Replace hash for player "2" move
        hash = first_updated_hash;

        // Player #2 move from every recorded previous step state
        let states: Vec<(&((usize, usize), (usize, usize)), &u128)> = hash.iter().collect();
        let mut second_updated_hash: HashMap<((usize, usize), (usize, usize)), u128> = HashMap::new();
        for i in 0..states.len() {
          
          // Get state position and score
          let permutations = states[i].1;
          let position = states[i].0.1.0;
          let score = states[i].0.1.1;

          // Move every possible dice thrown distance
          for distance in 0..dice_sums.len() {
            if dice_sums[distance] > 0 {
              // Calculate destination
              let updated_position: usize = (position + distance - 1) % 10 + 1;
              let updated_score: usize = score + updated_position;
              // Calculate way to get to this stte
              let permutations_updated: u128 = permutations * dice_sums[distance] as u128;
              // If player has won, store number of permutations into global count
              if updated_score >= 21 {
                second_wins += permutations_updated;
              }
              // ... else store state for next step
              else {
                let updated_state = (states[i].0.0.clone(), (updated_position, updated_score));
                let existing_solutions_option = second_updated_hash.get(&updated_state);
                let existing_solutions: u128 = match existing_solutions_option { Some(n) => n.clone(), _=> 0 };
                second_updated_hash.insert(updated_state, existing_solutions + permutations_updated);
              }
            }
          }
        }

        // Replace hash for next turn
        hash = second_updated_hash;
        
      }
      
      // Calculate and return result
      String::from(format!("{:?}", if first_wins > second_wins { first_wins } else { second_wins }))
    }

  );

  // Return registry ownership
  registry
}
