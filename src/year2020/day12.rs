//! 2020/12 puzzle
//! 
//! https://adventofcode.com/2020/day/12
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::inputs::*;
use crate::lib::puzzle::*;
use std::f64::consts::*;

/// Registers puzzles for the day
pub fn run (index: u32, key: &str, verbose: bool, obfuscate: bool) -> PuzzleExecutionStatistics {

  // Initialize stats
  let mut stats = PuzzleExecutionStatistics{
    ..Default::default()
  };

  // Initialize input
  let input = vec![
    String::from("F10"),
    String::from("N3"),
    String::from("F7"),
    String::from("R90"),
    String::from("F11")
  ];

  // Run puzzle
  if (index == 0) || (index == 1) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector1D(input.clone());
      stats.update(
        Puzzle::new(2020, 12, 1, "test", input, implementation1, |r| (r, Some(25)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d(load_input("./src/year2020/data/day12input.txt"), "\n");
      stats.update(
        Puzzle::new(2020, 12, 1, "solution", input, implementation1, |r| (r, Some(441)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Run puzzle
  if (index == 0) || (index == 2) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector1D(input.clone());
      stats.update(
        Puzzle::new(2020, 12, 2, "test", input, implementation2, |r| (r, Some(286)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d(load_input("./src/year2020/data/day12input.txt"), "\n");
      stats.update(
        Puzzle::new(2020, 12, 2, "solution", input, implementation2, |r| (r, Some(40014)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<String, usize, usize>, verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(instructions) => {
      let distance = navigate_directory(&instructions, verbose);
      return Ok(distance);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<String, usize, usize>, verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(instructions) => {
      let distance = navigate_via_waypoint(&instructions, verbose);
      return Ok(distance);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

/// Follows navigation instructions and returns manhattan distance to the origin
/// 
/// # Arguments
/// * `instructions` - Per move instructions on how to move
/// * `verbose`      - Outputs executing output of the puzzle to the console
fn navigate_directory (instructions: &Vec<String>, verbose: bool) -> usize {
  // Initialize starting coordinates
  let mut x: isize = 0;
  let mut y: isize = 0;
  let mut direction: isize = 0; // 0 east, left turns positive to north, right turns negative to south
  
  // Decode and process instructions
  for i in 0..instructions.len() {

    // Decode instructions
    let command = &instructions[i][0..1];
    let value = instructions[i][1..].parse::<isize>().expect("Parsing command value failed!");

    // Follow instruction
    let mut dx = 0;
    let mut dy = 0;
    let mut ddirection = 0;
    match command {
      "E" => { dx += value; },
      "W" => { dx -= value; },
      "N" => { dy += value; },
      "S" => { dy -= value; },
      "L" => { ddirection += value; },
      "R" => { ddirection -= value; },
      "F" => {
        dx += ((value as f64) * ((direction as f64) * PI / 180.0).cos()).round() as isize;
        dy += ((value as f64) * ((direction as f64) * PI / 180.0).sin()).round() as isize;
      }
      _ => ()
    }

    // Prompt
    if verbose {
      println!("{} -> {} {} -> [{}, {}, {}] -> [{}, {}, {}]", instructions[i], command, value, x, y, direction, (x + dx), (y + dy), (direction + ddirection));
    }

    // Apply instructed movement
    x += dx;
    y += dy;
    direction += ddirection;

  }

  // Return manhattan distance traveled
  return x.abs() as usize + y.abs() as usize;
}

/// Follows navigation instructions via a waypoint starting at (10, 1) and returns manhattan distance to the origin
/// 
/// # Arguments
/// * `instructions` - Per move instructions on how to move
/// * `verbose`      - Outputs executing output of the puzzle to the console
fn navigate_via_waypoint (instructions: &Vec<String>, verbose: bool) -> usize {
  // Initialize starting coordinates
  let mut x: isize = 0;
  let mut y: isize = 0;
  let mut wp_x: isize = 10;
  let mut wp_y: isize = 1;
  
  // Decode and process instructions
  for i in 0..instructions.len() {

    // Decode instructions
    let command = &instructions[i][0..1];
    let value = instructions[i][1..].parse::<isize>().expect("Parsing command value failed!");

    // Follow instruction
    let mut dwp_x = 0;
    let mut dwp_y = 0;
    let mut dx = 0;
    let mut dy = 0;
    match command {
      "E" => { dwp_x += value; },
      "W" => { dwp_x -= value; },
      "N" => { dwp_y += value; },
      "S" => { dwp_y -= value; },
      "L" => {
        if wp_x == -10 && wp_y == 23 {
          println!("");
        }
        // Calculate (updated) direction and distance
        let mut wp_direction: f64 =
          if wp_x == 0 {
            (PI / 2.0) * (if wp_y > 0 { 1.0 } else { -1.0 })
          } else if wp_y == 0 {
            (PI / 2.0) * (if wp_x > 0 { 0.0 } else { 2.0 })
          } else {
            ((if wp_x < 1 { 1.0 } else { 0.0 }) * (PI)) + (wp_y as f64 / wp_x as f64).atan()
          };
        wp_direction += (value as f64) * PI / 180.0;
        let wp_distance: f64 = ((wp_x * wp_x + wp_y * wp_y) as f64).sqrt();
        // Get rotated coordinates
        dwp_x = ((wp_distance * wp_direction.cos()).round() as isize) - wp_x;
        dwp_y = ((wp_distance * wp_direction.sin()).round() as isize) - wp_y;
      },
      "R" => {
        // Calculate (updated) direction and distance
        let mut wp_direction: f64 =
          if wp_x == 0 {
            (PI / 2.0) * (if wp_y > 0 { 1.0 } else { -1.0 })
          } else if wp_y == 0 {
            (PI / 2.0) * (if wp_x > 0 { 0.0 } else { 2.0 })
          } else {
            ((if wp_x < 1 { 1.0 } else { 0.0 }) * (PI)) + (wp_y as f64 / wp_x as f64).atan()
          };
        wp_direction -= (value as f64) * PI / 180.0;
        let wp_distance: f64 = ((wp_x * wp_x + wp_y * wp_y) as f64).sqrt();
        // Get rotated coordinates
        dwp_x = ((wp_distance * wp_direction.cos()).round() as isize) - wp_x;
        dwp_y = ((wp_distance * wp_direction.sin()).round() as isize) - wp_y;
      },
      "F" => {
        dx += value * wp_x;
        dy += value * wp_y;
      }
      _ => ()
    }

    // Prompt
    if verbose {
      println!("{} -> {} {} -> [{}, {} > {}, {}] -> [{}, {} > {}, {}]", instructions[i], command, value, x, y, wp_x, wp_y, (x + dx), (y + dy), (wp_x + dwp_x), (wp_y + dwp_y));
    }

    // Apply instructed movement
    wp_x += dwp_x;
    wp_y += dwp_y;
    x += dx;
    y += dy;

  }

  // Return manhattan distance traveled
  return x.abs() as usize + y.abs() as usize;
}
