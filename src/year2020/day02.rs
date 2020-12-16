//! 2020/02 puzzle
//! 
//! https://adventofcode.com/2020/day/2
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
      let input = PuzzleInput::Vector1D(vec![String::from("1-3 a: abcde"), String::from("1-3 b: cdefg"), String::from("2-9 c: ccccccccc")]);
      stats.update(
        Puzzle::new(2020, 2, 1, "test", input, implementation1, |r| (r, Some(2)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<String>(load_input("./src/year2020/data/day02input.txt"), "\n");
      stats.update(
        Puzzle::new(2020, 2, 1, "solution", input, implementation1, |r| (r, Some(591)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Run puzzle
  if (index == 0) || (index == 2) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector1D(vec![String::from("1-3 a: abcde"), String::from("1-3 b: cdefg"), String::from("2-9 c: ccccccccc")]);
      stats.update(
        Puzzle::new(2020, 2, 2, "test", input, implementation2, |r| (r, Some(1)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<String>(load_input("./src/year2020/data/day02input.txt"), "\n");
      stats.update(
        Puzzle::new(2020, 2, 2, "solution", input, implementation2, |r| (r, Some(335)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<String, u32, u32>, _verbose: bool) -> Result<u32, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(lines) => {
      // Count valida passwords
      let mut valid_passwords_count: u32 = 0;
      for line in lines {

        // Parse line
        match parse_input(line) {
          Some(parsed) => {

            // Initialize char count
            let mut password_char_count: usize = 0;
            // Count policy characted in password            
            for c in parsed.3.as_bytes() {
              if (*c as char) == parsed.2 {
                password_char_count += 1;
              }
            }
            // Check password matching policy's character count
            if (password_char_count >= parsed.0) && (password_char_count <= parsed.1) {
              valid_passwords_count += 1;
            }
          },
          None => {}
        };

      }

      // Return valid passwords count
      return Ok(valid_passwords_count);

    },
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<String, u32, u32>, _verbose: bool) -> Result<u32, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(lines) => {
      // Count valida passwords
      let mut valid_passwords_count: u32 = 0;
      for line in lines {

        // Parse line
        match parse_input(line) {
          Some(parsed) => {

            // Check password matching policy
            if (parsed.3.as_bytes()[parsed.0 - 1] as char == parsed.2) ^ (parsed.3.as_bytes()[parsed.1 - 1] as char == parsed.2) {
              valid_passwords_count += 1;
            }
          
          },
          None => {}
        };

      }

      Ok(valid_passwords_count)
    },
    _ => panic!("This shouldn't ever happen!")

  }
}

/// Parses a line of input and retuns parsed values
/// 
/// # Arguments
///
/// * `line` - Line of input to parse
fn parse_input (line: &str) -> Option<(usize, usize, char, &str)> {
  // Parse line
  let parsed_line: Vec<&str> = line.split(':').collect();
  if parsed_line.len() != 2 { return None; }
  
  // Parse policy
  let parsed_policy: Vec<&str> = parsed_line[0].trim().split(' ').collect();
  if parsed_policy.len() != 2 { return None; }        
  // Parse policy character
  let policy_char: char = parsed_policy[1].trim().as_bytes()[0] as char;
  // Parse policy bounds
  let parsed_values: Vec<&str> = parsed_policy[0].trim().split('-').collect();
  if parsed_values.len() != 2 { return None; }
  let policy_value_a: usize = match parsed_values[0].trim().parse::<usize>() { Ok(n) => n, Err(_) => { return None; } };
  let policy_value_b: usize = match parsed_values[1].trim().parse::<usize>() { Ok(n) => n, Err(_) => { return None; } };

  // Parse password
  let password = parsed_line[1].trim();

  // Return parsed result
  return Some((policy_value_a, policy_value_b, policy_char, password));
}
