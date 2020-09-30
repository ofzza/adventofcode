//! Puzzle ::new() implementation
//! 
//! Puzzle constructor
// -----------------------------------------------------------------------------

// Include dependencies
use std::fmt::Debug;
use super::*;

/// Puzzle ::new() implementation
/// 
/// # Generics
/// 
/// - `TInput`  - Input unit type
/// - `TOutput` - Implementation output type
/// - `TResult` - Value type to be extracted after implementation run and comnpared to expected value
/// 
/// Puzzle constructor
impl<TInput: Debug, TOutput, TResult: Debug + PartialOrd> Puzzle<TInput, TOutput, TResult> {
  
  /// Instantiate int-vector input puzzle from a string value
  /// 
  /// # Arguments
  ///
  /// * `year`           - Year the puzzle belongs to
  /// * `day`            - Day the puzzle belongs to
  /// * `index`          - Unique index of the puzzle within a day
  /// * `key`            - Additional puzzle desctiptor
  /// * `input`          - Input value to be used when running the puzzle
  /// * `implementation` - Custom implementation funciton for the puzzle
  /// * `expected`       - Result evaluating function (Returns expected value and optional received result)
  pub fn new (
    year: u32,
    day: u32,
    index: u32,
    key: &str,
    input: PuzzleInput<TInput>,
    implementation: fn(puzzle: &Puzzle<TInput, TOutput, TResult>, verbose: bool) -> Result<TOutput, &str>,
    expected: fn(result: TOutput) -> (TResult, Option<TResult>)
  ) -> Puzzle<TInput, TOutput, TResult> {
    Puzzle{
      year,
      day,
      index,
      key: String::from(key),
      input,
      implementation,
      expected
    }
  }

}
