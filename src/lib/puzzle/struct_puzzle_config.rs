//! Puzzle configuration struct
// -----------------------------------------------------------------------------

/// Puzzle configuration struct
/// 
/// TODO: more details ...
pub struct PuzzleConfig {
  pub input_value: String,
  pub input_path: String,
  pub input_parse_as_int_vec: bool,
  pub input_parse_as_int_vec_delimiter: char,
  pub expected: String
}

/// Puzzle configuration struct Default trait implementation
impl Default for PuzzleConfig {
  fn default() -> PuzzleConfig {
    PuzzleConfig{
      input_value:                      String::default(),
      input_path:                       String::default(),
      input_parse_as_int_vec:           false,
      input_parse_as_int_vec_delimiter: '\n',
      expected:                         String::default()
    }
  }
}
