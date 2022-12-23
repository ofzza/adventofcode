//! 2022 day 23 puzzle
//! 
//! https://adventofcode.com/2022/day/23
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2022::lib::unstable_diffusion::UnstableDiffusion;

/// Parses input data
fn parse<'a>(data: &'a String) -> Vec<Vec<&str>> {
  Input::parse(data.as_str().trim(), "\n", |data| {
    Input::parse(data, "", |data| {
      data
    })
  })
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 23,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize unstable diffusion
      let mut ud = UnstableDiffusion::new(data);
      // Play until stable
      for _ in 0..10 {
        // Play round
        ud.play_round();

        // Prompt
        // ud._prompt();
        // println!();
      }

      // Calculate empty spaces
      let bounds = ud.get_bounds();
      let spaces = bounds.0 * bounds.1;
      let spaces_taken = ud.points.len();
      let spaces_empty = spaces - spaces_taken;

      // Return result
      String::from(format!("{:?}", spaces_empty))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 23,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);
      
      // Initialize unstable diffusion
      let mut ud = UnstableDiffusion::new(data);
      // Play until stable
      let mut rounds_count = 0;
      loop {
        // Play until stable and count rounds
        if ud.play_round() != 0 {
          rounds_count += 1;
        }
        // Once stable, stop
        else {
          break;
        }
      }

      // Return result
      String::from(format!("{:?}", rounds_count + 1))
    }

  );

  // Return registry ownership
  registry
}
