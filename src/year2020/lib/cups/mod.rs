//! Ring of cups module
//! 
//! Implements a Ring of cups functionality
// -----------------------------------------------------------------------------

/// Ring of cups struct
#[derive(Debug, Default, Clone)]
pub struct RingOfCups {
  // Get currently pointed to value
  pub current_value: usize,
  // Holds cups by value
  pub cups_by_value: Vec<(usize, usize)>,
  // Full size of the ring
  pub length: usize
}

// Implements basic functionality of a ring of cups
impl RingOfCups {

  /// Creates a new ring of cups
  /// 
  /// # Arguments
  /// * `cups`               - Initial (partial) arrangement of cups
  /// * `cups_expanded_size` - Final number of cups, achieved by adding cups with increasing values after the last specified one
  pub fn new (cups: &Vec<usize>, cups_expanded_size: usize) -> RingOfCups {
    // Initialize cups
    let mut cups_by_value: Vec<(usize, usize)> = vec![(0, 0); cups_expanded_size + 1];
    // Initialize cups ring values
    for i in 0..cups_expanded_size {
      let value = RingOfCups::get_initial_value(i as isize, cups, cups_expanded_size);
      cups_by_value[value] = (
        RingOfCups::get_initial_value(i as isize - 1, cups, cups_expanded_size),
        RingOfCups::get_initial_value(i as isize + 1, cups, cups_expanded_size)
      );
    }
    // Return a ring of cups instance
    return RingOfCups {
      cups_by_value,
      current_value: cups[0],
      length: cups_expanded_size
    }
  }

  /// Compose initial state of a cup
  /// 
  /// # Arguments
  /// * `index`              - Index of a cup
  /// * `cups`               - Explicit values of a limited starting subset of all cups
  /// * `cups_expanded_size` - Total number of all cups
  fn get_initial_value (index: isize, cups: &Vec<usize>, cups_expanded_size: usize) -> usize {
    let index = (cups_expanded_size as isize + index) as usize % cups_expanded_size;
    return if index < cups.len() { cups[index] } else { index + 1 };
  }

  /// Moves a number of sequential cups from one position to another
  /// 
  /// # Arguments
  /// * `first_value`       - First in the sequence of values to move
  /// * `values_length`     - Length of sequence of values to move
  /// * `destination_value` - Value after which to place the moved sequence of values
  pub fn move_cups (&mut self, first_value: usize, values_length: usize, destination_value: usize) -> () {
    // Find last picked up values
    let mut last_value = first_value;
    for _ in 0..(values_length - 1) {
      last_value = self.cups_by_value[last_value].1;
    }
    // Detach picked up values
    let previous_value = self.cups_by_value[first_value].0;
    let next_value = self.cups_by_value[last_value].1;
    self.cups_by_value[previous_value].1 = next_value;
    self.cups_by_value[next_value].0 = previous_value;
    // Reattach picked up values
    let next_value = self.cups_by_value[destination_value].1;
    self.cups_by_value[destination_value].1 = first_value;
    self.cups_by_value[first_value].0 = destination_value;
    self.cups_by_value[last_value].1 = next_value;
    self.cups_by_value[next_value].0 = last_value;
  }

}
