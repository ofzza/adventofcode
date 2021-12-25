//! Amphipods burrow module
//! 
//! Implements amphipods burrow module for reordering amphipods
// -----------------------------------------------------------------------------

//    0 0 0 0 0 0 0 0 0 0 1
// 0: 0 1 2 3 4 5 6 7 8 9 0
// 1:     .   .   .   .
// 2:     .   .   .   .
// 3:     .   .   .   .
// 4:     .   .   .   .

// Include dependencies
use std::collections::hash_map::*;

/// Amphipod structur
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Amphipod {
  color: char,
  position: (usize, usize),
  finished: bool
}

/// Amphipods burrow structure
#[derive(Clone)]
pub struct AmphipodsBurrow {
  amphipods: Vec<Amphipod>,
  room_size: usize,
  room_ocupancy: Vec<usize>
}
/// Amphipods burrow implementation
impl AmphipodsBurrow {

  /// Constructor
  /// 
  /// # Arguments
  /// * amphipods: Vector of amphipods represented by their type in order of home positions (X first, Y second)
  pub fn new (data: Vec<char>) -> AmphipodsBurrow {
    // Initialize amphipods
    let room_size = data.len() / 4;
    let mut amphipods: Vec<Amphipod> = Vec::with_capacity(data.len());
    for i in 0..data.len() {
      let x: usize = 2 + 2 * (i / room_size);
      let y: usize = 1 + i % room_size;
      amphipods.push(Amphipod { color: data[i], position: (x, y), finished: false });
    }
    // Instantiate a burrow
    AmphipodsBurrow {
      amphipods,
      room_size,
      room_ocupancy: vec![0, 0, 0, 0]
    }
  }

  /// Recursivelly organizes amphipods and keeps track of minimal number of moves needed to reorganize all amphipods
  /// 
  /// # Arguments
  /// * burrow:   State of the burroww to organize from
  /// * cache:    Cache object to store calculation results and avoid recalculating same state multiple times
  /// * depth:    Recursive depth
  /// * verbose:  If true, all full state will be output aftre each step
  /// 
  /// Returns
  /// Option of minimal number of moves needed to reorganize all amphipods, or NOne if not able to organize
  fn organize_internal(burrow: &mut AmphipodsBurrow, cache: &mut HashMap<Vec<Amphipod>, Option<usize>>, depth: usize, verbose: bool) -> Option<usize> {
    // Print burrow state
    if verbose {
      println!("{}", burrow.to_string());
      println!("");
    }

    // Check if all amphipods in final state
    match burrow.amphipods.iter().find(|a| !a.finished) {
      None => {
        return Some(0)
      },
      _ => ()
    }

    // Check if state has a cached value
    match cache.get(&burrow.amphipods) {
      Some(count) => { return count.clone(); },
      None => ()
    }
    
    // Initialize minimal count
    let mut count: Option<usize> = None;

    // Try all possible moves for all amphipods
    for i in 0..burrow.amphipods.len() {
      let amphipod = &burrow.amphipods[i];
      let amphipod_energy_per_step = if amphipod.color == 'A' {
          1
        } else if amphipod.color == 'B' {
          10
        } else if amphipod.color == 'C' {
          100
        } else if amphipod.color == 'D' {
          1000
        } else {
          0
        };

      // If amphipod is finished positioning, skip to next one
      if amphipod.finished { continue; }

      // Check if positioned in the hallway
      if amphipod.position.1 == 0 {
        // Get destination based on color and if destination room is already partly full
        let destination: (usize, usize) = match amphipod.color {
          'A' => (2, burrow.room_size - burrow.room_ocupancy[0]),
          'B' => (4, burrow.room_size - burrow.room_ocupancy[1]),
          'C' => (6, burrow.room_size - burrow.room_ocupancy[2]),
          'D' => (8, burrow.room_size - burrow.room_ocupancy[3]),
          _ => panic!("Unknown amphipod color found!")
        };
        // Check if destination taken
        let destination_available = match burrow.amphipods.iter().find(|a| a.position.0 == destination.0 && a.position.1 <= destination.1) { None => true, _=> false };
        if destination_available {
          // Check if path clear to destination
          let path_clear_option = if amphipod.position.0 < destination.0 {
            burrow.amphipods.iter().find(|a| a.position.1 == 0 && a.position.0 > amphipod.position.0 && a.position.0 < destination.0)
          } else {
            burrow.amphipods.iter().find(|a| a.position.1 == 0 && a.position.0 > destination.0 && a.position.0 < amphipod.position.0)
          };
          match path_clear_option {
            // If path clear, go home
            None => {
              // Move 
              let mut next = burrow.clone();
              let path_energy = amphipod_energy_per_step * (destination.1 + (amphipod.position.0 as isize - destination.0 as isize).abs() as usize);
              next.amphipods[i].position.0 = destination.0;
              next.amphipods[i].position.1 = destination.1;
              next.amphipods[i].finished = true;
              next.room_ocupancy[destination.0 / 2 - 1] += 1;
              // Recursively reorganize
              match AmphipodsBurrow::organize_internal(&mut next, cache, depth + 1, verbose) {
                // Found organized position
                Some(c_next) => {
                  match count {
                    Some(c_current) => { if c_next + path_energy < c_current { count = Some(c_next + path_energy); } },
                    None => count = Some(c_next + path_energy)
                  }
                },
                // Didn't find organized position
                None => ()
              }
            },
            _ => ()
          }
        }
      }

      // Check if positioned inside a room
      let is_in_room = amphipod.position.1 != 0;
      let is_not_blocked_in_room = match burrow.amphipods.iter().find(|a| a.position.0 == amphipod.position.0 && a.position.1 < amphipod.position.1) { None => true, _ => false };
      if is_in_room && is_not_blocked_in_room {        
        // Search for amphipod blocking hallway to the left
        let mut x_leftmost_free = 0;
        for j in 0..burrow.amphipods.len() {
          // Check if in hallway
          if burrow.amphipods[j].position.1 != 0 { continue; }
          // Check if blocking the path
          let ax = burrow.amphipods[j].position.0;
          if ax + 1 <= amphipod.position.0 && ax + 1 > x_leftmost_free {
            x_leftmost_free = ax + 1;
          }
        }
        // Try all available hallway locations to the left
        for x in x_leftmost_free..amphipod.position.0 {
          // Check if allowed position in hallway
          if x == 2 || x == 4 || x == 6 || x == 8 { continue; }
          // Move 
          let mut next = burrow.clone();
          let path_energy = amphipod_energy_per_step * (amphipod.position.1 + (amphipod.position.0 - x));
          next.amphipods[i].position.0 = x;
          next.amphipods[i].position.1 = 0;
          // Recursively reorganize
          match AmphipodsBurrow::organize_internal(&mut next, cache, depth + 1, verbose) {
            // Found organized position
            Some(c_next) => {
              match count {
                Some(c_current) => { if c_next + path_energy < c_current { count = Some(c_next + path_energy); } },
                None => count = Some(c_next + path_energy)
              }
            },
            // Didn't find organized position
            None => ()
          }
        }
        // Search for amphipod blocking hallway to the right
        let mut x_rightmost_free = 10;
        for j in 0..burrow.amphipods.len() {
          // Check if in hallway
          if burrow.amphipods[j].position.1 != 0 { continue; }
          // Check if blocking the path
          let ax = burrow.amphipods[j].position.0;
          if ax >= amphipod.position.0 + 1 && ax < x_rightmost_free + 1 {
            x_rightmost_free = ax - 1;
          }
        }
        // Try all available hallway locations to the right
        for x in (amphipod.position.0 + 1)..(x_rightmost_free + 1) {
          // Check if allowed position in hallway
          if x == 2 || x == 4 || x == 6 || x == 8 { continue; }
          // Move 
          let mut next = burrow.clone();
          let path_energy = amphipod_energy_per_step * (amphipod.position.1 + (x - amphipod.position.0));
          next.amphipods[i].position.0 = x;
          next.amphipods[i].position.1 = 0;
          // Recursively reorganize
          match AmphipodsBurrow::organize_internal(&mut next, cache, depth + 1, verbose) {
            // Found organized position
            Some(c_next) => {
              match count {
                Some(c_current) => { if c_next + path_energy < c_current { count = Some(c_next + path_energy); } },
                None => count = Some(c_next + path_energy)
              }
            },
            // Didn't find organized position
            None => ()
          }
        }
      }
    }

    // Cache result    
    cache.insert(burrow.amphipods.clone(), count);

    // Return minimal found count
    count
  }

  /// Starts a recursive organization process and returns a minimal number of moves needed to reorganize all amphipods
  /// 
  /// # Arguments
  /// * verbose: If true, all full state will be output aftre each step
  /// 
  /// # Returns
  /// Minimal number of moves needed to reorganize all amphipods
  pub fn organize (&mut self, verbose: bool) -> usize {
    // Check any that can stay in place
    for y in (1..(self.room_size + 1)).rev() {
      for i in 0..self.amphipods.len() {
        let mut amphipod = &mut self.amphipods[i];
        if amphipod.color == 'A' && amphipod.position.0 == 2 && amphipod.position.1 == y && self.room_ocupancy[0] == (self.room_size - y) {
          amphipod.finished = true;
          self.room_ocupancy[0] += 1;
        }
        else if amphipod.color == 'B' && amphipod.position.0 == 4 && amphipod.position.1 == y && self.room_ocupancy[1] == (self.room_size - y) {
          amphipod.finished = true;
          self.room_ocupancy[1] += 1;
        }
        else if amphipod.color == 'C' && amphipod.position.0 == 6 && amphipod.position.1 == y && self.room_ocupancy[2] == (self.room_size - y) {
          amphipod.finished = true;
          self.room_ocupancy[2] += 1;
        }
        else if amphipod.color == 'D' && amphipod.position.0 == 8 && amphipod.position.1 == y && self.room_ocupancy[3] == (self.room_size - y) {
          amphipod.finished = true;
          self.room_ocupancy[3] += 1;
        }
      }
    }
    // Recursivelly organize
    match AmphipodsBurrow::organize_internal(self, &mut HashMap::new(), 0, verbose) {
      Some(count) => count,
      None => panic!("Failed to organize amphipods!")
    }
  }

  /// Reoresents current burrow state as a printable string
  pub fn to_string (&self) -> String {
    // Initialize positions represented by color characters
    let mut hallway: Vec<char> = vec!['_', '_', '.', '_', '.', '_', '.', '_', '.', '_', '_'];
    let mut room: Vec<char> = Vec::with_capacity(self.room_size);
    for i in 0..self.room_size { room.push('_'); }
    let mut rooms: Vec<Vec<char>> = vec![
      room.clone(),
      room.clone(),
      room.clone(),
      room.clone()
    ];
    // Fill positions
    for i in 0..self.amphipods.len() {
      let amphipod = &self.amphipods[i];
      let key = if amphipod.finished { amphipod.color.to_ascii_uppercase() } else { amphipod.color.to_ascii_lowercase() };
      if amphipod.position.1 == 0 {
        if hallway[amphipod.position.0] != '_' { panic!("Multiple amphipods in sae location detected?!"); }
        hallway[amphipod.position.0] = key;
      } else if amphipod.position.0 == 2 {
        rooms[0][amphipod.position.1 - 1] = key;
      } else if amphipod.position.0 == 4 {
        rooms[1][amphipod.position.1 - 1] = key;
      } else if amphipod.position.0 == 6 {
        rooms[2][amphipod.position.1 - 1] = key;
      } else if amphipod.position.0 == 8 {
        rooms[3][amphipod.position.1 - 1] = key;
      }
    }
    // Output positions as string
    let mut result: Vec<String> = vec![
      hallway.iter().collect::<String>(),
    ];
    for i in 0..self.room_size {
      result.push(vec!['.', '.', rooms[0][i], '.', rooms[1][i], '.', rooms[2][i], '.', rooms[3][i], '.', '.'].iter().collect::<String>());
    }
    result.join("\n")
  }

}
