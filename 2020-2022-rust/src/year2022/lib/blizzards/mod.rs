//! Blizzard Basin module
//! 
//! Blizzard Basin module
// -----------------------------------------------------------------------------

// Include dependencies
use std::collections::hash_set::HashSet;
use crate::year::lib::math::Math;


/// Blizzard Basin structure
pub struct BlizzardBasin {
  // Starting player position
  pub position_start: Vec<usize>,
  // End played position
  pub position_end: Vec<usize>,
  // Bounding coordinates for blizzards to move through
  blizzards_bounds: Vec<(usize, usize)>,
  // Bounding dimensions for blizzards to move through
  blizzards_dimensions: Vec<usize>,
  // Blizard starting positions and speeds
  blizzards: HashSet<(Vec<usize>, Vec<isize>)>,
  // Keeps track of all previously tested positions at all previous times
  _cache: HashSet<(Vec<usize>, usize)>
}

/// Blizzard Basin implementation
impl BlizzardBasin {

  /// Constructor
  pub fn new (data: Vec<Vec<char>>) -> BlizzardBasin {

    // Find starting position
    let mut position_start_x: usize = 0;
    for i in 0..data[0].len() {
      if data[0][i] == '.' {
        position_start_x = i;
        break;
      }
    }
    // Find ending position
    let mut position_end_x: usize = 0;
    for i in 0..data[data.len() - 1].len() {
      if data[data.len() - 1][i] == '.' {
        position_end_x = i;
        break;
      }
    }

    // Get blizzard bounds and dimensions
    let blizzards_bounds = vec![(1, data[0].len() - 1), (1, data.len() - 1)];
    let blizzards_dimensions = vec![blizzards_bounds[0].1 - blizzards_bounds[0].0, blizzards_bounds[1].1 - blizzards_bounds[1].0];

    // Initialize blizzards hashmap
    let mut blizzards: HashSet<(Vec<usize>, Vec<isize>)> = HashSet::new();
    for y in blizzards_bounds[1].0..blizzards_bounds[1].1 {
      for x in blizzards_bounds[0].0..blizzards_bounds[0].1 {
        // Check if blizzard
        if data[y][x] != '.' {

          if data[y][x] == '<' {
            blizzards.insert((vec![x, y], vec![-1,  0]));
          }
          else if data[y][x] == '>' {
            blizzards.insert((vec![x, y], vec![ 1,  0]));
          }
          else if data[y][x] == '^' {
            blizzards.insert((vec![x, y], vec![ 0, -1]));
          }
          else if data[y][x] == 'v' {
            blizzards.insert((vec![x, y], vec![ 0,  1]));
          }

        }
      }  
    }

    // Return initialized Blizzard Basin instance
    BlizzardBasin {
      // Starting player position
      position_start: vec![position_start_x, 0],
      // Starting player position
      position_end: vec![position_end_x, data.len() - 1],
      // Bounding coordinates for blizzards to move through
      blizzards_bounds,
      // Bounding dimensions for blizzards to move through
      blizzards_dimensions,
      // Blizard starting positions indexed by path
      blizzards,
      // Keeps track of all previously tested positions at all previous times
      _cache: HashSet::new()
    }
  }

  /// Finds the quickest path from start to end
  /// 
  /// # Arguments
  /// * start: Starting position
  /// * target: Target position 
  /// * time_offset: Initial time offset
  /// 
  /// # Returns
  /// Minimal number of steps it takes from start to end
  pub fn traverse (&mut self, start: Vec<usize>, target: Vec<usize>, time_offset: usize) -> usize {
    // Find shortest possible path length
    let limit = (self.position_end[0] as isize - self.position_start[0] as isize).abs() as usize
              + (self.position_end[1] as isize - self.position_start[1] as isize).abs() as usize;

    // Try finding a path with progressively more lax time limits
    let mut limit_factor = 2;
    loop {
      self._cache.clear();
      match self._traverse(start.clone(), target.clone(), time_offset, time_offset + limit * limit_factor) {
        Some(steps) => return steps,
        _ => ()
      };
      limit_factor += 1;
    }
  }

  /// Finds the quickest path from start to end
  /// 
  /// # Arguments
  /// * position: Current position
  /// * target: Target position
  /// * time_offset: Time offset from starting
  /// * time_limit: Maximum time a path is allowed to take
  /// 
  /// # Returns
  /// Minimal number of steps it takes from start to end
  fn _traverse (&mut self, position: Vec<usize>, target: Vec<usize>, time_offset: usize, time_limit: usize) -> Option<usize> {
    // Limit length of the path
    if time_offset > time_limit {
      return Option::None;      
    }
    
    // Check if collided with a blizzard
    if self.check_coordinates_for_blizards(position.clone(), time_offset) > 0 {
      return Option::None;
    }

    // Check if previously tested same "branch"
    if self._cache.contains(&(position.clone(), time_offset)) {
      return Option::None;
    }
    
    // Cache current position and time
    self._cache.insert((position.clone(), time_offset));

    // Check if target position reached
    if position == target {
      return Some(time_offset);
    }

    // Initialize lowest steps count
    let mut min_steps: usize = usize::MAX;

    // Attempt a step down
    let position_next = vec![position[0], position[1] + 1];
    if position_next[1] < self.blizzards_bounds[1].1 || position_next == self.position_end {
      match self._traverse(position_next, target.clone(), time_offset + 1, time_limit) {
        Some(min_steps_candidate) => { if min_steps_candidate < min_steps { min_steps = min_steps_candidate; } },
        _ => ()
      };
    }
    // Check if inside the field and horizontal movement is allowed
    if position[1] >= self.blizzards_bounds[1].0 && position[1] < self.blizzards_bounds[1].1 {
      // Attempt a step right
      let position_next = vec![position[0] + 1, position[1]];
      if position_next[0] < self.blizzards_bounds[0].1 {
        match self._traverse(position_next, target.clone(), time_offset + 1, time_limit) {
          Some(min_steps_candidate) => { if min_steps_candidate < min_steps { min_steps = min_steps_candidate; } },
          _ => ()
        };
      }
      // Attempt a step left
      if position[0] > 0 {
        let position_next = vec![position[0] - 1, position[1]];
        if position_next[0] >= self.blizzards_bounds[0].0 {
          match self._traverse(position_next, target.clone(), time_offset + 1, time_limit) {
            Some(min_steps_candidate) => { if min_steps_candidate < min_steps { min_steps = min_steps_candidate; } },
            _ => ()
          };
        }
      }
    }
    // Attempt a step up
    if position[1] > 0 {
      let position_next = vec![position[0], position[1] - 1];
      if position_next[1] >= self.blizzards_bounds[1].0 || position_next == self.position_start {
        match self._traverse(position_next, target.clone(), time_offset + 1, time_limit) {
          Some(min_steps_candidate) => { if min_steps_candidate < min_steps { min_steps = min_steps_candidate; } },
          _ => ()
        };
      }
    }
    // Attemp to stay in place
    match self._traverse(position, target.clone(), time_offset + 1, time_limit) {
      Some(min_steps_candidate) => { if min_steps_candidate < min_steps { min_steps = min_steps_candidate; } },
      _ => ()
    };

    // Return min steps found
    if min_steps == usize::MAX { Option::None } else { Option::Some(min_steps) }
  }

  /// Checks coordinates at a given point in time for passing blizzards
  /// 
  /// # Arguments
  /// * coordinates: Coordinates to check for blizzards
  /// * time_offse: Time offset from the initial conditions to check for blizzards
  /// 
  /// # Returns
  /// Number of blizzards found
  fn check_coordinates_for_blizards (&self, coordinates: Vec<usize>, time_offset: usize) -> usize {
    // Calculate candidate initial coordinates and speeds
    let candidate_coordinates_left = vec![
      Math::index_wraparound(coordinates[0] as isize - self.blizzards_bounds[0].0 as isize - time_offset as isize, self.blizzards_dimensions[0]) + self.blizzards_bounds[0].0,
      coordinates[1]
    ];
    let candidate_coordinates_right = vec![
      Math::index_wraparound(coordinates[0] as isize - self.blizzards_bounds[0].0 as isize + time_offset as isize, self.blizzards_dimensions[0]) + self.blizzards_bounds[0].0,
      coordinates[1]
    ];
    let candidate_coordinates_up = vec![
      coordinates[0],
      Math::index_wraparound(coordinates[1] as isize - self.blizzards_bounds[1].0 as isize - time_offset as isize, self.blizzards_dimensions[1]) + self.blizzards_bounds[1].0
    ];
    let candidate_coordinates_down = vec![
      coordinates[0],
      Math::index_wraparound(coordinates[1] as isize - self.blizzards_bounds[1].0 as isize + time_offset as isize, self.blizzards_dimensions[1]) + self.blizzards_bounds[1].0
    ];

    // Check candidate coordinates
    let blizzards_from_left  = self.blizzards.contains(&(candidate_coordinates_left, vec![ 1,  0]));
    let blizzards_from_right = self.blizzards.contains(&(candidate_coordinates_right, vec![-1,  0]));
    let blizzards_from_up    = self.blizzards.contains(&(candidate_coordinates_up, vec![ 0,  1]));
    let blizzards_from_down  = self.blizzards.contains(&(candidate_coordinates_down, vec![ 0, -1]));

    // Return blizzard count
    return  if blizzards_from_left  { 1 } else { 0 }
          + if blizzards_from_right { 1 } else { 0 }
          + if blizzards_from_up    { 1 } else { 0 }
          + if blizzards_from_down  { 1 } else { 0 }
  }

  /// Prompts the field at a given point in time
  /// 
  /// # Arguments
  /// * position: Current position of the player
  /// * time_offset: Time offset to draw the grid for
  fn _prompt (&self, position: &Vec<usize>, time_offset: usize) {
    // Draw line spacing
    println!();

    // Prompt time
    println!("Time: {}", time_offset);

    // Draw starting line
    print!("#");
    for x in 1..self.blizzards_dimensions[1] {
      let p = vec![x, 0];
      print!("{}", if &p == position { 'E' } else if p == self.position_start { '.' } else { '#' });
    }
    print!("#");
    println!();

    // Draw blizzards field
    for y in self.blizzards_bounds[1].0..self.blizzards_bounds[1].1 {
      // Line start
      print!("#");
      for x in self.blizzards_bounds[0].0..self.blizzards_bounds[0].1 {
        let p = vec![x, y];
        let blizzards_count = self.check_coordinates_for_blizards(p.clone(), time_offset);
        if &p == position {
          if blizzards_count > 0 {
            panic!("STEPPED INTO A BLIZZARD - SHOULD NEVER EVER HAPPEN!!!");
          }
          print!("E");
        } else if blizzards_count > 0 {
          print!("{}", '@');
        } else {
          print!("{}", '.');
        }
      }
      // Line end
      print!("#");
      println!();
    }

    // Draw end line
    print!("#");
    for x in 1..self.blizzards_dimensions[0] + 1 {
      let p = vec![x, self.blizzards_dimensions[1] + 1];
      print!("{}", if &p == position { 'E' } else if p == self.position_start { '.' } else { '#' });
    }
    print!("#");
    println!();

  }

}
