//! 2022 day 08 puzzle
//! 
//! https://adventofcode.com/2022/day/8
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;

/// Parses input data
fn parse(data: &String) -> Vec<Vec<usize>> {
  Input::parse(data.as_str().trim(), "\n", |data| {
    Input::parse(data, "", |x| {
      x.parse::<usize>().unwrap()
    })
  })
}

fn check_visibility (data_heights: &Vec<Vec<usize>>) -> (usize, Vec<Vec<bool>>) {
  // Check dimensions
  let height = data_heights.len();
  let width = data_heights[0].len();
  // Initialize visibility map and count
  let mut data_visibility: Vec<Vec<bool>> = vec![vec![false; width]; height];
  let mut count = 0;


  // Check visibility by row
  for y in 0..height {
    // Check from left
    let mut max = data_heights[y][0];
    if !data_visibility[y][0] {
      data_visibility[y][0] = true;
      count += 1;
    }
    for x in 1..width {
      if data_heights[y][x] > max {
        max = data_heights[y][x];
        if !data_visibility[y][x] {
          data_visibility[y][x] = true;
          count += 1;
        }
      }
    }
    // Check from right
    let mut max = data_heights[y][width - 1];
    if !data_visibility[y][width - 1] {
      data_visibility[y][width - 1] = true;
      count += 1;
    }
    for x in (0..(width-1)).rev() {
      if data_heights[y][x] > max {
        max = data_heights[y][x];
        if !data_visibility[y][x] {
          data_visibility[y][x] = true;
          count += 1;
        }
      }
    }
  }

  // Check visibility by column
  for x in 0..width {
    // Check from top
    let mut max = data_heights[0][x];
    if !data_visibility[0][x] {
      data_visibility[0][x] = true;
      count += 1;
    }
    for y in 1..height {
      if data_heights[y][x] > max {
        max = data_heights[y][x];
        if !data_visibility[y][x] {
          data_visibility[y][x] = true;
          count += 1;
        }
      }
    }
    // Check from bottom
    let mut max = data_heights[height - 1][x];
    if !data_visibility[height - 1][x] {
      data_visibility[height - 1][x] = true;
      count += 1;
    }
    for y in (0..(height-1)).rev() {
      if data_heights[y][x] > max {
        max = data_heights[y][x];
        if !data_visibility[y][x] {
          data_visibility[y][x] = true;
          count += 1;
        }
      }
    }
  }

  // Return visibility map and count
  (count, data_visibility)
}

fn find_best_view_position (data_heights: &Vec<Vec<usize>>) -> (usize, (usize, usize)) {
  // Check dimensions
  let height = data_heights.len();
  let width = data_heights[0].len();
  // Check trees' visibility
  let (_, data_visibility) = check_visibility(data_heights);
  // Initialize scenic score and position
  let mut score = 0;
  let mut position: (usize, usize) = (0, 0);

  // Find best scenic score
  for y in 1..(height - 1) {
    for x in 1..(width - 1) {
      if data_visibility[y][x] {
        // Get current point tree height
        let pvalue = data_heights[y][x];
        // Calculate scenic score
        let mut position_score = 1;

        // Look to the left
        let mut count = 0;
        for px in (0..x).rev() {
          count += 1;
          if data_heights[y][px] >= pvalue { break; }
        }
        position_score *= count;
        // Look to the right
        let mut count = 0;
        for px in (x + 1)..width {
          count += 1;
          if data_heights[y][px] >= pvalue { break; }
        }
        position_score *= count;
        // Look to the top
        let mut count = 0;
        for py in (0..y).rev() {
          count += 1;
          if data_heights[py][x] >= pvalue { break; }
        }
        position_score *= count;
        // Look to the bottom
        let mut count = 0;
        for py in (y + 1)..height {
          count += 1;
          if data_heights[py][x] >= pvalue { break; }
        }
        position_score *= count;

        // Check if position score is max so far
        if position_score > score {
          score = position_score;
          position = (x, y);
        }

      }
    }
  }

  // Return best found scenic score
  (score, position)
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 8,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data_heights = parse(&data);

      // Count visible trees
      let (count, _) = check_visibility(&data_heights);

      // Return result
      String::from(format!("{:?}", count))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 8,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data_heights = parse(&data);

      // Find tree with best view
      let (score, _) = find_best_view_position(&data_heights);

      // Return result
      String::from(format!("{:?}", score))
    }

  );

  // Return registry ownership
  registry
}
