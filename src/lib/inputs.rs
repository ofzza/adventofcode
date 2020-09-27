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
/// - `delimiter` - Character to split the data around
pub fn load_input<T: FromStr> (path: &str, delimiter: char) -> Vec<T> {
  
  // Read input
  let file_contents = fs::read_to_string(path)
    .expect("Failed reading from input file!");

  // Parse input (as integer array)
  let mut result: Vec<T> = Vec::new();
  let lines = file_contents.split(delimiter);
  for line in lines {
    if line.len() > 0 {
      match String::from(line.trim()).parse::<T>() {
        Ok(number) => result.push(number),
        Err(_) => panic!(format!("Failed parsing input value '{}'!", line))
      };
    }
  }

  // Return read and parsed values
  return result;

}
