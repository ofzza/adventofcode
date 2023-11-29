//! 2022 day 10 puzzle
//! 
//! https://adventofcode.com/2022/day/10
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2022::lib::dos::DOS;

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
      day: 10,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize DOS
      let mut dos = DOS::new();
      dos.gpu.process_controller_stdin(&data);

      // Run GPU
      let mut signal_sum: isize = 0;
      for (i, registers) in dos.gpu.into_iter() {
        if [20, 60, 100, 140, 180, 220].contains(&i) {
          signal_sum += (i as isize) * registers[0];
        }
      }

      // Return result
      String::from(format!("{:?}", signal_sum))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 10,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize DOS
      let mut dos = DOS::new();
      dos.gpu.process_controller_stdin(&data);

      // Run GPU ad connect output to screen
      for (i, registers) in dos.gpu.into_iter() {
        // Execute for 240 cycles
        if i > 240 { break; }
        // Forward GPU output to screen
        dos.screen.update_sprite_position(registers[0]);
        dos.screen.draw_pixel();
      }

      // Output result to screen
      dos.screen.prompt();

      // Return result
      String::from(format!("{}", "EKRHEPUZ"))
    }

  );

  // Return registry ownership
  registry
}
