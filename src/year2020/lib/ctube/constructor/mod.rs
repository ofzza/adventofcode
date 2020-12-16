//! "Conway tubes" ::new() implementation
//! 
//! "Conway tubes" constructor
// -----------------------------------------------------------------------------

// Include dependencies
use super::*;

/// "Conway tubes" ::new() implementation
/// 
/// "Conway tubes" constructor
impl ConwayTube {
  
  /// Instantiate a new Conway Tube
  /// 
  /// # Arguments
  ///
  /// * `input` - Initial layout
  pub fn new (input: &Vec<String>) -> ConwayTube {
    
    // Initialize Conway Tube
    let mut ctube = ConwayTube {
      cubes: HashMap::new(),
      ..Default::default()
    };

    // Decode input
    for y in 0..input.len() {
      let bytes = input[y].as_bytes();
      for x in 0..bytes.len() {
        match bytes[x] as char {
          '#' => {
            ctube.cubes.insert((x as isize, y as isize, 0, 0), true);
          },
          '.' => {
            ctube.cubes.insert((x as isize, y as isize, 0, 0), false);
          },
          _ => panic!("Unrecognized input character!")
        }
      }
    }

    // Return Conway Tube
    return ctube;
    
  }

}
