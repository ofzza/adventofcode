//! 2019/04 puzzle
//! 
//! https://adventofcode.com/2019/day/4
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;

/// Registers puzzles for the day
pub fn run (index: u32, key: &str, _verbose: bool, obfuscate: bool) -> PuzzleExecutionStatistics {

  // Initialize stats
  let mut stats = PuzzleExecutionStatistics{
    ..Default::default()
  };
  
  // Run puzzle
  if (index == 0) || (index == 1) {
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = PuzzleInput::Params(vec![128392, 643281]);
      stats.update(
        Puzzle::new(2019, 4, 1, "solution", input, implementation1, |d| (d, Some(2050)))
          .run(false, obfuscate)
      );
    }
  }

  // Run puzzle
  if (index == 0) || (index == 2) {
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = PuzzleInput::Params(vec![128392, 643281]);
      stats.update(
        Puzzle::new(2019, 4, 2, "solution", input, implementation2, |d| (d, Some(1390)))
          .run(false, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<u32, u32, u32>, _verbose: bool) -> Result<u32, &str> {
  match &puzzle.input {
    PuzzleInput::Params(limits) => {
      // Initialize
      let mut n: u32 = limits[0];
      let mut count: u32 = 0;
      // Find all numbers in scope matching conditions
      loop {
        // Get next number matching confitions
        n = compose_equal_or_larger(n, false);
        // Count if not outside interval
        if n <= limits[1] { count += 1; } else { break; }
        // Next candidate
        n += 1;
      }
      // Return count
      return Ok(count);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<u32, u32, u32>, _verbose: bool) -> Result<u32, &str> {
  match &puzzle.input {
    PuzzleInput::Params(limits) => {
      // Initialize
      let mut n: u32 = limits[0];
      let mut count: u32 = 0;
      // Find all numbers in scope matching conditions
      loop {
        // Get next number matching confitions
        n = compose_equal_or_larger(n, true);
        // Count if not outside interval
        if n <= limits[1] {
          count += 1;
        } else {
          break;
        }
        // Next candidate
        n += 1;
      }
      // Return count
      return Ok(count);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

fn compose_equal_or_larger (n: u32, exact_doubles: bool) -> u32 {
  // Disassemble number
  let mut disassembled = disassemble(n);
  // Find next candidate matching criteria
  disassembled = compose_equal_or_larger_inner(disassembled, exact_doubles);
  // Reassemble number
  reassemble(disassembled)
}

fn compose_equal_or_larger_inner (mut disassembled: Vec<u32>, exact_doubles: bool) -> Vec<u32> {
  // Find next candidate matching criteria
  let mut has_doubles = false;
  for i in (0..(disassembled.len() - 1)).rev() {
    // Check if next digit equal or greater than previous
    if disassembled[i] < disassembled[i + 1] {
      // Replace digit
      disassembled[i] = disassembled[i + 1];
      // Replace other digits
      for j in 0..i { disassembled[j] = 0; }
    }
    // Check if duplicate digit
    if disassembled[i + 1] == disassembled[i] {
      has_doubles = true;
    }
  }
  // Make sure candidate has double digits
  if !has_doubles {
    disassembled[1] = disassembled[0];
  }
  // Make sure candidate has exactly double digits
  if exact_doubles {
    // Check for exact doubles
    for j in 0..(disassembled.len() - 1) {
      if (disassembled[j] == disassembled[j + 1])
      && ((j == 0) || (disassembled[j] != disassembled[j - 1]))
      && ((j == (disassembled.len() - 2)) || (disassembled[j] != disassembled[j + 2])) {
        // Return candidate
        return disassembled;
      }
    }
    // Find next candidate
    return compose_equal_or_larger_inner(disassemble(reassemble(disassembled) + 1), exact_doubles);
  } else {
    // Return candidate
    return disassembled;
  }
}

fn disassemble (mut n: u32) -> Vec<u32> {
  let mut disassembled: Vec<u32> = Vec::new();
  while n > 0 {
    disassembled.push((n % 10) as u32);
    n = n / 10;
  }
  disassembled
}

fn reassemble (disassembled: Vec<u32>) -> u32 {
  let mut n = 0;
  for (i, d) in disassembled.iter().enumerate() {
    n += d * u32::pow(10, i as u32);
  }
  n
}
