//! Game of Seats ::new() implementation
//! 
//! Conway's Game of Seats constructor
// -----------------------------------------------------------------------------

// Include dependencies
use super::*;

/// Game of Seats ::new() implementation
/// 
/// Game of Seats constructor
impl GameOfSeats {
  
  /// Instantiate a new Game of Seats
  /// 
  /// # Arguments
  ///
  /// * `input` - Initial layout
  pub fn new (input: &Vec<String>) -> GameOfSeats {
    
    // Initialize game of seats
    let mut game = GameOfSeats {
      seats: vec![],
      _height: input.len(),
      _width: input[0].len(),
      ..Default::default()
    };

    // Initialize counts
    let mut count_empty = 0;
    let mut count_occupied = 0;
    // Decode input
    for y in 0..input.len() {
      let bytes = input[y].as_bytes();
      for x in 0..bytes.len() {
        match bytes[x] as char {
          '.' => {
            game.seats.push(SeatState::Floor);
          },
          'L' => {
            count_empty += 1;
            game.seats.push(SeatState::EmptySeat);
          },
          '#' => {
            count_occupied += 1;
            game.seats.push(SeatState::OccupiedSeat);
          },
          _ => panic!("Unrecognized input character!")
        }
      }
    }
    // Set counts
    game.count_empty = count_empty;
    game.count_occupied = count_occupied;

    // Return game of seats
    return game;
    
  }

  /// Initializes the game
  /// # Arguments
  /// * `directly_adjecent_only` - If true, only direcly adjecent seats will be considered (as opposed to line of sight)
  pub fn initialize (&mut self, directly_adjecent_only: bool) {
    // Find adjecent positions
    self._find_neighbouring_positions(directly_adjecent_only);
  }

  /// Finds positions considered neighbozring for each position in the seating grif
  ///
  /// # Arguments
  /// * `directly_adjecent_only` - If true, only direcly adjecent seats will be considered (as opposed to line of sight)
  fn _find_neighbouring_positions (&mut self, directly_adjecent_only: bool) {
    // Find positions of adjecent seats for every seat in the seating grid
    for y in 0..self._height {
      // Process row
      for x in 0..self._width {
        // Initialize positions of adjecent seats for every seat in the seating grid
        let mut positions: [(isize, isize); 8] = [(-1, -1); 8];

        // Find adjecent positions in case only directly adjecent are considered
        if directly_adjecent_only {

          // Check directly adjecent position
          let has_top: bool    = y > 0;
          let has_bottom: bool = y + 1 < self._height;
          let has_left: bool   = x > 0;
          let has_right: bool  = x + 1 < self._width;

          // Count directly adjecent seats
          let mut coords_index = 0;
          if has_top && self.seats[self._get_index(x, y - 1)] != SeatState::Floor {
            positions[coords_index] = (x as isize, (y - 1) as isize); // Top
            coords_index += 1;
          }
          if has_bottom && self.seats[self._get_index(x, y + 1)] != SeatState::Floor {
            positions[coords_index] = (x as isize, (y + 1) as isize); // Bottom
            coords_index += 1;
          }
          if has_left && self.seats[self._get_index(x - 1, y)] != SeatState::Floor {
            positions[coords_index] = ((x - 1) as isize, y as isize); // Left
            coords_index += 1;
          }
          if has_right && self.seats[self._get_index(x + 1, y)] != SeatState::Floor {
            positions[coords_index] = ((x + 1) as isize, y as isize); // Right
            coords_index += 1;
          }
          if has_top && has_left {
            if self.seats[self._get_index(x - 1, y - 1)] != SeatState::Floor {
              positions[coords_index] = ((x - 1) as isize, (y - 1) as isize); // Top - Left
              coords_index += 1;
            };
          }
          if has_top && has_right && self.seats[self._get_index(x + 1, y - 1)] != SeatState::Floor {
            positions[coords_index] = ((x + 1) as isize, (y - 1) as isize); // Top - Right
            coords_index += 1;
          }
          if has_bottom && has_left && self.seats[self._get_index(x - 1, y + 1)] != SeatState::Floor {
            positions[coords_index] = ((x - 1) as isize, (y + 1) as isize); // Bottom - Left
            coords_index += 1;
          }
          if has_bottom && has_right && self.seats[self._get_index(x + 1, y + 1)] != SeatState::Floor {
            positions[coords_index] = ((x + 1) as isize, (y + 1) as isize); // Bottom - Right
          }

        }
        
        // Find adjecent positions in case line-of-sight adjecent are considered
        else {

          // Count directly adjecent seats
          let mut index = 0;
          match self._check_line_of_sight_occupied(x, y,  0, -1) {
            Some(coords) => { positions[index] = (coords.0 as isize, coords.1 as isize); index += 1; }, // Top
            None => ()
          };
          match self._check_line_of_sight_occupied(x, y,  0, 1) {
            Some(coords) => { positions[index] = (coords.0 as isize, coords.1 as isize); index += 1; }, // Bottom
            None => ()
          };
          match self._check_line_of_sight_occupied(x, y,  -1, 0) {
            Some(coords) => { positions[index] = (coords.0 as isize, coords.1 as isize); index += 1; }, // Left
            None => ()
          };
          match self._check_line_of_sight_occupied(x, y,  1, 0) { // Right
            Some(coords) => { positions[index] = (coords.0 as isize, coords.1 as isize); index += 1; },
            None => ()
          };
          match self._check_line_of_sight_occupied(x, y,  -1, -1) {
            Some(coords) => { positions[index] = (coords.0 as isize, coords.1 as isize); index += 1; }, // Top - Left
            None => ()
          };
          match self._check_line_of_sight_occupied(x, y,  -1, 1) {
            Some(coords) => { positions[index] = (coords.0 as isize, coords.1 as isize); index += 1; }, // Top - Right
            None => ()
          };
          match self._check_line_of_sight_occupied(x, y,  1, -1) {
            Some(coords) => { positions[index] = (coords.0 as isize, coords.1 as isize); index += 1; }, // Bottom - Left
            None => ()
          };
          match self._check_line_of_sight_occupied(x, y,  1, 1) {
            Some(coords) => { positions[index] = (coords.0 as isize, coords.1 as isize); }, // Bottom - Right
            None => ()
          };

        }

        // Store positions of adjecent seats
        self._adjecent_coords.push(positions);
      }
    }
  }

  /// Checks a direction for first visible seat's position
  /// 
  /// # Arguments
  /// * `x`  - Starting x coordinate
  /// * `y`  - Starting y coordinate
  /// * `dx` - Change in x coordinate per each step in a specified direction
  /// * `dy` - Change in y coordinate per each step in a specified direction
  fn _check_line_of_sight_occupied (&mut self, x: usize, y: usize, dx: isize, dy: isize) -> Option<(usize, usize)> {
    // Walk dow the line-of-sight
    for i in 1..(self._width + self._height) {
      // Calculate next index
      let new_x: isize = x as isize + (i as isize * dx);
      let new_y: isize = y as isize + (i as isize * dy);
      // Check if next index is out of bounds
      if new_x < 0 || (new_x >= self._width as isize) || new_y < 0 || (new_y >= self._height as isize) {
        return None;
      }
      // Check if next index is a seat
      if self.seats[self._get_index(new_x as usize, new_y as usize)] != SeatState::Floor {
        return Some((new_x as usize, new_y as usize));
      }
    }
    // Return no occupied seats found
    return None;
  }
  
  /// Gets a linear index representing a 2D coordinate
  /// 
  /// # Arguments
  /// * `x`      - X coordinate
  /// * `y`      - Y coordinate
  pub(super) fn _get_index (&self, x: usize, y: usize) -> usize {
    return y * self._width + x;
  }

}
