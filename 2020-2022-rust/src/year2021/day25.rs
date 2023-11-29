//! 2021 day 25 puzzle
//! 
//! https://adventofcode.com/2021/day/25
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year::lib::matrix::*;

/// Parses input data
fn parse(data: &String) -> Vec<Vec<char>> {
  Input::parse(data.trim(), "\n", |line| {
    line.chars().collect::<Vec<char>>()
  })
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 25,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Convert data into a matrix backed vector
      let matrix = Matrix::new(vec![data[0].len(), data.len()]);
      let mut vector = matrix.create();
      for y in 0..data.len() {
        for x in 0..data[y].len() {
          vector.push(data[y][x]);
        }
      }

      let mut step: usize = 0;
      loop {
        // Track if anyone has moved in this cycle
        let mut has_moved = false;

        // Check all east-facing members
        let mut updated_vector = vector.clone();
        for position_index in 0..vector.len() {
          if vector[position_index] == '>' {
            // Calculate destination
            let position = matrix.index_to_coords(&position_index).unwrap();
            let mut destination = position.clone();
            destination[0] = (destination[0] + 1) % matrix.dimensions[0];
            let destination_index = matrix.coords_to_index(&destination).unwrap();
            // Move to destination if destination is free
            if vector[destination_index] == '.' {
              updated_vector[destination_index] = vector[position_index];
              updated_vector[position_index] = '.';
              has_moved = true;
            }
          }
        }
        vector = updated_vector;

        // Check all down-facing members
        let mut updated_vector = vector.clone();
        for position_index in 0..vector.len() {
          if vector[position_index] == 'v' {
            // Calculate destination
            let position = matrix.index_to_coords(&position_index).unwrap();
            let mut destination = position.clone();
            destination[1] = (destination[1] + 1) % matrix.dimensions[1];
            let destination_index = matrix.coords_to_index(&destination).unwrap();
            // Move to destination if destination is free
            if vector[destination_index] == '.' {
              updated_vector[destination_index] = vector[position_index];
              updated_vector[position_index] = '.';
              has_moved = true;
            }
          }
        }
        vector = updated_vector;

        // Display matrix state
        // DotDisplay::print_2d_matrix(&matrix, &vector);
        // println!("");

        // Check if anyone has moved
        if !has_moved { break; }
        step += 1;
      }

      // Calculate and return result
      String::from(format!("{:?}", step + 1))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 25,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |_: String| {
      // Calculate and return result
      String::from(format!("{}", "Done!"))
    }

  );

  // Return registry ownership
  registry
}
