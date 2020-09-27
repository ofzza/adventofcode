//! Puzzle configuration struct
// -----------------------------------------------------------------------------

// Include dependencies
use std::fmt::Debug;

/// Puzzle configuration struct
/// 
/// TODO: more details ...
/// TODO: Firgure out #[derive(Default)]
pub struct PuzzleConfig<TOutput, TResult: Debug> {
  pub expected: fn(result: TOutput) -> (TResult, Option<TResult>)
}
