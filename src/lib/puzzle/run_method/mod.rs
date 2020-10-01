//! Puzzle .run() implementation
//! 
//! Allows owner to be run by calling the .run(verbose: bool) method
// -----------------------------------------------------------------------------

// Include dependencies
use std::fmt::Debug;
use std::time::Instant;
use super::*;
use super::super::console::*;

/// Puzzle .run() implementation
/// 
/// Allows owner to be run by calling the .run(verbose: bool) method
impl<TInput: Debug, TOutput, TResult: Debug + PartialOrd> Puzzle<TInput, TOutput, TResult> {
  
  /// Loads input data if configured and runs puzzle implementation
  /// 
  /// # Arguments
  /// 
  /// * `verbose` - Outputs executing output of the puzzle to the console
  /// 
  /// # Remarks
  /// 
  /// * While processing, outputs puzzle processing status to console
  pub fn run (&mut self, verbose: bool) -> PuzzleExecutionStatitics {
    
    // Inizialize stats
    let mut stats = PuzzleExecutionStatitics{
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
      CONSOLE_COMMENT_BG, CONSOLE_COMMENT_FG, format!("INPUT (partial): {}", format_puzzle_input(&self.input)),
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
          format!("{}{}RESULT [FAIL != {:?}]", CONSOLE_FAIL_BG, CONSOLE_FAIL_FG, expected)
        }
      };
    println!("  {}{}:{}{}{}{}",
      result_label, CONSOLE_RESET,
      CONSOLE_COMMENT_BG, CONSOLE_COMMENT_FG, format!(" {:?} (in {} sec)", result.0, execution_time),
      CONSOLE_RESET);

    // Return collected execution stats
    return stats;
  }

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
      if output.len() < CONSOLE_CONCAT_STRING_LENGTH {
        output[..output.len()].to_string()
      } else {
        format!("{} ...", output[..CONSOLE_CONCAT_STRING_LENGTH].to_string())
      }
    },
    // Formatting 1D vector
    PuzzleInput::Vector1D(values) => {
      let output = format!("{:?}", values);
      if output.len() < CONSOLE_CONCAT_STRING_LENGTH {
        output[..output.len()].to_string()
      } else {
        format!("{} ...", output[..CONSOLE_CONCAT_STRING_LENGTH].to_string())
      }
    },
    // Formatting 1D vector with single paramater
    PuzzleInput::ParamVector1D(input, values) => {
      let output = format!("{:?}", values);
      if output.len() < CONSOLE_CONCAT_STRING_LENGTH {
        output[..output.len()].to_string()
      } else {
        format!("{:?}:{} ...", input, output[..CONSOLE_CONCAT_STRING_LENGTH].to_string())
      }
    },
    // Formatting 1D vector with multiple paramaters
    PuzzleInput::ParamsVector1D(inputs, values) => {
      let output = format!("{:?}", values);
      if output.len() < CONSOLE_CONCAT_STRING_LENGTH {
        output[..output.len()].to_string()
      } else {
        format!("{:?}:{} ...", inputs, output[..CONSOLE_CONCAT_STRING_LENGTH].to_string())
      }
    },
    // Formatting 2D vector
    PuzzleInput::Vector2D(values) => {
      let output = format!("{:?}", values);
      if output.len() < CONSOLE_CONCAT_STRING_LENGTH {
        output[..output.len()].to_string()
      } else {
        format!("{} ...", output[..CONSOLE_CONCAT_STRING_LENGTH].to_string())
      }
    },
    // Formatting 2D vector with single paramater
    PuzzleInput::ParamVector2D(input, values) => {
      let output = format!("{:?}", values);
      if output.len() < CONSOLE_CONCAT_STRING_LENGTH {
        output[..output.len()].to_string()
      } else {
        format!("{:?}:{} ...", input, output[..CONSOLE_CONCAT_STRING_LENGTH].to_string())
      }
    },
    // Formatting 2D vector with multiple paramaters
    PuzzleInput::ParamsVector2D(inputs, values) => {
      let output = format!("{:?}", values);
      if output.len() < CONSOLE_CONCAT_STRING_LENGTH {
        output[..output.len()].to_string()
      } else {
        format!("{:?}:{} ...", inputs, output[..CONSOLE_CONCAT_STRING_LENGTH].to_string())
      }
    }
  }
}
