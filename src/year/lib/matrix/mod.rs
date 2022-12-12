//! Matrix module
//! 
//! N-dimensional matrix implementation
// -----------------------------------------------------------------------------

/// Matrix struct
pub struct Matrix {
  pub length: usize,
  pub dimensions: Vec<usize>,
  dimension_offset_factors: Vec<usize>,
  neighbour_relative_coordinates: Vec<Vec<isize>>
}
/// Matrix implementation
impl Matrix {

  /// Constructor
  /// 
  /// # Arguments
  /// * dimensions: Dimensionality of the matrix
  pub fn new(dimensions: Vec<usize>) -> Matrix {
    // Calculate total length
    let mut length = 1;
    for i in 0..dimensions.len() {
      length *= dimensions[i];
    }
    // Calculate dimension offsets
    let mut dimension_offset_factors: Vec<usize> = Vec::with_capacity(dimensions.len());
    for i in 0..dimensions.len() {
      if i == 0 {
        dimension_offset_factors.push(1);
      } else {
        let mut dimension_offset_factor = 1;
        for j in i-1..i {
          dimension_offset_factor *= dimensions[j];
        }
        dimension_offset_factors.push(dimension_offset_factor);
      }
    }
    // Calculate neighbours' relative coordinates
    let mut neighbour_relative_coordinates: Vec<Vec<isize>> = vec![vec![]];
    for _ in 0..dimensions.len() {
      // Extend existing coordinates with an extra dimension
      for j in 0..neighbour_relative_coordinates.len() {
        neighbour_relative_coordinates[j].push(0);
      }
      // Add new coordinates
      for j in 0..neighbour_relative_coordinates.len() {
        let mut a = neighbour_relative_coordinates[j].clone();
        a.pop();
        a.push(1);
        neighbour_relative_coordinates.push(a);
        let mut b = neighbour_relative_coordinates[j].clone();
        b.pop();
        b.push(-1);
        neighbour_relative_coordinates.push(b);
      }
    }
    // Instantiate matrix
    Matrix {
      length,
      dimensions,
      dimension_offset_factors,
      neighbour_relative_coordinates: neighbour_relative_coordinates.iter()
        .filter(|c| { c.iter().find(|x| x != &&0) != None })
        .map(|c| { c.clone() })
        .collect()
    }
  }

  /// Creates a preallocated vector for storing data for a matrix with matxhing dimensions
  /// 
  /// # Returns
  /// Preallocated vector for storing data for a matrix with matxhing dimensions
  pub fn create<T>(&self) -> Vec<T> {
    Vec::with_capacity(self.length)
  }

  /// Gets N-dimensional coordinates from data index
  /// 
  /// # Arguments
  /// * Index: Data index
  /// 
  /// # Returns
  /// Calculated N-dimensional coordinates
  pub fn index_to_coords (&self, index: &usize) -> Option<Vec<usize>> {
    let mut coords: Vec<usize> = Vec::with_capacity(self.dimensions.len());
    let mut index: usize = index.clone();
    for i in (0..self.dimensions.len()).rev() {
      coords.push(index / self.dimension_offset_factors[i]);
      index = index % self.dimension_offset_factors[i];
    }
    coords.reverse();
    Some(coords)
  }

  /// Gets data index from N-dimensional coordinates
  /// 
  /// # Arguments
  /// * coords: Coordinates for requested data
  /// 
  /// # Returns
  /// Calculated index
  pub fn coords_to_index (&self, coords: &Vec<usize>) -> Option<usize> {
    let mut index = 0;
    for i in 0..coords.len() {
      if coords[i] < self.dimensions[i] {
        index += coords[i] * self.dimension_offset_factors[i];
      } else {
        return None;
      }
    }
    Some(index)
  }

  /// Gets neighbouring points to a requested point
  /// 
  /// # Arguments
  /// * index:              Index to find neighbours for
  /// * include_diagonals:  If diagonal neighbours should be included
  /// 
  /// # Returns
  /// Vector of indexes of neighbouring points
  pub fn get_neighbours(&self, index: &usize, include_diagonals: bool) -> Vec<usize> {
    // Find coordinates
    let coords = self.index_to_coords(index).unwrap();
    // Generate neighbour coordinates from pregenerated neighbour relative coordinates
    let mut neighbours: Vec<usize> = Vec::with_capacity(self.neighbour_relative_coordinates.len());
    for i in 0..self.neighbour_relative_coordinates.len() {
      if include_diagonals || self.neighbour_relative_coordinates[i].iter().find(|n| n == &&0) != None {
        let mut inbounds = true;
        let mut neighbour: Vec<usize> = coords.clone();
        for j in 0..self.neighbour_relative_coordinates[i].len() {
          let coordinate: isize = neighbour[j] as isize + self.neighbour_relative_coordinates[i][j];
          if coordinate >= 0 && coordinate < self.dimensions[j] as isize {
            neighbour[j] = (neighbour[j] as isize + self.neighbour_relative_coordinates[i][j]) as usize;
          } else {
            inbounds = false;
            break;
          }
        }
        if inbounds {
          neighbours.push(self.coords_to_index(&neighbour).unwrap());
        }
      }
    }
    // Return found neighbours
    neighbours
  }

}
