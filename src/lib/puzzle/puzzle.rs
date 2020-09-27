//! Puzzle struct
// -----------------------------------------------------------------------------

// Include dependencies
use super::*;
use std::fmt::Debug;

/// Puzzle struct
/// 
/// TODO: more details ...
/// TODO: Firgure out #[derive(Default)]
pub struct Puzzle<TInput: Debug, TOutput, TResult: Debug + PartialOrd> {
  // Puzzle year/day/index
  pub year: u32,
  pub day: u32,
  pub index: u32,
  pub key: String,
  // Puzzle configuration
  pub config: PuzzleConfig<TOutput, TResult>,
  // Puzzle input initilaized from file
  pub input: PuzzleInput<TInput>,
  // Puzzle implementation
  pub implementation: fn(puzzle: &Puzzle<TInput, TOutput, TResult>, verbose: bool) -> Result<TOutput, &str>
}
