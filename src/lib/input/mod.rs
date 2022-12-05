//! Input module
//! 
//! Provides functionality for reading and processing input data
// -----------------------------------------------------------------------------

// Include dependencies
use std::fs;

/// Input struct
pub struct Input {}
/// Input implementation
/// 
/// Provides functionality for reading and processing input data
impl Input {

  /// Reads input data from a file
  /// 
  /// # Arguments
  /// * path: Path to file which to read
  /// 
  /// # Returns
  /// File content as a string
  pub fn read_file(path: &String) -> String {
    return fs::read_to_string(path)
      .expect(format!("Failed reading from input file: \"{}\"!", path).as_str());
  }

  /// Parses a string of values
  /// 
  /// # Arguments
  /// * data:      Data string to parse
  /// * separator: String to split the data on while parsing
  /// * callback:  Callback function used to parse every split section
  pub fn parse<'a, Tout> (data: &'a str, separator: &str, callback: fn (&'a str) -> Tout) -> Vec<Tout> {
    let items: Vec<&str> = if separator == "" { data.split_terminator(separator).skip(1).collect() } else { data.split(separator).collect() };
    items.iter().map(|line| { callback(line) }).collect()
  }

}
