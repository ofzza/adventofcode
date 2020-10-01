//! Orbits module
//! 
//! Implements an Orbit diagram
// -----------------------------------------------------------------------------

// Import child modules
mod orbit;
mod constructor;

// (re)Export child modules
pub use orbit::*;
pub use constructor::*;

// Include dependencies
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

/// Orbit diagram struct
/// 
/// TODO: more details ...
pub struct OrbitDiagram {
  pub orbits: HashMap<String, Rc<RefCell<Orbit>>>
}
