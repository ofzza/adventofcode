//! Puzzle factory functions
//! 
//! Provide easy ways of instantiating different types of puzzles
// -----------------------------------------------------------------------------

// Include dependencies
use super::*;

/// Creates int-vector input puzzle from a string value
/// 
/// # Arguments
///
/// * `year`           - Year the puzzle belongs to
/// * `day`            - Day the puzzle belongs to
/// * `index`          - Unique index of the puzzle within a day
/// * `key`            - Additional puzzle desctiptor
/// * `input_value`    - Input value to be parsed and used when running the puzzle
/// * `expected`       - Expected output result from running the puzzle with the provided inputs
/// * `implementation` - Custom implementation funciton for the puzzle
pub fn create_intvec_from_value (year: u32, day: u32, index: u32, key: String, input_value: String, expected: String, implementation: fn(puzzle: &Puzzle, verbose: bool) -> String) -> Puzzle {
  return Puzzle{
    year,
    day,
    index,
    key,
    config: PuzzleConfig{
      input_value,
      input_parse_as_int_vec: true,
      input_parse_as_int_vec_delimiter: '\n',
      expected,
      ..Default::default()
    },
    implementation,
    ..Default::default()
  }
}

/// Creates int-vector input puzzle from a string file path
/// 
/// # Arguments
///
/// * `year`           - Year the puzzle belongs to
/// * `day`            - Day the puzzle belongs to
/// * `index`          - Unique index of the puzzle within a day
/// * `key`            - Additional puzzle desctiptor
/// * `input_path`     - Path to the input file to be parsed and used when running the puzzle
/// * `expected`       - Expected output result from running the puzzle with the provided inputs
/// * `implementation` - Custom implementation funciton for the puzzle
pub fn create_intvec_from_path (year: u32, day: u32, index: u32, key: String, input_path: String, expected: String, implementation: fn(puzzle: &Puzzle, verbose: bool) -> String) -> Puzzle {
  return Puzzle{
    year,
    day,
    index,
    key,
    config: PuzzleConfig{
      input_path,
      input_parse_as_int_vec: true,
      input_parse_as_int_vec_delimiter: '\n',
      expected,
      ..Default::default()
    },
    implementation,
    ..Default::default()
  }
}
