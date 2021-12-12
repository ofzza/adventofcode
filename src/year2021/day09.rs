//! 2021 day 09 puzzle
//! 
//! https://adventofcode.com/2021/day/9
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2021::lib::matrix::*;

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
      let mut matrix = Matrix::new(vec![data[0].len(), data.len()], vec![] as Vec<usize>);
      for y in 0..data.len() {
        for x in 0..data[y].len() {
          matrix.data.push(data[y][x]);
        }
      }

      // Find local minimums
      let mins: Vec<usize> = find_mins(&matrix).iter().map(|p| { matrix.get(&p).unwrap().clone() }).collect();

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
      let mut matrix = Matrix::new(vec![data[0].len(), data.len()], vec![] as Vec<usize>);
      for y in 0..data.len() {
        for x in 0..data[y].len() {
          matrix.data.push(data[y][x]);
        }
      }

      // Find local minimums
      let mins: Vec<Vec<usize>> = find_mins(&matrix);

      // Find size of pool around each local minimum
      let mut sizes: Vec<usize> = vec![];
      for i in 0..mins.len() {
        let mut disqualified: Vec<bool> = Vec::with_capacity(matrix.data.len());
        for _ in 0..matrix.data.len() {
          disqualified.push(false);
        }
        measure_pool_size(&matrix, &mins[i], &mut disqualified);
        let size: usize = disqualified.iter().filter(|b| { b == &&true }).collect::<Vec<&bool>>().len();
        sizes.push(size);
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
/// * matrix: Matrix with values to find minimums within
/// 
/// # Returns
/// Vector of coordinates of local minimums
fn find_mins (matrix: &Matrix<usize>) -> Vec<Vec<usize>> {
  // Initilalize disqualified fields cache
  let mut disqualified: Vec<bool> = Vec::with_capacity(matrix.data.len());
  for _ in 0..matrix.data.len() {
    disqualified.push(false);
  }

  // Search for minimums
  let mut mins: Vec<Vec<usize>> = vec![];
  for y in 0..matrix.dimensions[1] {
    for x in 0..matrix.dimensions[0] {
      // Check if already compared
      if disqualified[matrix.coords_to_index(&vec![x, y]).unwrap()] {
        continue;
      }
      // Get value and neighbours
      let value = matrix.get(&vec![x, y]).unwrap();
      let neighbours = matrix.get_neighbours(&vec![x, y], false);
      // Compare to neighbours
      let mut is_min = true;
      for i in 0..neighbours.len() {
        if value >= neighbours[i].1 {
          is_min = false; break;
        } else {
          disqualified[matrix.coords_to_index(&neighbours[i].0).unwrap()] = true;
        }
      }
      if is_min {
        mins.push(vec![x, y]);
      }
    }
  }

  // Return found minimums
  mins
}

/// Finds size of pool around a starting minimum
/// 
/// # Arguments
/// * matrix:       Matrix with values to find minimums within
/// * start:        Starting minimum to search around
/// * disqualified: Shared vector used to disqualify counted points form being counted again
fn measure_pool_size (matrix: &Matrix<usize>, start: &Vec<usize>, disqualified: &mut Vec<bool>) {
  // Check if already disqualified
  let index = matrix.coords_to_index(&start).unwrap();
  if !disqualified[index] {
    disqualified[index] = true;
    // Get neighbours of current point
    let neighbours = matrix.get_neighbours(&start, false);
    // Find new non-border neighbours
    for i in 0..neighbours.len() {
      if neighbours[i].1 < &9 {
        measure_pool_size(matrix, &neighbours[i].0, disqualified);
      }
    }
  }
}
