//! Puzzle module
//! 
//! Definition of each puzzle
// -----------------------------------------------------------------------------

// Import child modules
mod constructor;
mod run_method;
mod puzzle_input;
mod puzzle_stats;

// (re)Export child modules
pub use constructor::*;
pub use run_method::*;
pub use puzzle_input::*;
pub use puzzle_stats::*;

// Include dependencies
use std::fmt::Debug;

/// Puzzle struct
/// 
/// /// # Generics
/// 
/// - `TInput`  - Input unit type
/// - `TOutput` - Implementation output type
/// - `TResult` - Value type to be extracted after implementation run and comnpared to expected value
///
/// TODO: more details ...
/// TODO: Firgure out #[derive(Default)]
pub struct Puzzle<TInput: Debug, TOutput, TResult: Debug + PartialOrd> {
  // Puzzle year/day/index
  pub year: u32,
  pub day: u32,
  pub index: u32,
  pub key: String,
  // Puzzle input
  pub input: PuzzleInput<TInput>,
  // Puzzle implementation
  pub implementation: fn(puzzle: &Puzzle<TInput, TOutput, TResult>, verbose: bool) -> Result<TOutput, &str>,
  // Puzzle result extraction and expected result definition function
  pub expected: fn(result: TOutput) -> (TResult, Option<TResult>)
}
