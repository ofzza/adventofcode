//! Puzzle input struct
// -----------------------------------------------------------------------------

/// Puzzle input enum
/// 
/// TODO: more details ...
pub enum PuzzleInput<T> {
  Param(T),
  Params(Vec<T>),
  Vector1D(Vec<T>),
  ParamVector1D(T, Vec<T>),
  ParamsVector1D(Vec<T>, Vec<T>),
  Vector2D(Vec<Vec<T>>),
  ParamVector2D(T, Vec<Vec<T>>),
  ParamsVector2D(Vec<T>, Vec<Vec<T>>),
}
