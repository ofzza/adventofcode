//! 2021 demo puzzle
//! 
//! https://adventofcode.com/2021
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
// !DOCS: use crate::lib::input::*;
// use crate::lib::console::*;

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 99,
      index: 99,
      tag: String::from("demo")
    },

    // Implementation
    |_data: String| {
      // Process data 01
      // !DOCS: let data = Input::parse(data.as_str(), "\n", |x| { x.parse::<usize>().unwrap() });
      // Process data 02
      // !DOCS: let data = Input::parse(data.as_str(), "\n", |data| {
      //   Input::parse(data, " ", |x| { x.parse::<usize>().unwrap() })
      // });
      // Process data 03
      // !DOCS: let data = Input::parse(data.as_str(), "\n\n", |data| {
      //   Input::parse(data, "\n", |x| { x.parse::<usize>().unwrap() })
      // });
      // !DOCS: Process data 04
      // let data = Input::parse(data.as_str(), "\n\n", |data| {
      //   Input::parse(data, "\n", |data| {
      //     Input::parse(data, " ", |x| { x.parse::<usize>().unwrap()
      //   })
      // }) });

      // Output result
      String::from("Hello world")
    }

  );
  registry
}
