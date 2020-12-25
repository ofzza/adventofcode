//! 2020/24 puzzle
//! 
//! https://adventofcode.com/2020/day/24
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::inputs::*;
use crate::lib::puzzle::*;
use std::collections::HashMap;

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
      let input = PuzzleInput::Vector1D(vec![
        String::from("sesenwnenenewseeswwswswwnenewsewsw"),
        String::from("neeenesenwnwwswnenewnwwsewnenwseswesw"),
        String::from("seswneswswsenwwnwse"),
        String::from("nwnwneseeswswnenewneswwnewseswneseene"),
        String::from("swweswneswnenwsewnwneneseenw"),
        String::from("eesenwseswswnenwswnwnwsewwnwsene"),
        String::from("sewnenenenesenwsewnenwwwse"),
        String::from("wenwwweseeeweswwwnwwe"),
        String::from("wsweesenenewnwwnwsenewsenwwsesesenwne"),
        String::from("neeswseenwwswnwswswnw"),
        String::from("nenwswwsewswnenenewsenwsenwnesesenew"),
        String::from("enewnwewneswsewnwswenweswnenwsenwsw"),
        String::from("sweneswneswneneenwnewenewwneswswnese"),
        String::from("swwesenesewenwneswnwwneseswwne"),
        String::from("enesenwswwswneneswsenwnewswseenwsese"),
        String::from("wnwnesenesenenwwnenwsewesewsesesew"),
        String::from("nenewswnwewswnenesenwnesewesw"),
        String::from("eneswnwswnwsenenwnwnwwseeswneewsenese"),
        String::from("neswnwewnwnwseenwseesewsenwsweewe"),
        String::from("wseweeenwnesenwwwswnew")
      ]);
      stats.update(
        Puzzle::new(2020, 24, 1, "test", input, implementation1, |r| (r, Some(10)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<String>(load_input("./src/year2020/data/day24input.txt"), "\n");
      stats.update(
        Puzzle::new(2020, 24, 1, "solution", input, implementation1, |r| (r, Some(497)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Run puzzle
  if (index == 0) || (index == 2) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector1D(vec![
        String::from("sesenwnenenewseeswwswswwnenewsewsw"),
        String::from("neeenesenwnwwswnenewnwwsewnenwseswesw"),
        String::from("seswneswswsenwwnwse"),
        String::from("nwnwneseeswswnenewneswwnewseswneseene"),
        String::from("swweswneswnenwsewnwneneseenw"),
        String::from("eesenwseswswnenwswnwnwsewwnwsene"),
        String::from("sewnenenenesenwsewnenwwwse"),
        String::from("wenwwweseeeweswwwnwwe"),
        String::from("wsweesenenewnwwnwsenewsenwwsesesenwne"),
        String::from("neeswseenwwswnwswswnw"),
        String::from("nenwswwsewswnenenewsenwsenwnesesenew"),
        String::from("enewnwewneswsewnwswenweswnenwsenwsw"),
        String::from("sweneswneswneneenwnewenewwneswswnese"),
        String::from("swwesenesewenwneswnwwneseswwne"),
        String::from("enesenwswwswneneswsenwnewswseenwsese"),
        String::from("wnwnesenesenenwwnenwsewesewsesesew"),
        String::from("nenewswnwewswnenesenwnesewesw"),
        String::from("eneswnwswnwsenenwnwnwwseeswneewsenese"),
        String::from("neswnwewnwnwseenwseesewsenwsweewe"),
        String::from("wseweeenwnesenwwwswnew")
      ]);
      stats.update(
        Puzzle::new(2020, 24, 2, "test", input, implementation2, |r| (r, Some(2208)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<String>(load_input("./src/year2020/data/day24input.txt"), "\n");
      stats.update(
        Puzzle::new(2020, 24, 2, "solution", input, implementation2, |r| (r, Some(4156)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<String, usize, usize>, verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(paths) => {
      // Parse paths
      let paths = paths.iter().map(|path| parse_path(path.clone(), verbose)).collect();
      let floor = tile_floor(paths, verbose);

      // Return result
      return Ok(count_tiles_black(&floor));
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<String, usize, usize>, verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(paths) => {
      // Parse paths
      let paths = paths.iter().map(|path| parse_path(path.clone(), verbose)).collect();
      let floor = tile_floor(paths, verbose);
      let floor = maintain_floor(floor, 100, verbose);

      // Return result
      return Ok(count_tiles_black(&floor));
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

/// Parses a path as a series of hexagonal directions from  string representation
/// 
/// # Arguments
/// * `path`    - String representation of the path
/// * `verbose` - Outputs executing output of the puzzle to the console
fn parse_path (path: String, verbose: bool) -> Vec<Direction> {
  // Initialize parsed path
  let mut path_directions: Vec<Direction> = vec![];
  // Compose parsed path
  let path_chars = path.as_bytes();
  let mut i = 0;
  while i < path_chars.len() {
    // Check value
    match path_chars[i] as char {
      'e' => path_directions.push(Direction::E),
      'w' => path_directions.push(Direction::W),
      'n' => {
        // Check sub-direction
        if i + 1 < path_chars.len() {
          match path_chars[i + 1] as char {
            'e' => {
              path_directions.push(Direction::NE);
              i += 1;
            },
            'w' => {
              path_directions.push(Direction::NW);
              i += 1;
            },
            _ => ()
          }
        }
      },
      's' => {
        // Check sub-direction
        if i + 1 < path_chars.len() {
          match path_chars[i + 1] as char {
            'e' => {
              path_directions.push(Direction::SE);
              i += 1;
            },
            'w' => {
              path_directions.push(Direction::SW);
              i += 1;
            },
            _ => ()
          }
        }
      },
      _ => ()
    }
    // Next
    i += 1;
  }

  // Prompt
  if verbose {
    println!("  {} -> {:?}", path, path_directions);
  }

  // Return parsed path
  return path_directions;
}

/// Tiles a hex-grid floor by flipping colors of tiles specified by paths
/// 
/// # Arguments
/// * `paths`   - Paths to tiles to flip
/// * `verbose` - Outputs executing output of the puzzle to the console
fn tile_floor (paths: Vec<Vec<Direction>>, verbose: bool) -> HashMap<(isize, isize), usize> {
  // Initialize floor
  let mut floor: HashMap<(isize, isize), usize> = HashMap::new();
  
  // Follow paths and toggle tiles
  for i in 0..paths.len() {
    // Get path and starting coordinates
    let path = &paths[i];
    let mut coords: (isize, isize) = (0, 0);

    // Prompt
    if verbose {
      print!("  {}: (0,0)", i);
    }

    // Follow path
    for direction in path {
      // Update coords
      match direction {
        Direction::E  => {
          coords.0 += 1;
          coords.1 += 0;
        },
        Direction::NE => {
          coords.0 += if (coords.1 % 2) == 0 { 1 } else { 0 };
          coords.1 += 1;
        },
        Direction::SE => {
          coords.0 += if (coords.1 % 2) == 0 { 1 } else { 0 };
          coords.1 += -1;
        },
        Direction::W  => {
          coords.0 += -1;
          coords.1 += 0;
        },
        Direction::NW => {
          coords.0 += if (coords.1 % 2) != 0 { -1 } else { 0 };
          coords.1 += 1;
        },
        Direction::SW => {
          coords.0 += if (coords.1 % 2) != 0 { -1 } else { 0 };
          coords.1 += -1;
        }
      }
      // Prompt
      if verbose {
        print!("-[{:?}]->{:?}", direction, coords);
      }

    }

    // Toggle coordinates
    let count: usize;
    if floor.contains_key(&coords) {
      // Tile floor
      count = floor.get(&coords).unwrap() + 1;
      // Prompt
      if verbose {
        println!("\n       -> Flipped back to white");
      }
    } else {
      // Tile floor
      count = 1;
      // Prompt
      if verbose {
        println!("\n       -> Flipped to black");
      }
    }    
    floor.insert(coords, count);

  }

  // Return tiled floor
  return floor;
}

/// Iterates a tiled hex-grid floor through a number of iterations following preset rules on which tiles to flip
/// 
/// # Arguments
/// * `floor`      - Already tiled hex-grid floor
/// * `iterations` - Number of iterations
/// * `verbose`    - Outputs executing output of the puzzle to the console
fn maintain_floor (mut floor: HashMap<(isize, isize), usize>, iterations: usize, verbose: bool) -> HashMap<(isize, isize), usize> {
  // Prompt
  if verbose {
    println!("  0 -> {} of {}", count_tiles_black(&floor), floor.len());
  }

  // Maintain for a requested number of iterations
  for i in 0..iterations {

    // Collect candidate tiles to check
    let mut candidate_coordinates: HashMap<(isize, isize), bool> = HashMap::new();
    for coords in floor.keys()
    {
      // Skip white squares (will either be added as neighbouring black squares, or have no chance of toggling anyway)
      if floor.get(&coords).unwrap().clone() % 2 == 0 { continue; }
      // Compose candidates
      /* -  */ candidate_coordinates.insert((coords.0, coords.1), true);
      /* E  */ candidate_coordinates.insert((coords.0 + 1, coords.1 + 0), true);
      /* NE */ candidate_coordinates.insert((coords.0 + if (coords.1 % 2) == 0 { 1 } else { 0 }, coords.1 + 1), true);
      /* SE */ candidate_coordinates.insert((coords.0 + if (coords.1 % 2) == 0 { 1 } else { 0 }, coords.1 - 1), true);
      /* W  */ candidate_coordinates.insert((coords.0 - 1, coords.1 + 0), true);
      /* NW */ candidate_coordinates.insert((coords.0 + if (coords.1 % 2) != 0 { -1 } else { 0 }, coords.1 + 1), true);
      /* SW */ candidate_coordinates.insert((coords.0 + if (coords.1 % 2) != 0 { -1 } else { 0 }, coords.1 - 1), true);
    }

    // Check which candidates need to be toggled
    let mut toggle_coordinates: HashMap<(isize, isize), bool> = HashMap::new();
    for coords in candidate_coordinates.keys() {

      // Check if already processed
      if toggle_coordinates.contains_key(&coords) {
        continue;
      }

      // Count adjacent black tiles
      let adjacent_coordinates: Vec<(isize, isize)> = vec![
        /* E  */ (coords.0 + 1, coords.1 + 0),
        /* NE */ (coords.0 + if (coords.1 % 2) == 0 { 1 } else { 0 }, coords.1 + 1),
        /* SE */ (coords.0 + if (coords.1 % 2) == 0 { 1 } else { 0 }, coords.1 - 1),
        /* W  */ (coords.0 - 1, coords.1 + 0),
        /* NW */ (coords.0 + if (coords.1 % 2) != 0 { -1 } else { 0 }, coords.1 + 1),
        /* SW */ (coords.0 + if (coords.1 % 2) != 0 { -1 } else { 0 }, coords.1 - 1)
      ];

      // Check candidate tile
      let exists = floor.contains_key(&coords);
      let existing_toggles_count = if exists { floor.get(&coords).unwrap().clone() } else { 0 };
      // Check tiles bordering to white tile
      if existing_toggles_count % 2 == 0 {
        let mut adj_black_tiles_count = 0;
        for adj_coords in adjacent_coordinates {
          if floor.contains_key(&adj_coords) && floor.get(&adj_coords).unwrap() % 2 != 0 {
            adj_black_tiles_count += 1;
          }
        }
        if adj_black_tiles_count == 2 {
          toggle_coordinates.insert(coords.clone(), true);
          continue;
        }
      }
      
      // Check tiles bordering to black tile
      else if existing_toggles_count % 2 != 0 {
        let mut adj_black_tiles_count = 0;
        for adj_coords in adjacent_coordinates {
          if floor.contains_key(&adj_coords) && floor.get(&adj_coords).unwrap() % 2 != 0 {
            adj_black_tiles_count += 1;
            if adj_black_tiles_count > 2 { break;}
          }
        }
        if adj_black_tiles_count == 0 || adj_black_tiles_count > 2 {
          toggle_coordinates.insert(coords.clone(), true);
          continue;
        }
      }
      
      // This should never happen
      else {
        panic!("Somehow a tile is not black or white!");
      }

      // Store as skip-able for next time
      toggle_coordinates.insert(coords.clone(), false);
    }

    // Update all coordinates that need toggling
    for coords in toggle_coordinates.keys() {
      if toggle_coordinates.get(coords).unwrap().clone() {
        let exists = floor.contains_key(&coords);
        let toggles_count = if exists { floor.get(&coords).unwrap().clone() } else { 0 };
        floor.insert(coords.clone(), toggles_count + 1);
      }
    }

    // Prompt
    if verbose {
      println!("  {} -> {} of {}", i + 1, count_tiles_black(&floor), floor.len());
    }

  }

  // Return floor
  return floor;
}

/// Counts black tiles on a floor
/// 
/// # Arguments
/// * `floor`      - Already tiled hex-grid floor
fn count_tiles_black (floor: &HashMap<(isize, isize), usize>) -> usize {
  return floor
    .values()
    .map(|value| value.clone())
    .filter(|value| value % 2 != 0)
    .collect::<Vec<usize>>()
    .len();
}

/// Enumerates directions on a hex grid
#[derive(Debug)]
enum Direction { NE, E, SE, SW, W, NW }
