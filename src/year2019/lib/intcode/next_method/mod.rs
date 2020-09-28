//! IntCode .next() implementation
//! 
//! Implements a generator pattern working on top of the IntCode instance performing
//! the actual processing of the implemented Touring machine
// -----------------------------------------------------------------------------

// Include dependencies
use super::*;

/// IntCode .next() implementation
/// 
/// Implements a generator pattern working on top of the IntCode instance performing
/// the actual processing of the implemented Touring machine
impl IntCode {
  
  /// Executes next Touring machine step and returns that step's output value after storing it
  /// 
  /// # Arguments
  /// 
  /// * `verbose` - Outputs executing output of the puzzle to the console
  /// 
  /// # Remarks
  /// 
  /// The machine supports following opcodes:
  /// 
  /// - `1`  - Addition by reference:       `*(IP + 1) + *(IP + 2) -> *(IP + 3); IP += 3;`
  /// - `2`  - Multiplication by reference: `*(IP + 1) x *(IP + 2) -> *(IP + 3); IP += 3;`
  /// - `99` - End execution
  pub fn next (&mut self, verbose: bool) -> Option<i32> {
    // Decode instruction (Check if out of bounds)
    let result: Option<i32> = match self.memory.get(self._ip) {
      // Opcode decoded
      Some(opcode) => {

        // Execute opcode
        match opcode {

          // Addition by reference opcode
          1  => {
            // Get inputs
            let input1_address = self.memory[(self._ip + 1) as usize] as usize;
            let input1 = self.memory[input1_address];
            let input2_address = self.memory[(self._ip + 2) as usize] as usize;
            let input2 = self.memory[input2_address];
            // Calculate and store output
            let output_address = self.memory[(self._ip + 3) as usize] as usize;
            self.output = input1 + input2;
            self.memory[output_address] = self.output;
            // Move instruction pointer
            self._ip += 4;
            // If verbose, output calculation
            if verbose == true {
              println!("[&{} => {}] + [&{} => {}] = {} => &{}", input1_address, input1, input2_address, input2, self.output, output_address);
            }
            // Return calculated value
            Some(self.output)
          },
    
          // Multiplication by reference opcode
          2  => {
            // Get inputs
            let input1_address = self.memory[(self._ip + 1) as usize] as usize;
            let input1 = self.memory[input1_address];
            let input2_address = self.memory[(self._ip + 2) as usize] as usize;
            let input2 = self.memory[input2_address];
            // Calculate and store output
            let output_address = self.memory[(self._ip + 3) as usize] as usize;
            self.output = input1 * input2;
            self.memory[output_address] = self.output;
            // Move instruction pointer
            self._ip += 4;
            // If verbose, output calculation
            if verbose == true {
              println!("[&{} => {}] x [&{} => {}] = {} => &{}", input1_address, input1, input2_address, input2, self.output, output_address);
            }
            // Return calculated value
            Some(self.output)
          },
    
          // End execution opcode
          99 => {
            // Set no error flag
            self.flag_err = false;
            // If verbose, output calculation
            if verbose == true {
              println!("END!");
            }
            // Return no value
            None
          }
    
          // Unknown opcode
          _  => {
            // Set error flag
            self.flag_err = true;
            // Return no value
            None
          }
    
        }
    
      },
      // Instruction pointer out of bounds
      _ => {
        // Set error flag
        self.flag_err = true;
        // Return no value
        None
      }
    };
    
    return result;
  }

}
