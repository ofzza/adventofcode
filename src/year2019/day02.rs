//! 2019/02 puzzle
//! 
//! https://adventofcode.com/2019/day/1
// -----------------------------------------------------------------------------

// Include dependencies
use super::super::lib::inputs::*;
use super::super::lib::puzzle::*;
use super::intcode::*;

/// Registers puzzles for the day
pub fn run (index: u32, key: &str, verbose: bool) {

  // Run 1st puzzle
  if (index == 0) || (index == 1) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
      create_vec(2019, 2, 1, "test", input, implementation1, |c| (c.memory[0], Some(3500)))
        .run(verbose);
      // Test
      let input = vec![1, 0, 0, 0, 99];
      create_vec(2019, 2, 1, "test", input, implementation1, |c| (c.memory[0], Some(2)))
        .run(verbose);
      // Test
      let input = vec![2, 3, 0, 3, 99];
      create_vec(2019, 2, 1, "test", input, implementation1, |c| (c.memory[3], Some(6)))
        .run(verbose);
      // Test
      let input = vec![2, 4, 4, 5, 99, 0];
      create_vec(2019, 2, 1, "test", input, implementation1, |c| (c.memory[5], Some(9801)))
        .run(verbose);
      // Test
      let input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
      create_vec(2019, 2, 1, "test", input, implementation1, |c| (c.memory[0], Some(30)))
        .run(verbose);
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let mut input = load_input::<i32>("./src/year2019/data/day02input.txt", ',');
      input[1] = 12;
      input[2] = 2;
      create_vec(2019, 2, 1, "solution", input, implementation1, |c| (c.memory[0], Some(3101878)))
        .run(verbose);
    }
  }

  // Run 2nd puzzle
  if (index == 0) || (index == 2) {
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = load_input::<i32>("./src/year2019/data/day02input.txt", ',');
      create_vec(2019, 2, 2, "solution", input, implementation2, |c| ((100 * c.memory[1] + c.memory[2]), Some(8444)))
        .run(false);
    }
  }
}

fn implementation1 (puzzle: &Puzzle<i32, IntCode, i32>, verbose: bool) -> Result<IntCode, &str> {
  // Initialize IntCode
  let mut computer = IntCode{
    memory: puzzle.input.value_vec.clone(),
    ..Default::default()
  };
  // Run IntCode
  loop {
    match computer.next(verbose) {
      Some(_) => continue,
      None => {
        if computer.flag_err != true {
          // Return as done
          return Ok(computer);
        } else {
          // Execution error
          return Err("IntCode exited with error!");
        }
      }
    }
  }
}

fn implementation2 (puzzle: &Puzzle<i32, IntCode, i32>, verbose: bool) -> Result<IntCode, &str> {
  // 
  for i in 0..99 {
    for j in 0..99 {
      // Update memory
      let mut memory = puzzle.input.value_vec.clone();
      memory[1] = i;
      memory[2] = j;
      // Initialize IntCode
      let mut computer = IntCode{
        memory,
        ..Default::default()
      };
      // Run IntCode
      loop {
        match computer.next(verbose) {
          Some(_) => continue,
          None => if computer.flag_err != true {
            if computer.memory[0] == 19690720 {
              // Return as done
              return Ok(computer);
            } else {
              // Execution done - look for next candidate
              break;
            }
          } else {
            // Execution error - look for next candidate
            break;
          }
        }
      }
    }
  }
  // Return no matches found error
  return Err("Execution finished with no matches found!");
}
