//! Wiring segment struct
// -----------------------------------------------------------------------------

// Include dependencies
use super::*;

/// Wiring segment struct
/// 
/// TODO: more details ...
#[derive(Default, Clone)]
pub struct WireSegment {
  pub wire_index: u32,
  pub accumulated_distance: u32,
  pub start: WiringCoordinates,
  pub end: WiringCoordinates
}

/// Wiring segment orientation
#[derive(Eq, PartialEq, Hash)]
pub enum WireSegmentOrientation {
  Horizontal,
  Vertical
}

// Witing segment methods
impl WireSegment {

  /// Calculates wiring segment orientation (hotizontal/vertical)
  pub fn get_orientation (&self) -> WireSegmentOrientation {
    return if self.start.x != self.end.x { WireSegmentOrientation::Horizontal } else { WireSegmentOrientation::Vertical }
  }

  /// Finds an intersection between wire segments
  /// 
  /// # Arguments
  /// - `segment` - Wire segment instance to check for intersection with
  pub fn find_intersection (&self, intersection_candidate_segment: &Self) -> Option<WiringCoordinates> {
    // Check if different orientations
    if self.get_orientation() != intersection_candidate_segment.get_orientation() {
      // Check if different wires
      if &self.wire_index != &intersection_candidate_segment.wire_index {
        // Check if intersection
        let horizontal_section = if self.get_orientation() == WireSegmentOrientation::Horizontal { self } else { intersection_candidate_segment };
        let horizontal_start_x = if horizontal_section.start.x < horizontal_section.end.x { horizontal_section.start.x } else { horizontal_section.end.x };
        let horizontal_end_x   = if horizontal_section.start.x > horizontal_section.end.x { horizontal_section.start.x } else { horizontal_section.end.x };
        let vertical_section   = if self.get_orientation() == WireSegmentOrientation::Vertical { self } else { intersection_candidate_segment };
        let vertical_start_y   = if vertical_section.start.y < horizontal_section.end.y { vertical_section.start.y } else { vertical_section.end.y };
        let vertical_end_y     = if vertical_section.start.y > horizontal_section.end.y { vertical_section.start.y } else { vertical_section.end.y };
        if ((horizontal_start_x < vertical_section.start.x) && (horizontal_end_x > vertical_section.start.x))
        && ((vertical_start_y < horizontal_section.start.y) && (vertical_end_y > horizontal_section.start.y)) {
          return Some(
            WiringCoordinates{
              x: vertical_section.start.x,
              y: horizontal_section.start.y
            }
          );
        }
      }
    }
    // Return default no intersection
    None
  }

}
