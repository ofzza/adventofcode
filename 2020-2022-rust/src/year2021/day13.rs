//! 2021 day 13 puzzle
//! 
//! https://adventofcode.com/2021/day/13
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year::lib::dot_display::*;

/// Parses input data
fn parse(data: &String) -> (Vec<(usize, usize)>, Vec<(&str, usize)>) {
  let sections = Input::parse(data.trim(), "\n\n", |section| { section.trim() });
  let coordinates: Vec<(usize, usize)> = Input::parse(sections[0], "\n", |line| {
    let coords = Input::parse(line.trim(), ",", |coord| { coord.parse::<usize>().unwrap() } );
    (coords[0], coords[1])
  });
  let folds: Vec<(&str, usize)> = Input::parse(sections[1], "\n", |line| {
    let fold = Input::parse(line.trim(), "=", |coord| { coord } );
    (fold[0], fold[1].parse::<usize>().unwrap())
  });
  (coordinates, folds)
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 13,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Ready points
      let mut points = data.0;
      
      // Perform first fold
      fold(&mut points, data.1[0]);

      // Remove duplicate points
      points.sort();
      points.dedup();

      // Calculate and return result
      String::from(format!("{:?}", points.len()))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 13,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Ready points
      let mut points = data.0;
      
      // Perform folds
      for i in 0..data.1.len() {
        fold(&mut points, data.1[i]);
      }

      // Remove duplicate points
      points.sort();
      points.dedup();

      // Display points
      DotDisplay::print_binary(points);

      // Calculate and return result
      String::from(format!("{}", "PGHZBFJC"))
    }

  );

  // Return registry ownership
  registry
}

/// Performs a fold on the plain of 2D points
/// 
/// # Arguments
/// * points: Vector of point coordinates to be mutated by the fold operation
/// * fold:   Fold definition containing the orientation and coordinate of the fold
fn fold (points: &mut Vec<(usize, usize)>, fold: (&str, usize)) {
  for i in 0..points.len() {
    // Fold horizontally
    if fold.0 == "fold along x" {
      if points[i].0 > fold.1 {
        points[i].0 = 2 * fold.1 - points[i].0;
      }
    }
    // Fold vertically
    else if fold.0 == "fold along y" {
      if points[i].1 > fold.1 {
        points[i].1 = 2 * fold.1 - points[i].1;
      }
    }
  }
}
