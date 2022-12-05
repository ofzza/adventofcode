//! 2022 day 02 puzzle
//! 
//! https://adventofcode.com/2022/day/2
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;

/// Parses input data
fn parse(data: &String) -> Vec<Vec<&str>> {
  Input::parse(data.as_str().trim(), "\n", |data| {
    Input::parse(data, " ", |x| {
      x
    })
  })
}

fn game_translate(game_strings: &Vec<Vec<&str>>) -> Vec<(usize, usize)> {
  // Return result
  let mut game_numbers: Vec<(usize, usize)> = vec![];
  // Map characters to values
  for round in game_strings {
    game_numbers.push(
      (
        (round[0].chars().nth(0).unwrap() as usize) - 65,
        (round[1].chars().nth(0).unwrap() as usize) - 88
      )
    )
  }
  // Return result
  game_numbers
}


/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 2,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let game_strings = parse(&data);
      let game_numbers = game_translate(&game_strings);

      // Calculate score
      let mut score: usize = 0;
      for round in &game_numbers {
        let result = if round.0 as isize % 3 == round.1 as isize % 3 { 3 } // Shows same play -> Draw
               else if round.0 as isize % 3 == (round.1 as isize + 1) % 3 { 0 }   // Shows one lower play -> Loss
               else if (round.0 as isize + 1) % 3 == round.1 as isize % 3 { 6 }   // Shows one higher play -> Win
               else { panic!("This can never happen!") };
        let scores = ((round.1 % 3) + 1, result);
        score += scores.0 + scores.1;
      }

      // Return result
      String::from(format!("{:?}", score))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 2,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let game_strings = parse(&data);
      let game_numbers = game_translate(&game_strings);

      // Calculate score
      let mut score: usize = 0;
      for round in &game_numbers {
        let play: usize = if round.1 == 0 { ((round.0 as isize - 1 + 3) % 3) as usize } // -> Loss -> Show one lower play
                     else if round.1 == 1 { (round.0 as isize) as usize } // -> Draw -> Show same play
                     else if round.1 == 2 { ((round.0 as isize + 1) % 3) as usize } // -> Win -> Show one higher play
                     else { panic!("This can never happen!") };
        let scores = (play + 1, round.1 * 3);
        score += scores.0 + scores.1;
      }

      // Return result
      String::from(format!("{:?}", score))
    }

  );

  // Return registry ownership
  registry
}
