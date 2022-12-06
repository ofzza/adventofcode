//! 202x demo puzzle
//! 
//! https://adventofcode.com/202x
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
// !DOCS: use crate::lib::input::*;
// use crate::lib::console::*;

/// Registers puzzles for the day
#[allow(dead_code)]
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  registry.register(

    // Info
    PuzzleInfo {
      year: 9999,
      day: 99,
      index: 99,
      tag: String::from("demo")
    },

    // Implementation
    |_data: String| {
      // Process data 01
      // !DOCS: let data = Input::parse(data.as_str().trim(), "\n", |x| { x.parse::<usize>().unwrap() });
      // Process data 02
      // !DOCS: let data = Input::parse(data.as_str().trim(), "\n", |data| {
      // !DOCS:   Input::parse(data, " ", |x| { x.parse::<usize>().unwrap() })
      // !DOCS: });
      // Process data 03
      // !DOCS: let data = Input::parse(data.as_str().trim(), "\n\n", |data| {
      // !DOCS:   Input::parse(data, "\n", |x| { x.parse::<usize>().unwrap() })
      // !DOCS: });
      // Process data 04
      // !DOCS: let data = Input::parse(data.as_str().trim(), "\n\n", |data| {
      // !DOCS:   Input::parse(data, "\n", |data| {
      // !DOCS:     Input::parse(data, " ", |x| { x.parse::<usize>().unwrap()
      // !DOCS:   })
      // !DOCS: });

      // Output result
      String::from("Hello world")
    }

  );
  registry
}
