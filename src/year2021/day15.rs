//! 2021 day 15 puzzle
//! 
//! https://adventofcode.com/2021/day/15
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2021::lib::matrix::*;

/// Parses input data
fn parse(data: &String) -> Vec<Vec<usize>> {
  Input::parse(data.trim(), "\n", |line| {
    Input::parse(line.trim(), "", |n| { n.parse::<usize>().unwrap() })
  })
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 15,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize a matrix for the data
      let matrix = Matrix::new(vec![data[0].len(), data.len()]);
      let mut vector: Vec<usize> = Vec::with_capacity(matrix.length);
      for y in 0..matrix.dimensions[1] {
        for x in 0..matrix.dimensions[0] {
          vector.push(data[y][x]);
        }
      }

      // Generate distance map
      let distances = find_distances(&vector, &matrix, 1);

      // Calculate and return result
      String::from(format!("{:?}", distances.last().unwrap()))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 15,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize a matrix for the data
      let matrix = Matrix::new(vec![data[0].len(), data.len()]);
      let mut vector: Vec<usize> = Vec::with_capacity(matrix.length);
      for y in 0..matrix.dimensions[1] {
        for x in 0..matrix.dimensions[0] {
          vector.push(data[y][x]);
        }
      }

      // Generate distance map
      let distances = find_distances(&vector, &matrix, 5);

      // Calculate and return result
      String::from(format!("{:?}", distances.last().unwrap()))
    }

  );

  // Return registry ownership
  registry
}

/// Generates a 2D distance map of all points on the full, scaled map
/// 
/// # Arguments
/// * vector:       Vector of 2D data of original risk factors
/// * data_matrix:  Matrix instance describing the 2D data
/// * scale:        Scale of the full map relative to the data section
/// 
/// # Returns
/// 2D distance map of all points on the full, scaled map
fn find_distances (vector: &Vec<usize>, data_matrix: &Matrix, scale: usize) -> Vec<usize> {
  // Initialize distances
  let mut distances: Vec<usize> = Vec::with_capacity(data_matrix.length * scale * scale);
  let distances_matrix: Matrix = Matrix::new(vec![data_matrix.dimensions[0] * scale, data_matrix.dimensions[1] * scale]);
  
  distances.push(0);
  for _ in 1..(data_matrix.length * scale * scale) {
    distances.push(10 * data_matrix.length * scale * scale);
  }

  // Iterate on optimizing distance values
  let mut optimizations_found = true;
  while optimizations_found {
    // Reset optimizations found
    optimizations_found = false;

    // Search for optimizations for every point
    for i in 0..(data_matrix.length * scale * scale) {

      // Get coordinates of current point
      let coords = distances_matrix.index_to_coords(&i).unwrap();

      // Get value of corresponding data field
      let data_x = coords[0] % data_matrix.dimensions[0];
      let data_correction_x = coords[0] / data_matrix.dimensions[0];
      let data_y = coords[1] % data_matrix.dimensions[1];
      let data_correction_y = coords[1] / data_matrix.dimensions[1];
      let data_index = data_matrix.coords_to_index(&vec![data_x, data_y]).unwrap();
      let data_value = (vector[data_index] + data_correction_x + data_correction_y - 1) % 9 + 1;

      // Check optimization based on relative point: top
      if coords[1] > 0 {
        let top_coords = vec![coords[0], coords[1] - 1];
        let top_index = distances_matrix.coords_to_index(&top_coords).unwrap();
        if distances[top_index] + data_value < distances[i] {
          distances[i] = distances[top_index] + data_value;
          optimizations_found = true;
        }
      }

      // Check optimization based on relative point: left
      if coords[0] > 0 {
        let left_coords = vec![coords[0] - 1, coords[1]];
        let left_index = distances_matrix.coords_to_index(&left_coords).unwrap();
        if distances[left_index] + data_value < distances[i] {
          distances[i] = distances[left_index] + data_value;
          optimizations_found = true;
        }
      }

      // Check optimization based on relative point: right
      if coords[0] < distances_matrix.dimensions[0] - 1 {
        let left_coords = vec![coords[0] + 1, coords[1]];
        let left_index = distances_matrix.coords_to_index(&left_coords).unwrap();
        if distances[left_index] + data_value < distances[i] {
          distances[i] = distances[left_index] + data_value;
          optimizations_found = true;
        }
      }

      // Check optimization based on relative point: bottom
      if coords[1] < distances_matrix.dimensions[1] - 1 {
        let top_coords = vec![coords[0], coords[1] + 1];
        let top_index = distances_matrix.coords_to_index(&top_coords).unwrap();
        if distances[top_index] + data_value < distances[i] {
          distances[i] = distances[top_index] + data_value;
          optimizations_found = true;
        }
      }
    }
  }

  // Return calculated distances
  distances
}
