//! Game of Seats .next() implementation
//! 
//! Implements a generator pattern working on top of the Game of Seats running
//! the actual game steps
// -----------------------------------------------------------------------------

// Include dependencies
use super::*;

/// Game of Seats .next() implementation
/// 
/// Implements a generator pattern working on top of the Game of Seats running
/// the actual game steps
impl GameOfSeats {
  
  /// Executes next Game of Seats step
  /// 
  /// # Arguments
  /// 
  /// * `tolerance`         - Number of adjecent occupied seats people will tolerate before leaving their seat
  /// * `verbose`           - Outputs executing output of the puzzle to the console
  pub fn next (&mut self, tolerance: usize, _verbose: bool) -> bool {

    // Initialize next step in the game
    let mut next_seats: Vec<SeatState> = vec![];
    let mut count_empty = self.count_empty;
    let mut count_occupied = self.count_occupied;
    let mut changes = false;

    // Process a step in the game
    for y in 0..self._height {
      for x in 0..self._width {

        // Claculate index
        let index = self._get_index(x, y);

        // Check if seat or floor
        if self.seats[index] == SeatState::Floor {
          next_seats.push(self.seats[index].clone());
          continue;
        }

        // Get adjecents for seat coordinates
        let adjecents = self._adjecent_coords[index];

        // Count number of occupied adjecent seats
        let mut count = 0;
        for i in 0..8 {
          let coords = adjecents[i];
          if coords.0 < 0 || coords.1 < 0 {
            break;
          }
          if self.seats[self._get_index(coords.0 as usize, coords.1 as usize)] == SeatState::OccupiedSeat {
            count += 1;
          }
        }        

        // Apply rules
        if self.seats[index] == SeatState::EmptySeat && count == 0 {
          next_seats.push(SeatState::OccupiedSeat);
          count_empty -= 1;
          count_occupied += 1;
          changes = true;
        }
        else if self.seats[index] == SeatState::OccupiedSeat && count >= tolerance {
          next_seats.push(SeatState::EmptySeat);
          count_empty += 1;
          count_occupied -= 1;
          changes = true;
        } else {
          next_seats.push(self.seats[index].clone());
        }

      }
    }

    // Apply next step in the game
    self.seats = next_seats;
    self.count_empty = count_empty;
    self.count_occupied = count_occupied;
    self.count_occupied = count_occupied;

    // Return if any changes were made
    return changes;
  }

}
