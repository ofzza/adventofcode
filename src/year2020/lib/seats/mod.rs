//! Game of Seats module
//! 
//! Implements Conway's Game of Seats
// -----------------------------------------------------------------------------

// Import child modules
mod constructor;
mod next_method;
mod to_string;

// (re)Export child modules
pub use constructor::*;
pub use next_method::*;
pub use to_string::*;

/// Game of Seats seat enum
#[derive(Debug, Clone, PartialEq)]
pub enum SeatState {
  Floor        = '.' as isize,
  EmptySeat    = 'L' as isize,
  OccupiedSeat = '#' as isize
}

/// Game of Seats struct
#[derive(Debug, Default, Clone)]
pub struct GameOfSeats {
  // Holds state of all seats
  pub seats: Vec<SeatState>,
  // Holds coordinates of adjecent seats for every seat
  pub _adjecent_coords: Vec<[(isize, isize); 8]>,
  // Holds dimensions of the seat grid
  _width: usize,
  _height: usize,
  // Holds count of occupied seats
  pub count_empty: usize,
  pub count_occupied: usize,
}
