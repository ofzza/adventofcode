//! 2021 day 10 puzzle
//! 
//! https://adventofcode.com/2021/day/10
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;

/// Parses input data
fn parse(data: &String) -> Vec<Vec<char>> {
  Input::parse(data.trim(), "\n", |line| {
    Input::parse(line.trim(), "", |c| { c.chars().next().unwrap() })
  })
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 10,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Validaet all lines
      let mut score = 0;
      for i in 0..data.len() {
        match validate_line(&data[i]) {
          Err(c) => {
            if c == ')' { score += 3; }
            if c == ']' { score += 57; }
            if c == '}' { score += 1197; }
            if c == '>' { score += 25137; }
          },
          Ok(_) => ()
        }
      }

      // Calculate and return result
      String::from(format!("{:?}", score))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 10,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Complete all valid lines
      let mut scores: Vec<usize> = vec![];
      for i in 0..data.len() {
        match validate_line(&data[i]) {
          Ok(closing) => {
            let mut score = 0;
            for j in 0..closing.len() {
              score *= 5;
              if closing[j] == '(' { score += 1; }
              if closing[j] == '[' { score += 2; }
              if closing[j] == '{' { score += 3; }
              if closing[j] == '<' { score += 4; }
            }
            scores.push(score);
          },
          Err(_) => ()
        }
      }
      scores.sort();

      // Calculate and return result
      String::from(format!("{:?}", scores[(scores.len() as f64 / 2.0) as usize]))
    }

  );

  // Return registry ownership
  registry
}

/// Validates a line of bracketted syntax
/// 
/// # Arguments
/// * line: Line of syntax to validate
/// 
/// # Returns
/// A Result containing a completion of the line if the syntax was valid or a character where the syntax first became invalid
fn validate_line (line: &Vec<char>) -> Result<Vec<char>, char> {
  // Initialize opening stack
  let mut stack: Vec<char> = vec![];
  // Validate line
  for i in 0..line.len() {
    // Process opening brackets
    if line[i] == '(' || line[i] == '[' || line[i] == '{' || line[i] == '<' {
      stack.push(line[i]);
    }
    // Process closing brackets
    else if line[i] == ')' || line[i] == ']' || line[i] == '}' || line[i] == '>' {
      // CLose legal bracket on the stack
      let last = stack[stack.len() - 1];
      if (last == '(' && line[i] == ')') || (last == '[' && line[i] == ']') || (last == '{' && line[i] == '}') || (last == '<' && line[i] == '>') {
        // Close the bracket on the stack
        stack.pop();
      }
      // Handle illegal bracket
       else { return Err(line[i]) }
    }
  }
  // Complete and return completed line
  stack.reverse();
  Ok(stack)
}
