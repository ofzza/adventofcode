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

  /// Normalizes a vector of points to only positive coordinates
  /// 
  /// # Arguments
  /// * points: Pointe to normalize
  /// 
  /// # Returns
  /// Normalized coordiantes
  #[allow(dead_code)]
  pub fn normalize (points: &Vec<(isize, isize)>) -> Vec<(usize, usize)> {
    // Find min x and y coordinates
    let min: (isize, isize) = (
      points.iter().map(|p| p.0).collect::<Vec<isize>>().iter().min().unwrap().clone(),
      points.iter().map(|p| p.1).collect::<Vec<isize>>().iter().min().unwrap().clone()
    );
    // Calculate offset
    let offset = (
      if min.0 < 0 { min.0.abs() } else { 0 },
      if min.1 < 0 { min.1.abs() } else { 0 }
    );
    // Normalize coordinates
    points.iter().map(|c| ((c.0 + offset.0) as usize, (c.1 + offset.1) as usize)).collect::<Vec<(usize, usize)>>()
  }

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
