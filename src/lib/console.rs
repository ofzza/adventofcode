//! Console module
//! 
//! Contains constants for styling console output
// -----------------------------------------------------------------------------

// Include dependencies
extern crate termion;
use termion::{color, style};

// Console styles flags
// -----------------------------------------------------------------------------

/// Console RESET style
pub const CONSOLE_RESET: style::Reset = style::Reset;

/// Maximun item count to be outputted to the console when cropping output nummeric array
pub const CONSOLE_CONCAT_ITEM_COUNT: usize = 16;
/// Maximun string length to be outputted to the console when cropping output nummeric array
pub const CONSOLE_CONCAT_STRING_LENGTH: usize = 128;

/// Console TITLE BG style
pub const CONSOLE_TITLE_BG: style::Reset                  = style::Reset;
/// Console TITLE FG style
pub const CONSOLE_TITLE_FG: color::Fg<color::Rgb>         = color::Fg(color::Rgb(255, 255, 0));
/// Console SUBTITLE BG style
pub const CONSOLE_SUBTITLE_BG: style::Reset               = style::Reset ;
/// Console SUBTITLE FG style
pub const CONSOLE_SUBTITLE_FG: color::Fg<color::Rgb>      = color::Fg(color::Rgb(0, 0, 255));
/// Console COMMENT BG style
pub const CONSOLE_COMMENT_BG: style::Reset                = style::Reset;
/// Console COMMENT FG style
pub const CONSOLE_COMMENT_FG: color::Fg<color::Rgb>       = color::Fg(color::Rgb(200, 200, 200));
/// Console EXECUTION output BG style
pub const CONSOLE_EXECUTION_BG: style::Reset              = style::Reset;
/// Console EXECUTION output FG style
pub const CONSOLE_EXECUTION_FG: color::Fg<color::Rgb>     = color::Fg(color::Rgb(155, 155, 155));
/// Console VERBOSE output full BG style
pub const CONSOLE_VERBOSE_FULL_BG: color::Bg<color::Rgb>  = color::Bg(color::Rgb(100, 100, 100));
/// Console VERBOSE output empty BG style
pub const CONSOLE_VERBOSE_EMPTY_BG: style::Reset          = style::Reset;
/// Console EXECUTION result FG style
pub const CONSOLE_VERBOSE_FG: color::Fg<color::Rgb>       = color::Fg(color::Rgb(155, 155, 155));
/// Console EXECUTION result BG style
pub const CONSOLE_RESULT_BG: color::Bg<color::Rgb>        = color::Bg(color::Rgb(255, 255, 0));
/// Console EXECUTION result FG style
pub const CONSOLE_RESULT_FG: color::Fg<color::Rgb>        = color::Fg(color::Rgb(0, 0, 0));
/// Console EXECUTION successful result BG style
pub const CONSOLE_SUCCESS_BG: color::Bg<color::Rgb>       = color::Bg(color::Rgb(0, 255, 0));
/// Console EXECUTION successful result FG style
pub const CONSOLE_SUCCESS_FG: color::Fg<color::Rgb>       = color::Fg(color::Rgb(0, 0, 0));
/// Console EXECUTION failed result BG style
pub const CONSOLE_FAIL_BG: color::Bg<color::Rgb>          = color::Bg(color::Rgb(255, 0, 0));
/// Console EXECUTION failed result FG style
pub const CONSOLE_FAIL_FG: color::Fg<color::Rgb>          = color::Fg(color::Rgb(255, 255, 255));
/// Console TITLE BG style
pub const CONSOLE_ERROR_BG: color::Bg<color::Rgb>         = color::Bg(color::Rgb(255, 0, 0));
/// Console TITLE FG style
pub const CONSOLE_ERROR_FG: color::Fg<color::Rgb>         = color::Fg(color::Rgb(255, 255, 255));
