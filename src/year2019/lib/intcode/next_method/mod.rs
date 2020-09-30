//! IntCode .next() implementation
//! 
//! Implements a generator pattern working on top of the IntCode instance performing
//! the actual processing of the implemented Touring machine
// -----------------------------------------------------------------------------

// Include dependencies
use super::*;

/// OpCode execution events
/// 
/// # Events
/// 
/// - `OpCode::ExecutedWithoutOutput`        - Opcode executed with no output
/// - `OpCode::ExecutionProducedOutput(i32)` - Execution of OpCode produced output
/// - `OpCode::ExecutionRequestedInput`      - Execution of next OpCode is requesting input to be set
/// - `OpCode::End`                          - Execution end reached
/// - `OpCode::Error(String)`                - Execution error
pub enum OpCodeResult {
  ExecutedWithoutOutput,
  ExecutionProducedOutput(i32),
  ExecutionRequestedInput,
  End,
  Error(String)
}

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
  pub fn next (&mut self, verbose: bool) -> OpCodeResult {
    // Decode instruction (Check if out of bounds)
    match self.memory.get(self._ip) {

      // Opcode decoded
      Some(instruction_code) => {

        // Parse instruction code into opcode and parameter modes
        let opcode = instruction_code % 100;
        let param_modes = instruction_code / 100;
        let param_mode: Vec<i32> = vec![
          (param_modes / 1)   % 10,
          (param_modes / 10)  % 10,
          (param_modes / 100) % 10,
        ];
        // If verbose, output operation description
        if verbose {
          print!("&({}) => {}<{}>: ", self._ip, opcode, param_modes);
        }

        // Execute opcode
        match opcode {

          // Addition opcode
          1  => {
            // Get inputs
            let input1_address = self.get_argument_address(0, param_mode[0]);
            let input1 = self.memory[input1_address];            
            let input2_address = self.get_argument_address(1, param_mode[1]);
            let input2 = self.memory[input2_address];
            // Calculate and store output
            let output_address = self.get_argument_address(2, param_mode[2]);
            let output = input1 + input2;
            self.memory[output_address as usize] = output;
            // Move instruction pointer
            self._ip += 4;
            // If verbose, output operation description
            if verbose {
              println!(
                "[&{}:{}] + [&{}:{}] = [&{};{}]",
                input1_address, input1,
                input2_address, input2,
                output_address, output
              );
            }
            // Return execution done
            OpCodeResult::ExecutedWithoutOutput
          },
    
          // Multiplication opcode
          2  => {
            // Get inputs
            let input1_address = self.get_argument_address(0, param_mode[0]);
            let input1 = self.memory[input1_address];            
            let input2_address = self.get_argument_address(1, param_mode[1]);
            let input2 = self.memory[input2_address];
            // Calculate and store output
            let output_address = self.get_argument_address(2, param_mode[2]);
            let output = input1 * input2;
            self.memory[output_address as usize] = output;
            // Move instruction pointer
            self._ip += 4;
            // If verbose, output operation description
            if verbose {
              println!(
                "[&{}:{}] x [&{}:{}] = [&{};{}]",
                input1_address, input1,
                input2_address, input2,
                output_address, output
              );
            }
            // Return execution done
            OpCodeResult::ExecutedWithoutOutput
          },

          // Read input opcode
          3 => {
            // Check if input is available
            match self.input {
              // Store input value
              Some(input) => {
                // Write input to memory
                let input_write_address = self.get_argument_address(0, param_mode[0]);
                self.memory[input_write_address] = input;
                // Move instruction pointer
                self._ip += 2;
                // If verbose, output operation description
                if verbose {
                  println!("[input {} => &({})]", input, input_write_address);
                }
                // Return execution done
                OpCodeResult::ExecutedWithoutOutput
              },
              // Request input value
              None => {
                // If verbose, output operation description
                if verbose {
                  println!("awaiting input ...");
                }
                // Return awaiting input
                OpCodeResult::ExecutionRequestedInput
              }
            }
          },

          // Write output opcode
          4 => {
            // Read output value
            let output_read_address = self.get_argument_address(0, param_mode[0]);
            let output = self.memory[output_read_address];
            // Store output value
            self.output = Some(output);
            // Move instruction pointer
            self._ip += 2;
            // If verbose, output operation description
            if verbose {
              println!("[output &({}) => {}]", output_read_address, output);
            }
            // Return output value
            OpCodeResult::ExecutionProducedOutput(output)
          },

          // Jump if True opcode
          5 => {
            // Read condition value
            let condition_address = self.get_argument_address(0, param_mode[0]);
            let condition_value = self.memory[condition_address];
            // Read conditional pointer value
            let ip_address = self.get_argument_address(1, param_mode[1]);
            let ip_value = self.memory[ip_address];
            // Check jump condition
            if condition_value != 0 {
              // Jump; update instruction pointer
              self._ip = ip_value as usize;
            } else {
              // Move instruction pointer
              self._ip += 3;
            }
            // If verbose, output operation description
            if verbose {
              println!(
                "[&{}:{}] ---[{}]---> [&{};{}]",
                condition_address, condition_value,
                if condition_value != 0 { "true" } else { "false" },
                ip_address, ip_value
              );
            }
            // Return execution done
            OpCodeResult::ExecutedWithoutOutput
          },

          // Jump if False opcode
          6 => {
            // Read condition value
            let condition_address = self.get_argument_address(0, param_mode[0]);
            let condition_value = self.memory[condition_address];
            // Read conditional pointer value
            let ip_address = self.get_argument_address(1, param_mode[1]);
            let ip_value = self.memory[ip_address];
            // Check jump condition
            if condition_value == 0 {
              // Jump; update instruction pointer
              self._ip = ip_value as usize;
            } else {
              // Move instruction pointer
              self._ip += 3;
            }
            // If verbose, output operation description
            if verbose {
              println!(
                "[&{}:{}] ---[{}]---> [&{};{}]",
                condition_address, condition_value,
                if condition_value == 0 { "true" } else { "false" },
                ip_address, ip_value
              );
            }
            // Return execution done
            OpCodeResult::ExecutedWithoutOutput
          },

          // Less-then opcode
          7  => {
            // Get inputs
            let input1_address = self.get_argument_address(0, param_mode[0]);
            let input1 = self.memory[input1_address];            
            let input2_address = self.get_argument_address(1, param_mode[1]);
            let input2 = self.memory[input2_address];
            // Calculate and store output
            let output_address = self.get_argument_address(2, param_mode[2]);
            let output = if input1 < input2 { 1 } else { 0 };
            self.memory[output_address as usize] = output;
            // Move instruction pointer
            self._ip += 4;
            // If verbose, output operation description
            if verbose {
              println!(
                "[&{}:{}] < [&{}:{}] = [&{};{}]",
                input1_address, input1,
                input2_address, input2,
                output_address, output
              );
            }
            // Return execution done
            OpCodeResult::ExecutedWithoutOutput
          },

          // Equal-to opcode
          8  => {
            // Get inputs
            let input1_address = self.get_argument_address(0, param_mode[0]);
            let input1 = self.memory[input1_address];            
            let input2_address = self.get_argument_address(1, param_mode[1]);
            let input2 = self.memory[input2_address];
            // Calculate and store output
            let output_address = self.get_argument_address(2, param_mode[2]);
            let output = if input1 == input2 { 1 } else { 0 };
            self.memory[output_address as usize] = output;
            // Move instruction pointer
            self._ip += 4;
            // If verbose, output operation description
            if verbose {
              println!(
                "[&{}:{}] == [&{}:{}] = [&{};{}]",
                input1_address, input1,
                input2_address, input2,
                output_address, output
              );
            }
            // Return execution done
            OpCodeResult::ExecutedWithoutOutput
          },

          // End execution opcode
          99 => {
            // Set no error flag
            self.flag_err = false;
            // If verbose, output calculation
            if verbose {
              println!("END!");
            }
            // Return no value
            OpCodeResult::End
          }
    
          // Unknown opcode
          _  => {
            // Set error flag
            self.flag_err = true;
            // Return unknown OpCode error
            OpCodeResult::Error(format!("Unknown opcode: {}", opcode))
          }
    
        }
    
      },

      // Instruction pointer out of bounds
      _ => {
        // Set error flag
        self.flag_err = true;
        // Return failed reading from memory
        OpCodeResult::Error(format!("Unreachable memory location: {}", self._ip))
      }

    }
  }

  /// Gets argument adress for any parameter mode
  /// 
  /// # Arguments
  /// 
  /// - `param_index` - Index of the instruction parameter being referenced
  /// - `param_mode`  - Parameter fecth mode (1: direct, 0: indirected)
  fn get_argument_address (&self, param_index: usize, param_mode: i32) -> usize {
    if param_mode == 1 {
      // Return argument offset as direct argument address
      (&self._ip + 1 + param_index) as usize
    } else {
      // Return argument offset value as argument address
      self.memory[(&self._ip + 1 + param_index) as usize] as usize
    }
  }

}
