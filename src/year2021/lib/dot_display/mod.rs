//! Dot display module
//! 
//! Implements dot display functionality
// -----------------------------------------------------------------------------

/// Dot display struct
pub struct DotDisplay {}
/// Dot display implementation
impl DotDisplay {
  /// Print points to display
  /// 
  /// # Arguments
  /// * points: Vector of points to display
  pub fn print (points: Vec<(usize, usize)>) {
    // Find max coordinates
    let max: (usize, usize) = (
      points.iter().map(|p| p.0).collect::<Vec<usize>>().iter().max().unwrap().clone(),
      points.iter().map(|p| p.1).collect::<Vec<usize>>().iter().max().unwrap().clone()
    );
    // Print out display
    for y in 0..(max.1 + 1) {
      for x in 0..(max.0 + 1) {
        print!("{}", if points.iter().find(|p| p.0 == x && p.1 == y) != None { "#" } else { "." });
      }
      println!();
    }
  }
}
