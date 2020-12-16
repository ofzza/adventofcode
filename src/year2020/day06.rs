//! 2020/06 puzzle
//! 
//! https://adventofcode.com/2020/day/6
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
      let input = PuzzleInput::Vector1D(vec![
        String::from("abc"),
        String::from("a\nb\nc"),
        String::from("ab\nac"),
        String::from("a\na\na\na"),
        String::from("b")
      ]);
      stats.update(
        Puzzle::new(2020, 6, 1, "test", input, implementation1, |r| (r, Some(11)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<String>(load_input("./src/year2020/data/day06input.txt"), "\n\n");
      stats.update(
        Puzzle::new(2020, 6, 1, "solution", input, implementation1, |r| (r, Some(6596)))
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
        String::from("abc"),
        String::from("a\nb\nc"),
        String::from("ab\nac"),
        String::from("a\na\na\na"),
        String::from("b")
      ]);
      stats.update(
        Puzzle::new(2020, 6, 2, "test", input, implementation2, |r| (r, Some(6)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<String>(load_input("./src/year2020/data/day06input.txt"), "\n\n");
      stats.update(
        Puzzle::new(2020, 6, 2, "solution", input, implementation2, |r| (r, Some(3219)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<String, usize, usize>, verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(groups) => {
      // Initialize total count
      let mut count: usize = 0;
      // Count all groups
      for group in groups {
        count += process_group_join(group, verbose).len();
      }
      // Return total count
      return Ok(count);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<String, usize, usize>, verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(groups) => {
      // Initialize total count
      let mut count: usize = 0;
      // Count all groups
      for group in groups {
        count += process_group_intersect(group, verbose).1;
      }
      // Return total count
      return Ok(count);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

/// Processes answers for a group and returns vector of all unique answer characters
/// 
/// # Arguments
/// * `group`   - '\n' separated answers for each member of the group
/// * `verbose` - Outputs executing output of the puzzle to the console
fn process_group_join(group: &String, verbose: bool) -> Vec<char> {
  // Initialize result
  let mut result: Vec<char> = vec![];
  let group_replaced = group.replace('\n', "");
  let mut bytes = group_replaced.as_bytes().to_vec();
  bytes.sort();
  // Collect unique answers
  for b in bytes {
    let c = b.clone() as char;
    if result.len() == 0 || result.last().expect("Failed fetching last member!") != &c {
      result.push(c);
    }
  }
  // If verbose, prompt result
  if verbose {
    println!("{} -> {:?}", group.replace('\n', ", "), result);
  }
  // Return result
  return result;
}

/// Processes answers for a group and returns answers everyone in the group responded with
/// Returned value is a tuple of:
/// - a vecor of a tuple: 
///   - answer character
///   - count of members that answered with this answer
///   - if everyone answered with this answer
/// - total count of answers everyone answered with
/// 
/// # Arguments
/// * `group`   - '\n' separated answers for each member of the group
/// * `verbose` - Outputs executing output of the puzzle to the console
fn process_group_intersect(group: &String, verbose: bool) -> (Vec<(char, usize, bool)>, usize) {
  // Initialize result
  let mut result: Vec<(char, usize, bool)> = vec![];
  let mut count: usize = 0;
  let members: Vec<&str> = group.split("\n").collect();
  let group_replaced = group.replace('\n', "");
  let mut bytes = group_replaced.as_bytes().to_vec();
  bytes.sort();
  // Collect unique answers
  for b in bytes {
    let c = b.clone() as char;
    // Check if new character
    if result.len() == 0 || result[result.len() - 1].0 != c {
      result.push((c, 1, false));
    } else {
      let i = result.len() - 1;
      result[i].1 += 1;
    }
  }
  // Check which answers counts match group members count
  for i in 0..result.len() {
    result[i].2 = result[i].1 == members.len();
    if result[i].2 {
      count += 1;
    }
  }
  // If verbose, prompt result
  if verbose {
    println!("{} -> {:?} -> {}", group.replace('\n', ", "), result, count);
  }
  // Return result
  return (result, count);
}
