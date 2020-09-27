//! Puzzle factory functions
//! 
//! Provide easy ways of instantiating different types of puzzles
// -----------------------------------------------------------------------------

// Include dependencies
use super::*;
use std::fmt::Debug;

/// Creates int-vector input puzzle from a string value
/// 
/// # Arguments
///
/// * `year`           - Year the puzzle belongs to
/// * `day`            - Day the puzzle belongs to
/// * `index`          - Unique index of the puzzle within a day
/// * `key`            - Additional puzzle desctiptor
/// * `input_value`    - Input value to be parsed and used when running the puzzle
/// * `input_delimiter - Character separating input values
/// * `implementation` - Custom implementation funciton for the puzzle
/// * `expected`       - Result evaluating function (Returns expected value and optional received result)
pub fn create_vec<TInput: Debug, TOutput, TResult: Debug + PartialOrd> (
  year: u32,
  day: u32,
  index: u32,
  key: &str,
  input_values: Vec<TInput>,
  implementation: fn(puzzle: &Puzzle<TInput, TOutput, TResult>, verbose: bool) -> Result<TOutput, &str>,
  expected: fn(result: TOutput) -> (TResult, Option<TResult>)
) -> Puzzle<TInput, TOutput, TResult> {
  return Puzzle{
    year,
    day,
    index,
    key: String::from(key),
    config: PuzzleConfig{
      expected,
    },
    input: PuzzleInput{
      value_vec: input_values
    },
    implementation,
  }
}
