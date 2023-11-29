//! Sequences module
//! 
//! Numeric sequence manipulation module
// -----------------------------------------------------------------------------

/// Sequences struct
pub struct NumericSequence {
  pub interval: usize,
  values: Vec<usize>,
  sum: usize
}
/// Sequences implementation
impl NumericSequence {

  /// Constructor
  /// 
  /// # Arguments
  /// * data
  /// * definition: Iterative definition of the sequence
  pub fn new<T> (data: &T, definition: fn(i: usize, prev: &usize, data: &T) -> (usize, usize)) -> NumericSequence {
    // Detect repeating interval
    let mut i = 0;
    let mut values: Vec<(usize, usize)> = vec![];
    loop {
      // Calculate next value
      let value = definition(i, if i > 0 { &values[values.len() - 1].1 } else { &0 }, &data);
      values.push(value);
      i += 1;
      // Check for repeating interval
      if values.len() > 2 {
        let prelast_values: (usize, usize) = values[values.len() - 2];
        let last_values: (usize, usize) = values[values.len() - 1];
        if values[0].0 == prelast_values.0 && values[0].1 == prelast_values.1 && values[1].0 == last_values.0 && values[1].1 == last_values.1 {
          // Numeric sequence defined, return numeric sequence
          let values: Vec<usize> = values[0..(values.len() - 2)].iter().map(|v| v.1).collect();
          let sum = values.iter().sum();
          return NumericSequence {
            interval: values.len(),
            values,
            sum: sum
          }
        }
      }
    }
  }

  /// Gets value for any element of the sequence
  /// 
  /// # Arguments
  /// * index: Index of the element
  /// 
  /// # Returns
  /// Value of the element
  pub fn get_value_for_index (&self, index: &usize) -> usize {
    return self.values[index % self.interval];
  }

  /// Gets sum of all values up to a provided index
  /// 
  /// # Arguments
  /// * index: Index of the last element to sum
  /// 
  /// # Returns
  /// Sum of all elements up the the provided index
  pub fn get_sum_for_index(&self, index: &usize) -> usize {
    // Skip indexes until right interval
    let skipped_intervals: usize = index / self.interval;
    // Return sum
    let extra_sum: usize = self.values[0..(index % (self.interval))].iter().sum();
    (skipped_intervals * self.sum) + extra_sum
  }

  /// Gets index and sum where sum of all elements in the sequence up to this point is larger than a provided limit
  /// 
  /// # Arguments
  /// * limit: Limit being searched for
  /// 
  /// # Returns
  /// Tuple of (index, sum) for the index where sum is larger than te requested limit
  pub fn get_index_where_sum_more_than (&self, limit: usize) -> (usize, usize) {
    // Skip indexes until the right interval
    let skipped_intervals: usize = limit / self.sum;
    // Find index where sum over limit
    let mut i: usize = 0;
    let mut sum = skipped_intervals * self.sum;
    while sum <= limit {
      sum += self.values[i];
      i += 1;
    }
    // Return index where sum is over limit
    (skipped_intervals * self.interval + i - 1, sum)
  }

}
