//! 2019/03 puzzle
//! 
//! https://adventofcode.com/2019/day/3
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::inputs::*;
use crate::lib::puzzle::*;
use super::lib::wiring::*;

/// Registers puzzles for the day
pub fn run (index: u32, key: &str, _verbose: bool, obfuscate: bool) -> PuzzleExecutionStatistics {

  // Initialize stats
  let mut stats = PuzzleExecutionStatistics{
    ..Default::default()
  };
  
  // Run puzzle
  if (index == 0) || (index == 1) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector2D(vec![
        vec![String::from("R8"), String::from("U5"), String::from("L5"), String::from("D3")],
        vec![String::from("U7"), String::from("R6"), String::from("D4"), String::from("L4")]
      ]);
      stats.update(
        Puzzle::new(2019, 3, 1, "test", input, implementation1, |d| (d, Some(6)))
          .run(false, obfuscate)
      );
      // Test
      let input = PuzzleInput::Vector2D(vec![
        vec![String::from("R75"), String::from("D30"), String::from("R83"), String::from("U83"), String::from("L12"), String::from("D49"), String::from("R71"), String::from("U7"), String::from("L72")],
        vec![String::from("U62"), String::from("R66"), String::from("U55"), String::from("R34"), String::from("D71"), String::from("R55"), String::from("D58"), String::from("R83")]
      ]);
      stats.update(
        Puzzle::new(2019, 3, 1, "test", input, implementation1, |d| (d, Some(159)))
          .run(false, obfuscate)
      );
      // Test
      let input = PuzzleInput::Vector2D(vec![
        vec![String::from("R98"), String::from("U47"), String::from("R26"), String::from("D63"), String::from("R33"), String::from("U87"), String::from("L62"), String::from("D20"), String::from("R33"), String::from("U53"), String::from("R51")],
        vec![String::from("U98"), String::from("R91"), String::from("D20"), String::from("R16"), String::from("D67"), String::from("R40"), String::from("U7"), String::from("R15"), String::from("U6"), String::from("R7")]
      ]);
      stats.update(
        Puzzle::new(2019, 3, 1, "test", input, implementation1, |d| (d, Some(135)))
          .run(false, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_2d::<String>(load_input("./src/year2019/data/day03input.txt"), "\n", ",");
      stats.update(
        Puzzle::new(2019, 3, 1, "solution", input, implementation1, |d| (d, Some(1626)))
          .run(false, obfuscate)
      );
    }
  }

  // Run puzzle
  if (index == 0) || (index == 2) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector2D(vec![
        vec![String::from("R8"), String::from("U5"), String::from("L5"), String::from("D3")],
        vec![String::from("U7"), String::from("R6"), String::from("D4"), String::from("L4")]
      ]);
      stats.update(
        Puzzle::new(2019, 3, 2, "test", input, implementation2, |d| (d, Some(30)))
          .run(false, obfuscate)
      );
      // Test
      let input = PuzzleInput::Vector2D(vec![
        vec![String::from("R75"), String::from("D30"), String::from("R83"), String::from("U83"), String::from("L12"), String::from("D49"), String::from("R71"), String::from("U7"), String::from("L72")],
        vec![String::from("U62"), String::from("R66"), String::from("U55"), String::from("R34"), String::from("D71"), String::from("R55"), String::from("D58"), String::from("R83")]
      ]);
      stats.update(
        Puzzle::new(2019, 3, 2, "test", input, implementation2, |d| (d, Some(610)))
          .run(false, obfuscate)
      );
      // Test
      let input = PuzzleInput::Vector2D(vec![
        vec![String::from("R98"), String::from("U47"), String::from("R26"), String::from("D63"), String::from("R33"), String::from("U87"), String::from("L62"), String::from("D20"), String::from("R33"), String::from("U53"), String::from("R51")],
        vec![String::from("U98"), String::from("R91"), String::from("D20"), String::from("R16"), String::from("D67"), String::from("R40"), String::from("U7"), String::from("R15"), String::from("U6"), String::from("R7")]
      ]);
      stats.update(
        Puzzle::new(2019, 3, 2, "test", input, implementation2, |d| (d, Some(410)))
          .run(false, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_2d::<String>(load_input("./src/year2019/data/day03input.txt"), "\n", ",");
      stats.update(
        Puzzle::new(2019, 3, 2, "solution", input, implementation2, |d| (d, Some(27330)))
          .run(false, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<String, u32, u32>, _verbose: bool) -> Result<u32, &str> {
  match &puzzle.input {
    PuzzleInput::Vector2D(instructions) => {
      // Initialize wiring
      let wiring = Wiring::new(instructions.clone());
      // // Find closest intersection
      let mut closest_intersection_distance: u32 = std::u32::MAX;
      for coordinates in wiring.intersections.keys() {
        let manhattan_distance: u32 = coordinates.x.abs() as u32 + coordinates.y.abs() as u32;
        if manhattan_distance < closest_intersection_distance {
          closest_intersection_distance = manhattan_distance;
        }
      }
      // Return closest intersection
      return Ok(closest_intersection_distance);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<String, u32, u32>, _verbose: bool) -> Result<u32, &str> {
  match &puzzle.input {
    PuzzleInput::Vector2D(instructions) => {
      // Initialize wiring
      let wiring = Wiring::new(instructions.clone());
      // Find closest intersection
      let mut closest_intersection_distance: u32 = std::u32::MAX;
      for coordinates in wiring.intersections.keys() {
        let intersecting_sections = wiring.intersections.get(&coordinates)
          .expect("This should never, ever happen!");
        let mut intersection_distance: u32 = 0;
        for intersecting_section in intersecting_sections {
          intersection_distance += intersecting_section.accumulated_distance;
          intersection_distance += if intersecting_section.get_orientation() == WireSegmentOrientation::Horizontal {
              (coordinates.x - intersecting_section.start.x).abs() as u32
            } else {
              (coordinates.y - intersecting_section.start.y).abs() as u32
            };
        }
        if intersection_distance < closest_intersection_distance {
          closest_intersection_distance = intersection_distance;
        }
      }
      // Return closest intersection
      return Ok(closest_intersection_distance);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}
