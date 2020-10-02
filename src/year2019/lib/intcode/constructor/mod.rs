//! IntCode ::new() implementation
//! 
//! Implementes often used constructor patterns
// -----------------------------------------------------------------------------

// Include dependencies
use super::*;

/// IntCode ::new() implementation
/// 
/// Implementes often used constructor patterns
impl IntCode {
  
  /// Instantiates a new IntCode instance
  /// 
  /// # Arguments
  /// 
  /// * `memory` - Initial memory setup
  pub fn new (memory: Vec<i64>) -> IntCode {
    IntCode{
      _ip: 0,
      memory,
      ..Default::default()
    }
  }

}
