//! Wiring module
//! 
//! Implements a Wiring diagram
// -----------------------------------------------------------------------------

// Import child modules
mod constructor;
mod add_method;
mod wiring_coordinates;
mod wiring_segment;

// (re)Export child modules
pub use constructor::*;
pub use add_method::*;
pub use wiring_coordinates::*;
pub use wiring_segment::*;

// Include dependencies
use std::collections::HashMap;

/// Wiring struct
/// 
/// TODO: more details ...
#[derive(Default)]
pub struct Wiring {
  // Number of wires added
  _count: u32,
  // Holds all wire segments
  pub segments: Vec<WireSegment>,
  // Holds all wire segments
  pub segments_by_orientation: HashMap<WireSegmentOrientation, Vec<WireSegment>>,
  // Holds all wire segments
  pub segments_by_wire_index: HashMap<u32, Vec<WireSegment>>,
  // Holds all intersections (wire numbers vector) for per coordinates
  pub intersections: HashMap<WiringCoordinates, Vec<WireSegment>>,
}
