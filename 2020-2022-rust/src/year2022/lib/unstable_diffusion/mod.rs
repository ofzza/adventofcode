//! Unstable Diffusion module
//! 
//! Unstable Diffusion module
// -----------------------------------------------------------------------------

// Include dependencies
use std::collections::hash_set::HashSet;
use std::collections::hash_map::HashMap;
use crate::year::lib::dot_display::DotDisplay;

/// Unstable Diffusion structure
pub struct UnstableDiffusion  {
  // Directions stack
  directions: [char; 4],
  // The field of coordinates
  pub points: HashSet<(isize, isize)>
}

/// Unstable Diffusion implementation
impl UnstableDiffusion {

  /// Constructor
  pub fn new (data: Vec<Vec<&str>>) -> UnstableDiffusion {
    // Initialize a hashset
    let mut points: HashSet<(isize, isize)> = HashSet::with_capacity(data.len());
    // Fill hashset
    for y in 0..data.len() {
      for x in 0..data[y].len() {
        if data[y][x] == "#" {
          points.insert((x as isize, y as isize));
        }
      }
    }
    // Return new instance of Unstable Diffusion
    UnstableDiffusion {
      // Directions stack
      directions: ['N', 'S', 'W', 'E'],
      // The field of coordinates
      points
    }
  }

  /// Play a single round of movement according to Unstable Diffusion rules
  /// 
  /// # Returns
  /// Number of coordinates that have chamged
  pub fn play_round (&mut self) -> usize {
    // Plan next step
    let mut hash: HashMap<(isize, isize), Vec<(isize, isize)>> = HashMap::with_capacity(self.points.len());
    for point in self.points.iter() {
      
      // Consider directions
      let mut point_next: (isize, isize) = point.clone();
      // Consider not moving
      if self.points.contains(&(point.0 + 1, point.1 - 1))
      || self.points.contains(&(point.0 + 1, point.1 + 0))
      || self.points.contains(&(point.0 + 1, point.1 + 1))
      || self.points.contains(&(point.0 + 0, point.1 - 1))
      || self.points.contains(&(point.0 + 0, point.1 + 1))
      || self.points.contains(&(point.0 - 1, point.1 - 1))
      || self.points.contains(&(point.0 - 1, point.1 + 0))
      || self.points.contains(&(point.0 - 1, point.1 + 1)) {

        for direction in self.directions {
          // Consider moving north
          if direction == 'N' {
            if !self.points.contains(&(point.0 + 0, point.1 - 1))
            && !self.points.contains(&(point.0 - 1, point.1 - 1))
            && !self.points.contains(&(point.0 + 1, point.1 - 1)) {
              point_next = (point.0 + 0, point.1 - 1);
              break;
            }
          }
          // Consider moving south
          else if direction == 'S' {
            if !self.points.contains(&(point.0 + 0, point.1 + 1))
            && !self.points.contains(&(point.0 - 1, point.1 + 1))
            && !self.points.contains(&(point.0 + 1, point.1 + 1)) {
              point_next = (point.0 + 0, point.1 + 1);
              break;
            }
          }
          // Consider moving east
          else if direction == 'W' {
            if !self.points.contains(&(point.0 - 1, point.1 + 0))
            && !self.points.contains(&(point.0 - 1, point.1 - 1))
            && !self.points.contains(&(point.0 - 1, point.1 + 1)) {
              point_next = (point.0 - 1, point.1 + 0);
              break;
            }
          }
          // Consider moving west
          else if direction == 'E' {
            if !self.points.contains(&(point.0 + 1, point.1 + 0))
            && !self.points.contains(&(point.0 + 1, point.1 - 1))
            && !self.points.contains(&(point.0 + 1, point.1 + 1)) {
              point_next = (point.0 + 1, point.1 + 0);
              break;
            }
          }
        }

      }

      // Store next point
      if !hash.contains_key(&point_next) {
        hash.insert(point_next, vec![point.clone()]);
      }
      // Update next point
      else {
        hash.get_mut(&point_next).unwrap().push(point.clone());
      }

    }

    // Rotate directions
    self.directions.rotate_left(1);

    // Calculate next points
    let mut points: HashSet<(isize, isize)> = HashSet::with_capacity(self.points.capacity());
    let mut moves_count = 0;    
    for (point_next, points_previous) in hash.iter() {
      // Move if only candidate for the coordinates
      if points_previous.len() == 1 {
        points.insert(point_next.clone());
        if points_previous[0].0 != point_next.0 || points_previous[0].1 != point_next.1 {
          moves_count += 1;
        }
      }
      // .... or stay put
      else {
        for point_previous in points_previous {
          points.insert(point_previous.clone());
        }
      }
    }

    // Store next points
    self.points = points;

    // Return number of moves
    moves_count
  }

  /// Gets bounds of all the coordinates on the field
  /// 
  /// # Returns
  /// Bounds of all the coordinates on the field
  pub fn get_bounds (&self) -> (usize, usize) {
    // Find min/max points
    let max: (isize, isize) = (
      self.points.iter().map(|p| p.0).collect::<Vec<isize>>().iter().max().unwrap().clone(),
      self.points.iter().map(|p| p.1).collect::<Vec<isize>>().iter().max().unwrap().clone()
    );
    let min: (isize, isize) = (
      self.points.iter().map(|p| p.0).collect::<Vec<isize>>().iter().min().unwrap().clone(),
      self.points.iter().map(|p| p.1).collect::<Vec<isize>>().iter().min().unwrap().clone()
    );
    // Calculate and return bounds
    ((max.0 - min.0 + 1) as usize, (max.1 - min.1 + 1) as usize)
  }

  /// Prompts current state of the field of coordinates
  pub fn _prompt (&self) {
    // Draw coordinates
    let points = &self.points.iter().map(|p| p.clone()).collect::<Vec<(isize, isize)>>();
    let normalized = DotDisplay::normalize(points);
    DotDisplay::print_binary(normalized);
  }
}
