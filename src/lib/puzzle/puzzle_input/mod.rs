//! Puzzle input struct
// -----------------------------------------------------------------------------

// Include dependencies
use std::fmt::Debug;
use crate::lib::console::*;

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

impl<T: Debug> PuzzleInput<T> {

  /// Formats input as printable string
  /// 
  /// # Arguments
  /// 
  /// - `input` - Puzzle input to format
  pub fn format_puzzle_input<> (input: &PuzzleInput<T>) -> String {
    match input {
      // Formatting single paramater
      PuzzleInput::Param(param) => {
        format!("{:?}", param)
      },
      // Formatting multiple paramaters
      PuzzleInput::Params(params) => {
        let output = format!("{:?}", params);
        if output.len() < CONSOLE_CONCAT_STRING_LENGTH {
          output[..output.len()].to_string()
        } else {
          format!("{} ...", output[..CONSOLE_CONCAT_STRING_LENGTH].to_string())
        }
      },
      // Formatting 1D vector
      PuzzleInput::Vector1D(values) => {
        let output = format!("{:?}", values);
        if output.len() < CONSOLE_CONCAT_STRING_LENGTH {
          output[..output.len()].to_string()
        } else {
          format!("{} ...", output[..CONSOLE_CONCAT_STRING_LENGTH].to_string())
        }
      },
      // Formatting 1D vector with single paramater
      PuzzleInput::ParamVector1D(input, values) => {
        let output = format!("{:?}", values);
        if output.len() < CONSOLE_CONCAT_STRING_LENGTH {
          output[..output.len()].to_string()
        } else {
          format!("{:?}:{} ...", input, output[..CONSOLE_CONCAT_STRING_LENGTH].to_string())
        }
      },
      // Formatting 1D vector with multiple paramaters
      PuzzleInput::ParamsVector1D(inputs, values) => {
        let output = format!("{:?}", values);
        if output.len() < CONSOLE_CONCAT_STRING_LENGTH {
          output[..output.len()].to_string()
        } else {
          format!("{:?}:{} ...", inputs, output[..CONSOLE_CONCAT_STRING_LENGTH].to_string())
        }
      },
      // Formatting 2D vector
      PuzzleInput::Vector2D(values) => {
        let output = format!("{:?}", values);
        if output.len() < CONSOLE_CONCAT_STRING_LENGTH {
          output[..output.len()].to_string()
        } else {
          format!("{} ...", output[..CONSOLE_CONCAT_STRING_LENGTH].to_string())
        }
      },
      // Formatting 2D vector with single paramater
      PuzzleInput::ParamVector2D(input, values) => {
        let output = format!("{:?}", values);
        if output.len() < CONSOLE_CONCAT_STRING_LENGTH {
          output[..output.len()].to_string()
        } else {
          format!("{:?}:{} ...", input, output[..CONSOLE_CONCAT_STRING_LENGTH].to_string())
        }
      },
      // Formatting 2D vector with multiple paramaters
      PuzzleInput::ParamsVector2D(inputs, values) => {
        let output = format!("{:?}", values);
        if output.len() < CONSOLE_CONCAT_STRING_LENGTH {
          output[..output.len()].to_string()
        } else {
          format!("{:?}:{} ...", inputs, output[..CONSOLE_CONCAT_STRING_LENGTH].to_string())
        }
      }
    }
  }

}
