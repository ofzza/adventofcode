//! 2021 day 11 puzzle
//! 
//! https://adventofcode.com/2021/day/11
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2021::lib::matrix::*;

/// Parses input data
fn parse(data: &String) -> Vec<Vec<isize>> {
  Input::parse(data.trim(), "\n", |line| {
    Input::parse(line.trim(), "", |n| { n.parse::<isize>().unwrap() })
  })
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 11,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Store data into a 2D grid
      let matrix: Matrix = Matrix::new(vec![data[0].len(), data.len()]);
      let mut vector: Vec<isize> = Vec::with_capacity(matrix.length);
      for y in 0..data.len() {
        for x in 0..data[y].len() {
          vector.push(data[y][x]);
        }
      }

      let mut total = 0;
      let mut cache: Vec<isize> = Vec::with_capacity(matrix.length);
      for _ in 0..matrix.length { cache.push(-1); }
      for i in 0..100 {
        let popped = do_step(i as isize, &matrix, &mut vector, 10 as isize, 0 as isize, &mut cache, |_index| {});
        total += popped.len();
      }

      // Calculate and return result
      String::from(format!("{:?}", total))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 11,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Store data into a 2D grid
      let matrix: Matrix = Matrix::new(vec![data[0].len(), data.len()]);
      let mut vector: Vec<isize> = Vec::with_capacity(matrix.length);
      for y in 0..data.len() {
        for x in 0..data[y].len() {
          vector.push(data[y][x]);
        }
      }

      let mut step = 0;
      let mut cache: Vec<isize> = Vec::with_capacity(matrix.length);
      for _ in 0..matrix.length { cache.push(-1); }
      loop {
        let popped = do_step(step as isize, &matrix, &mut vector, 10 as isize, 0 as isize, &mut cache, |_index| {});
        if popped.len() != vector.len() {
          step += 1;
        } else {
          break;
        }
      }

      // Calculate and return result
      String::from(format!("{:?}", step + 1))
    }

  );

  // Return registry ownership
  registry
}

/// Performs a single time step of the model
/// 
/// # Arguments
/// * step:         Index of the time step being modeled
/// * matrix:       Matrix of 2D data being used
/// * vector:       Vector containing data being used
/// * pop_value:    Evergy value over which octopuses "pop"
/// * reset_value:  Energy value octopuses reset to once they had popped
/// * cache:        Cache used to keep track which octopus has last popped during which step
/// * callback:     Callback function called when an octopus "pops"
/// 
/// # Returns
/// Vector of indexes of octopuses that have "popped" during this time step
/// 
fn do_step (step: isize, matrix: &Matrix, vector: &mut Vec<isize>, pop_value: isize, reset_value: isize, cache: &mut Vec<isize>, callback: fn(i: usize) -> ()) -> Vec<usize> {

  // Increase energy level of al octopuses
  for i in 0..vector.len() {
    vector[i] += 1;
  }

  // Find any octopi ready to "pop"
  let mut ready_to_pop: Vec<usize> = Vec::with_capacity(vector.len());
  for i in 0..vector.len() {
    if vector[i] >= pop_value {
      ready_to_pop.push(i);
    }
  }

  // Keep "popping" octopi which are ready to "pop" and hadn't yet in this step
  let mut popped: Vec<usize> = Vec::with_capacity(vector.len());
  let mut i: usize = 0;
  while i < ready_to_pop.len() {

    // Get index of current octopus
    let index = ready_to_pop[i];
    i += 1;

    // Check if octopus already popped this step
    if cache[index] == step { continue; }

    // Pop octopus
    cache[index] = step;
    popped.push(index);
    callback(i);

    // Find neighbouring octopi and try scheduling them to "pop" as well
    let neighbours = matrix.get_neighbours(&index, true);
    for i in 0..neighbours.len() {
      // Increase neighbouring octopus energy level
      vector[neighbours[i]] += 1;
      // Schedule those ready to "pop" to "pop"
      if vector[neighbours[i]] >= pop_value && cache[neighbours[i]] != step {
        ready_to_pop.push(neighbours[i]);
      }
    }
  }

  // Reset all octopuses which have popped
  for i in 0..popped.len() {
    vector[popped[i]] = reset_value;
  }

  // Return array of popped octopi
  popped
}
