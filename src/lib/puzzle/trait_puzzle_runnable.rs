//! Puzzleps PuzzleRunnable trait
//! 
//! Allows owner to be run by calling the .run(verbose: bool) method
// -----------------------------------------------------------------------------

// Include dependencies
use std::fs;
use super::*;
use super::super::console::*;

/// PuzzleRunnable trait definition
/// 
/// Allows owner to be run by calling the .run(verbose: bool) method
pub trait PuzzleRunnable  {
  fn run (&mut self, verbose: bool);
}

/// Puzzle's PuzzleRunnable trait implementation
impl PuzzleRunnable for Puzzle {
  
  /// Loads input data if configured and runs puzzle implementation
  /// 
  /// # Arguments
  /// 
  /// * `verbose` - Outputs executing output of the puzzle to the console
  /// 
  /// # Remarks
  /// 
  /// * While processing, outputs puzzle processing status to console
  fn run (&mut self, verbose: bool) {
    
    // Initialize puzzle
    println!();
    println!("{}{}{}{}{}{}{}",
      CONSOLE_TITLE_BG, CONSOLE_TITLE_FG, format!("PUZZLE {}/{}.{}: ", self.year, self.day, self.index),
      CONSOLE_SUBTITLE_BG, CONSOLE_SUBTITLE_FG, format!("{}", self.key),
      CONSOLE_RESET);

    // Load puzzle input
    load_input(self);

    // Print puzzle input
    let input_values = if self.config.input_parse_as_int_vec == true {
        let len = if self.input.value_int_vec.len() < CONSOLE_NUMBER_COUNT { self.input.value_int_vec.len() } else { CONSOLE_NUMBER_COUNT };  
        format!("{:?}", &self.input.value_int_vec[0..len])
      } else {
        let len = if self.input.text.len() < CONSOLE_STRING_LEN { self.input.text.len() } else { CONSOLE_STRING_LEN };
        format!("{:?}", (&self.input.text[0..len]).replace('\n', ", "))
      };
    println!("  {}{}{}{}",
      CONSOLE_COMMENT_BG, CONSOLE_COMMENT_FG, format!("INPUTS: {}", input_values),
      CONSOLE_RESET);

    // Execute puzzle
    if verbose == true {
      println!("  {}{}Executing:{}", CONSOLE_COMMENT_BG, CONSOLE_COMMENT_FG, CONSOLE_RESET);
      println!("{}{}", CONSOLE_EXECUTION_BG, CONSOLE_EXECUTION_FG);
    }
    let result = (self.implementation)(self, verbose);
    if verbose == true {
      println!("{}", CONSOLE_RESET);
    }

    // Print puzzle result
    let result_label = if self.config.expected == String::default() {
        format!("{}{}RESULT", CONSOLE_SUCCESS_BG, CONSOLE_SUCCESS_FG)
      } else if result == self.config.expected {
        format!("{}{}RESULT [OK]", CONSOLE_SUCCESS_BG, CONSOLE_SUCCESS_FG)
      } else {
        format!("{}{}RESULT [FAIL]", CONSOLE_FAIL_BG, CONSOLE_FAIL_FG)
      };
    println!("  {}{}:{}{}{}{}",
      result_label, CONSOLE_RESET,
      CONSOLE_COMMENT_BG, CONSOLE_COMMENT_FG, format!(" {}", result),
      CONSOLE_RESET);
  }

}

/// Loads puzzle input data
/// 
/// # Arguments
/// 
/// - `puzzle` - Puzzle definition to load input data for
/// 
/// # Remarks
/// 
/// Input can be loaded from the directly set value or, when no direct `config.input_value` value is set
/// it will be loaded from a file with path configured as `config.input_path`
fn load_input (puzzle: &mut Puzzle) {
  
  // Read input
  puzzle.input.text = match puzzle.config.input_value != String::default() {
    true => {
      puzzle.config.input_value.clone()
    },
    false => {
      fs::read_to_string(&puzzle.config.input_path)
        .expect("Failed reading from input file!")
    }
  };

  // Parse input (as integer array)
  if puzzle.config.input_parse_as_int_vec {
    let lines = puzzle.input.text.split(puzzle.config.input_parse_as_int_vec_delimiter);
    for line in lines {
      match String::from(line).parse() {
        Ok(number) => {
          puzzle.input.value_int_vec.push(number);
        },
        Err(e) => {
          if line != "" { eprintln!("Failed parsing input value as number: {}", e); }
        },
      };
    }
  }

}
