//! Puzzle input struct
// -----------------------------------------------------------------------------

/// Puzzle input struct
/// 
/// TODO: more details ...
pub struct PuzzleInput {
  pub text: String,
  pub value_int_vec: Vec<u32>
}

/// Puzzle input struct Default trait implementation
impl Default for PuzzleInput {
  fn default() -> PuzzleInput {
    PuzzleInput{
      text: String::default(),
      value_int_vec: Vec::new()
    }
  }
}
