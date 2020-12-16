//! 2019/10 puzzle
//! 
//! https://adventofcode.com/2019/day/10
// -----------------------------------------------------------------------------

// Include dependencies
use std::collections::HashMap;
use crate::lib::inputs::*;
use crate::lib::puzzle::*;
use super::image::*;

/// Registers puzzles for the day
pub fn run (index: u32, key: &str, verbose: bool, obfuscate: bool) -> PuzzleExecutionStatistics {

  // Initialize stats
  let mut stats = PuzzleExecutionStatistics{
    ..Default::default()
  };

  // Run puzzle
  if (index == 0) || (index == 1) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::ParamsVector1D(vec![5, 5], ".#..#.....#####....#...##".as_bytes().to_vec());
      stats.update(
        Puzzle::new(2019, 10, 1, "test", input, implementation1, |outputs| (outputs.2, Some(8)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamsVector1D(vec![10, 10], "......#.#.#..#.#......#######..#.#.###...#..#.......#....#.##..#....#..##.#..#####...#..#..#....####".as_bytes().to_vec());
      stats.update(
        Puzzle::new(2019, 10, 1, "test", input, implementation1, |outputs| (outputs.2, Some(33)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamsVector1D(vec![10, 10], "#.#...#.#..###....#..#....#...##.#.#.#.#....#.#.#..##..###.#..#...##....##....##......#....####.###.".as_bytes().to_vec());
      stats.update(
        Puzzle::new(2019, 10, 1, "test", input, implementation1, |outputs| (outputs.2, Some(35)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamsVector1D(vec![10, 10], ".#..#..#######.###.#....###.#...###.##.###.##.#.#.....###..#..#.#..#.##..#.#.###.##...##.#.....#.#..".as_bytes().to_vec());
      stats.update(
        Puzzle::new(2019, 10, 1, "test", input, implementation1, |outputs| (outputs.2, Some(41)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::ParamsVector1D(vec![20, 20], ".#..##.###...#########.############..##..#.######.########.#.###.#######.####.#.#####.##.#.##.###.##..#####..#.##############################.####....###.#.#.####.######################.##.###..####....######..##.###########.##.####...##..#.#####..#.######.#####...#.##########...#.##########.#######.####.#.###.###.#.##....##.##.###..#####.#.#.###########.####.#.#.#####.####.######.##.####.##.#..##".as_bytes().to_vec());
      stats.update(
        Puzzle::new(2019, 10, 1, "test", input, implementation1, |outputs| (outputs.2, Some(210)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = PuzzleInput::ParamsVector1D(vec![24, 24], load_input("./src/year2019/data/day10input.txt").replace('\n', "").as_bytes().to_vec());
      stats.update(
        Puzzle::new(2019, 10, 1, "solution", input, implementation1, |outputs| (outputs.2, Some(280)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Run puzzle
  if (index == 0) || (index == 2) {
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = PuzzleInput::ParamsVector1D(vec![24, 24, 20, 18, 200], load_input("./src/year2019/data/day10input.txt").replace('\n', "").as_bytes().to_vec());
      stats.update(
        Puzzle::new(2019, 10, 2, "solution", input, implementation2, |outputs| ((outputs.0 * 100) + outputs.1, Some(706)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<u8, (usize, usize, usize), usize>, verbose: bool) -> Result<(usize, usize, usize), &str> {
  match &puzzle.input {
    PuzzleInput::ParamsVector1D(params, input) => {
      // If verbose, print out image
      if verbose {
        let image = SpaceImage::new(params[0] as usize, params[1] as usize, input.clone());
        println!("{}", image.to_string(0, None));
      }
      // Extract asteroid coordinates from the image
      let positions = extract_asteroids_positions(params[0] as usize, params[1] as usize, input);
      // Find best vantage point
      let mut source: (usize, usize) = (0, 0);
      let mut max_targets: usize = 0;
      for position in &positions {
        // Group targets by relative angle from seleted vantage point
        let angles = group_by_angle(&position, &positions);
        // Count visible targets
        let count: usize = angles.values().len();
        // Check if best vantage point
        if count > max_targets {
          max_targets = count;
          source = position.clone();
        }
      }
      // If verbose, print out best location
      if verbose {
        println!("[x: {}, y: {}] => {}", &source.0, &source.1, &max_targets);
      }
      // Return best detection
      Ok((source.0, source.1, max_targets))
    },
    _ => panic!("This should never, ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<u8, (usize, usize), usize>, verbose: bool) -> Result<(usize, usize), &str> {
  match &puzzle.input {
    PuzzleInput::ParamsVector1D(params, input) => {
      // If verbose, print out image
      if verbose {
        let image = SpaceImage::new(params[0] as usize, params[1] as usize, input.clone());
        println!("{}", image.to_string(0, None));
      }
      // Extract asteroid coordinates from the image
      let positions = extract_asteroids_positions(params[0] as usize, params[1] as usize, input);
      let source = (params[2] as usize, params[3] as usize);
      // Group targets by relative angle from seleted vantage point
      let angles = group_by_angle(&source, &positions);
      // Sort targets by clockwise angles
      let mut sorted = sort_as_clockwise_angle(&source, angles);
      // Find N-th target clockwise
      let mut count: usize = 0;
      let nth: usize = params[4] as usize;
      loop {
        // Find N-th target or drop earlier targets
        for i in 0..sorted.len() {
          // Check if angle has remaining targets
          if sorted[i].1.len() > 0 {
            // Remove next target
            count += 1;
            let target = sorted[i].1.remove(0);
            // Check if N-th target
            if count == nth {
              // If verbose, print out N-th target
              if verbose {
                println!("{}-th target: [x: {}, y: {}]", nth, &target.0, &target.1);
              }
              // Get next target as N-th
              return Ok(target);
            }
          }
        }
        // Drop empty angles
        for i in (0..sorted.len()).rev() {
          if sorted[i].1.len() == 0 {
            sorted.remove(i);
          }
        }
      }
    },
    _ => panic!("This should never, ever happen!")
  }
}

fn extract_asteroids_positions (width: usize, height: usize, input: &Vec<u8>) -> Vec<(usize, usize)> {
  // Initialize coordiantes
  let mut positions: Vec<(usize, usize)> = vec![];
  // Extract asteroid positions from image
  for y in 0..height {
    for x in 0..width {
      if input[y * width + x] == '#' as u8 {
        positions.push((x as usize, y as usize));
      }
    }
  }
  // Return extracted positions
  positions
}

fn group_by_angle (source: &(usize, usize), targets: &Vec<(usize, usize)>) -> HashMap<String, Vec<(usize, usize)>> {
  // Initialize angles
  let mut angles: HashMap<String, Vec<(usize, usize)>> = HashMap::new();
  // Sort targets by angles
  for target in targets {
    // Check if source        
    if (target.0 == source.0) && (target.1 == source.1) { continue; }
    // Calculate relative angle
    let dx = target.0 as f32 - source.0 as f32;
    let dy = source.1 as f32 - target.1 as f32;
    let angle = match dx == 0.0 {
      true => (if dy > 0.0 { 0.5 * std::f32::consts::PI } else { 1.5 * std::f32::consts::PI }).to_string(),
      false => match dy == 0.0 {
        true  => (if dx > 0.0 { 0.0 } else { std::f32::consts::PI }).to_string(),
        false => (if dx > 0.0 { 0.0 } else { std::f32::consts::PI } + (if dx > 0.0 && dy < 0.0 { 2.0 * std::f32::consts::PI } else { 0.0 }) + (dy / dx).atan()).to_string()
      }
    };
    // Register target by angle
    if !angles.contains_key(&angle) {
      angles.insert(angle.clone(), vec![]);
    }
    match angles.get_mut(&angle) {
      Some(positions) => {
        positions.push(target.clone());
      },
      None => {}
    }
  }
  // Return grouped angles
  angles
}

fn sort_as_clockwise_angle (source: &(usize, usize), angles: HashMap<String, Vec<(usize, usize)>>) -> Vec<(f32, Vec<(usize, usize)>)> {
  // Compose angles into a vector for sorting
  let mut sorted: Vec<(f32, Vec<(usize, usize)>)> = vec![];
  for angle in angles.keys() {
    let radians_angle: f32 = angle.parse().unwrap();
    let clockwise_angle = (0.5 * std::f32::consts::PI) - radians_angle + (if radians_angle > 0.5 * std::f32::consts::PI { 2.0 * std::f32::consts::PI} else { 0.0 });
    let mut targets_by_distance = angles[angle].clone();
    targets_by_distance.sort_by(|a, b| {
      let da = (((source.0 as i32 - a.0 as i32).pow(2) + (source.1 as i32 - a.1 as i32).pow(2)) as f32).sqrt();
      let db = (((source.0 as i32 - b.0 as i32).pow(2) + (source.1 as i32 - b.1 as i32).pow(2)) as f32).sqrt();
      da.partial_cmp(&db).unwrap()
    });
    sorted.push((clockwise_angle, targets_by_distance));
  }
  // Sort vector
  sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
  // Return sorted vector
  sorted
}
