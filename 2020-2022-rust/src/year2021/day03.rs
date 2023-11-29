//! 2021 day 03 puzzle
//! 
//! https://adventofcode.com/2021/day/3
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2021::lib::diagnostic_report::*;

/// Parses input data
fn parse(data: &String) -> Vec<Vec<bool>>{
  Input::parse(data.as_str().trim(), "\n", |line| {
    Input::parse(line.trim(), "", |x| {
      if x == "1" { true } else { false }
    })
  })
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 3,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Instantiate diagnostic report
      let report = DiagnosticReport::new(&data);
      // Aggregate messages
      let aggregate = report.aggregate();
      
      // Decode messages
      let gamma = DiagnosticMessage {
        code: aggregate.iter().map(|b| if b.0 > b.1 { false } else { true }).collect()
      };
      let epsilon = DiagnosticMessage {
        code: aggregate.iter().map(|b| if b.0 < b.1 { false } else { true }).collect()
      };

      // Calculate power consumption
      let power_consumption = gamma.as_usize() * epsilon.as_usize();

      // Return result
      String::from(format!("{:?}", power_consumption))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 3,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // // Instantiate diagnostic report
      let mut report = DiagnosticReport::new(&data);
      
      // Decode messages
      let o2_generator_rating = report.process(
        // Check stop condition
        |state| { state.messages.len() > 1 },
        // On every step update path
        |mut state, branches| {
          state.filter.code.push(if branches.1.len() >= branches.0.len() { true } else { false });
          state.messages = if branches.1.len() >= branches.0.len() { branches.1 } else { branches.0 };
          state
        }
      );
      let co2_scrubber_rating = report.process(
        // Check stop condition
        |state| { state.messages.len() > 1 },
        // On every step update path
        |mut state, branches| {
          state.filter.code.push(if branches.0.len() <= branches.1.len() { false } else { true });
          state.messages = if branches.0.len() <= branches.1.len() { branches.0 } else { branches.1 };
          state
        }
      );

      // Calculate power consumption
      let file_support_rating = o2_generator_rating.messages[0].as_usize() * co2_scrubber_rating.messages[0].as_usize();

      // Return result
      String::from(format!("{:?}", file_support_rating))
    }

  );

  // Return registry ownership
  registry
}
