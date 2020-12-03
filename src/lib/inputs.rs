//! Inputs module
//! 
//! Contains functions required to required puzzle inputs
// -----------------------------------------------------------------------------

// Include dependencies
use std::fs;
use std::str::FromStr;
use crate::lib::puzzle::*;

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
/// - `delimiter` - String to split the data around
pub fn parse_1d<T: FromStr> (input: String, delimiter: &str) -> PuzzleInput<T> {

  // Parse input (as integer array)
  let mut result: Vec<T> = Vec::new();
  let lines: Vec<&str> = input.split(delimiter).collect();
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
/// - `delimiter1` - String to split the data around (1st level)
/// - `delimiter2` - String to split the data around (2nd level)
pub fn parse_2d<T: FromStr> (input: String, delimiter1: &str, delimiter2: &str) -> PuzzleInput<T> {

  // Parse input (as integer array)
  let mut result: Vec<Vec<T>> = Vec::new();
  let lines: Vec<&str> = input.split(delimiter1).collect();
  for line in lines {
    if line.len() > 0 {
      let mut subresult: Vec<T> = Vec::new();
      let fields: Vec<&str> = line.trim().split(delimiter2).collect();
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
