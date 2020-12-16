//! 2020/14 puzzle
//! 
//! https://adventofcode.com/2020/day/14
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::inputs::*;
use crate::lib::puzzle::*;
use std::collections::HashMap;
use super::bitmask::*;

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
        String::from("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
        String::from("mem[8] = 11"),
        String::from("mem[7] = 101"),
        String::from("mem[8] = 0")
      ]);
      stats.update(
        Puzzle::new(2020, 14, 1, "test", input, implementation1, |r| (r, Some(165)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d(load_input("./src/year2020/data/day14input.txt"), "\n");
      stats.update(
        Puzzle::new(2020, 14, 1, "solution", input, implementation1, |r| (r, Some(14553106347726)))
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
        String::from("mask = 000000000000000000000000000000X1001X"),
        String::from("mem[42] = 100"),
        String::from("mask = 00000000000000000000000000000000X0XX"),
        String::from("mem[26] = 1")
      ]);
      stats.update(
        Puzzle::new(2020, 14, 2, "test", input, implementation2, |r| (r, Some(208)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d(load_input("./src/year2020/data/day14input.txt"), "\n");
      stats.update(
        Puzzle::new(2020, 14, 2, "solution", input, implementation2, |r| (r, Some(2737766154126)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<String, usize, usize>, _verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(lines) => {
      // Parse input
      let instructions = &parse_input(&lines);
      // Initialize memory
      let mut memory: HashMap<usize, usize> = HashMap::new();
      // Initialize mask
      let mut mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".as_bytes().to_vec();
      // Execute instructions
      for instruction in instructions {
        match &instruction.0[..] {
          "mask" => {
            // Update mask
            mask = instruction.2.as_bytes().to_vec();
          },
          "mem" => {
            // Store masked value to memory
            let value = instruction.2.parse::<usize>().expect("Failed parsing mem instruction assignment value!");
            let value = format!("{:b}", value);
            let value = format!("{:0>36}", value);
            let masked_value = BitMask::mask(&value.as_bytes().to_vec(), &mask, 'X');
            memory.insert(instruction.1, BitMask::evaluate(masked_value));
          },
          _ => ()
        };
      }
      // Calculate checksum
      let mut checksum: usize = 0;
      for value in memory.values() {
        checksum += value;
      }
      // Return initialized memory checksum
      return Ok(checksum);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<String, usize, usize>, _verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(lines) => {
      // Parse input
      let instructions = &parse_input(&lines);
      // Initialize memory
      let mut memory: HashMap<usize, usize> = HashMap::new();
      // Initialize mask
      let mut mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".as_bytes().to_vec();
      // Execute instructions
      for instruction in instructions {
        match &instruction.0[..] {
          "mask" => {
            // Update mask
            mask = instruction.2.as_bytes().to_vec()
          },
          "mem" => {
            // Apply mask to value
            let value = instruction.2.parse::<usize>().expect("Failed parsing mem instruction assignment value!");
            // Apply mask to address and generate all possible addresses
            let address = format!("{:b}", instruction.1);
            let address = format!("{:0>36}", address);
            let masked_address = BitMask::mask(&address.as_bytes().to_vec(), &mask, '0');
            // Store permutated addresses to memory
            let addresses = BitMask::permutate_mask(masked_address, 'X');
            for address in addresses {
              let address = BitMask::evaluate(address);
              memory.insert(address, value);
            }
          },
          _ => ()
        };
      }
      // Calculate checksum
      let mut checksum: usize = 0;
      for value in memory.values() {
        checksum += value;
      }
      // Return initialized memory checksum
      return Ok(checksum);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

/// Parses input into instructions
/// 
/// # Arguments
/// * `lines` - Input lines to parse
fn parse_input (lines: &Vec<String>) -> Vec<(String, usize, String)> {
  // Initialize result
  let mut result: Vec<(String, usize, String)> = vec![];
  // Parse instruction lines
  for line in lines {
    let parsed_line: Vec<&str> = line.trim().split('=').collect();
    let value: &str = &format!("{:0>36}", parsed_line[1].trim())[..];
    let instruction: &str = parsed_line[0].trim();
    let instruction: String = instruction.replace("]", "");
    let instruction_parsed: Vec<&str> = instruction.split('[').collect();
    result.push((
      String::from(instruction_parsed[0].trim()),
      if instruction_parsed.len() > 1 { instruction_parsed[1].parse::<usize>().expect("Failed parsing instruction argument!") } else { 0 },
      String::from(value)
    ));
  }
  // Return result
  return result;
}
