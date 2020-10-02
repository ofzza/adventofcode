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
  ExecutionProducedOutput(i64),
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
        let param_mode: Vec<i64> = vec![
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
            let input1 = self.read_from_memory(input1_address);            
            let input2_address = self.get_argument_address(1, param_mode[1]);
            let input2 = self.read_from_memory(input2_address);
            // Calculate and store output
            let output_address = self.get_argument_address(2, param_mode[2]);
            let output = input1 + input2;
            self.write_to_memory(output_address as usize, output);
            // Move instruction pointer
            self._ip += 4;
            // If verbose, output operation description
            if verbose {
              println!(
                "ADD  [&{}:{}] + [&{}:{}] = [&{};{}]",
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
            let input1 = self.read_from_memory(input1_address);            
            let input2_address = self.get_argument_address(1, param_mode[1]);
            let input2 = self.read_from_memory(input2_address);
            // Calculate and store output
            let output_address = self.get_argument_address(2, param_mode[2]);
            let output = input1 * input2;
            self.write_to_memory(output_address as usize, output);
            // Move instruction pointer
            self._ip += 4;
            // If verbose, output operation description
            if verbose {
              println!(
                "MULT [&{}:{}] x [&{}:{}] = [&{};{}]",
                input1_address, input1,
                input2_address, input2,
                output_address, output
              );
            }
            // Return execution done
            OpCodeResult::ExecutedWithoutOutput
          },

          // Input to memory input opcode
          3 => {
            // Check if input is available
            match self.input {
              // Store input value
              Some(input) => {
                // Clear input
                self.input = None;
                // Write input to memory
                let input_write_address = self.get_argument_address(0, param_mode[0]);
                self.write_to_memory(input_write_address, input);
                // Move instruction pointer
                self._ip += 2;
                // If verbose, output operation description
                if verbose {
                  println!("IN   [input {} => &({})]", input, input_write_address);
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

          // Output from memory opcode
          4 => {
            // Read output value
            let output_read_address = self.get_argument_address(0, param_mode[0]);
            let output = self.read_from_memory(output_read_address);
            // Store output value
            self.output = Some(output);
            // Move instruction pointer
            self._ip += 2;
            // If verbose, output operation description
            if verbose {
              println!("OUT  [output &({}) => {}]", output_read_address, output);
            }
            // Return output value
            OpCodeResult::ExecutionProducedOutput(output)
          },

          // Jump if True opcode
          5 => {
            // Read condition value
            let condition_address = self.get_argument_address(0, param_mode[0]);
            let condition_value = self.read_from_memory(condition_address);
            // Read conditional pointer value
            let ip_address = self.get_argument_address(1, param_mode[1]);
            let ip_value = self.read_from_memory(ip_address);
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
                "JMP  [&{}:{}] ---[{}]---> [&{};{}]",
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
            let condition_value = self.read_from_memory(condition_address);
            // Read conditional pointer value
            let ip_address = self.get_argument_address(1, param_mode[1]);
            let ip_value = self.read_from_memory(ip_address);
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
                "JMP  [&{}:{}] ---[{}]---> [&{};{}]",
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
            let input1 = self.read_from_memory(input1_address);            
            let input2_address = self.get_argument_address(1, param_mode[1]);
            let input2 = self.read_from_memory(input2_address);
            // Calculate and store output
            let output_address = self.get_argument_address(2, param_mode[2]);
            let output = if input1 < input2 { 1 } else { 0 };
            self.write_to_memory(output_address as usize, output);
            // Move instruction pointer
            self._ip += 4;
            // If verbose, output operation description
            if verbose {
              println!(
                "LT   [&{}:{}] < [&{}:{}] = [&{};{}]",
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
            let input1 = self.read_from_memory(input1_address);            
            let input2_address = self.get_argument_address(1, param_mode[1]);
            let input2 = self.read_from_memory(input2_address);
            // Calculate and store output
            let output_address = self.get_argument_address(2, param_mode[2]);
            let output = if input1 == input2 { 1 } else { 0 };
            self.write_to_memory(output_address as usize, output);
            // Move instruction pointer
            self._ip += 4;
            // If verbose, output operation description
            if verbose {
              println!(
                "EQ   [&{}:{}] == [&{}:{}] = [&{};{}]",
                input1_address, input1,
                input2_address, input2,
                output_address, output
              );
            }
            // Return execution done
            OpCodeResult::ExecutedWithoutOutput
          },

          // Update relative base opcode
          9  => {
            // Get inputs
            let input_address = self.get_argument_address(0, param_mode[0]);
            let input = self.read_from_memory(input_address);            
            // Update relative base
            self._relative_base += input as i64;
            // Move instruction pointer
            self._ip += 2;
            // If verbose, output operation description
            if verbose {
              println!(
                "REL  [&{}:{}]",
                input_address, input
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
  fn get_argument_address (&self, param_index: usize, param_mode: i64) -> usize {
    if param_mode == 0 {
      // Return argument offset value as argument address
      self.memory[(&self._ip + 1 + param_index) as usize] as usize
    } else if param_mode == 1 {
      // Return argument offset as direct argument address
      (&self._ip + 1 + param_index) as usize
    } else if param_mode == 2 {
      // Return relative value of the argument as argument address
      (self._relative_base as i64 + self.memory[(&self._ip + 1 + param_index) as usize] as i64) as usize
    } else {
      // Unknown param mode
      panic!("IntCode received unknown param mode: {}!", param_mode);
    }
  }

  /// Reads from memory address
  /// 
  /// # Arguments
  /// 
  /// - `address` - Memory address to read from
  fn read_from_memory (&self, address: usize) -> i64 {
    // Attempt reading form memory
    match self.memory.get(address) {
      // Return read value
      Some(value) => value,
      // Default to 0
      _ => &0
    }.clone()
  }

  /// Writes to memory address
  /// 
  /// # Arguments
  /// 
  /// - `address` - Memory address to write to
  /// - `value`   - Value to write to memory
  fn write_to_memory (&mut self, address: usize, value: i64) {    
    // Expand memory if needed
    if (address + 1) > self.memory.len() {
      for _ in 0..((address + 1) - self.memory.len()) {
        self.memory.push(0);
      }
    }
    // Write to memory
    self.memory[address] = value;
  }

}
