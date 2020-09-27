//! Puzzle module
//! 
//! Contains structs, traits and factories needed to implement and run custom puzzles
// -----------------------------------------------------------------------------

// (re)Export modules
pub mod puzzle;
pub use puzzle::*;
pub mod puzzle_config;
pub use puzzle_config::*;
pub mod puzzle_input;
pub use puzzle_input::*;
pub mod puzzle_runnable;
pub use puzzle_runnable::*;
pub mod puzzle_factory;
pub use puzzle_factory::*;
