//! 2021 day 04 puzzle
//! 
//! https://adventofcode.com/2021/day/4
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2021::lib::bingo::*;

/// Parses input data
fn parse(data: &String) -> (Vec<u8>, Vec<Vec<Vec<u8>>>) {
  let parsed: Vec<&str> = data.as_str().trim().splitn(2, "\n\n").collect();
  let numbers = Input::parse(parsed[0].trim(), ",", |n| n.parse::<u8>().unwrap());
  let cards = Input::parse(parsed[1].trim(), "\n\n", |card| {
    Input::parse(card.trim(), "\n", |nums| {
      Input::parse(&nums.to_string().as_str().trim().replace("  ", " "), " ", |n| n.parse::<u8>().unwrap())
    })
  });
  (numbers, cards)
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 4,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize bing and cards
      let numbers = data.0;
      let cards = data.1;
      let mut bingo = Bingo::new(5, 5);
      for i in 0..cards.len() {
        bingo.add_card(&cards[i]);
      }

      // Play bingo until a win
      for i in 0..numbers.len() {
        let winner = bingo.draw(numbers[i].clone());
        match winner {
          Some(card) => {
            return String::from(format!("{:?}", numbers[i] as usize * card.borrow().get_remaining_numbers_sum()));
          },
          _ => {}
        }
      }

      // Return result
      String::from(format!("{:?}", "No winning card found!"))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 4,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize bing and cards
      let numbers = data.0;
      let cards = data.1;
      let mut bingo = Bingo::new(5, 5);
      for i in 0..cards.len() {
        bingo.add_card(&cards[i]);
      }

      // Play bingo until a win
      let mut last_win: usize = 0;
      for i in 0..numbers.len() {
        let winner = bingo.draw(numbers[i].clone());
        match winner {
          Some(card) => {
            last_win = numbers[i] as usize * card.borrow().get_remaining_numbers_sum();
          },
          _ => {}
        }
      }
      
      // Return result
      String::from(format!("{:?}", last_win))
    }

  );

  // Return registry ownership
  registry
}
