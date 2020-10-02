//! Orbit struct
//! 
//! Implements an Orbit struct
// -----------------------------------------------------------------------------

// Include dependencies
use std::rc::Rc;
use std::cell::RefCell;

/// Orbit struct
/// 
/// TODO: more details ...
#[derive(Default)]
pub struct Orbit {
  pub name: String,
  pub parent: Option<Rc<RefCell<Orbit>>>,
  pub children: Vec<Rc<RefCell<Orbit>>>,
  _parents_cache: Option<Vec<Rc<RefCell<Orbit>>>>,
}

/// Orbit implementation
impl Orbit {

  /// Instantiates a new ORbit instance and returns a clonable, counted reference to it
  /// 
  /// # Arguments
  /// 
  /// - `name`   - Orbit name
  /// - `parent` - Optional parent orbit clonable, counted reference
  pub fn new_rc (name: String, parent: Option<Rc<RefCell<Orbit>>>) -> Rc<RefCell<Orbit>> {
    Rc::new(
      RefCell::new(
        Orbit{
          name,
          parent,
          children: vec![],
          ..Default::default()
        }
      )
    )
  }

  /// Composes a vector of all parents and grandparents, etc ... of an orbit instance
  pub fn get_parents (&mut self) -> Vec<Rc<RefCell<Orbit>>> {
    // Check if has parent
    match &self.parent {
      // If parent, get all parents
      Some(parent) => {
        // Check if cached value
        match &self._parents_cache {
          // Return cached parents
          Some(cache) => cache.clone(),
          // Fetch and cache all parents
          None => {
            // Initialize result
            let mut result:Vec<Rc<RefCell<Orbit>>> = Vec::new();
            // Add parent and grandparents
            result.push(parent.clone());
            for grandparent in parent.borrow_mut().get_parents() {
              result.push(grandparent);
            }
            // Store cache
            self._parents_cache = Some(result);
            // Return result
            return match &self._parents_cache {
              Some(cache) => cache.clone(),
              None => vec![]
            };
          }
        }
      },
      // Return no parents
      None => vec![]
    }
  }

}
