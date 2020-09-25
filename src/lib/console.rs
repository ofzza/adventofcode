//! Console module
//! 
//! Contains constants for styling console output
// -----------------------------------------------------------------------------

// Include dependencies
extern crate termion;
use termion::{color, style};

/// Console RESET style
pub const CONSOLE_RESET: style::Reset = style::Reset;

/// Maximun character length to be putputted to the console when cropping output text
pub const CONSOLE_STRING_LEN: usize = 128;
/// Maximun number count to be putputted to the console when cropping output nummeric array
pub const CONSOLE_NUMBER_COUNT: usize = 16;

/// Console TITLE BG style
pub const CONSOLE_TITLE_BG: style::Reset              = style::Reset;
/// Console TITLE FG style
pub const CONSOLE_TITLE_FG: color::Fg<color::Rgb>     = color::Fg(color::Rgb(0, 0, 255));
/// Console SUBTITLE BG style
pub const CONSOLE_SUBTITLE_BG: style::Reset           = style::Reset ;
/// Console SUBTITLE FG style
pub const CONSOLE_SUBTITLE_FG: color::Fg<color::Rgb>  = color::Fg(color::Rgb(255, 255, 0));
/// Console COMMENT BG style
pub const CONSOLE_COMMENT_BG: style::Reset            = style::Reset;
/// Console COMMENT FG style
pub const CONSOLE_COMMENT_FG: color::Fg<color::Rgb>   = color::Fg(color::Rgb(200, 200, 200));
/// Console EXECUTION output BG style
pub const CONSOLE_EXECUTION_BG: style::Reset          = style::Reset;
/// Console EXECUTION output FG style
pub const CONSOLE_EXECUTION_FG: color::Fg<color::Rgb> = color::Fg(color::Rgb(155, 155, 155));
/// Console EXECUTION successful result BG style
pub const CONSOLE_SUCCESS_BG: color::Bg<color::Rgb>   = color::Bg(color::Rgb(0, 255, 0));
/// Console EXECUTION successful result FG style
pub const CONSOLE_SUCCESS_FG: color::Fg<color::Rgb>   = color::Fg(color::Rgb(0, 0, 0));
/// Console EXECUTION failed result BG style
pub const CONSOLE_FAIL_BG: color::Bg<color::Rgb>      = color::Bg(color::Rgb(255, 0, 0));
/// Console EXECUTION failed result FG style
pub const CONSOLE_FAIL_FG: color::Fg<color::Rgb>      = color::Fg(color::Rgb(255, 255, 255));
