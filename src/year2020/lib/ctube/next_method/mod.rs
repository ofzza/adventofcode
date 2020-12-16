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

    // For every current cube, generate neighbouring cubes (and copy state if defined)
    for coords in self.cubes.keys() {
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

    // For every newly existing cube, check neighbours and update state
    for coords in cubes.keys() {

      // Count neighbours
      let mut count_enabled = 0;
      for dx in -1..2 {
        for dy in -1..2 {
          for dz in if dimensionality > 2 { -1..2 } else { 0..1} {
            for dw in if dimensionality > 3 { -1..2 } else { 0..1} {
              if dx != 0 || dy != 0 || dz != 0 || dw != 0 {
                
                // Count neighbour states
                let neighbour_coords = (coords.0 + dx, coords.1 + dy, coords.2 + dz, coords.3 + dw);
                match cubes.get(&neighbour_coords) {
                  Some(state) => {
                    if state.clone() {
                      count_enabled += 1;
                    }
                  },
                  None => ()
                }

              }
            }
          }
        }
      }

      // Update state
      let current_state: bool = match cubes.get(&coords) {
        Some(state) => state.clone(),
        None => false
      };
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
