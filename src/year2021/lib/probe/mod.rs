//! Probe module
//! 
//! Probe launching module
// -----------------------------------------------------------------------------

/// Probe struct
pub struct Probe {}
/// Probe implementation
impl Probe {

  /// Calculates position based on initial velicity and time stepspassed
  /// 
  /// # Arguments
  /// * velocity: Initial velocity given in 2 coordinates
  /// * n:        Time step
  /// 
  /// # Returns
  /// Position on requested time step
  pub fn calc_position_after_launch(velocity: &(isize, isize), n: &isize) -> (isize, isize) {
    let vxabs = velocity.0.abs();
    let correction: (isize, isize) = (
      if n == &0 { 0 } else if n <= &vxabs { n * (n - 1) / 2 } else { (vxabs * (vxabs - 1) / 2) + ((n - vxabs) * vxabs) },
      if n == &0 { 0 } else { n * (n - 1) / 2 }
    );
    (
      (n * velocity.0) + (if velocity.0 >= 0 { -1isize } else { 1isize }) * correction.0,
      (n * velocity.1) - correction.1
    )
  }

  /// Tests if a trajectory with given initial velocy will hit a target
  /// 
  /// # Arguments
  /// * velocity: Initial velocity given in 2 coordinates
  /// * target:   Target area given in 2 coordinate ranges
  /// 
  /// # Returns
  /// If trajectory is within the target area at any of it's steps
  pub fn test_trajectory(velocity: &(isize, isize), target: &((isize, isize), (isize, isize))) -> bool {
    let mut n: isize = 0;
    loop {
      // Calculate position on current step
      let p = Probe::calc_position_after_launch(&velocity, &n);
      // Check if tarrget hit
      if p.0 >= target.0.0 && p.0 <= target.0.1 && p.1 >= target.1.0 && p.1 <= target.1.1 {
        return true;
      }
      // Check if target missed
      if p.0 > target.0.1 || p.1 < target.1.0 {
        return false;
      }
      // Proceed to next time step
      n += 1;
    }
  }

}
