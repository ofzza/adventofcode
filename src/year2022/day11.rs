//! 2022 day 11 puzzle
//! 
//! https://adventofcode.com/2022/day/11
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2022::keep_away::*;

/// Parses input data
fn parse<'a>(data: &'a String) -> Vec<PlayerState<'a>> {
  Input::parse(data.as_str().trim(), "\n\n", |data| {
    let data = Input::parse(data, "\n", |x| x);

    // Parse starting items
    let items = data[1].trim().split(":").collect::<Vec<&str>>()[1].split(",").map(|x| x.trim().parse::<usize>().unwrap()).collect();
    
    // Parse operation
    let operation_formula: Vec<&str> = data[2].trim().split(":").collect::<Vec<&str>>()[1].trim().split(" ").collect();
    let operation = (
      operation_formula[3],
      if operation_formula[4] == "old" {
        Option::None
      } else {
        Option::Some(operation_formula[4].parse::<usize>().unwrap())
      }      
    );

    // Parse divisibility test
    let test_divisibility_parsed = data[3].trim().split(" ").collect::<Vec<&str>>();
    let test_divisibility = test_divisibility_parsed[3].parse::<usize>().unwrap();
    if test_divisibility_parsed[3] == "old" {
      Option::None
    } else {
      Option::Some(test_divisibility_parsed[3].parse::<usize>().unwrap())
    };

    // Parse dividsibility result: TRUE/FALSE
    let test_divisibility_result_true_parsed = data[4].trim().split(" ").collect::<Vec<&str>>();
    let test_divisibility_result_true = test_divisibility_result_true_parsed[5].parse::<usize>().unwrap();
    let test_divisibility_result_false_parsed = data[5].trim().split(" ").collect::<Vec<&str>>();
    let test_divisibility_result_false = test_divisibility_result_false_parsed[5].parse::<usize>().unwrap();

    // Compose and return player state
    PlayerState {
      items,
      operation,
      test_divisibility: (test_divisibility, test_divisibility_result_true, test_divisibility_result_false)
    }
  })
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 11,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);
      
      // Initialize Keep-Away
      let mut keepaway = KeepAway::new(data, 3);

      // Play for 20 rounds and keep track of how active each monkey is
      let rounds_count = 20;
      let mut monkeys: Vec<usize> = vec![0; keepaway.players.len()];
      for i in 0..(rounds_count * keepaway.players.len()) {
        // Check active monkey activity in next round
        let index = i % keepaway.players.len();
        monkeys[index] += keepaway.players[index].items.len();
        // Play next round's next monkey
        keepaway.execute_turn();
      }

      // Find top 2 active monkeys
      let mut monkeys_sorted = monkeys.clone();
      monkeys_sorted.sort();
      monkeys_sorted.reverse();
      // Calculate level of monkey bussiness
      let monkey_bussiness = monkeys_sorted[0] * monkeys_sorted[1];

      // Return result
      String::from(format!("{:?}", monkey_bussiness))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 11,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);
      
      // Initialize Keep-Away
      let mut keepaway = KeepAway::new(data, 0);

      // Play for 20 rounds and keep track of how active each monkey is
      let rounds_count = 10000;
      let mut monkeys: Vec<usize> = vec![0; keepaway.players.len()];
      for i in 0..(rounds_count * keepaway.players.len()) {
        // Check active monkey activity in next round
        let index = i % keepaway.players.len();
        monkeys[index] += keepaway.players[index].items.len();
        // Play next round's next monkey
        keepaway.execute_turn();
      }

      // Find top 2 active monkeys
      let mut monkeys_sorted = monkeys.clone();
      monkeys_sorted.sort();
      monkeys_sorted.reverse();
      // Calculate level of monkey bussiness
      let monkey_bussiness = monkeys_sorted[0] * monkeys_sorted[1];

      // Return result
      String::from(format!("{:?}", monkey_bussiness))
    }

  );

  // Return registry ownership
  registry
}
