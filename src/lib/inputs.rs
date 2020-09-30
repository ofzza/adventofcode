//! Inputs module
//! 
//! Contains functions required to required puzzle inputs
// -----------------------------------------------------------------------------

// Include dependencies
use std::fs;
use std::str::FromStr;
use std::fmt::Debug;
use super::puzzle::*;
use super::console::*;

/// Loads puzzle input data
/// 
/// # Arguments
/// 
/// - `path` - File path to read input data from
pub fn load_input (path: &str) -> String {
  
  // Read input
  return fs::read_to_string(path)
    .expect("Failed reading from input file!");

}

/// Parse 1D puzzle input data
/// 
/// # Arguments
/// 
/// - `path`      - File path to read input data from
/// - `delimiter` - Character to split the data around
pub fn parse_1d<T: FromStr> (input: String, delimiter: char) -> PuzzleInput<T> {

  // Parse input (as integer array)
  let mut result: Vec<T> = Vec::new();
  let lines = input.split(delimiter);
  for line in lines {
    if line.len() > 0 {
      match String::from(line.trim()).parse::<T>() {
        Ok(value) => result.push(value),
        Err(_) => panic!(format!("Failed parsing input value '{}'!", line))
      };
    }
  }

  // Return read and parsed values
  return PuzzleInput::Vector1D(result);

}

/// Parse 2D puzzle input data
/// 
/// # Arguments
/// 
/// - `path`       - File path to read input data from
/// - `delimiter1` - Character to split the data around (1st level)
/// - `delimiter2` - Character to split the data around (2nd level)
pub fn parse_2d<T: FromStr> (input: String, delimiter1: char, delimiter2: char) -> PuzzleInput<T> {

  // Parse input (as integer array)
  let mut result: Vec<Vec<T>> = Vec::new();
  let lines = input.split(delimiter1);
  for line in lines {
    if line.len() > 0 {
      let mut subresult: Vec<T> = Vec::new();
      let fields = line.trim().split(delimiter2);
      for field in fields {
        if field.len() > 0 {
          match String::from(field.trim()).parse::<T>() {
            Ok(value) => subresult.push(value),
            Err(_) => panic!(format!("Failed parsing input value '{}'!", line))
          };
        }
      }
      result.push(subresult);
    }
  }

  // Return read and parsed values
  return PuzzleInput::Vector2D(result);
}

/// Formats input as printable string
/// 
/// # Arguments
/// 
/// - `input` - Puzzle input to format
pub fn format_puzzle_input<T: Debug> (input: &PuzzleInput<T>) -> String {
  match input {
    // Formatting single paramater
    PuzzleInput::Param(param) => {
      format!("{:?}", param)
    },
    // Formatting multiple paramaters
    PuzzleInput::Params(params) => {
      let output = format!("{:?}", params);
      let length = if output.len() < CONSOLE_CONCAT_STRING_LENGTH { output.len() } else { CONSOLE_CONCAT_STRING_LENGTH };
      output[..length].to_string()
    },
    // Formatting 1D vector
    PuzzleInput::Vector1D(values) => {
      let output = format!("{:?}", values);
      let length = if output.len() < CONSOLE_CONCAT_STRING_LENGTH { output.len() } else { CONSOLE_CONCAT_STRING_LENGTH };
      output[..length].to_string()
    },
    // Formatting 1D vector with single paramater
    PuzzleInput::ParamVector1D(input, values) => {
      let output = format!("{:?}, {:?}", input, values);
      let length = if output.len() < CONSOLE_CONCAT_STRING_LENGTH { output.len() } else { CONSOLE_CONCAT_STRING_LENGTH };
      output[..length].to_string()
    },
    // Formatting 1D vector with multiple paramaters
    PuzzleInput::ParamsVector1D(inputs, values) => {
      let output = format!("{:?}, {:?}", inputs, values);
      let length = if output.len() < CONSOLE_CONCAT_STRING_LENGTH { output.len() } else { CONSOLE_CONCAT_STRING_LENGTH };
      output[..length].to_string()
    },
    // Formatting 2D vector
    PuzzleInput::Vector2D(values) => {
      let output = format!("{:?}", values);
      let length = if output.len() < CONSOLE_CONCAT_STRING_LENGTH { output.len() } else { CONSOLE_CONCAT_STRING_LENGTH };
      output[..length].to_string()
    },
    // Formatting 2D vector with single paramater
    PuzzleInput::ParamVector2D(input, values) => {
      let output = format!("{:?}, {:?}", input, values);
      let length = if output.len() < CONSOLE_CONCAT_STRING_LENGTH { output.len() } else { CONSOLE_CONCAT_STRING_LENGTH };
      output[..length].to_string()
    },
    // Formatting 2D vector with multiple paramaters
    PuzzleInput::ParamsVector2D(inputs, values) => {
      let output = format!("{:?}, {:?}", inputs, values);
      let length = if output.len() < CONSOLE_CONCAT_STRING_LENGTH { output.len() } else { CONSOLE_CONCAT_STRING_LENGTH };
      output[..length].to_string()
    }
  }
}
