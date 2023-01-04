//! Monkey Map module
//! 
//! Monkey Map module
// -----------------------------------------------------------------------------

// Include dependencies
use std::collections::hash_set::HashSet;
use crate::year::lib::math::Math;
use crate::year::lib::matrix::Matrix;

/// Monkey Map Face structure
pub struct MonkeyMapFace {
  coords: Vec<char>
}

enum MonkeyMapDirection {
  Distance(usize),
  Turn(char)
}

/// Monkey Map structure
pub struct MonkeyMap {
  // Dimensions of a map face
  pub face_length: usize,
  // Map faces matrix
  faces_matrix: Matrix,
  // Map single face matrix
  face_matrix: Matrix,
  // Map faces
  faces: Vec<Option<MonkeyMapFace>>,
  // Map movement directions
  directions: Vec<MonkeyMapDirection>,
  // Current position on the map (face, x, y)
  pub position: ((usize, usize), (usize, usize)),
  // Current orientation on the map as next delta move (x, y) on current face
  pub orientation: (isize, isize)
}

/// Monkey Map implementation
impl MonkeyMap {

  /// Constructor
  pub fn new (data_coordinates: Vec<Vec<char>>, data_directions: &str) -> MonkeyMap {
    // Initialize starting position and orientation
    let mut position_set = false;
    let mut position = ((0, 0), (0, 0));
    let orientation = (1, 0);

    // Count coordinates and calculate face dimensions
    let mut coordinates_count = 0;
    for y in 0..data_coordinates.len() {
      for x in 0..data_coordinates[y].len() {
        if data_coordinates[y][x] != ' ' {
          coordinates_count += 1;
        }
      }
    }
    let face_length = (coordinates_count as f64 / 6 as f64).sqrt() as usize;
    // Extract faces ...
    let mut faces: Vec<Option<MonkeyMapFace>> = vec![];
    for fy in 0..4 {
      for fx in 0..4 {
        // Calculate face starting coordinates
        let x0 = fx * face_length;
        let y0 = fy * face_length;
        // Check if face defined
        if data_coordinates.len() > y0 && data_coordinates[y0].len() > x0 && data_coordinates[y0][x0] != ' ' {
          // Read face
          let mut face: MonkeyMapFace = MonkeyMapFace {
            coords: Vec::with_capacity(face_length * face_length)
          };
          // Copy face coordinates
          for y in y0..(y0 + face_length) {
            for x in x0..(x0 + face_length) {
              // Copy coordinate
              face.coords.push(data_coordinates[y][x]);
              // Check if starting cordinate
              if !position_set && data_coordinates[y][x] == '.' {
                position_set = true;
                position = ((fx, fy), (x - x0, y - y0));
              }
            }
          }
          // Add face
          faces.push(Option::Some(face));
        }
        // Face not defined        
        else {
          faces.push(Option::None);
        }
      }
    }

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
      // Map faces matrix
      faces_matrix: Matrix::new(vec![4, 4]),
      // Map single face matrix
      face_matrix: Matrix::new(vec![face_length, face_length]),
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

  /// Starts following directions on the map provided
  /// 
  /// # Arguments
  /// * coords_callback: Callback function used to determine the next coordinates
  /// 
  /// # Returns
  /// Final coordinates having followed all directions
  pub fn follow_directions (&mut self, coords_callback: fn(map: &MonkeyMap, position: ((usize, usize), (usize, usize)), orientation: (isize, isize)) -> (((usize, usize), (usize, usize)), (isize, isize))) -> (usize, usize) {
    // Prompt map state
    // self._prompt(true, format!("Face: ({}, {})", self.position.0.0, self.position.0.1).as_str(), Option::None);

    // Follow each direction
    let mut path: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
    for direction in &self.directions {
      match direction {

        // Turn
        MonkeyMapDirection::Turn(direction) => {
          // Turn
          self.orientation = self.turn(self.orientation, direction.clone());
          // Prompt map state
          // self._prompt(true, format!("Face: ({}, {}); Turn {}", self.position.0.0, self.position.0.1, direction).as_str(), Option::None);
        }

        // Move
        MonkeyMapDirection::Distance(distance) => {
          // Move step by step
          for _ in 0..distance.clone() {
            // Get next position
            let (mut position, mut orientation) = coords_callback(self, self.position.clone(), self.orientation.clone());
            // Verify next position
            (position, orientation) = match self.faces_matrix.coords_to_index(&vec![position.0.0, position.0.1]) {
              // Coordinates of a non existent face
              Option::None => (self.position, self.orientation),
              // Continue within the face
              Option::Some(index) => match &self.faces[index] {
                // Face does not exist
                Option::None => (self.position, self.orientation),
                // Find next position when direct candidate face exists
                Option::Some(face) => match self.face_matrix.coords_to_index(&vec![position.1.0, position.1.1]) {
                  // Coordinates don't exist within the current face
                  Option::None => (self.position, self.orientation),
                  // Continue with found coordinates
                  Option:: Some(index) => match face.coords[index] {
                    // Empty space, should never happen
                    ' ' => panic!("Empty spaces should never get suggested!"),
                    // Hit a wall, remain in place
                    '#' => (self.position, self.orientation),
                    // Other
                    _ => {
                      // Write path
                      path.insert(position);
                      // Accept position
                      (position, orientation)
                    }
                  }
                }
              }
            };
            // Check if position was moved
            if position != self.position || orientation != self.orientation {
              // Move to next position and orientation
              self.position = position;              
              self.orientation = orientation;
              // Prompt map state
              // self._prompt(true, format!("Face: ({}, {}); Move {}: {}", position.0.0, position.0.1, distance, i + 1).as_str(), Option::None);
            } else {              
              break;
            }
          }
        }

      }      
    }

    // Prompt final path
    // self._prompt(false, "DONE", Option::Some(&path));

    // Return final position
    (
      (self.position.0.0 * self.face_length + self.position.1.0 + 1),
      (self.position.0.1 * self.face_length + self.position.1.1 + 1)
    )
  }

  /// Turns orientation coordinates left or right
  /// 
  /// # Arguments
  /// * orientation: Current orientation expressed as unit vector
  /// * direction: Direction to turn to: 'L' or 'R'
  /// 
  /// # Returns
  /// Turned orientation expressed as unit vector
  fn turn (&self, orientation: (isize, isize), direction: char) -> (isize, isize) {
    // Turn left
    if direction == 'L' {
           if orientation == ( 1,  0) { ( 0, -1) } // Right -> Top
      else if orientation == ( 0,  1) { ( 1,  0) } // Bottom -> Right
      else if orientation == (-1,  0) { ( 0,  1) } // Left -> Bottom
      else if orientation == ( 0, -1) { (-1,  0) } // Top -> Left
      else { panic!("Orientation not supported!") }
    }
    // Turn right
    else if direction == 'R' {
           if orientation == ( 1,  0) { ( 0,  1) } // Right -> Bottom
      else if orientation == ( 0,  1) { (-1,  0) } // Bottom -> Left
      else if orientation == (-1,  0) { ( 0, -1) } // Left -> Top
      else if orientation == ( 0, -1) { ( 1,  0) } // Top -> Right
      else { panic!("Orientation not supported!") }
    }
    // Unknown turn direction
    else {
      panic!("Direction not supported!")
    }
  }

  /// Determines the next coordinates based on wrap-around logic
  ///
  /// # Arguments
  /// * position_next: Candidate position to be checked and translated
  /// * orientation: Orientation
  /// 
  /// # Returns
  /// * Next coordinates after the move
  pub fn determine_next_coordinates_with_wraparound (&self, position_current: &((usize, usize), (usize, usize)), orientation: &(isize, isize)) -> (((usize, usize), (usize, usize)), (isize, isize)) {
    // Find next candidate coordinates
    let mut next_position = self.determine_next_candidate_coordinates(position_current, orientation);

    // Check if next coordinates point to existing face
    next_position = match self.faces_matrix.coords_to_index(&vec![next_position.0.0, next_position.0.1]) {
      // Coordinates of a non existent face
      Option::None => position_current.clone(),
      // Continue within the face
      Option::Some(index) => match &self.faces[index] {
        // Found direct face, use next as is
        Option::Some(_) => next_position,
        // Find next position when direct candidate face doesn't exist
        Option::None => {
          // Find next face
          for i in 0..4 {
            // Teleporting field coordinates
            let fx = Math::index_wraparound(next_position.0.0 as isize + (orientation.0 * i), 4);
            let fy = Math::index_wraparound(next_position.0.1 as isize + (orientation.1 * i), 4);
            // Check teleporting field exists and can be landed at
            match self.faces_matrix.coords_to_index(&vec![fx, fy]) {
              Option::Some(index) => match &self.faces[index] {
                Option::Some(face) => {
                  // Found next face and position
                  let x = Math::index_wraparound(next_position.1.0 as isize, self.face_length);
                  let y = Math::index_wraparound(next_position.1.1 as isize, self.face_length);
                  next_position = ((fx, fy), (x, y));
                  // Check if next position can be traveled to
                  let coords_index = self.face_matrix.coords_to_index(&vec![x, y]).unwrap();
                  if face.coords[coords_index] == '#' {
                    next_position = position_current.clone();
                  }
                  // Stop search for face
                  break;
                }
                Option::None => ()
              },
              Option::None => ()
              
            }
          }

          next_position
        }
      }
    };
    
    // Return next coordinates
    (next_position, self.orientation)
  }
  /// Determines the next coordinates on a cube
  ///
  /// # Arguments
  /// * coords: Current position
  /// * orientation: Current orientation
  /// 
  /// # Returns
  /// * Next coordinates after the move
  pub fn determine_next_coordinates_on_cube (&self, position_current: &((usize, usize), (usize, usize)), orientation: &(isize, isize)) -> (((usize, usize), (usize, usize)), (isize, isize)) {
    // Find next candidate coordinates
    let mut next_position = self.determine_next_candidate_coordinates(position_current, orientation);
    let mut next_orientation = self.orientation;

    // Check if next coordinates point to existing face
    (next_position, next_orientation) = match self.faces_matrix.coords_to_index(&vec![next_position.0.0, next_position.0.1]) {
      // Coordinates of a non existent face
      Option::None => self.navigate_to_indirect_cube_face(&position_current, &next_orientation),
      // Continue within the face
      Option::Some(index) => match &self.faces[index] {
        // Found direct face, use next as is
        Option::Some(_) => (next_position, next_orientation),
        // Find next position when direct candidate face doesn't exist
        Option::None => self.navigate_to_indirect_cube_face(&position_current, &next_orientation)
      }
    };
    
    // Return next coordinates
    (next_position, next_orientation)
  }
  /// Finds a matching coordinate on an indirectly connected cube face
  ///
  /// # Arguments
  /// * coords: Current position
  /// * orientation: Current orientation
  /// 
  /// # Returns
  /// * Next coordinates after the move
  fn navigate_to_indirect_cube_face (&self, position_current: &((usize, usize), (usize, usize)), orientation: &(isize, isize)) -> (((usize, usize), (usize, usize)), (isize, isize)) {
    // if position_current == &((0, 3), (18, 49)) && orientation == &(0, 1)  {
    //   println!("DEBUG!");
    // }
    
    // Try finding a face on relative position: L+F | R+F
    let orientation_turn_l = self.turn(orientation.clone(), 'L');
    let orientation_turn_r = self.turn(orientation.clone(), 'R');
    let candidate_coordinates: Vec<((Vec<(usize, usize)>, (usize, usize)), (isize, isize))> = vec![
      // L+F
      (
        (
          // L+F faces
          vec![
            // F
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_l.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_l.1, 4)
            ),
            // F+L
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_l.0 + orientation.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_l.1 + orientation.1, 4)
            )
          ],
          // L+F coordinates
          match orientation {
            ( 1,  0) => (position_current.1.1, self.face_length - 1),                                       // Right  : x=y; y=L
            ( 0,  1) => (0, Math::index_mirror(position_current.1.0, self.face_length)),                    // Bottom : x=0; y=!x
            (-1,  0) => (position_current.1.1, 0),                                                          // Left   : x=y; y=0
            ( 0, -1) => (self.face_length - 1, Math::index_mirror(position_current.1.0, self.face_length)), // Top    : x=L; y=!x
            _ => panic!("Orientation not supported!")
          }
        ),
        // L+F orientation
        orientation_turn_l
      ),
      // R+F
      (
        (
          // R+F face
          vec![
            // F
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_r.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_r.1, 4)
            ),
            // R+F
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_r.0 + orientation.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_r.1 + orientation.1, 4)
            )
          ],
          // R+F coordinates
          match orientation {
            ( 1,  0) => (Math::index_mirror(position_current.1.1, self.face_length), 0),                    // Right  : x=!y; y=0
            ( 0,  1) => (self.face_length - 1, position_current.1.0),                                       // Bottom : x=L; y=x
            (-1,  0) => (Math::index_mirror(position_current.1.1, self.face_length), self.face_length - 1), // Left   : x=!y; y=L
            ( 0, -1) => (0, position_current.1.0),                                                          // Top    : x=0; y=x
            _ => panic!("Orientation not supported!")
          }
        ),
        // R+F orientation
        orientation_turn_r
      ),
    ];
    // Check if candidate coordinates exist
    for (next_position, next_orientation) in candidate_coordinates {
      for face_index in 0..next_position.0.len() {
        let face_coordinates = next_position.0[face_index];
        match self.faces_matrix.coords_to_index(&vec![face_coordinates.0, face_coordinates.1]) {
          Option::Some(index) => match &self.faces[index] {
            Option::Some(_) => {
              // If final face found, return coordinates
              if face_index + 1 == next_position.0.len() {
                return ((face_coordinates, next_position.1), next_orientation);
              }
            },
            Option::None => break
          },
          Option::None => break
        }
      }
    }

    // Try finding a face on relative position: B+L+L | L+L+F | B+R+R | R+R+F
    let orientation_turn_b = (-1 * orientation.0, -1 * orientation.1);
    let orientation_transposed = match orientation {
      ( 1,  0) => (position_current.1.0, Math::index_mirror(position_current.1.1, self.face_length)), // Right  : x=x; y=!y
      ( 0,  1) => (Math::index_mirror(position_current.1.0, self.face_length), position_current.1.1), // Bottom : x=!x; y=y
      (-1,  0) => (position_current.1.0, Math::index_mirror(position_current.1.1, self.face_length)), // Left   : x=x; y=!y
      ( 0, -1) => (Math::index_mirror(position_current.1.0, self.face_length), position_current.1.1), // Top    : x=!x; y=y
      _ => panic!("Orientation not supported!")
    };
    let candidate_coordinates: Vec<((Vec<(usize, usize)>, (usize, usize)), (isize, isize))> = vec![
      // B+L+L
      (
        (
          // B+L+L face
          vec![
            // B
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_b.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_b.1, 4)
            ),
            // B+L
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_b.0 + orientation_turn_l.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_b.1 + orientation_turn_l.1, 4)
            ),
            // B+L+L
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_b.0 + orientation_turn_l.0 + orientation_turn_l.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_b.1 + orientation_turn_l.1 + orientation_turn_l.1, 4)
            )
          ],
          // B+L+L coordinates
          orientation_transposed
        ),
        // B+L+L orientation
        orientation_turn_b
      ),
      // L+L+F
      (
        (
          // L+L+F face
          vec![
            // L
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_l.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_l.1, 4)
            ),
            // L+L
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_l.0 + orientation_turn_l.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_l.1 + orientation_turn_l.1, 4)
            ),
            // L+L+F
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_l.0 + orientation_turn_l.0 + orientation.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_l.1 + orientation_turn_l.1 + orientation.1, 4)
            )
          ],
          // L+L+F coordinates
          orientation_transposed
        ),
        // L+L+F orientation
        orientation_turn_b
      ),
      // B+R+R
      (
        (
          // B+R+R face
          vec![
            // B
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_b.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_b.1, 4)
            ),
            // B+R
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_b.0 + orientation_turn_r.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_b.1 + orientation_turn_r.1, 4)
            ),
            // B+R+R
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_b.0 + orientation_turn_r.0 + orientation_turn_r.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_b.1 + orientation_turn_r.1 + orientation_turn_r.1, 4)
            )
          ],
          // B+R+R coordinates
          orientation_transposed
        ),
        // B+R+R orientation
        orientation_turn_b
      ),
      // R+R+F
      (
        (
          // R+R+F face
          vec![
            // R
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_r.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_r.1, 4)
            ),
            // R+R
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_r.0 + orientation_turn_r.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_r.1 + orientation_turn_r.1, 4)
            ),
            // R+R+F
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_r.0 + orientation_turn_r.0 + orientation.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_r.1 + orientation_turn_r.1 + orientation.1, 4)
            )
          ],
          // R+R+F coordinates
          orientation_transposed
        ),
        // R+R+F orientation
        orientation_turn_b
      ),
    ];
    // Check if candidate coordinates exist
    for (next_position, next_orientation) in candidate_coordinates {
      for face_index in 0..next_position.0.len() {
        let face_coordinates = next_position.0[face_index];
        match self.faces_matrix.coords_to_index(&vec![face_coordinates.0, face_coordinates.1]) {
          Option::Some(index) => match &self.faces[index] {
            Option::Some(_) => {
              // If final face found, return coordinates
              if face_index + 1 == next_position.0.len() {
                return ((face_coordinates, next_position.1), next_orientation);
              }
            },
            Option::None => break
          },
          Option::None => break
        }
      }
    }

    // Try finding a face on relative position: B+B+L+B | B+B+R+B
    let candidate_coordinates: Vec<((Vec<(usize, usize)>, (usize, usize)), (isize, isize))> = vec![
      // B+B+L+B
      (
        (
          // B+B+L+B
          vec![
            // B
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_b.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_b.1, 4)
            ),
            // B+B
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_b.0 + orientation_turn_b.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_b.1 + orientation_turn_b.1, 4)
            ),
            // B+B+L
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_b.0 + orientation_turn_b.0 + orientation_turn_l.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_b.1 + orientation_turn_b.1 + orientation_turn_l.1, 4)
            ),
            // B+B+L+B
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_b.0 + orientation_turn_b.0 + orientation_turn_l.0 + orientation_turn_b.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_b.1 + orientation_turn_b.1 + orientation_turn_l.1 + orientation_turn_b.1, 4)
            ),
          ],
          // B+B+L+B coordinates
          match orientation {
            ( 1,  0) => (Math::index_mirror(position_current.1.1, self.face_length), 0),                    // Right  : x=!y; y=0
            ( 0,  1) => (self.face_length - 1, position_current.1.0),                                       // Bottom : x=L; y=x
            (-1,  0) => (Math::index_mirror(position_current.1.1, self.face_length), self.face_length - 1), // Left   : x=!y; y=L
            ( 0, -1) => (0, position_current.1.0),                                                          // Top    : x=0; y=x
            _ => panic!("Orientation not supported!")
          }
        ),
        // B+B+L+B orientation
        orientation_turn_r
      ),
      // B+B+R+B
      (
        (
          // B+B+R+B
          vec![
            // B
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_b.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_b.1, 4)
            ),
            // B+B
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_b.0 + orientation_turn_b.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_b.1 + orientation_turn_b.1, 4)
            ),
            // B+B+R
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_b.0 + orientation_turn_b.0 + orientation_turn_r.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_b.1 + orientation_turn_b.1 + orientation_turn_r.1, 4)
            ),
            // B+B+R+B
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_b.0 + orientation_turn_b.0 + orientation_turn_r.0 + orientation_turn_b.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_b.1 + orientation_turn_b.1 + orientation_turn_r.1 + orientation_turn_b.1, 4)
            ),
          ],
          // B+B+R+B coordinates
          match orientation {
            ( 1,  0) => (position_current.1.1, self.face_length - 1),                                       // Right  : x=y; y=L
            ( 0,  1) => (0, Math::index_mirror(position_current.1.0, self.face_length)),                    // Bottom : x=0; y=!x
            (-1,  0) => (position_current.1.1, 0),                                                          // Left   : x=y; y=0
            ( 0, -1) => (self.face_length - 1, Math::index_mirror(position_current.1.0, self.face_length)), // Top    : x=L; y=!x
            _ => panic!("Orientation not supported!")
          }
        ),
        // B+B+R+B orientation
        orientation_turn_l
      ),
    ];
    // Check if candidate coordinates exist
    for (next_position, next_orientation) in candidate_coordinates {
      for face_index in 0..next_position.0.len() {
        let face_coordinates = next_position.0[face_index];
        match self.faces_matrix.coords_to_index(&vec![face_coordinates.0, face_coordinates.1]) {
          Option::Some(index) => match &self.faces[index] {
            Option::Some(_) => {
              // If final face found, return coordinates
              if face_index + 1 == next_position.0.len() {
                return ((face_coordinates, next_position.1), next_orientation);
              }
            },
            Option::None => break
          },
          Option::None => break
        }
      }
    }

    // Try finding a face on relative position: L+B+L+L | R+B+R+R
    let candidate_coordinates: Vec<((Vec<(usize, usize)>, (usize, usize)), (isize, isize))> = vec![
      // L+B+L+L
      (
        (
          // L+B+L+L
          vec![
            // L
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_l.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_l.1, 4)
            ),
            // L+B
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_l.0 + orientation_turn_b.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_l.1 + orientation_turn_b.1, 4)
            ),
            // L+B+L
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_l.0 + orientation_turn_b.0 + orientation_turn_l.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_l.1 + orientation_turn_b.1 + orientation_turn_l.1, 4)
            ),
            // L+B+L+L
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_l.0 + orientation_turn_b.0 + orientation_turn_l.0 + orientation_turn_l.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_l.1 + orientation_turn_b.1 + orientation_turn_l.1 + orientation_turn_l.1, 4)
            ),
          ],
          // L+B+L+L coordinates
          match orientation {
            ( 1,  0) => (Math::index_mirror(position_current.1.1, self.face_length), 0),                    // Right  : x=!y; y=0
            ( 0,  1) => (self.face_length - 1, position_current.1.0),                                       // Bottom : x=L; y=x
            (-1,  0) => (Math::index_mirror(position_current.1.1, self.face_length), self.face_length - 1), // Left   : x=!y; y=L
            ( 0, -1) => (0, position_current.1.0),                                                          // Top    : x=0; y=x
            _ => panic!("Orientation not supported!")
          }
        ),
        // L+B+L+L orientation
        orientation_turn_r
      ),
      // R+B+R+R
      (
        (
          // R+B+R+R
          vec![
            // R
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_r.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_r.1, 4)
            ),
            // R+B
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_r.0 + orientation_turn_b.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_r.1 + orientation_turn_b.1, 4)
            ),
            // R+B+R
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_r.0 + orientation_turn_b.0 + orientation_turn_r.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_r.1 + orientation_turn_b.1 + orientation_turn_r.1, 4)
            ),
            // R+B+R+R
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_r.0 + orientation_turn_b.0 + orientation_turn_r.0 + orientation_turn_r.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_r.1 + orientation_turn_b.1 + orientation_turn_r.1 + orientation_turn_r.1, 4)
            ),
          ],
          // R+B+R+R coordinates
          match orientation {
            ( 1,  0) => (position_current.1.1, self.face_length - 1),                                       // Right  : x=y; y=L
            ( 0,  1) => (0, Math::index_mirror(position_current.1.0, self.face_length)),                    // Bottom : x=0; y=!x
            (-1,  0) => (position_current.1.1, 0),                                                          // Left   : x=y; y=0
            ( 0, -1) => (self.face_length - 1, Math::index_mirror(position_current.1.0, self.face_length)), // Top    : x=L; y=!x
            _ => panic!("Orientation not supported!")
          }
        ),
        // R+B+R+R orientation
        orientation_turn_l
      ),
    ];
    // Check if candidate coordinates exist
    for (next_position, next_orientation) in candidate_coordinates {
      for face_index in 0..next_position.0.len() {
        let face_coordinates = next_position.0[face_index];
        match self.faces_matrix.coords_to_index(&vec![face_coordinates.0, face_coordinates.1]) {
          Option::Some(index) => match &self.faces[index] {
            Option::Some(_) => {
              // If final face found, return coordinates
              if face_index + 1 == next_position.0.len() {
                return ((face_coordinates, next_position.1), next_orientation);
              }
            },
            Option::None => break
          },
          Option::None => break
        }
      }
    }

    // Try finding a face on relative position: B+L+B+B+L | L+B+B+L+B | B+R+B+B+R | R+B+B+R+B
    let orientation_transposed = match orientation {
      ( 1,  0) => (0, position_current.1.1),                    // Right  : x=0; y=y
      ( 0,  1) => (position_current.1.0, 0),                    // Bottom : x=x; y=0
      (-1,  0) => (self.face_length - 1, position_current.1.1), // Left   : x=L; y=y
      ( 0, -1) => (position_current.1.0, self.face_length - 1), // Top    : x=x; y=L
      _ => panic!("Orientation not supported!")
    };
    let candidate_coordinates: Vec<((Vec<(usize, usize)>, (usize, usize)), (isize, isize))> = vec![
      // B+L+B+B+L
      (
        (
          // B+L+B+B+L
          vec![
            // B
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_b.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_b.1, 4)
            ),
            // B+L
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_b.0 + orientation_turn_l.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_b.1 + orientation_turn_l.1, 4)
            ),
            // B+L+B
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_b.0 + orientation_turn_l.0 + orientation_turn_b.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_b.1 + orientation_turn_l.1 + orientation_turn_b.1, 4)
            ),
            // B+L+B+B
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_b.0 + orientation_turn_l.0 + orientation_turn_b.0 + orientation_turn_b.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_b.1 + orientation_turn_l.1 + orientation_turn_b.1 + orientation_turn_b.1, 4)
            ),
            // B+L+B+B+L
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_b.0 + orientation_turn_l.0 + orientation_turn_b.0 + orientation_turn_b.0 + orientation_turn_l.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_b.1 + orientation_turn_l.1 + orientation_turn_b.1 + orientation_turn_b.1 + orientation_turn_l.1, 4)
            ),
          ],
          // B+L+B+B+L coordinates
          orientation_transposed
        ),
        // B+L+B+B+L orientation
        orientation.clone()
      ),
      // L+B+B+L+B
      (
        (
          // L+B+B+L+B
          vec![
            // L
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_l.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_l.1, 4)
            ),
            // L+B
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_l.0 + orientation_turn_b.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_l.1 + orientation_turn_b.1, 4)
            ),
            // L+B+B
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_l.0 + orientation_turn_b.0 + orientation_turn_b.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_l.1 + orientation_turn_b.1 + orientation_turn_b.1, 4)
            ),
            // L+B+B+L
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_l.0 + orientation_turn_b.0 + orientation_turn_b.0 + orientation_turn_l.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_l.1 + orientation_turn_b.1 + orientation_turn_b.1 + orientation_turn_l.1, 4)
            ),
            // L+B+B+L+B
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_l.0 + orientation_turn_b.0 + orientation_turn_b.0 + orientation_turn_l.0 + orientation_turn_b.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_l.1 + orientation_turn_b.1 + orientation_turn_b.1 + orientation_turn_l.1 + orientation_turn_b.1, 4)
            ),
          ],
          // L+B+B+L+B coordinates
          orientation_transposed
        ),
        // L+B+B+L+B orientation
        orientation.clone()
      ),
      // B+R+B+B+R
      (
        (
          // B+R+B+B+R
          vec![
            // B
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_b.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_b.1, 4)
            ),
            // B+R
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_b.0 + orientation_turn_r.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_b.1 + orientation_turn_r.1, 4)
            ),
            // B+R+B
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_b.0 + orientation_turn_r.0 + orientation_turn_b.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_b.1 + orientation_turn_r.1 + orientation_turn_b.1, 4)
            ),
            // B+R+B+B
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_b.0 + orientation_turn_r.0 + orientation_turn_b.0 + orientation_turn_b.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_b.1 + orientation_turn_r.1 + orientation_turn_b.1 + orientation_turn_b.1, 4)
            ),
            // B+R+B+B+R
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_b.0 + orientation_turn_r.0 + orientation_turn_b.0 + orientation_turn_b.0 + orientation_turn_r.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_b.1 + orientation_turn_r.1 + orientation_turn_b.1 + orientation_turn_b.1 + orientation_turn_r.1, 4)
            ),
          ],
          // B+R+B+B+R coordinates
          orientation_transposed
        ),
        // B+R+B+B+R orientation
        orientation.clone()
      ),
      (
        (
          // R+B+B+R+B
          vec![
            // L
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_r.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_r.1, 4)
            ),
            // L+B
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_r.0 + orientation_turn_b.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_r.1 + orientation_turn_b.1, 4)
            ),
            // L+B+B
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_r.0 + orientation_turn_b.0 + orientation_turn_b.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_r.1 + orientation_turn_b.1 + orientation_turn_b.1, 4)
            ),
            // L+B+B+L
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_r.0 + orientation_turn_b.0 + orientation_turn_b.0 + orientation_turn_r.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_r.1 + orientation_turn_b.1 + orientation_turn_b.1 + orientation_turn_r.1, 4)
            ),
            // L+B+B+L+B
            (
              Math::index_wraparound(position_current.0.0 as isize + orientation_turn_r.0 + orientation_turn_b.0 + orientation_turn_b.0 + orientation_turn_r.0 + orientation_turn_b.0, 4),
              Math::index_wraparound(position_current.0.1 as isize + orientation_turn_r.1 + orientation_turn_b.1 + orientation_turn_b.1 + orientation_turn_r.1 + orientation_turn_b.1, 4)
            ),
          ],
          // R+B+B+R+B coordinates
          orientation_transposed
        ),
        // R+B+B+R+B orientation
        orientation.clone()
      )
    ];
    // Check if candidate coordinates exist
    for (next_position, next_orientation) in candidate_coordinates {
      for face_index in 0..next_position.0.len() {
        let face_coordinates = next_position.0[face_index];
        match self.faces_matrix.coords_to_index(&vec![face_coordinates.0, face_coordinates.1]) {
          Option::Some(index) => match &self.faces[index] {
            Option::Some(_) => {
              // If final face found, return coordinates
              if face_index + 1 == next_position.0.len() {
                return ((face_coordinates, next_position.1), next_orientation);
              }
            },
            Option::None => break
          },
          Option::None => break
        }
      }
    }

    // If face found, panic
    self._prompt(false, format!("Face: ({}, {})", self.position.0.0, self.position.0.1).as_str(), Option::None);
    panic!("Could not find an indirectly connected cube face!");
  }
  /// Determines next candidate coordinates bases only on current coordinates and orientation
  /// 
  /// # Arguments
  /// * coords: Current position
  /// * orientation: Current orientation
  /// 
  /// # Returns
  /// * Next candidate coordinates to try and move to
  pub fn determine_next_candidate_coordinates (&self, position: &((usize, usize), (usize, usize)), orientation: &(isize, isize)) -> ((usize, usize), (usize, usize)) {
    (
      (
        Math::index_wraparound(position.0.0 as isize + if position.1.0 as isize + orientation.0 < 0 { -1 as isize } else if position.1.0 as isize + orientation.0 >= self.face_length as isize { 1 as isize } else { 0 as isize }, 4),
        Math::index_wraparound(position.0.1 as isize + if position.1.1 as isize + orientation.1 < 0 { -1 as isize } else if position.1.1 as isize + orientation.1 >= self.face_length as isize { 1 as isize } else { 0 as isize }, 4)
      ),
      (
        Math::index_wraparound(position.1.0 as isize + orientation.0, self.face_length),
        Math::index_wraparound(position.1.1 as isize + orientation.1, self.face_length)
      )
    )
  }

  /// Prompts current state of the map
  /// 
  /// # Arguments
  /// * single_face: If only current face should be prompted
  /// * comment: Message to print next to the layout
  /// * path: Optional path already traveled
  pub fn _prompt (&self, single_face: bool, comment: &str, path: Option<&HashSet<((usize, usize), (usize, usize))>>) {
    // Skip partial prompts
    if single_face { return; }
    // Prompt state
    for fy in if single_face { self.position.0.1..(self.position.0.1 + 1) } else { 0..4 } {
      for y in 0..self.face_length {
        // Print line
        for fx in if single_face { self.position.0.0..(self.position.0.0 + 1) } else { 0..4 } {
          for x in 0..self.face_length {
            // Prompt current position
            if ((fx, fy), (x, y)) == self.position {
              print!("{}", match self.orientation {
                ( 1,  0 ) => ">",
                ( 0,  -1) => "^",
                (-1,  0 ) => "<",
                ( 0,  1 ) => "v",
                _ => panic!("Orientation not supported!")
              });
            }
            // Prompt map element
            else {
              match &self.faces[self.faces_matrix.coords_to_index(&vec![fx, fy]).unwrap()] {
                Option::None => {
                  print!(" ");
                },
                Option::Some(face) => {
                  let position = face.coords[self.face_matrix.coords_to_index(&vec![x, y]).unwrap()];
                  print!("{}", match path {
                    Option::None => position,
                    Option::Some(path) => if path.contains(&((fx, fy), (x, y))) { 'x' } else { position }
                  });
                }
              }
              
            }
          }
        }
        // New line
        if fy == self.position.0.1 && y == 0 { print!(" {}", comment); }
        println!();
      }
    }
    println!();
  }
}
