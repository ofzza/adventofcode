//! 2022 day 05 puzzle
//! 
//! https://adventofcode.com/2022/day/5
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;

/// Parses input data
fn parse(data: &String) -> (Vec<Vec<String>>, Vec<(usize, usize, usize)>) {
  let data = Input::parse(data, "\n\n", |data| data);
  // Parse stacks' levels
  let levels =     Input::parse(data[0], "\n", |data| data ).iter()
    .filter(|row| {
      row.contains("[") && row.contains("]")
    })
    .map(|row| {
      (0..((row.len() as f32 / 4.0).ceil() as usize))
        .map(|i| row.clone()[(4 * i)..(4 * (i+1)).min(row.len() - 1)].to_string().trim().replace("[", "").replace("]", ""))
        .collect()
    })
    .collect();
  // Parse moves
  let moves = Input::parse(data[1].trim(), "\n", |data| {
    let data = Input::parse(data, " ", |data| data);
    (data[1].parse::<usize>().unwrap(), data[3].parse::<usize>().unwrap(), data[5].parse::<usize>().unwrap())
  });
  (levels, moves)
}

fn levels_to_stacks (levels: &Vec<Vec<String>>) -> Vec<Vec<&str>> {
  // Initialize stacks
  let mut stacks: Vec<Vec<&str>> = vec![vec![]; levels[0].len()];
  // Process levels into stacks
  for i in (0..levels.len()).rev() {
    for j in 0..levels[i].len() {
      if levels[i][j] != "" {
        stacks[j].push(levels[i][j].as_str());
      }
    }
  }
  // Return stacks
  stacks
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 5,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let (levels, moves) = parse(&data);
      let mut stacks = levels_to_stacks(&levels);

      // Execute moves
      for mv in moves {
        for _ in 0..mv.0 {
          let item = stacks[mv.1 - 1].pop().unwrap();
          stacks[mv.2 - 1].push(item);
        }
      }

      // Collect top items
      let tops = stacks.iter().map(|stack| stack.last().unwrap().clone()).collect::<Vec<&str>>().join("");

      // Return result
      String::from(format!("{}", tops))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 5,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let (levels, moves) = parse(&data);
      let mut stacks = levels_to_stacks(&levels);

      // Execute moves
      for mv in moves {
        let mut crane: Vec<&str> = vec![];
        for _ in 0..mv.0 {
          let item = stacks[mv.1 - 1].pop().unwrap();
          crane.push(item);
        }
        for _ in 0..mv.0 {
          let item = crane.pop().unwrap();
          stacks[mv.2 - 1].push(item);
        }
      }

      // Collect top items
      let tops = stacks.iter().map(|stack| stack.last().unwrap().clone()).collect::<Vec<&str>>().join("");

      // Return result
      String::from(format!("{}", tops))
    }

  );

  // Return registry ownership
  registry
}
