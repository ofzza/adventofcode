//! 2020/10 puzzle
//! 
//! https://adventofcode.com/2020/day/10
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

  // Run puzzle
  if (index == 0) || (index == 1) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector1D(vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]);
      stats.update(
        Puzzle::new(2020, 10, 1, "test", input, implementation1, |r| (r.1 * r.3, Some(35)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::Vector1D(vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3]);
      stats.update(
        Puzzle::new(2020, 10, 1, "test", input, implementation1, |r| (r.1 * r.3, Some(220)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<usize>(load_input("./src/year2020/data/day10input.txt"), "\n");
      stats.update(
        Puzzle::new(2020, 10, 1, "solution", input, implementation1, |r| (r.1 * r.3, Some(2592)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Run puzzle
  if (index == 0) || (index == 2) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector1D(vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]);
      stats.update(
        Puzzle::new(2020, 10, 2, "test", input, implementation2, |r| (r, Some(8)))
          .run(verbose, obfuscate)
      );
      // Test
      let input = PuzzleInput::Vector1D(vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3]);
      stats.update(
        Puzzle::new(2020, 10, 2, "test", input, implementation2, |r| (r, Some(19208)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<usize>(load_input("./src/year2020/data/day10input.txt"), "\n");
      stats.update(
        Puzzle::new(2020, 10, 2, "solution", input, implementation2, |r| (r, Some(198428693313536)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<usize, (usize, usize, usize, usize), usize>, verbose: bool) -> Result<(usize, usize, usize, usize), &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(adapters) => {
      // Sort adapters
      let mut adapters: Vec<usize> = adapters.clone();
      adapters.insert(0, 0);
      adapters.sort();
      // Calculate device joltage
      let device = adapters.last().unwrap() + 3;
      adapters.push(device);
      // Prompt adapters chain
      if verbose {
        println!("Adapters: {:?}", adapters);
      }
      // Cound joltage diffs
      let mut diffs: (usize, usize, usize) = (0, 0, 0);
      for i in 1..adapters.len() {
        if i > 0 && adapters[i] - adapters[i - 1] == 1 {
          diffs.0 += 1;
        }
        if i > 0 && adapters[i] - adapters[i - 1] == 2 {
          diffs.1 += 1;
        }
        if i > 0 && adapters[i] - adapters[i - 1] == 3 {
          diffs.2 += 1;
        }
      }

      // Prompt result
      if verbose {
        println!("Device joltage: {}, Diffs: {:?}", device, diffs);
      }

      // Return result
      return Ok((device, diffs.0, diffs.1, diffs.2));
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<usize, usize, usize>, verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(adapters) => {
      // Sort adapters
      let mut adapters: Vec<usize> = adapters.clone();
      adapters.insert(0, 0);
      adapters.sort();
      // Calculate device joltage
      let device = adapters.last().unwrap() + 3;
      adapters.push(device);
      // Prompt adapters chain
      if verbose {
        println!("Adapters: {:?}", adapters);
      }
      // Find indispensible adapters
      let indispensibles = find_indispencibles(&adapters);
      if verbose {
        println!("Indispensible adapter indexes: {:?}", indispensibles);
        println!("Indispensible adapter values: {:?}", indispensibles.clone().into_iter().map(|i| adapters[i]).collect::<Vec<usize>>());
      }
      // Find permutations between indispensible adapters
      let mut product: usize = 1;
      for i in 1..indispensibles.len() {
        if indispensibles[i] - indispensibles[i - 1] > 1 {
          // Calculate permutations between indispensibles
          let optional_range = &adapters[indispensibles[i - 1]..indispensibles[i] + 1].to_vec();
          let count = 1 + count_permutations(&optional_range, false, 0);
          product *= count;
          // Prompt
          if verbose {
            println!("Permutations between {} [{}] - {} [{}]: {:?} => {}", adapters[indispensibles[i - 1]], indispensibles[i - 1], adapters[indispensibles[i]], indispensibles[i], optional_range, count)
          }
        }
      }
      // Count remaining adapters
      return Ok(product);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

/// Locates all adapters with sufficient joltage diff to make them indispensible
///
/// # Arguments
/// * `adapters` - Joltages of all adapters
fn find_indispencibles (adapters: &Vec<usize>) -> Vec<usize> {
  // Initialize indispensibles
  let mut indispensibles = vec![0];
  // Find indispensibles
  for i in 0..(adapters.len() - 1) {
    if (i == 0 && adapters[i + 1] > 3) || (i > 0 && adapters[i + 1] - adapters[i - 1] > 3) {
      indispensibles.push(i);
    }
  }
  // Return indispensibles
  return indispensibles;
}

/// Counts all the ways optional adapters can be dropped to keep the rest functioning with no diff to large
///
/// # Arguments
/// * `adapters` - Joltages of all adapters
/// * `verbose`  - Outputs executing output of the puzzle to the console
/// * `index`    - (Internal) Index after which the adapters can be dropped (used internally to prevent double processing of adapters)
fn count_permutations (adapters: &Vec<usize>, verbose: bool, index: usize) -> usize {
  // Count optional adapters
  let mut count: usize = 0;
  // Find an optional adapter
  for i in index..(adapters.len() - 1) {
    // Check if adapter is optional
    if i > 0 && adapters[i + 1] - adapters[i - 1] <= 3 {
      // Get remaining adapters
      let mut remaining_adapters: Vec<usize> = adapters.clone();
      let extra = remaining_adapters.remove(i);
      // Prompt reduced adapters chain
      if verbose {
        println!("{:?} => {}", remaining_adapters, extra);
      }
      // REcursivelly search for other optional adapters
      count += 1 + count_permutations(&remaining_adapters, verbose, i);
    }
  }
  // Return count of permutations
  return count;
}
