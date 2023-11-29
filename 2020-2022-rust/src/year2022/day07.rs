//! 2022 day 07 puzzle
//! 
//! https://adventofcode.com/2022/day/7
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2022::lib::dos::*;

/// Parses input data
fn parse(data: &String) -> Vec<&str> {
  Input::parse(data.as_str().trim(), "\n", |data| data)
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 7,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize DOS
      let mut dos = DOS::new();

      // Parse out individial commands
      dos.process_terminal_stdout(&data);
      // Refresh directory structure sizes
      dos.fs.refresh_sizes();      
      // Traverse FS
      let size: usize = dos.fs.traverse(|directory, _, aggregate| {
        // Match directories
        match directory {
          Option::Some(directory) => aggregate + if directory.size <= 100000 { directory.size } else { 0 },
          _ => aggregate
        }
      }, 0);

      // Return result
      String::from(format!("{:?}", size))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 7,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize DOS
      let mut dos = DOS::new();

      // Parse out individial commands
      dos.process_terminal_stdout(&data);
      // Refresh directory structure sizes
      dos.fs.refresh_sizes();      
      // Get total used space
      let total = 70000000;
      let used = dos.fs.root.size;
      let free = total - used;
      // Traverse FS to find directory to delete
      let (_, size): (usize, usize) = dos.fs.traverse(|directory, _, aggregate| {
        // Match directories
        match directory {
          Option::Some(directory) => if (aggregate.0 + directory.size) >= 30000000 && directory.size < aggregate.1 { (aggregate.0, directory.size) } else { aggregate },
          _ => aggregate
        }
      }, (free, usize::MAX));

      // Return result
      String::from(format!("{:?}", size))
    }

  );

  // Return registry ownership
  registry
}
