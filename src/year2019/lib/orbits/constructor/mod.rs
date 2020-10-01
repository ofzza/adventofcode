//! Orbits ::new() implementation
//! 
//! Orbits constructor
// -----------------------------------------------------------------------------

// Include dependencies
use super::*;

/// Orbits ::new() implementation
/// 
/// Orbits constructor
impl OrbitDiagram {
  
  /// Instantiate a new Orbits diagram
  /// 
  /// # Arguments
  ///
  /// * `input` - Orbits syntax to parse
  pub fn new (input: Vec<String>) -> OrbitDiagram {
    
    // Initialize diagram
    let mut diagram = OrbitDiagram{
      orbits: HashMap::new()
    };
    
    // Parse orbits
    for line in input {
      
      // Parse orbit syntax
      let parsed_line: Vec<&str> = line.split(')').collect();
      if parsed_line.len() != 2 {
        panic!("Expecting orbit syntax to be 'XXX/YYY'!");
      }

      // Get orbit names
      let parent_orbit_name = String::from(parsed_line[0]);
      let current_orbit_name = String::from(parsed_line[1]);

      // Create or get (un)registered parent orbit
      let parent_orbit = if !diagram.orbits.contains_key(&parent_orbit_name) {
          Orbit::new_rc(parent_orbit_name.clone(), None)
        } else {
          diagram.orbits.remove(&parent_orbit_name)
            .expect("This should never, ever happen!")
        };

      // Register current orbit if not already registered
      let current_orbit = if !diagram.orbits.contains_key(&current_orbit_name) {
          Orbit::new_rc(current_orbit_name.clone(), Some(parent_orbit.clone()))
        } else {
          let orbit = diagram.orbits.remove(&current_orbit_name)
            .expect("This should never, ever happen!");
          orbit.borrow_mut().parent = Some(parent_orbit.clone());
          orbit
        };

      // Add current to parent as child
      parent_orbit.borrow_mut().children.push(current_orbit.clone());

      // (re)Register parent and current orbit
      diagram.orbits.insert(parent_orbit_name, parent_orbit);
      diagram.orbits.insert(current_orbit_name, current_orbit);
      
    }

    // Return diagram
    return diagram;
    
  }

}
