//! VARGS module
//! 
//! Holds and processes startup arguments
// -----------------------------------------------------------------------------

// Include dependencies
use crate::PuzzleInfo;

/// VArgs struct
/// 
/// Describes puzzle by year/day/index/type
pub struct VArgs {
  pub puzzle: PuzzleInfo,
  pub input_file: String,
  pub input_value: String,
  pub expect: String,
  pub verbose: bool,
  pub obfuscate: bool
}
/// VArgs implementation
/// 
/// Provides processing of startup arguments
impl VArgs {
  /// Constructor
  /// 
  /// Processes startup arguments
  /// * `--year`        - Only executes puzzles marked with same year  (Set 0 or omit for all years)
  /// * `--day`         - Only executes puzzles marked with same day   (Set 0 or omit for all days)
  /// * `--index`       - Only executes puzzles marked with same index (Set 0 or omit for all indices)
  /// * `--tag`        - Only executes puzzles marked with same tag  (Leave empty or omit for all tags)
  /// * `--input-file`  - Path to the input data file (Leave empty if you want to use explicit input value)
  /// * `--input-value` - Explicit input data (Leave empty if you want to use data from the input file instead)
  /// * `--expect`      - Expected result for the puzzle to output (Leave empty or omit for no expected result)
  /// * `--verbose`     - Outputs executing output of the puzzle to the console
  /// * `--obfuscate`   - Outputs executing output of the puzzle to the console while obfuscating "spoilers"
  /// 
  /// # Arguments
  /// * argv: Process startup arguments
  pub fn new (args: Vec<String>) -> VArgs {
      // Initialize arguments
      let mut year: u32           = 0;
      let mut day:u32             = 0;
      let mut index: u32          = 0;
      let mut tag: String         = String::default();
      let mut input_file: String  = String::default();
      let mut input_value: String = String::default();
      let mut expect: String      = String::default();
      let mut verbose: bool       = false;
      let mut obfuscate: bool     = false;

    // Process arguments
    for i in 0..args.len() {
      // Get year argument
      if (args[i] == "--year") && (args.len() > i) {
        year = args[i + 1].parse::<u32>().expect("Failed parsing '--year' parameter - expecting a positive, whole number!");
      }
      // Get day argument
      if (args[i] == "--day") && (args.len() > i) {
        day = args[i + 1].parse::<u32>().expect("Failed parsing '--day' parameter - expecting a positive, whole number!");
      }
      // Get index argument
      if (args[i] == "--index") && (args.len() > i) {
        index = args[i + 1].parse::<u32>().expect("Failed parsing '--index' parameter - expecting a positive, whole number!");
      }
      // Get key argument
      if (args[i] == "--tag") && (args.len() > i) {
        tag = args[i + 1].trim().to_string();
      }
      // Get input data file path argument
      if (args[i] == "--input-file") &&  (args.len() > i) {
        input_file = args[i + 1].trim().to_string();
      }
      // Get input data explicit value argument
      if (args[i] == "--input-value") &&  (args.len() > i) {
        input_value = args[i + 1].trim().to_string();
      }
      // Get expected result argument
      if (args[i] == "--expect") &&  (args.len() > i) {
        expect = args[i + 1].trim().to_string();
      }
      // Get verbose argument
      if args[i] == "--verbose" {
        verbose = true;
      }
      // Get obfuscate argument
      if args[i] == "--obfuscate" {        
        obfuscate = true;
      }
    }

    // Return processed arguments
    VArgs {
      puzzle: PuzzleInfo {
        year,
        day,
        index,
        tag
      },
      input_file,
      input_value,
      expect,
      verbose,
      obfuscate
    }
  }
}
