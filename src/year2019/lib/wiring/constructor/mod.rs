//! Wiring ::new() implementation
//! 
//! Wiring constructor
// -----------------------------------------------------------------------------

// Include dependencies
use super::*;

/// Wiring ::new() implementation
/// 
/// Wiring constructor
impl Wiring {
  
  /// Instantiate a new Wiring and add wires instructions
  /// 
  /// # Arguments
  ///
  /// * `wires` - Wires' instructions
  pub fn new (
    mut wires: Vec<Vec<String>>
  ) -> Wiring {
    
    // Instantiate wiring
    let mut wiring = Wiring{
      ..Default::default()
    };

    // Instantiate orientations
    wiring.segments_by_orientation.insert(WireSegmentOrientation::Horizontal, Vec::new());
    wiring.segments_by_orientation.insert(WireSegmentOrientation::Vertical, Vec::new());

    // Add wires
    for wire in wires.iter_mut() {
      wiring.add(wire);
    }

    // Return initialized wiring
    wiring
    
  }

}
