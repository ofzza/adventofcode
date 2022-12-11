//! Rope module
//! 
//! Rope Physics Simulation module
// -----------------------------------------------------------------------------

// Include dependecies
// ...

/// Rope Physics Simulation structure
pub struct Rope {
  pub sections: Vec<(isize, isize)>
}

/// Rope Physics Simulation implementation
impl Rope {

  /// Constructor
  /// 
  /// # Arguments
  /// * len: Number of sections simulating the rope
  pub fn new (len: usize) -> Rope {
    // Check for minimal length
    if len < 2 {
      panic!("Rope of length < 2 not allowed!");
    }
    // Initialize Rope
    Rope {
      sections: vec![(0, 0); len]
    }
  }

  /// Moves the head segment in a requested direction
  ///
  /// # Arguments
  /// * direction: Direction to move in ("U", "D", "L", or "R")
  /// * distance: Distance to move in the requested direction
  /// * callback: Callback function called after each incremental move
  /// * aggregate: Aggregating value being passed between itnerations of the callback execution
  /// 
  /// # Returns
  /// Aggregating value being passed between itnerations of the callback execution
  pub fn move_head<T>(&mut self, direction: &str, distance: usize, callback: fn(rope: &Rope, aggregate: T) -> T, mut aggregate: T) -> T  {
    // Initialize relative movement
    let d: (isize, isize);

    // Move up
    if direction == "U" {
      d = (0, -1);
    }
    // Move down
    else if direction == "D" {
      d = (0, 1);
    }
    // Move left
    else if direction == "L" {
      d = (-1, 0);
    }
    // Move right
    else if direction == "R" {
      d = (1, 0);
    } else {
      panic!("Unknown direction: {}!", direction);
    }

    // Move single step at a time and realign segments as you go
    for _ in 0..distance {
      // Move head
      self.sections[0] = (self.sections[0].0 + d.0, self.sections[0].1 + d.1);
      // Realign segments
      self.realign_segments();
      // Execute callback
      aggregate = callback(self, aggregate);
    }

    // Return aggregate value
    aggregate
  }

  /// Realigns all segments to their preceeding segment (if needed)
  fn realign_segments (&mut self) {
    for i in 1..self.sections.len() {
      // Get relative position
      let d = (self.sections[i].0 - self.sections[i - 1].0, self.sections[i].1 - self.sections[i - 1].1);
      // Check if correction required
      if isize::abs(d.0) > 1 || isize::abs(d.1) > 1 {
        self.sections[i] = (
          self.sections[i].0 - (if d.0 == 0 { 0 } else { d.0 / isize::abs(d.0) }),
          self.sections[i].1 - (if d.1 == 0 { 0 } else { d.1 / isize::abs(d.1) })
        );
      }
    }
  }

}

