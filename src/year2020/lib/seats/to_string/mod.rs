//! Game of Seats .to_string() implementation
//! 
//! Returns a string representation of current state
// -----------------------------------------------------------------------------

// Include dependencies
use super::*;

/// Game of Seats .to_string() implementation
/// 
/// Returns a string representation of current state
impl GameOfSeats {
  
  /// Returns a string representation of current state
  pub fn to_string (&mut self) -> String {
    let mut rows: Vec<String> = vec![];
    for i in 0..(self.seats.len() / self._width) {
      let row = self.seats[(i * self._width)..((i+1) * self._width)].iter().map(|seat|  match seat {
        SeatState::Floor        => ".",
        SeatState::EmptySeat    => "L",
        SeatState::OccupiedSeat => "#"
      }).collect::<Vec<&str>>().join("");
      rows.push(row);
    }
    return rows.join("\n");
  }
}
