//! Math utils module
//! 
//! Math functionality
// -----------------------------------------------------------------------------

/// Math utils struct
#[allow(dead_code)]
pub struct Math {}

/// Math utils implementation
impl Math {

  /// Wraps index value within a range between 0 and a provided "modulo" number
  /// 
  /// # Arguments
  /// * value: Value to wrap
  /// * module: High end of the range to wrap into
  /// 
  /// # Returns
  /// Wrapped value
  pub fn index_wraparound (value: isize, modulo: usize) -> usize {
    ((value % modulo as isize) + modulo as isize) as usize % modulo
  }

  /// Mirrors index value within a range between 0 and a provided "modulo" number
  /// 
  /// # Arguments
  /// * value: Value to mirror
  /// * module: High end of the range to mirror within
  /// 
  /// # Returns
  /// Mirrored value
  pub fn index_mirror (value: usize, modulo: usize) -> usize {
    modulo - value - 1
  }

}
