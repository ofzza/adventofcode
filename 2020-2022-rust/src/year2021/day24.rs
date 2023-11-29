//! 2021 day 24 puzzle
//! 
//! https://adventofcode.com/2021/day/24
// -----------------------------------------------------------------------------

// Code analisys:
// 
// 00 // inp w    //                                                                          // w = {input} = input()
// 01 // mul x 0  //                                                                          //
// 02 // add x z  //     +    +    +    +    +    -    +    -    -    +    -    -    -    -   //
// 03 // mod x 26 //                                                                          // x = z % 26
// 04 // div z N1 // e{ +1,  +1,  +1,  +1,  +1, +26,  +1, +26, +26,  +1, +26, +26, +26, +26}  // z = z / N1
// 05 // add x N2 // e{+13, +11, +12, +10, +14,  -1, +14, -26,  -8, +12, -16, -13,  -6,  -6}  //
// 06 // eql x w  //     .    .    .    .    .    .    .    .    .    .    .    .    .    .   //
// 07 // eql x 0  //     .    .    .    .    .    .    .    .    .    .    .    .    .    .   //
// 08 // mul y 0  //     .    .    .    .    .    .    .    .    .    .    .    .    .    .   //
// 09 // add y 25 //     .    .    .    .    .    .    .    .    .    .    .    .    .    .   //
// 10 // mul y x  //     .    .    .    .    .    .    .    .    .    .    .    .    .    .   //
// 11 // add y 1  //     .    .    .    .    .    .    .    .    .    .    .    .    .    .   //
// 12 // mul z y  //     .    .    .    .    .    .    .    .    .    .    .    .    .    .   //
// 13 // mul y 0  //     .    .    .    .    .    .    .    .    .    .    .    .    .    .   //
// 14 // add y w  //     .    .    .    .    .    .    .    .    .    .    .    .    .    .   //
// 15 // add y N3 // e{ +6, +11,  +5,  +6,  +8, +14,  +4,  +9,  +7, +13, +11, +11,  +6,  +1}  //
// 16 // mul y x  //                                                                          //
// 17 // add z y  //                                                                          // z = if (x + N2) !== {input} { (z * 26) + {input} + N3 } else { z }
//
// Code equivalent:
//
// z(n) = if (z(n-1) % 26) + N1 !== input(n) {
//   (z(n-1) / N2 * 26) + {input} + N3
// } else {
//   z(n-1)
// }

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2021::lib::alu::*;

/// Parses input data
fn parse(data: &String) -> Vec<Vec<String>> {
  Input::parse(data.trim(), "\n", |line| {
    Input::parse(line.trim(), " ", |symbol| symbol.to_string())
  })
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 24,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Find first valid serial number (in descending order)
      let serial = find_first_serial(&data, true);
      let serial_string = serial.iter().map(|n| n.to_string()).collect::<Vec<String>>().join("");

      // Calculate and return result
      String::from(format!("{}", serial_string))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 24,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Find first valid serial number (in descending order)
      let serial = find_first_serial(&data, false);
      let serial_string = serial.iter().map(|n| n.to_string()).collect::<Vec<String>>().join("");

      // Calculate and return result
      String::from(format!("{}", serial_string))
    }

  );

  // Return registry ownership
  registry
}

/// Finds and returns first valid 14 digit serial number
/// 
/// # Arguments
/// * data:       Input data, ALU program to validate serial numbers
/// * high_first: If serials should be checked lowest-to-highest, or highest-to-lowest
/// 
/// # Returns
/// First valid 14 digit serial number
fn find_first_serial (data: &Vec<Vec<String>>, high_first: bool) -> Vec<usize> {
  let all_serials = find_serials(data, high_first, |serial| serial.len() != 14);
  all_serials[0].clone()
}

/// Finds all valid 14 digit serial numbers
/// 
/// # Arguments
/// * data:       Input data, ALU program to validate serial numbers
/// * high_first: If serials should be checked lowest-to-highest, or highest-to-lowest
/// * callback:   Callback function that is executed every time a valid serial number is found. If the callback retuns false,
///               finding further serial numbers is stopped.
/// 
/// # Returns
/// All valid 14 digit serial numbers, or numbers found up to the point of execution being stopped by the callback function
fn find_serials (data: &Vec<Vec<String>>, high_first: bool, callback: fn(serial: &Vec<usize>) -> bool) -> Vec<Vec<usize>> {
  // Initialize the ALU
  let mut alu = ALU::new('w'..('w' as u8 + 4) as char);
  alu.load(data);
  
  // Search for valid numbers
  let distances = [5, 1, 0, 1, 0, 0, 0];
  let mut step = 0;
  let mut all_optimized_inputs: Vec<Vec<usize>> = vec![];
  loop {

    // Copy optimized imputs of given length and clear it for next length
    let all_optimized_inputs_previous = if all_optimized_inputs.len() > 0 { all_optimized_inputs.clone() } else { vec![vec![]] };
    let all_optimized_inputs_len = if all_optimized_inputs.len() > 0 { all_optimized_inputs[0].len() } else { 0 };
    all_optimized_inputs.clear();

    // Choose one of optimized inputs and try building on top 
    for i in 0..all_optimized_inputs_previous.len() {
      let optimized_inputs: Vec<usize> = all_optimized_inputs_previous[i].clone();

      // Try different inputs in hope of finding a good combination
      let range = if optimized_inputs.len() >= 12 || distances[step] > 0 { 1..10 } else { 0..1 };
      for j in if high_first { range.rev().collect::<Vec<usize>>() } else { range.collect::<Vec<usize>>() } {
        let range = if optimized_inputs.len() >= 12 || distances[step] > 1 { 1..10 } else { 0..1 };
        for k in if high_first { range.rev().collect::<Vec<usize>>() } else { range.collect::<Vec<usize>>() } {
          let range = if distances[step] > 2 { 1..10 } else { 0..1 };
          for l in if high_first { range.rev().collect::<Vec<usize>>() } else { range.collect::<Vec<usize>>() } {
            let range = if distances[step] > 3 { 1..10 } else { 0..1 };
            for m in if high_first { range.rev().collect::<Vec<usize>>() } else { range.collect::<Vec<usize>>() } {
              let range = if distances[step] > 4 { 1..10 } else { 0..1 };
              for n in if high_first { range.rev().collect::<Vec<usize>>() } else { range.collect::<Vec<usize>>() } {

                // Initialize inputs
                let mut inputs: Vec<usize> = optimized_inputs.clone();
                let real_inputs_count = optimized_inputs.len() + if optimized_inputs.len() >= 12 { 2 } else { distances[step] };
                // Set inputs being tested
                inputs.push(j);
                inputs.push(k);
                inputs.push(l);
                inputs.push(m);
                inputs.push(n);
                // Set another input just to get into one more executino cycle based off of previous inputs
                inputs.push(0);
                
                // Execute program and monitor relevant instructions
                let result = alu.execute(&inputs, |ec: usize, instruction: &Vec<String>, alu: &ALU, ic: usize, inputs: &Vec<usize>| {
                  
                  // If insufficient inputs to hit execution end, monitor values after instruction #7 and check if x==0
                  if real_inputs_count < 14 && ec % 18 == 5 {
                    // Parse intruction and get relevant register values
                    let x = alu.register_values[1];
                    let d = instruction[2].parse::<isize>().unwrap();

                    // If a step can't be zerroed because positive value is being added, continue
                    if d > 0 { return None; }

                    // If condition can be collapsed to x==0, add any not yet known inputs and start over from there ...
                    let can_be_zerroed = x + d < 10 && x + d > 0;
                    if can_be_zerroed && optimized_inputs.len() < ic {
                      // Keep soution and break execution to store the solution
                      let mut inputs_to_keep: Vec<usize> = Vec::with_capacity(ic - 1);
                      for i in optimized_inputs.len()..(ic - 1) {
                        inputs_to_keep.push(inputs[i]);
                      }
                      inputs_to_keep.push((alu.register_values[1] + d) as usize);
                      // Prompt found solution
                      // println!("Found a solution to set x==0 for step #{} ({:?}): {:?} + {:?}", 1 + (ec / 18), instruction, optimized_inputs, inputs_to_keep);
                      // Return solution
                      return Some(inputs_to_keep);
                    }

                    // If condition is collapsed to x==0, and consumed inputs are already known ones, proceed
                    else if can_be_zerroed {
                      return None;
                    }

                    // If condition can not be collapsed to x==0, break and try another conbination of inputs
                    else {
                      return Some(vec![]);
                    }

                  }

                  // If sufficient inputs to hit execution end, monitor values after very last instruction to find return value = 0
                  else if real_inputs_count == 14 {
                    if ec == 251 && alu.register_values[1] == 0 && alu.register_values[3] == 0 {
                      // Keep soution and break execution to store the solution
                      let mut inputs_to_keep: Vec<usize> = Vec::with_capacity(ic - 1);
                      for i in optimized_inputs.len()..ic {
                        inputs_to_keep.push(inputs[i]);
                      }
                      // Prompt found solution
                      println!("Found a solution to get z==0 for step #{} ({:?}): {:?} + {:?}", 1 + (ec / 18), instruction, optimized_inputs, inputs_to_keep);
                      // Return solution
                      return Some(inputs_to_keep);
                    }
                  }

                  // Allow continued execution
                  None

                });

                // Check if execution was broken, and store solution if it was
                match result {
                  Some(inputs) => {
                    // Check if solution, or just breaking unviable execution
                    if inputs.len() > 0 {
                      // Store solution
                      let mut solution_inputs = optimized_inputs.clone();                            
                      for i in 0..inputs.len() {
                        solution_inputs.push(inputs[i]);
                      }
                      all_optimized_inputs.push(solution_inputs);
                      // Execute callback and break further execution if callback returns false
                      if !callback(&all_optimized_inputs[all_optimized_inputs.len() - 1]) {
                        return all_optimized_inputs;
                      }
                    }
                  },
                  None => ()
                }

              }
            }
          }
        }
      }
    }

    // Check if done or if spinning in place
    step += 1;
    if all_optimized_inputs.len() == 0 || all_optimized_inputs[0].len() >= 14 || all_optimized_inputs[0].len() == all_optimized_inputs_len {
      break;
    }
  }

  // Return all found valid codes
  all_optimized_inputs
}
