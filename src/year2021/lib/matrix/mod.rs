//! Matrix module
//! 
//! N-dimensional matrix implementation
// -----------------------------------------------------------------------------

/// Matrix struct
pub struct Matrix<T> {
  pub dimensions: Vec<usize>,
  pub data: Vec<T>,
  dimension_offset_factors: Vec<usize>
}
/// Matrix implementation
impl<T> Matrix<T> {

  /// Constructor
  /// 
  /// # Arguments
  /// * dimensions: Dimensionality of the matrix
  /// * data:       Data to fill the matrix with
  pub fn new(dimensions: Vec<usize>, data: Vec<T>) -> Matrix<T> where T: Clone {
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
    // Instantiate matrix
    Matrix {
      dimensions,
      data,
      dimension_offset_factors
    }
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

  /// Gets the data by its N-dimensional coordinates
  /// 
  /// # Arguments
  /// * coords: Coordinates for requested data
  /// 
  /// # Returns
  /// Data at requested coordinates
  pub fn get(&self, coords: &Vec<usize>) -> Option<&T> {
    match self.coords_to_index(coords) {
      Some(index) => Some(&self.data[index]),
      None => None
    }
  }

  /// Sets data to particular N-dimensional coordinates
  /// 
  /// # Arguments
  /// * coords: Coordinates to write data to
  /// * value:  Value to write to
  pub fn set(&mut self, coords: &Vec<usize>, value: T) {
    match self.coords_to_index(coords) {
      Some(index) => self.data[index] = value,
      None => ()
    }
  }

  /// Gets neighbouring points to a requested point
  /// 
  /// # Arguments
  /// * coords:             Coordinates to find neighbours for
  /// * include_diagonals:  If diagonal neighbours should be included
  /// 
  /// # Returns
  /// Vector of point coordinate-value pairs for neighbouring points
  pub fn get_neighbours(&self, coords: &Vec<usize>, include_diagonals: bool) -> Vec<(Vec<usize>, &T)> {
    // Initialize neighbouring points
    let mut points: Vec<Vec<usize>> = Vec::with_capacity(3usize.pow(coords.len() as u32));
    // Add neighbouring points (ignoring diagonals)
    for i in 0..coords.len() {
      if coords[i] >= 1 {
        let mut p = coords.clone();
        p[i] -= 1;
        points.push(p);  
      }
      if coords[i] <= self.dimensions[i] - 2 {
        let mut p = coords.clone();
        p[i] += 1;
        points.push(p);  
      }
    }
    // Map points to point-value pairs
    points.iter().map(|p| (p.clone(), self.get(p).unwrap())).collect()
  }

}
