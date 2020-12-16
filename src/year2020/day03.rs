//! 2020/03 puzzle
//! 
//! https://adventofcode.com/2020/day/3
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::inputs::*;
use crate::lib::puzzle::*;

/// Registers puzzles for the day
pub fn run (index: u32, key: &str, verbose: bool, obfuscate: bool) -> PuzzleExecutionStatistics {

  // Initialize stats
  let mut stats = PuzzleExecutionStatistics{
    ..Default::default()
  };

  // Define test input
  let test_input: Vec<String> = vec![
    String::from("..##.........##.........##.........##.........##.........##......."),
    String::from("#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#.."),
    String::from(".#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#."),
    String::from("..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#"),
    String::from(".#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#."),
    String::from("..#.##.......#.##.......#.##.......#.##.......#.##.......#.##....."),
    String::from(".#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#"),
    String::from(".#........#.#........#.#........#.#........#.#........#.#........#"),
    String::from("#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#..."),
    String::from("#...##....##...##....##...##....##...##....##...##....##...##....#"),
    String::from(".#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#"),
  ];
  
  // Run puzzle
  if (index == 0) || (index == 1) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector1D(test_input.clone());
      stats.update(
        Puzzle::new(2020, 3, 1, "test", input, implementation1, |r| (r, Some(7)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<String>(load_input("./src/year2020/data/day03input.txt"), "\n");
      stats.update(
        Puzzle::new(2020, 3, 1, "solution", input, implementation1, |r| (r, Some(289)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Run puzzle
  if (index == 0) || (index == 2) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector1D(test_input.clone());
      stats.update(
        Puzzle::new(2020, 3, 2, "test", input, implementation2, |r| (r, Some(336)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<String>(load_input("./src/year2020/data/day03input.txt"), "\n");
      stats.update(
        Puzzle::new(2020, 3, 2, "solution", input, implementation2, |r| (r, Some(5522401584)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<String, usize, usize>, _verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(map) => Ok(count_trees(map, 3, 1)),
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<String, usize, usize>, _verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(map) => Ok(
      count_trees(map, 1, 1) *
      count_trees(map, 3, 1) * 
      count_trees(map, 5, 1) * 
      count_trees(map, 7, 1) * 
      count_trees(map, 1, 2)
    ),
    _ => panic!("This shouldn't ever happen!")
  }
}

/// Counts trees down a given slope
/// 
/// # Arguments
/// 
/// * `map`   - Definition of the map where '.' is en empty cell and '#' is a tree
/// * `right` - Relative right movement per move
/// * `down`  - Relative down movement per tick
fn count_trees (map: &Vec<String>, right: usize, down: usize) -> usize {
  // Initialize coordinates
  let mut count: usize = 0;

  // Run a slope
  let map_width: usize = map[0].as_bytes().len();
  for i in 1..(map.len() / down) {
    // Check if tree
    let y = i * down;
    let x = (i * right) % map_width;
    if (map[y].as_bytes()[x] as char) == '#' {
      count += 1;
    }
  }
  
  // Return count
  return count;
}
