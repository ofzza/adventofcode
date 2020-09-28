//! Inputs module
//! 
//! Contains functions required to required puzzle inputs
// -----------------------------------------------------------------------------

// Include dependencies
use std::fs;
use std::str::FromStr;

/// Loads puzzle input data
/// 
/// # Arguments
/// 
/// - `path`      - File path to read input data from
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
pub fn parse_1d<T: FromStr> (input: String, delimiter: char) -> Vec<T> {

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
  return result;

}

/// Parse 2D puzzle input data
/// 
/// # Arguments
/// 
/// - `path`       - File path to read input data from
/// - `delimiter1` - Character to split the data around (1st level)
/// - `delimiter2` - Character to split the data around (2nd level)
pub fn parse_2d<T: FromStr> (input: String, delimiter1: char, delimiter2: char) -> Vec<Vec<T>> {

  // Parse input (as integer array)
  let mut result1: Vec<Vec<T>> = Vec::new();
  let lines = input.split(delimiter1);
  for line in lines {
    if line.len() > 0 {
      let mut result2: Vec<T> = Vec::new();
      let fields = line.trim().split(delimiter2);
      for field in fields {
        if field.len() > 0 {
          match String::from(field.trim()).parse::<T>() {
            Ok(value) => result2.push(value),
            Err(_) => panic!(format!("Failed parsing input value '{}'!", line))
          };
        }
      }
      result1.push(result2);
    }
  }

  // Return read and parsed values
  return result1;

}
