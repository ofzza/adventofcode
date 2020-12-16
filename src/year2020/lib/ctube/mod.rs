//! Game of Conway Tubes module
//! 
//! Implements "Conway tubes"
// -----------------------------------------------------------------------------

// Import child modules
mod constructor;
mod next_method;
mod to_string;
use std::collections::HashMap;

// (re)Export child modules
pub use constructor::*;
pub use next_method::*;
pub use to_string::*;

/// Conway Tube struct
#[derive(Debug, Default, Clone)]
pub struct ConwayTube {
  // Holds states of all existing cubes
  pub cubes: HashMap<(isize, isize, isize, isize), bool>,
}
