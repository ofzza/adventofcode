//! Voxel Space module
//! 
//! Voxel Space module
// -----------------------------------------------------------------------------

// Include dependencies
use std::collections::hash_map::HashMap;

/// Voxel Space structure
pub struct VoxelSpace<'a> {
  // Coordinates of all voxels in the space
  pub voxels: Vec<Vec<isize>>,
  // Min/Max bounds of the space in all 3 coordinates
  bounds: Vec<(isize, isize)>,
  // Polygon enumeration for rendering all the voxels as 3d cubes
  // (coordinate values assumed to be divided by 2 before rendering)
  pub polygons: Vec<(Vec<isize>, &'a str)>
}

/// Voxel Space implementation
impl<'a> VoxelSpace<'a> {

  /// Constructor
  /// 
  /// # Arguments
  /// * data: Vector of ciirdinate vectors of voxels in the space
  pub fn new<'b> (voxels: Vec<Vec<isize>>) -> VoxelSpace<'b> {
    // Initialize bounds
    let mut bounds: Vec<(isize, isize)> = vec![(isize::MAX, isize::MIN); 3];
    // Compose space bounds
    for voxel in &voxels {
      for d in 0..3 {
        if voxel[d] < bounds[d].0 { bounds[d].0 = voxel[d] };
        if voxel[d] > bounds[d].1 { bounds[d].1 = voxel[d] };
      }
    }
    // Construct and return a vulcano instance
    VoxelSpace {
      // Coordinates of all voxels in the space
      voxels,
      // Min/Max bounds of the space in all 3 coordinates
      bounds,
      // Polygon enumeration for rendering all the voxels as 3d cubes
      polygons: vec![]
    }
  }

  /// Find any voxels fully enclosed by other voxles
  /// 
  /// # Returns
  /// Vector of fully enclosed voxels
  pub fn find_fully_enclosed_voxels (&mut self) -> Vec<Vec<isize>> {
    // Initialize a (1 unit extended) space wiht empty and full voxels organized by coordinates
    let size =
        ((self.bounds[0].1 + 2) - (self.bounds[0].0 - 1)).abs() as usize
      * ((self.bounds[1].1 + 2) - (self.bounds[1].0 - 1)).abs() as usize
      * ((self.bounds[2].1 + 2) - (self.bounds[2].0 - 1)).abs() as usize;
    let mut voxels_by_coordinate: HashMap<Vec<isize>, bool> = HashMap::with_capacity(size);
    // Fill with empty voxels
    for x in (self.bounds[0].0 - 1)..(self.bounds[0].1 + 2) {
      for y in (self.bounds[0].0 - 1)..(self.bounds[1].1 + 2) {
        for z in (self.bounds[0].0 - 1)..(self.bounds[2].1 + 2) {
          voxels_by_coordinate.insert(vec![x, y, z], false);
        }
      }
    }
    // Fill in full voxels
    for voxel in &self.voxels {
      voxels_by_coordinate.insert(voxel.clone(), true);
    }

    // Start "filling in water" from a corner
    let mut water_candidate_coordinates: Vec<Vec<isize>> = Vec::with_capacity(size);
    water_candidate_coordinates.push(vec![self.bounds[0].0 - 1, self.bounds[1].0 - 1, self.bounds[2].0 - 1]);
    while water_candidate_coordinates.len() > 0 {
      // Take pop candidate coordinates to fill in with water
      let candidate = water_candidate_coordinates.pop().unwrap();
      // Check if candidate already taken by a voxel or already "filled with water"
      if !voxels_by_coordinate.get(&candidate).unwrap() {
        // Set candidate coordiates as filled by water
        voxels_by_coordinate.insert(candidate.clone(), true);
        // Set next candidates
        if candidate[0] > self.bounds[0].0 {
          let next = vec![candidate[0] - 1, candidate[1], candidate[2]];
          if !voxels_by_coordinate.get(&next).unwrap() { water_candidate_coordinates.push(next); }
        }
        if candidate[0] <= self.bounds[0].1 {
          let next = vec![candidate[0] + 1, candidate[1], candidate[2]];
          if !voxels_by_coordinate.get(&next).unwrap() { water_candidate_coordinates.push(next); }
        }
        if candidate[1] > self.bounds[1].0 {
          let next = vec![candidate[0], candidate[1] - 1, candidate[2]];
          if !voxels_by_coordinate.get(&next).unwrap() { water_candidate_coordinates.push(next); }
        }
        if candidate[1] <= self.bounds[1].1 {
          let next = vec![candidate[0], candidate[1] + 1, candidate[2]];
          if !voxels_by_coordinate.get(&next).unwrap() { water_candidate_coordinates.push(next); }
        }
        if candidate[2] > self.bounds[2].0 {
          let next = vec![candidate[0], candidate[1], candidate[2] - 1];
          if !voxels_by_coordinate.get(&next).unwrap() { water_candidate_coordinates.push(next); }
        }
        if candidate[2] <= self.bounds[2].1 {
          let next = vec![candidate[0], candidate[1], candidate[2] + 1];
          if !voxels_by_coordinate.get(&next).unwrap() { water_candidate_coordinates.push(next); }
        }
      }
    }

    // Find and return coordinates not take up by voxels or "filled by water"
    voxels_by_coordinate.iter().filter(|(_, val)| !val.clone()).map(|(key, _)| key.clone()).collect::<Vec<Vec<isize>>>()
  }

  /// Enumerates visible polygons needed to render voxels as cubes
  /// 
  /// # Returns
  /// Polygon enumeration for rendering all the voxels as 3d cubes
  // (coordinate values assumed to be divided by 2 before rendering)
  pub fn enumerate_polygons (&mut self) -> &Vec<(Vec<isize>, &'a str)> {
    // Initialize a hashmap for deduplicating polygons
    let mut hash: HashMap<Vec<isize>, (Vec<isize>, &str)> = HashMap::with_capacity(self.voxels.len() * 6);
    
    // Generate polygons for all voxels
    for voxel in &self.voxels {
      // Define polygons (with double the coordinates to leave space in between the voxel centers)
      let voxel_polygons:Vec<(Vec<isize>, &str)> = vec![
        (vec![voxel[0] as isize * 2 - 1, voxel[1] as isize * 2, voxel[2] as isize * 2], "left"),
        (vec![voxel[0] as isize * 2 + 1, voxel[1] as isize * 2, voxel[2] as isize * 2], "right"),
        (vec![voxel[0] as isize * 2, voxel[1] as isize * 2 - 1, voxel[2] as isize * 2], "top"),
        (vec![voxel[0] as isize * 2, voxel[1] as isize * 2 + 1, voxel[2] as isize * 2], "bottom"),
        (vec![voxel[0] as isize * 2, voxel[1] as isize * 2, voxel[2] as isize * 2 - 1], "forward"),
        (vec![voxel[0] as isize * 2, voxel[1] as isize * 2, voxel[2] as isize * 2 + 1], "back"),
      ];
      // Test each polygon and:
      // - Add if unique
      // - Remove if boundry between 2 voxels
      for voxel_polygon in voxel_polygons {
        if !hash.contains_key(&voxel_polygon.0) {
          hash.insert(voxel_polygon.0.clone(), voxel_polygon);
        } else {
          hash.remove(&voxel_polygon.0);
        }
      }
    }

    // Store polygons
    self.polygons = hash.values().map(|v| v.clone()).collect();

    // Return polygons
    &self.polygons
  }
}
