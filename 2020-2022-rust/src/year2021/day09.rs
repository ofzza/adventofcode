//! 2021 day 09 puzzle
//! 
//! https://adventofcode.com/2021/day/9
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year::lib::matrix::*;

/// Parses input data
fn parse(data: &String) -> Vec<Vec<usize>> {
  Input::parse(data.trim(), "\n", |line| {
    Input::parse(line.trim(), "", |n| {
      n.parse::<usize>().unwrap()
    })
  })
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 9,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Store as 2D grid
      let matrix = Matrix::new(vec![data[0].len(), data.len()]);
      let mut vector: Vec<usize> = matrix.create();
      for y in 0..data.len() {
        for x in 0..data[y].len() {
          vector.push(data[y][x]);
        }
      }

      // Find local minimums
      let mins: Vec<usize> = find_mins(&matrix, &vector).iter().map(|i| { vector[i.clone()] }).collect();

      // Calculate and return result
      String::from(format!("{:?}", mins.iter().sum::<usize>() + mins.len()))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 9,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Store as 2D grid
      let matrix = Matrix::new(vec![data[0].len(), data.len()]);
      let mut vector: Vec<usize> = matrix.create();
      for y in 0..data.len() {
        for x in 0..data[y].len() {
          vector.push(data[y][x]);
        }
      }

      // Find local minimums
      let mins: Vec<usize> = find_mins(&matrix, &vector);
      let mut disqualified: Vec<bool> = Vec::with_capacity(vector.len());
      for _ in 0..vector.len() {
        disqualified.push(false);
      }

      // Find size of pool around each local minimum
      let mut sizes_total: usize = 0;
      let mut sizes: Vec<usize> = vec![];
      for i in 0..mins.len() {
        measure_pool_size(&matrix, &vector, mins[i], &mut disqualified);
        let size: usize = disqualified.iter().filter(|b| { b == &&true }).collect::<Vec<&bool>>().len();
        sizes.push(size - sizes_total);
        sizes_total = size;
      }

      // Find produt of sizes of 3 largest pools
      sizes.sort();
      let mut product: usize = 1;
      for i in (sizes.len() - 3)..sizes.len() {
        product *= sizes[i];
      }

      // Calculate and return result
      String::from(format!("{:?}", product))
    }

  );

  // Return registry ownership
  registry
}

/// Finds coordinates of local minimums
/// 
/// # Arguments
/// * matrix: Matrix of same dimensions and shape as the data
/// * vector: Linear vector of matrix data
/// 
/// # Returns
/// Vector of indexes of local minimums
fn find_mins (matrix: &Matrix, vector: &Vec<usize>) -> Vec<usize> {
  // Initilalize disqualified fields cache
  let mut disqualified: Vec<bool> = Vec::with_capacity(vector.len());
  for _ in 0..vector.len() {
    disqualified.push(false);
  }

  // Search for minimums
  let mut mins: Vec<usize> = vec![];
  for i in 0..vector.len() {
    // Check if already compared
    if disqualified[i] {
      continue;
    }
    // Get value and neighbours
    let value = vector[i];
    let neighbours = matrix.get_neighbours(&i, false);
    // Compare to neighbours
    let mut is_min = true;
    for i in 0..neighbours.len() {
      if value >= vector[neighbours[i]] {
        is_min = false; break;
      } else {
        disqualified[neighbours[i]] = true;
      }
    }
    if is_min {
      mins.push(i);
    }
  }

  // Return found minimums
  mins
}

/// Finds size of pool around a starting minimum
/// 
/// # Arguments
/// * matrix:       Matrix of same dimensions and shape as the data
/// * vector:       Linear vector of matrix data
/// * index:        Current position index being searched around
/// * disqualified: Shared vector used to disqualify counted points form being counted again
fn measure_pool_size (matrix: &Matrix, vector: &Vec<usize>, index: usize, disqualified: &mut Vec<bool>) {
  // Check if already disqualified
  if !disqualified[index] {
    disqualified[index] = true;
    // Get neighbours of current point
    let neighbours = matrix.get_neighbours(&index, false);
    // Find new non-border neighbours
    for i in 0..neighbours.len() {
      if vector[neighbours[i]] < 9 {
        measure_pool_size(matrix, &vector, neighbours[i], disqualified);
      }
    }
  }
}
