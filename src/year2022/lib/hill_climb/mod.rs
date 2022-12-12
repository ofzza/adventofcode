//! Hill Climb module
//! 
//! Hill Climb module
// -----------------------------------------------------------------------------

// Include dependecies
use crate::year::lib::matrix::Matrix;

/// Hill Climb structure
pub struct HillClimb {  
  // Hightmap matrix
  pub heightmap: Matrix,
  // Hightmap data
  heightmap_data: Vec<usize>,
  // Coordinates of the starting point as represented in height data
  pub start: Vec<usize>,
  // Coordinates of the end point as represented in height data
  pub end: Vec<usize>,
  // Callback function checking if step from one height to another height is allowed
  is_step_allowed_callback: fn(hillclimb: &HillClimb, from_height: usize, to_height: usize) -> bool,
  // Callback function checking if a position is the final position
  is_position_final_callback: fn(hillclimb: &HillClimb, coords: Vec<usize>, height: usize) -> bool
}

/// Hill Climb implementation
impl HillClimb {

  /// Constructor
  /// 
  /// # Arguments
  /// * data: Original hightmap data, where lower case chars are heights and "S" and "E" are start and end
  /// * is_step_allowed_callback: Callback function checking if step from one height to another height is allowed
  /// * is_position_final_callback: Callback function checking if a position is the final position
  pub fn new (
    data: Vec<Vec<&str>>,
    is_step_allowed_callback: fn(hillclimb: &HillClimb, from_height: usize, to_height: usize) -> bool,
    is_position_final_callback: fn(hillclimb: &HillClimb, coords: Vec<usize>, height: usize) -> bool
  ) -> HillClimb {
    // Initialize hight map data
    let hightmap = Matrix::new(vec![data[0].len(), data.len()]);    
    let mut hightmap_data: Vec<usize> = vec![usize::MAX; hightmap.length];
    let mut start: Vec<usize> = vec![];
    let mut end: Vec<usize> = vec![];

    // Copy input data into the hightmap
    for y in 0..data.len() {
      for x in 0..data[y].len() {
        // Get index in height map and point
        let height = data[y][x];
        let index = hightmap.coords_to_index(&vec![x, y]).unwrap();
        // Check if start
        if height == "S" {
          start = vec![x, y];
          hightmap_data[index] = 'a' as usize - 96;
        }
        // Check if end
        else if height == "E" {
          end = vec![x, y];
          hightmap_data[index] = 'z' as usize - 96;
        }
        // Set height
        else {
          hightmap_data[index] = height.chars().collect::<Vec<char>>()[0] as usize - 96;
        }
      }
    }

    // Return created hill climb instance
    HillClimb {
      start,
      end,
      heightmap: hightmap,
      heightmap_data: hightmap_data,
      is_step_allowed_callback,
      is_position_final_callback
    }
  }

  /// Calculates distances from given point to every other point
  /// 
  /// # Arguments
  /// * start: Point to calculate all the distances from
  /// 
  /// # Returns
  /// * Matrix data (of same measure as .hightmap matrix) containing distances from given start point
  ///   to all points of the height map
  pub fn calculate_distances(&mut self, start: &Vec<usize>) -> Vec<usize> {
    // Initialize result distances
    let mut distances = vec![usize::MAX; self.heightmap.length];

    // Set starting distance
    distances[self.heightmap.coords_to_index(start).unwrap()] = 0;

    // Calculate distances
    let start_index = self.heightmap.coords_to_index(start).unwrap();
    let mut calculated_points_latest_indexes: Vec<usize> = vec![start_index];
    let mut calculated_points_count = calculated_points_latest_indexes.len();    
    while calculated_points_count < self.heightmap.length {

      // Deduplicate latest calculated points
      calculated_points_latest_indexes.dedup();

      // Check if stuck and unable to unstuck
      if calculated_points_latest_indexes.len() == 0 {
        panic!("Can't find a path forward or any miscalculated points!");
      }

      // Find points next to latest calculated points
      let mut calculated_points_next_indexes: Vec<(usize, usize)> = vec![];
      for point_index in calculated_points_latest_indexes {
        // Find neighbours
        let neignbouring_point_indexes = self.heightmap.get_neighbours(&point_index, false);
        // Add neighbours as points to calculate next
        for neignbouring_point_index in neignbouring_point_indexes {
          calculated_points_next_indexes.push((point_index, neignbouring_point_index));
        }
      }

      // Clear latest calculated points
      calculated_points_latest_indexes = vec![];

      // Calculate next points
      for (from_point_index, to_point_index) in calculated_points_next_indexes {
        // Check if point can be reached
        let from_height = self.heightmap_data[from_point_index];
        let to_height = self.heightmap_data[to_point_index];
        if (self.is_step_allowed_callback)(self, from_height, to_height) {
          // Set point distance
          let distance = distances[from_point_index] + 1;
          if distance < distances[to_point_index] {
            // Update calculated points count (only if first calculation)
            if distances[to_point_index] == usize::MAX {
              calculated_points_count += 1;
            }
            // Update distance
            distances[to_point_index] = distance;
            // Add point as one of latest calculated point
            calculated_points_latest_indexes.push(to_point_index);
          }
          // Check if end point
          let to_point = self.heightmap.index_to_coords(&to_point_index).unwrap();
          if (self.is_position_final_callback)(self, to_point, to_height) {
            return distances;
          }
        }
      }
    }

    // Return result distances
    panic!("All points processed and no path to end found! This should bever happen!");
  }

}
