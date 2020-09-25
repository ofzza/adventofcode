//! Puzzle module
//! 
//! Contains structs, traits and factories needed to implement and run custom puzzles
// -----------------------------------------------------------------------------

// (re)Export modules
pub mod struct_puzzle;
pub use struct_puzzle::*;
pub mod struct_puzzle_config;
pub use struct_puzzle_config::*;
pub mod struct_puzzle_input;
pub use struct_puzzle_input::*;
pub mod trait_puzzle_runnable;
pub use trait_puzzle_runnable::*;
pub mod factory;
pub use factory::*;
