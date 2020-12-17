//! "Conway tubes" .next() implementation
//! 
//! Implements a generator pattern working on top of the "Conway tubes" running
//! the actual iteration steps
// -----------------------------------------------------------------------------

// Include dependencies
use super::*;

/// "Conway tubes" .next() implementation
/// 
/// Implements a generator pattern working on top of the "Conway tubes" running
/// the actual iteration steps
impl ConwayTube {
  
  /// Executes next "Conway tube" step
  /// 
  /// # Arguments
  /// 
  /// * `verbose`           - Outputs executing output of the puzzle to the console
  /// * `dimensionality`    - Number of dimensions being considered when checking neighbours (must be larger or equal to 2)
  pub fn next (&mut self, _verbose: bool, dimensionality: usize) -> () {

    // Initialize next state of the tube
    let mut cubes: HashMap<(isize, isize, isize, isize), bool> = self.cubes.clone();

    // Collect enabled coordinates and for every current cube, seed neighbouring cubes
    let mut enabled_coords: Vec<(isize, isize, isize, isize)> = vec![];
    for coords in self.cubes.keys() {
      // Check if cube enabled
      if self.cubes.get(&coords).expect("Failed fetching cube status!").clone() {
        enabled_coords.push(coords.clone());
      }
      // Generate neighbour cubes
      for dx in -1..2 {
        for dy in -1..2 {
          for dz in if dimensionality > 2 { -1..2 } else { 0..1} {
            for dw in if dimensionality > 3 { -1..2 } else { 0..1} {
              if dx != 0 || dy != 0 || dz != 0 || dw != 0 {
                let neighbour_coords = (coords.0 + dx, coords.1 + dy, coords.2 + dz, coords.3 + dw);
                let neighbour_state: bool = match cubes.get(&neighbour_coords) {
                  Some(state) => state.clone(),
                  None => false
                };
                cubes.insert(neighbour_coords, neighbour_state);
              }
            }
          }
        }
      }
    }

    // Count enabled neighbours of existing cubes
    let mut enabled_counts: HashMap<(isize, isize, isize, isize), usize> = HashMap::new();
    for coords in enabled_coords {

      // Count neighbours
      let mut count_enabled = 0;
      for dx in -1..2 {
        for dy in -1..2 {
          for dz in if dimensionality > 2 { -1..2 } else { 0..1} {
            for dw in if dimensionality > 3 { -1..2 } else { 0..1} {
              if dx != 0 || dy != 0 || dz != 0 || dw != 0 {

                // Get neighbour coords and state
                let neighbour_coords = (coords.0 + dx, coords.1 + dy, coords.2 + dz, coords.3 + dw);
                let neighbour_state = match cubes.get(&neighbour_coords) {
                  Some(state) => state.clone(),
                  None => false
                };

                // Count neighbour states of currently considered, enabled cube
                if neighbour_state {
                  // Count as enabled neighbour of currently considered, enabled cube
                  count_enabled += 1;
                } else {
                  // Add currently considered, enabled cube towards a count of enabled neighbours of the neighbour
                  let count = match enabled_counts.get(&neighbour_coords) {
                    Some(count) => count.clone(),
                    None => 0
                  };
                  enabled_counts.insert(neighbour_coords, count + 1);
                }

              }
            }
          }
        }
      }

      // Store currently considered, enabled cube's enabled neighbours count
      enabled_counts.insert(coords, count_enabled);
      
    }

    // Update states of all existing cubes
    for coords in cubes.keys() {
      // Get enabled neighbours count
      let count_enabled = match enabled_counts.get(coords) {
        Some(count) => count.clone(),
        None => 0
      };
      // Get current state
      let current_state: bool = match cubes.get(&coords) {
        Some(state) => state.clone(),
        None => false
      };
      // Update state
      if current_state && count_enabled != 2 && count_enabled != 3 {
        self.cubes.insert(coords.clone(), false);
      } else if !current_state && count_enabled == 3 {
        self.cubes.insert(coords.clone(), true);
      } else {
        self.cubes.insert(coords.clone(), current_state);
      }
    }

  }

}
