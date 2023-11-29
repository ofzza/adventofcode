//! Grove Positioning System module
//! 
//! Grove Positioning System module
// -----------------------------------------------------------------------------

/// Grove Positioning System structure
pub struct GPS {
  // Original values as set
  values: Vec<i64>,
  // Current position of each mixed value indexed by its original position
  position_by_index: Vec<u64>
}

/// Grove Positioning System implementation
impl GPS {

  /// Constructor
  pub fn new (values: Vec<i64>) -> GPS {
    // Make a clone of the positions vector 
    let mut position_by_index: Vec<u64> = vec![0; values.len()];
    for i in 0..position_by_index.len() {
      position_by_index[i] = i as u64;
    }
    // Return new instance of GPS
    GPS {
      // Original values as set
      values,
      // Current position of each mixed value indexed by its original position
      position_by_index,
    }
  }

  /// Mixes up the values by moving them accoring to their values
  /// 
  /// # Returns
  /// Mixed up values
  pub fn mix (&mut self) -> Vec<i64> {
    // Mix up values, one by one
    for i in 0..self.position_by_index.len() {
      // Move according to value
      self.move_value(i);
    }

    // Collect values by their new positions
    self.get_values()
  }

  /// Moves a value a requested distance
  /// 
  /// # Arguments
  /// * index: Index to move
  fn move_value (&mut self, i: usize) {
    // Check if value to move by is 0
    if self.values[i] == 0 { return; }
    // Get value and current position
    let len: u64 = self.position_by_index.len() as u64;
    let current_position = self.position_by_index[i];
    let distance = (if self.values[i] >= 0 { 1 } else { -1 }) * ((self.values[i].abs() as u64) % (len as i64 - 1) as u64) as i64;
    let distance = (distance + (len - 1) as i64) as u64 % (len - 1);
    // Determine starting and ending position
    let start_position: u64 = current_position;
    let end_position: u64 = if current_position + distance >= len { (current_position + distance) % (len - 1) } else { current_position + distance };
    // Check if value to move by is 0
    if end_position == start_position { return; }
    // Move all items in between
    for j in 0..(len as usize) {
      let position = self.position_by_index[j];
      if (position > start_position && position <= end_position) || (position >= end_position && position < start_position) {
        let diff = if start_position < end_position { -1 } else { 1 };
        self.position_by_index[j] = (self.position_by_index[j] as i64 + diff) as u64;
      }
    }
    // Move item in question
    self.position_by_index[i] = end_position;
  }

  /// Composes values in their current, mixed order
  /// 
  /// # Returns
  /// Values in their current, mixed order
  fn get_values (&self) -> Vec<i64> {
    // Get current positions
    let mut index_by_position: Vec<u64> = vec![0; self.position_by_index.len()];
    for i in 0..self.position_by_index.len() {
      index_by_position[self.position_by_index[i] as usize] = i as u64;
    }
    // Prompt values in their correct positions
    index_by_position.iter().map(|i| self.values[i.clone() as usize]).collect::<Vec<i64>>()
  }
}
