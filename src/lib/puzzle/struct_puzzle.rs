//! Puzzle struct
// -----------------------------------------------------------------------------

// Include dependencies
use super::*;

/// Puzzle struct
/// 
/// TODO: more details ...
pub struct Puzzle {
  // Puzzle year/day/index
  pub year: u32,
  pub day: u32,
  pub index: u32,
  pub key: String,
  // Puzzle configuration
  pub config: PuzzleConfig,
  // Puzzle input initilaized from file
  pub input: PuzzleInput,
  // Puzzle implementation
  pub implementation: fn(puzzle: &Puzzle, verbose: bool) -> String
}

/// Puzzle definition struct Default trait implementation
impl Default for Puzzle {
  fn default() -> Puzzle {
    Puzzle{
      year: 0,
      day: 0,
      index: 0,
      key: String::default(),
      input: PuzzleInput{
        ..Default::default()
      },
      config: PuzzleConfig{
        ..Default::default()
      },
      implementation: |_p, _v| {
        return String::from("Not implemented!");
      }
    }
  }
}
