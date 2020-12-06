//! Wiring coordinates struct
// -----------------------------------------------------------------------------

// Include dependencies
use std::cmp::PartialEq;

/// Wiring coordinates struct
/// 
/// TODO: more details ...
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct WiringCoordinates {
  pub x: i32,
  pub y: i32
}
