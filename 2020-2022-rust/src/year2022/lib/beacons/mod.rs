//! Beacon Exclusion Zone module
//! 
//! Beacon Exclusion Zone module
// -----------------------------------------------------------------------------

// Include dependencies
use std::collections::HashSet;

/// Beacon Exclusion Zone structure
pub struct BeaconExclusionZone {
  data: Vec<(Vec<isize>, Vec<isize>)>
}

/// Beacon Exclusion Zone implementation
impl BeaconExclusionZone {

  /// Constructor
  /// 
  /// # Arguments
  /// * data: Coordinates of all sensor-beacon pairs
  pub fn new (data: Vec<(Vec<isize>, Vec<isize>)>) -> BeaconExclusionZone {
    BeaconExclusionZone {
      data
    }
  }

  /// Detects bounds within witch a beacon could be detected
  /// 
  /// # Returns
  /// Tuple of X and Y bounds found within the data
  pub fn detect_bounds (&mut self) -> (Vec<isize>, Vec<isize>) {
    // Initialize bounds
    let mut bounds = (vec![isize::MAX, isize::MIN], vec![isize::MAX, isize::MIN]);
    // Search for bounds
    for pair in &self.data {
      let radius: isize = BeaconExclusionZone::manhattan_distance(&pair.0, &pair.1) as isize;
      // Find min X
      if pair.0[0] - radius < bounds.0[0] { bounds.0[0] = pair.0[0] - radius; }
      if pair.1[0] - radius < bounds.0[0] { bounds.0[0] = pair.1[0] - radius; }
      // Find max X
      if pair.0[0] + radius > bounds.0[1] { bounds.0[1] = pair.0[0] + radius; }
      if pair.1[0] + radius > bounds.0[1] { bounds.0[1] = pair.1[0] + radius; }
      // Find min Y
      if pair.0[1] - radius < bounds.1[0] { bounds.1[0] = pair.0[1] - radius; }
      if pair.1[1] - radius < bounds.1[0] { bounds.1[0] = pair.1[1] - radius; }
      // Find max Y
      if pair.0[1] + radius > bounds.1[1] { bounds.1[1] = pair.0[1] + radius; }
      if pair.1[1] + radius > bounds.1[1] { bounds.1[1] = pair.1[1] + radius; }
    }
    // Return bounds
    bounds
  }

  // Deduplicates and returns all unique sensors and beacons
  pub fn get_distinct_sensors_and_beacons (&mut self) -> (Vec<Vec<isize>>, Vec<Vec<isize>>) {
    // Initialize hashsets
    let mut sensors: HashSet<Vec<isize>> = HashSet::new();
    let mut beacons: HashSet<Vec<isize>> = HashSet::new();
    // Dedupliacate sensors and beacons
    for pair in &self.data {
      sensors.insert(pair.0.clone());
      beacons.insert(pair.1.clone());
    }
    // Return found sensors and beacons
    (
      sensors.drain().collect::<Vec<Vec<isize>>>(),
      beacons.drain().collect::<Vec<Vec<isize>>>()
    )
  }

  /// Scans within given bounds for points of no coverage
  /// 
  /// # Arguments
  /// * bounds: Tuple of X and Y bounds to search within
  /// * callback: Function called every time a point with no coverage is found
  /// * aggregate: Value passed between every call to the callback function
  pub fn scan_bounds_for_missing_coverage<T> (&mut self, bounds: (Vec<isize>, Vec<isize>), callback: fn(T, Vec<isize>) -> T, mut aggregate: T) -> T {
    
    // Split bounds
    let x_left = bounds.0[0];
    let x_right = bounds.0[1];
    let y_top = bounds.1[0];
    let y_bottom = bounds.1[1];
    for pair in &self.data {
      // Calculate radius of current sensor
      let radius = BeaconExclusionZone::manhattan_distance(&pair.0, &pair.1);
      // Calculate corners of the bounded area
      let corner_tl = vec![x_left, y_top];
      let corner_tr = vec![x_right, y_top];
      let corner_bl = vec![x_left, y_bottom];
      let corner_br = vec![x_right, y_bottom];
      // Check if all corners of the bounded area are within the sensor coverage area
      if BeaconExclusionZone::manhattan_distance(&pair.0, &corner_tl) <= radius
      && BeaconExclusionZone::manhattan_distance(&pair.0, &corner_tr) <= radius
      && BeaconExclusionZone::manhattan_distance(&pair.0, &corner_bl) <= radius
      && BeaconExclusionZone::manhattan_distance(&pair.0, &corner_br) <= radius {
        // Entire area covered, no poin ts to execute callback on
        return aggregate;
      }
    }

    // If bounds are 1x1 in size, they aren't covered
    if x_left == x_right && y_top == y_bottom {
      aggregate = callback(aggregate, vec![bounds.0[0], bounds.1[0]]);
    }

    // If not fully covered, split bounds into sections and check individually
    else {

      // Calculate midpoints
      let x_mid = ((x_left + x_right) as f64 / 2.0).floor() as isize;
      let y_mid = ((y_top + y_bottom) as f64 / 2.0).floor() as isize;

      // If single dimension of bounds can't be split, split in 2
      if x_left == x_right {
        aggregate = self.scan_bounds_for_missing_coverage((vec![x_mid, x_mid], vec![y_top, y_mid]), callback, aggregate);
        aggregate = self.scan_bounds_for_missing_coverage((vec![x_mid, x_mid], vec![y_mid + 1, y_bottom]), callback, aggregate);
      }
      else if y_top == y_bottom {
        aggregate = self.scan_bounds_for_missing_coverage((vec![x_left, x_mid], vec![y_mid, y_mid]), callback, aggregate);
        aggregate = self.scan_bounds_for_missing_coverage((vec![x_mid + 1, x_right], vec![y_mid, y_mid]), callback, aggregate);
      }
      // ... else split in 4
      else {
        aggregate = self.scan_bounds_for_missing_coverage((vec![x_left, x_mid], vec![y_top, y_mid]), callback, aggregate);
        aggregate = self.scan_bounds_for_missing_coverage((vec![x_mid + 1, x_right], vec![y_top, y_mid]), callback, aggregate);
        aggregate = self.scan_bounds_for_missing_coverage((vec![x_left, x_mid], vec![y_mid + 1, y_bottom]), callback, aggregate);
        aggregate = self.scan_bounds_for_missing_coverage((vec![x_mid + 1, x_right], vec![y_mid + 1, y_bottom]), callback, aggregate);
      }

    }

    // Return aggregated
    return aggregate;
  }

  /// Calculates manhattan distance beterrn 2 points
  /// 
  /// # Arguments
  /// * a: First point coordinates
  /// * b: Second point coordinates
  /// 
  /// # Returns
  /// Manhattan distance between the points
  fn manhattan_distance (a: &Vec<isize>, b: &Vec<isize>) -> usize {
    ((a[0] - b[0]).abs() + (a[1] - b[1]).abs()) as usize
  }
}
