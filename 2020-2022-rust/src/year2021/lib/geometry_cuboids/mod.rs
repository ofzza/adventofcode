//! Geometry cuboids module
//! 
//! Implements geometry for N-dimensional cuboids in an N-dimensional volume
// -----------------------------------------------------------------------------

/// Geometry cuboid struct
#[derive(Debug, Clone)]
pub struct GeometryPoint {
  pub coords: Vec<isize>
}

/// Geometry cuboid struct
#[derive(Debug, Clone)]
pub struct GeometryCuboid {
  pub min: GeometryPoint,
  pub max: GeometryPoint
}
/// Geometry cuboid implementation
impl GeometryCuboid {
  /// Constructor
  /// 
  /// # Arguments
  /// * a: First point defining the cuboid
  /// * b: Second point defining the cuboid
  pub fn new (a: &GeometryPoint, b: &GeometryPoint) -> GeometryCuboid {
    // Find min/max points
    let mut min: Vec<isize> = Vec::with_capacity(a.coords.len());
    let mut max: Vec<isize> = Vec::with_capacity(a.coords.len());
    for i in 0..a.coords.len() {
      if a.coords[i] < b.coords[i] {
        min.push(a.coords[i].clone());
        max.push(b.coords[i].clone());
      } else {
        min.push(b.coords[i].clone());
        max.push(a.coords[i].clone());
      }
    }
    // Generate actual points    
    GeometryCuboid {
      min: GeometryPoint { coords: min },
      max: GeometryPoint { coords: max }
    }
  }
}

/// Geometry cuboid space struct
#[derive(Debug)]
pub struct GeometryCuboidSpace {
  dimensionality: usize,
  cuboids: Vec<GeometryCuboid>
}
/// Geometry cuboid implementation
impl GeometryCuboidSpace {

  /// Constructor
  /// 
  /// # Arguments
  /// * dimensionality: Dimensionality of the cuboid space
  pub fn new (dimensionality: usize) -> GeometryCuboidSpace {
    GeometryCuboidSpace {
      dimensionality,
      cuboids: vec![]
    }
  }

  /// Finds intersecting cuboid volume between 2 cuboid volumes
  /// 
  /// # Arguments
  /// * a: First cuboid volume to intersect
  /// * b: First cuboid volume to intersect
  /// 
  /// # Returns
  /// Option of found intersecting cuboid voumne
  pub fn find_intersection (&self, a: &GeometryCuboid, b: &GeometryCuboid) -> Option<GeometryCuboid> {
    // Find intersection ranges by dimension
    let mut min: Vec<isize> = Vec::with_capacity(self.dimensionality);
    let mut max: Vec<isize> = Vec::with_capacity(self.dimensionality);
    for i in 0..self.dimensionality {
      let mut range: Vec<isize> = Vec::with_capacity(2);
      if a.min.coords[i] >= b.min.coords[i] && a.min.coords[i] <= b.max.coords[i] { range.push(a.min.coords[i]); }
      else if b.min.coords[i] >= a.min.coords[i] && b.min.coords[i] <= a.max.coords[i] { range.push(b.min.coords[i]); }
      if a.max.coords[i] >= b.min.coords[i] && a.max.coords[i] <= b.max.coords[i] { range.push(a.max.coords[i]); }
      else if b.max.coords[i] >= a.min.coords[i] && b.max.coords[i] <= a.max.coords[i] { range.push(b.max.coords[i]); }
      if range.len() != 2 { return None; }
      min.push(if range[0] < range[1] { range[0] } else { range[1] });
      max.push(if range[0] > range[1] { range[0] } else { range[1] });
    }
    Some(
      GeometryCuboid {
        min: GeometryPoint { coords: min },
        max: GeometryPoint { coords: max }
      }
    )
  }

  /// Adds a cobuid volume to the space, splitting any previously added cuboid volumes up such that
  /// no part of the total volumne belongs to more than single cuboid
  /// 
  /// # Arguments
  /// * cuboid: Cuboid to add to the space
  pub fn add (&mut self, cuboid: &GeometryCuboid) {
    // Subtract the cuboid
    self.subtract(cuboid);
    // Add new cuboid in full
    self.cuboids.push(cuboid.clone());
  }
  
  /// Removes a cobuid volume from the space, splitting any previously added cuboid volumes up such that
  /// the removed volume no longer belongs to any of the previously added cuboids
  /// 
  /// # Arguments
  /// * cuboid: Cuboid to add to the space
  pub fn subtract (&mut self, cuboid: &GeometryCuboid) {
    // Initialize updated cuboid collection
    let mut updated_cuboids: Vec<GeometryCuboid> = Vec::with_capacity(self.cuboids.len());
    // Find cuboid intersection with all existing cuboids
    for i in 0..self.cuboids.len() {
      // Get existing cuboid volume and find intersection
      let existing = &self.cuboids[i];      
      match self.find_intersection(existing, &cuboid) {
        // Break apart existidng volume and leave out the intersection
        Some(intersection) => {
          let mut remaining_cuboid = existing.clone();
          for j in 0..self.dimensionality {
            // Break off "bottom" half
            if intersection.min.coords[j] > remaining_cuboid.min.coords[j] {
              // Store broken off cuboid
              let min: GeometryPoint = remaining_cuboid.min.clone();
              let mut max: GeometryPoint = remaining_cuboid.max.clone();
              max.coords[j] = intersection.min.coords[j] - 1;
              updated_cuboids.push(GeometryCuboid { min, max });
              // Update remaining cuboid after breaking off a piece
              let mut min: GeometryPoint = remaining_cuboid.min.clone();
              let max: GeometryPoint = remaining_cuboid.max.clone();
              min.coords[j] = intersection.min.coords[j];
              remaining_cuboid = GeometryCuboid { min, max };
            }
            // Break off "top" half
            if intersection.max.coords[j] < remaining_cuboid.max.coords[j] {
              // Store broken off cuboid
              let mut min: GeometryPoint = remaining_cuboid.min.clone();
              let max: GeometryPoint = remaining_cuboid.max.clone();
              min.coords[j] = intersection.max.coords[j] + 1;
              updated_cuboids.push(GeometryCuboid { min, max });
              // Update remaining cuboid after breaking off a piece
              let min: GeometryPoint = remaining_cuboid.min.clone();
              let mut max: GeometryPoint = remaining_cuboid.max.clone();
              max.coords[j] = intersection.max.coords[j];
              remaining_cuboid = GeometryCuboid { min, max };
            }
          }
        },
        // Leave cuboid alone
        None => {
          updated_cuboids.push(existing.clone());
        }
      }
      
    }
    // Update cuboids
    self.cuboids = updated_cuboids;
  }

  /// Gets total volume of all cuboids in the space
  /// 
  /// # Returns
  /// Total volume of all cuboids in the space
  pub fn get_volume (&self) -> usize {
    let mut volume: usize = 0;
    for i in 0..self.cuboids.len() {
      let mut cuboid_volume: usize = 1;
      for j in 0..self.dimensionality {
        cuboid_volume *= (self.cuboids[i].max.coords[j] - self.cuboids[i].min.coords[j] + 1) as usize
      }
      volume += cuboid_volume;      
    }
    volume
  }

}
