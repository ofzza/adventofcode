//! STDOUT module
//! 
//! Provides functionality for formatting stdout output
// -----------------------------------------------------------------------------

// Include dependencies
extern crate termion;
use termion::{color};

/// Enumerates coloring styles
pub enum StdOutColoring {
  #[allow(dead_code)] RESET,
  UNKNOWN,
  VALID,
  INVALID
}

/// Holds FG/BG color definition
struct Reset {
  foreground: color::Fg<color::Reset>,
  background: color::Bg<color::Reset>
}
const STDOUT_RESET: Reset = Reset {
  foreground: color::Fg(color::Reset),
  background: color::Bg(color::Reset),
};

/// Holds FG/BG color definition
struct Color {
  foreground: color::Fg<color::Rgb>,
  background: color::Bg<color::Rgb>
}
const STDOUT_UNKNOWN: Color = Color {
  foreground: color::Fg(color::Rgb(255, 255, 255)),
  background: color::Bg(color::Rgb(150, 150, 150))
};
const STDOUT_VALID: Color = Color {
  foreground: color::Fg(color::Rgb(0, 0, 0)),
  background: color::Bg(color::Rgb(0, 255, 0))
};
const STDOUT_INVALID: Color = Color {
  foreground: color::Fg(color::Rgb(0, 0, 0)),
  background: color::Bg(color::Rgb(255, 0, 0))
};

/// STDIO struct
pub struct StdOut {}
/// STDIO implementation
/// 
/// Provides functionality for formatting stdout output
impl StdOut {

  /// Prints text to STDOUT
  /// 
  /// # Arguments
  /// * text:     Text to print out
  /// * coloring: Text coloring
  pub fn print (text: String, coloring: Option<StdOutColoring>) {
    let reset = format!("{}{}", STDOUT_RESET.foreground, STDOUT_RESET.background);
    let style = match coloring {
      None => format!("{}{}", STDOUT_RESET.foreground, STDOUT_RESET.background),
      Some(StdOutColoring::RESET) => format!("{}{}", STDOUT_RESET.foreground, STDOUT_RESET.background),
      Some(StdOutColoring::UNKNOWN) => format!("{}{}", STDOUT_UNKNOWN.foreground, STDOUT_UNKNOWN.background),
      Some(StdOutColoring::VALID) => format!("{}{}", STDOUT_VALID.foreground, STDOUT_VALID.background),
      Some(StdOutColoring::INVALID) => format!("{}{}", STDOUT_INVALID.foreground, STDOUT_INVALID.background)
    };
    print!("{}{}{}", style, text, reset);
  }

  /// Prints a line of text to STDOUT
  /// 
  /// # Arguments
  /// * text: Text to print out
  /// * coloring: Text coloring
  pub fn println (text: String, coloring: Option<StdOutColoring>) {
    StdOut::print(text, coloring);
    StdOut::print(String::from("\n"), None);
  }

}
