//! Puzzle PuzzleRunnable trait
//! 
//! Allows owner to be run by calling the .run(verbose: bool) method
// -----------------------------------------------------------------------------

// Include dependencies
use std::fmt::Debug;
use std::time::Instant;
use super::*;
use super::super::console::*;

/// PuzzleRunnable trait definition
/// 
/// Allows owner to be run by calling the .run(verbose: bool) method
pub trait PuzzleRunnable  {
  // Puzzle run method
  fn run (&mut self, verbose: bool);
}

/// Puzzle's PuzzleRunnable trait implementation
impl<TInput: Debug, TOutput, TResult: Debug + PartialOrd> PuzzleRunnable for Puzzle<TInput, TOutput, TResult> {
  
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

    // Print puzzle input
    let input_values = if self.input.value_vec.len() > 0 {
        // Concat vector input type
        let len = if self.input.value_vec.len() < CONSOLE_ITEM_COUNT { self.input.value_vec.len() } else { CONSOLE_ITEM_COUNT };  
        format!("{:?}", &self.input.value_vec[0..len])
      } else {
        // Concat unknown input type
        format!("-")
      };
    println!("  {}{}{}{}",
      CONSOLE_COMMENT_BG, CONSOLE_COMMENT_FG, format!("INPUT (partial): {}", input_values),
      CONSOLE_RESET);

    // Padding if verbose
    if verbose == true {
      println!("  {}{}Executing:{}", CONSOLE_COMMENT_BG, CONSOLE_COMMENT_FG, CONSOLE_RESET);
      println!("{}{}", CONSOLE_EXECUTION_BG, CONSOLE_EXECUTION_FG);
    }
    // Time execution
    let now = Instant::now();
    // Execute puzzle
    let output = match (self.implementation)(self, verbose) {
        // If output, proceed
        Ok(output) => output,
        // If error, execution failed - output error and continue
        Err(e) => {
          println!("  {}{}ERROR: {}!{}",
            CONSOLE_ERROR_BG, CONSOLE_ERROR_FG, e, CONSOLE_RESET);
          return;
        }
      };
    // End timing execution
    let execution_duration = now.elapsed().as_secs_f32();
    // Padding if verbose
    if verbose == true {
      println!("{}", CONSOLE_RESET);
    }

    // Print puzzle result
    let result = (self.config.expected)(output);
    let result_label = match result.1 {
        None => {
          format!("{}{}RESULT", CONSOLE_RESULT_BG, CONSOLE_RESULT_FG)
        },
        Some(expected) => if result.0 == expected {
          format!("{}{}RESULT [OK]", CONSOLE_SUCCESS_BG, CONSOLE_SUCCESS_FG)
        } else {
          format!("{}{}RESULT [FAIL != {:?}]", CONSOLE_FAIL_BG, CONSOLE_FAIL_FG, expected)
        }
      };
    println!("  {}{}:{}{}{}{}",
      result_label, CONSOLE_RESET,
      CONSOLE_COMMENT_BG, CONSOLE_COMMENT_FG, format!(" {:?} (in {} ms)", result.0, execution_duration),
      CONSOLE_RESET);
  }

}
