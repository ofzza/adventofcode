//! Wiring .add() implementation
//! 
//! Adds a new wire to the wiring structure
// -----------------------------------------------------------------------------

// Include dependencies
use super::*;

/// Wiring .add() implementation
/// 
/// Adds a new wire to the wiring structure
impl Wiring {
  
  /// Loads wire instructions into the wiring struct
  /// 
  /// # Arguments
  /// 
  /// * `instructions` - Wire layout instructions
  pub fn add (&mut self, instructions: &Vec<String>) {

    // Initialize new wire wire
    let wire_index = self._count;
    self._count += 1;

    // Initialize coordinates
    let mut start = WiringCoordinates{ x: 0, y: 0 };
    let mut end = WiringCoordinates{ x: 0, y: 0 };
    let mut accumulated_distance: u32 = 0;

    // Process instructions
    for instruction in instructions.iter() {

      // Decode instruction
      let direction = &instruction[0..1];
      let distance: u32 = String::from(&instruction[1..]).parse::<u32>()
        .expect("Failed parsing distance!");
      match direction {
        "U" => end.y -= distance as i32,
        "D" => end.y += distance as i32,
        "L" => end.x -= distance as i32,
        "R" => end.x += distance as i32,
        _   => {}
      }

      // Instantiate wire segment
      let wire_segment = WireSegment{
        wire_index,
        accumulated_distance,
        start: start.clone(),
        end: end.clone()
      };

      // Register wire segment
      &self.segments.push(wire_segment.clone());
      // Register wire segment by orientation
      self.segments_by_orientation.get_mut(&wire_segment.get_orientation())
        .expect("This should never, ever happen!")
        .push(wire_segment.clone());
      // Register wire segment by wire index
      if !self.segments_by_wire_index.contains_key(&wire_segment.wire_index) {
        let segments_by_wire_index: Vec<WireSegment> = Vec::new();
        self.segments_by_wire_index.insert(wire_segment.wire_index, segments_by_wire_index);
      };
      self.segments_by_wire_index.get_mut(&wire_segment.wire_index)
        .expect("This should never, ever happen!")
        .push(wire_segment.clone());

      // Check for intersections
      let intersection_candidate_segments = &self.segments_by_orientation.get(
          if wire_segment.get_orientation() == WireSegmentOrientation::Horizontal {
            &WireSegmentOrientation::Vertical
          } else {
            &WireSegmentOrientation::Horizontal
          }
        )
        .expect("This should never, ever happen!");
      for intersection_candidate_segment in intersection_candidate_segments.iter() {
        match wire_segment.find_intersection(intersection_candidate_segment) {
          Some(intersection_coordinates) => {
            // Register intersection
            if !self.intersections.contains_key(&intersection_coordinates) {
              let intersecting_segments: Vec<WireSegment> = vec![intersection_candidate_segment.clone()];
              self.intersections.insert(intersection_coordinates.clone(), intersecting_segments);
            };
            self.intersections.get_mut(&intersection_coordinates)
              .expect("This should never, ever happen!")
              .push(wire_segment.clone());
          },
          None => {}
        }
      }

      // Set up for next instruction
      start.x = end.x;
      start.y = end.y;
      accumulated_distance += distance;

    }

  }

}
