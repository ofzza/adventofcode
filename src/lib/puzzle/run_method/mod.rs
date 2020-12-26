//! Puzzle .run() implementation
//! 
//! Allows owner to be run by calling the .run(verbose: bool) method
// -----------------------------------------------------------------------------

// Include dependencies
use std::fmt::Debug;
use std::time::Instant;
use crate::lib::console::*;
use super::*;

/// Puzzle .run() implementation
/// 
/// Allows owner to be run by calling the .run(verbose: bool) method
impl<TInput: Debug, TOutput, TResult: Debug + PartialOrd> Puzzle<TInput, TOutput, TResult> {
  
  /// Runs puzzle implementation
  /// 
  /// # Arguments
  /// 
  /// * `verbose`   - Outputs executing output of the puzzle to the console
  /// * `obfuscate` - If values should be obfuscated in output
  /// 
  /// # Remarks
  /// 
  /// * While processing, outputs puzzle processing status to console
  pub fn run (&mut self, verbose: bool, obfuscate: bool) -> PuzzleExecutionStatistics {
    
    // Inizialize stats
    let mut stats = PuzzleExecutionStatistics{
      ..Default::default()
    };

    // Initialize puzzle
    println!();
    println!("{}{}{}{}{}{}{}",
      CONSOLE_TITLE_BG, CONSOLE_TITLE_FG, format!("PUZZLE {}/{}.{}: ", self.year, self.day, self.index),
      CONSOLE_SUBTITLE_BG, CONSOLE_SUBTITLE_FG, format!("{}", self.key),
      CONSOLE_RESET);

    // Print puzzle input
    println!("  {}{}{}{}",
      CONSOLE_COMMENT_BG, CONSOLE_COMMENT_FG, format!("INPUT (partial): {}", PuzzleInput::format_puzzle_input(&self.input)),
      CONSOLE_RESET);

    // Padding if verbose
    if verbose {
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
          return stats;
        }
      };
    // End timing execution
    let execution_time = now.elapsed().as_secs_f32();
    stats.execution_time = execution_time;
    // Padding if verbose
    if verbose {
      println!("{}", CONSOLE_RESET);
    }

    // Print puzzle result
    let result = (self.expected)(output);
    let result_label = match result.1 {
        None => {
          stats.total_count += 1;
          format!("{}{}RESULT", CONSOLE_RESULT_BG, CONSOLE_RESULT_FG)
        },
        Some(expected) => if result.0 == expected {
          stats.total_count += 1;
          stats.successful_count += 1;
          format!("{}{}RESULT [OK]", CONSOLE_SUCCESS_BG, CONSOLE_SUCCESS_FG)
        } else {
          stats.total_count += 1;
          stats.failed_count += 1;
          if !obfuscate {
            format!("{}{}RESULT [FAIL != {:?}]", CONSOLE_FAIL_BG, CONSOLE_FAIL_FG, expected)
          } else {
            format!("{}{}RESULT [FAIL != ???]", CONSOLE_FAIL_BG, CONSOLE_FAIL_FG)
          }
        }
      };
    println!("  {}{}:{}{}{}{}",
      result_label, CONSOLE_RESET,
      CONSOLE_COMMENT_BG, CONSOLE_COMMENT_FG,
      if !obfuscate {
        format!(" {:?} (in {} sec)", result.0, execution_time)
      } else {
        format!(" ??? (in {} sec)", execution_time)
      },
      CONSOLE_RESET);

    // Return collected execution stats
    return stats;
  }

  /// Prompts a message instead of running a puzzle
  /// 
  /// Arguments
  /// * `msg` - Message to prompt
  pub fn prompt (&mut self, msg: &str) {
    // Initialize puzzle
    println!();
    println!("{}{}{}{}{}{}{}",
      CONSOLE_TITLE_BG, CONSOLE_TITLE_FG, format!("PUZZLE {}/{}.{}: ", self.year, self.day, self.index),
      CONSOLE_SUBTITLE_BG, CONSOLE_SUBTITLE_FG, format!("{}", self.key),
      CONSOLE_RESET);

    // Print puzzle input
    println!("  {}{}{}{}",
      CONSOLE_COMMENT_BG, CONSOLE_COMMENT_FG, msg,
      CONSOLE_RESET);
  }

}
