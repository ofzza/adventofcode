//! Puzzle module
//! 
//! Definition of each puzzle
// -----------------------------------------------------------------------------

// Include dependencies
use std::cmp::Eq;
use std::collections::HashMap;
use std::time::Instant;
use crate::lib::vargs::*;
use crate::lib::stdout::*;

/// PuzzleInfo struct
/// 
/// Describes puzzle by year/day/index/type
#[derive(Hash)]
pub struct PuzzleInfo {  
  pub year: u32,
  pub day: u32,
  pub index: u32,
  pub tag: String,
}
/// PuzzleInfo implementation
/// 
/// Implements PartialEq and Eq traits
impl PuzzleInfo {
  pub fn matches(&self, other: &Self) -> bool {
    (self.year == 0 || other.year == 0 || self.year == other.year)
    && (self.day == 0 || other.day == 0 || self.day == other.day)
    && (self.index == 0 || other.index == 0 || self.index == other.index)
    && (self.tag.is_empty() || other.tag.is_empty() || self.tag == other.tag)
  }
}
impl Eq for PuzzleInfo {}
impl PartialEq for PuzzleInfo {
  fn eq(&self, other: &Self) -> bool {
    self.year == other.year
    && self.day == other.day
    && self.index == other.index
    && self.tag == other.tag
  }
}

/// PuzzleRegistry struct
/// 
/// Keeps a registry of all puzzle and provides functionality required for
/// registering and running puzzles
pub struct PuzzleRegistry {
  pub puzzles: HashMap<PuzzleInfo, Box<fn(data: String) -> String>>
}
/// PuzzleRegistry implementation
/// 
/// Keeps a registry of all puzzle and provides functionality required for
/// registering and running puzzles
impl PuzzleRegistry {

  /// Executes a puzzle
  /// 
  /// # Arguments
  /// * info:           Information of the puzzle being executed
  /// * start_instant:  Instant of puzzle started reading input data
  /// * f:              Puzzle implementation function
  /// * input:          Puzzle input data
  /// * args:           Startup arguments
  pub fn execute (info: &PuzzleInfo, start_instant: Instant, f: &Box<fn(data: String) -> String>, input: String, args: &VArgs) {
    // Time function execution
    let processing_instant = Instant::now();
    let result = f(input);
    let start_elapsed = start_instant.elapsed().as_secs_f32();
    let processing_elapsed = processing_instant.elapsed().as_secs_f32();
    // Check result
    let valid: Option<bool> = if args.expect.is_empty() { None } else { Some(result.trim() == args.expect.trim()) };
    // Obfuscate result
    let output = if !args.obfuscate { result } else { String::from("*****") };
    // Output execution value
    if args.verbose {
      StdOut::println(String::default(), None);
      StdOut::println(format!("Executing puzzle {:04}/{:02}.{} ({}):", info.year, info.day, info.index, info.tag), None);
      StdOut::println(format!("Executed in {}ms ({}ms with input fetching) with result:", (processing_elapsed * 1000.0), (start_elapsed * 1000.0)), None);
    }
    match valid {
      None => StdOut::println(format!("{}", output), Some(StdOutColoring::UNKNOWN)),
      Some(true) => StdOut::println(format!("{}", output), Some(StdOutColoring::VALID)),
      Some(false) => StdOut::println(format!("{}", output), Some(StdOutColoring::INVALID)),
    }
    
}

  /// Constructor
  pub fn new () -> PuzzleRegistry {
    PuzzleRegistry {
      puzzles: HashMap::new()
    }
  }

  /// Registers a puzzle
  /// 
  /// Registers a puzzle's implementation associated to the puzzle's information
  /// 
  /// # Arguments
  /// * info: Puzzle information
  /// * f:    Puzzle implementation function `fn(data: String) -> String)`
  pub fn register(&mut self, info: PuzzleInfo, f: fn(data: String) -> String) {
    self.puzzles.insert(info, Box::new(f));
  }
}

