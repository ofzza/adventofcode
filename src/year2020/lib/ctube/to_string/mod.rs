//! "Conway tubes" .to_string() implementation
//! 
//! Returns a string representation of current state
// -----------------------------------------------------------------------------

// Include dependencies
use super::*;

/// "Conway tubes" .to_string() implementation
/// 
/// Returns a string representation of current state
impl ConwayTube {
  
  /// Returns a string representation of current state
  pub fn to_string (&mut self) -> String {
    let count_enabled = self.cubes.values()
      .map(|state| state.clone())
      .filter(|state| state.clone())
      .collect::<Vec<bool>>()
      .len();
    let count_disabled = self.cubes.values()
      .map(|state| state.clone())
      .filter(|state| !state.clone())
      .collect::<Vec<bool>>()
      .len();
    return format!("Total/Enabled/Disabled: {}/{}/{}", self.cubes.len(), count_enabled, count_disabled);
  }
}
