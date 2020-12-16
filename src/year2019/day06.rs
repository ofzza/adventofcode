//! 2019/06 puzzle
//! 
//! https://adventofcode.com/2019/day/6
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::inputs::*;
use crate::lib::puzzle::*;
use super::orbits::*;

/// Registers puzzles for the day
pub fn run (index: u32, key: &str, verbose: bool, obfuscate: bool) -> PuzzleExecutionStatistics {

  // Initialize stats
  let mut stats = PuzzleExecutionStatistics{
    ..Default::default()
  };
  
  // Run puzzle
  if (index == 0) || (index == 1) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector1D(vec![
        String::from("COM)B"), String::from("B)C"), String::from("C)D"), String::from("D)E"),
        String::from("E)F"), String::from("B)G"), String::from("G)H"), String::from("D)I"),
        String::from("E)J"), String::from("J)K"), String::from("K)L")
      ]);
      stats.update(
        Puzzle::new(2019, 6, 1, "test", input, implementation1, |n| (n, Some(42)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<String>(load_input("./src/year2019/data/day06input.txt"), "\n");
      stats.update(
        Puzzle::new(2019, 6, 1, "solution", input, implementation1, |n| (n, Some(204521)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Run puzzle
  if (index == 0) || (index == 2) {
      // Test
      let input = PuzzleInput::Vector1D(vec![
        String::from("COM)B"), String::from("B)C"), String::from("C)D"), String::from("D)E"),
        String::from("E)F"), String::from("B)G"), String::from("G)H"), String::from("D)I"),
        String::from("E)J"), String::from("J)K"), String::from("K)L"), String::from("K)YOU"), String::from("I)SAN")
      ]);
      stats.update(
        Puzzle::new(2019, 6, 2, "test", input, implementation2, |n| (n, Some(4)))
          .run(false, obfuscate)
      );
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<String>(load_input("./src/year2019/data/day06input.txt"), "\n");
      stats.update(
        Puzzle::new(2019, 6, 2, "solution", input, implementation2, |n| (n, Some(307)))
          .run(false, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<String, u32, u32>, verbose: bool) -> Result<u32, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(orbits) => {
      // Initialize orbits diagram
      let orbits = OrbitDiagram::new(orbits.clone()).orbits;
      // Count direct and indirect orbits
      let mut total = 0;
      for orbit_name in orbits.keys() {
        // Initialize parents count        
        let mut orbit = orbits.get(&orbit_name.clone())
          .expect("This should never, ever happen!")
          .borrow_mut();
        // Get parents
        let parents = orbit.get_parents();
        // If verbose, map orbit and parents
        if verbose {
          // Compose parent names
          let mut parent_names: Vec<String> = Vec::new();
          for parent in &parents {
            parent_names.push(parent.borrow().name.clone());
          }
          // Print orbit nad parents
          println!("{} => ({}){}", orbit_name, parents.len(), PuzzleInput::format_puzzle_input(&PuzzleInput::Vector1D(vec![..parent_names])));
        }
        // Add parents count
        total += parents.len();
      }
      // Return count
      Ok(total as u32)
    },
    _ => panic!("This should never, ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<String, u32, u32>, _verbose: bool) -> Result<u32, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(orbits) => {
      // Initialize orbits diagram
      let diagram = OrbitDiagram::new(orbits.clone());
      // Get diagram endpoints and parents
      let you = diagram.orbits.get("YOU");
      let you_parents = match you {
          Some(orbit) => orbit.borrow_mut().get_parents(),
          _ => panic!("This should never, ever happen!")
        };
      let san = diagram.orbits.get("SAN");
      let san_parents = match san {
          Some(orbit) => orbit.borrow_mut().get_parents(),
          _ => panic!("This should never, ever happen!")
        };
      // Find first common parent
      for (i, you_parent) in you_parents.iter().enumerate() {
        for (j, san_parent) in san_parents.iter().enumerate() {
          if you_parent.borrow().name == san_parent.borrow().name {
            return Ok((i + j) as u32);
          }
        }
      }
      // Return count
      Ok(0)
    },
    _ => panic!("This should never, ever happen!")
  }
}
