//! Monkey Map module
//! 
//! Monkey Map module
// -----------------------------------------------------------------------------

/// Monkey Map Face structure
pub struct MonkeyMapFace {
  coords: Vec<Vec<char>>
}

enum MonkeyMapDirection {
  Distance(usize),
  Turn(char)
}

/// Monkey Map structure
pub struct MonkeyMap {
  // Dimensions of a map face
  face_length: usize,
  // Map faces
  faces: [Option<MonkeyMapFace>; 16],
  // Map movement directions
  directions: Vec<MonkeyMapDirection>,
  // Current position on the map (face, x, y)
  position: (usize, usize, usize),
  // Current orientation on the map as next delta move (x, y) on current face
  orientation: (usize, usize)
}

/// Monkey Map implementation
impl MonkeyMap {

  /// Constructor
  pub fn new (data_coordinates: Vec<Vec<char>>, data_directions: &str) -> MonkeyMap {
    // Process map faces
    let mut coordinates_count = 0;
    // TODO: Extract faces ...
    let faces: [Option<MonkeyMapFace>; 16] = [
      Option::Some(MonkeyMapFace { coords: vec![] }), Option::Some(MonkeyMapFace { coords: vec![] }), Option::Some(MonkeyMapFace { coords: vec![] }), Option::Some(MonkeyMapFace { coords: vec![] }),
      Option::Some(MonkeyMapFace { coords: vec![] }), Option::Some(MonkeyMapFace { coords: vec![] }), Option::Some(MonkeyMapFace { coords: vec![] }), Option::Some(MonkeyMapFace { coords: vec![] }),
      Option::Some(MonkeyMapFace { coords: vec![] }), Option::Some(MonkeyMapFace { coords: vec![] }), Option::Some(MonkeyMapFace { coords: vec![] }), Option::Some(MonkeyMapFace { coords: vec![] }),
      Option::Some(MonkeyMapFace { coords: vec![] }), Option::Some(MonkeyMapFace { coords: vec![] }), Option::Some(MonkeyMapFace { coords: vec![] }), Option::Some(MonkeyMapFace { coords: vec![] })
    ];
    coordinates_count = 96;
    let face_length = (coordinates_count as f64 / 6 as f64).sqrt() as usize;

    // Find starting position and orientation
    // TODO: ...
    let position = (0, 0, 0);
    let orientation = (1, 0);

    // Process map directions
    let mut directions: Vec<MonkeyMapDirection> = Vec::with_capacity(data_directions.len());
    let mut directions_buffer: Vec<char> = vec![];
    for c in data_directions.chars() {
      // Proces turn direction
      if c == 'L' || c == 'R' {
        // Flush distance buffer into a distance direction
        if directions_buffer.len() != 0 {
          directions.push(MonkeyMapDirection::Distance(directions_buffer.iter().collect::<String>().parse::<usize>().unwrap()));
          directions_buffer.clear();
        }
        // Add turn direction
        directions.push(MonkeyMapDirection::Turn(c));
      }
      // Accumulate distance directions
      else {
        directions_buffer.push(c);
      }
    }
    // Flush remaining distance buffer into a distance direction
    if directions_buffer.len() != 0 {
      directions.push(MonkeyMapDirection::Distance(directions_buffer.iter().collect::<String>().parse::<usize>().unwrap()));
    }

    // Return instance
    MonkeyMap {
      // Dimensions of a map face
      face_length,
      // Map faces
      faces,
      // Map movement directions
      directions,
      // Current position on the map (face, x, y)
      position,
      // Current orientation on the map as next delta move (x, y) on current face
      orientation
    }
  }
}
