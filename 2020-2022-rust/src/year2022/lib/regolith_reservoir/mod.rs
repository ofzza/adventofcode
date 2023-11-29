//! Regolith Reservoir module
//! 
//! Regolith Reservoir module
// -----------------------------------------------------------------------------

// Include dependencies
use std::collections::hash_map::HashMap;
use crate::year::lib::dot_display::DotDisplay;

/// Regolith Reservoir structure
pub struct RegolithReservoir {
  // Coordinates of sand source
  source: (usize, usize),
  // Hashmap of all occupied coordinates
  pub hash: HashMap<(usize, usize), bool>,
  // Floor Y coordinate
  pub floor: usize,
  // If infinite floor should be simulated
  pub infinite_floor: bool,
  // Current falling sand grain coordinates
  pub current: Option<(usize, usize)>
}

/// Regolith Reservoir implementation
impl RegolithReservoir {

  /// Constructor
  /// 
  /// # Arguments
  /// * source: Coordinates for the source of the falling sand
  /// * walls: Inflection points of all defined walls
  /// * infinite_floor_offset: Offset of the "infinite floor" under the lowest real floor height, if "infinite floor" exists
  pub fn new (source: (usize, usize), walls: Vec<Vec<(usize, usize)>>, infinite_floor_offset: Option<usize>) -> RegolithReservoir {
    // Initialize hashmap
    let mut hash: HashMap<(usize, usize), bool> = HashMap::new();

    // Initialize floor
    let mut floor: usize = 0;

    // Insert walls into the hashmap
    for wall in &walls {
      for i in 1..wall.len() {
        // Get start and end points of the wall
        let start = wall[i - 1];
        let end = wall[i];
        // Adjust floor
        if start.1 > floor { floor = start.1; }
        if end.1 > floor { floor = end.1; }
        // Get wall orientation
        let diff: (isize, isize) =
          // Horizontal wall going right
          if start.1 == end.1 && start.0 < end.0 {
            (1, 0)
          }
          // Horizontal wall going left
          else if start.1 == end.1 && start.0 > end.0 {
            (-1, 0)
          }
          // Vertical wall going down
          else if start.0 == end.0 && start.1 < end.1 {
            (0, 1)
          }
          // Vertical wall going down
          else if start.0 == end.0 && start.1 > end.1 {
            (0, -1)
          }
          // Never happens
          else {
            panic!("This can never happen!");
          };
        // Insert wall sections into the hashmap
        let mut coords = start.clone();
        loop {
          // Insert wall section
          hash.insert(coords, true);
          // Check if last section
          if coords.0 == end.0 && coords.1 == end.1 {
            break;
          }
          // Proceed to next section
          coords.0 = (coords.0 as isize + diff.0) as usize;
          coords.1 = (coords.1 as isize + diff.1) as usize;
        }
      }
    }

    // Check if simulating infinite floor
    let mut infinite_floor = false;
    match infinite_floor_offset {
      Option::Some(offset) => {
        floor += offset;
        infinite_floor = true;
      },
      _ => ()
    }

    // Return created regolith reservoir instance
    RegolithReservoir {
      source,
      hash,
      infinite_floor,
      floor,
      current: Option::None
    }
  }

  /// Simulates a single step of falling sand from the source onto the walls structure as defined
  pub fn step (&mut self) {
    // If no current grain of sand, spawn grain of sand
    match self.current {
      // If no current grand of sand, spawn current grapn of sand
      Option::None => self.current = Option::Some(self.source.clone()),
      // If current grain of sand, animate current grain of sand
      Option::Some(current) => {
        // Check if reached infinite floor
        let reached_floor =  self.infinite_floor && (current.1 + 1) == self.floor;
        // Check if space below is free
        if !reached_floor && !self.hash.contains_key(&(current.0, current.1 + 1)) {
          self.current = Option::Some((current.0, current.1 + 1));
        }
        // Check if space below and to the left is free
        else if !reached_floor && !self.hash.contains_key(&(current.0 - 1, current.1 + 1)) {
          self.current = Option::Some((current.0 - 1, current.1 + 1));
        }
        // Check if space below and to the right is free
        else if !reached_floor && !self.hash.contains_key(&(current.0 + 1, current.1 + 1)) {
          self.current = Option::Some((current.0 + 1, current.1 + 1));
        }
        // If no place to fall or roll, stay where you are and spawn next grain of sand
        else {
          self.hash.insert(current, false);
          self.current = Option::None;
        }
      }
    }
  }

  /// Prompts current reservoir state
  pub fn _prompt(&self) {
    // Prompt reservoir
    let values = self.hash.keys().map(|k| k.clone()).collect::<Vec<(usize, usize)>>();
    DotDisplay::print(
      [
        values,
        match self.current {
          Option::None => vec![self.source],
          Option::Some(coords) => vec![self.source, coords],
        }        
      ].concat().to_vec(),
      |hash, coords| match hash.get(&coords) {
        Option::None => 'o',
        Option::Some(value) => if value.clone() { '#' } else { 'o' }
      },
      &self.hash
    );
  }
}
