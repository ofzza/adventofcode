//! Dot display module
//! 
//! Implements dot display functionality
// -----------------------------------------------------------------------------

// Include dependencies
use crate::year::lib::matrix::*;

/// Dot display struct
pub struct DotDisplay {}
/// Dot display implementation
impl DotDisplay {

  /// Print points to display
  /// 
  /// # Arguments
  /// * points: Vector of points to display
  pub fn print_binary (points: Vec<(usize, usize)>) {
    DotDisplay::print(points, |_, _| '#', ());
  }

  /// Print points to display
  /// 
  /// # Arguments
  /// * points: Vector of points to display
  /// * callback: Callback which returns a character to represent any coordinates
  /// * reference: Reference value passed to all calls of the callback function
  pub fn print<T> (points: Vec<(usize, usize)>, callback: fn(&T, (usize, usize)) -> char, reference: T) {
    // Find max coordinates
    let max: (usize, usize) = (
      points.iter().map(|p| p.0).collect::<Vec<usize>>().iter().max().unwrap().clone(),
      points.iter().map(|p| p.1).collect::<Vec<usize>>().iter().max().unwrap().clone()
    );
    let min: (usize, usize) = (
      points.iter().map(|p| p.0).collect::<Vec<usize>>().iter().min().unwrap().clone(),
      points.iter().map(|p| p.1).collect::<Vec<usize>>().iter().min().unwrap().clone()
    );
    // Print out display
    for y in min.1..(max.1 + 1) {
      for x in min.0..(max.0 + 1) {
        print!("{}", if points.iter().find(|p| p.0 == x && p.1 == y) != None { callback(&reference, (x, y)) } else { '.' });
      }
      println!();
    }
  }

  /// Print matrix to display
  /// 
  /// # Arguments
  /// * points: Vector of points to display
  #[allow(dead_code)]
  pub fn print_2d_matrix<T> (matrix: &Matrix, vector: &Vec<T>) where T: std::fmt::Display {
    // Print out display
    for y in 0..matrix.dimensions[1] {
      for x in 0..matrix.dimensions[0] {
        print!("{}", vector[matrix.coords_to_index(&vec![x, y]).unwrap()]);
      }
      println!();
    }
  }


}
