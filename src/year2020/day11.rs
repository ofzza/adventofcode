//! 2020/11 puzzle
//! 
//! https://adventofcode.com/2020/day/11
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::inputs::*;
use crate::lib::puzzle::*;
use super::seats::*;

/// Registers puzzles for the day
pub fn run (index: u32, key: &str, verbose: bool, obfuscate: bool) -> PuzzleExecutionStatistics {

  // Initialize stats
  let mut stats = PuzzleExecutionStatistics{
    ..Default::default()
  };

  // Initialize input
  let input: Vec<String> = vec![
    String::from("L.LL.LL.LL"),
    String::from("LLLLLLL.LL"),
    String::from("L.L.L..L.."),
    String::from("LLLL.LL.LL"),
    String::from("L.LL.LL.LL"),
    String::from("L.LLLLL.LL"),
    String::from("..L.L....."),
    String::from("LLLLLLLLLL"),
    String::from("L.LLLLLL.L"),
    String::from("L.LLLLL.LL")
  ];

  // Run puzzle
  if (index == 0) || (index == 1) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector1D(input.clone());
      stats.update(
        Puzzle::new(2020, 11, 1, "test", input, implementation1, |r| (r, Some(37)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d(load_input("./src/year2020/data/day11input.txt"), "\n");
      stats.update(
        Puzzle::new(2020, 11, 1, "solution", input, implementation1, |r| (r, Some(2427)))
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
        Puzzle::new(2020, 11, 2, "test", input, implementation2, |r| (r, Some(26)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d(load_input("./src/year2020/data/day11input.txt"), "\n");
      stats.update(
        Puzzle::new(2020, 11, 2, "solution", input, implementation2, |r| (r, Some(2199)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<String, usize, usize>, verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(input) => {
      // Load game
      let mut game = GameOfSeats::new(&input);
      // Prompt initial game state
      if verbose {
        println!("{}\n", game.to_string());
      }
      // Initialize game
      game.initialize(true);
      // Run game until stable
      while game.next(4, verbose) {
        // Prompt game state
        if verbose {
          println!("{}\n", game.to_string());
        }
      }
      // Return number of occupied seats
      return Ok(game.count_occupied);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<String, usize, usize>, verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(input) => {
      // Load game
      let mut game = GameOfSeats::new(&input);
      // Prompt initial game state
      if verbose {
        println!("{}\n", game.to_string());
      }
      // Initialize game
      game.initialize(false);
      // Prompt initial game state
      if verbose {
        println!("{}\n", game.to_string());
      }
      // Run game until stable
      while game.next(5, verbose) {
        // Prompt game state
        if verbose {
          println!("{}\n", game.to_string());
        }
      }
      // Return number of occupied seats
      return Ok(game.count_occupied);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}
