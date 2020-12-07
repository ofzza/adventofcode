//! Assembler intepreter .next() implementation
//! 
//! Implements a generator pattern working on top of the Assembler interpreter performing
//! the actual processing of the implemented interpreter
// -----------------------------------------------------------------------------

// Include dependencies
use super::*;

/// Instruction execution events
/// 
/// # Events
/// 
/// - `OpCode::Ok(u32)`       - Instruction executed at given instruction pointer
/// - `OpCode::Error(String)` - Execution error
pub enum InstructionResult {
  InstructionOk(usize, isize),
  InstructionError(String),
  LoopDetectedError(usize, isize),
  End(usize, isize)
}

/// Assembler intepreter .next() implementation
/// 
/// Implements a generator pattern working on top of the Assembler interpreter performing
/// the actual processing of the implemented interpreter
impl AssemblerInterpreter {
  
  /// Executes next Touring machine step and returns that step's output value after storing it
  /// 
  /// # Arguments
  /// 
  /// * `verbose` - Outputs executing output of the puzzle to the console
  /// 
  /// # Remarks
  /// 
  /// The machine supports following instructions:
  /// 
  /// - `acc` - Accumulator: Update accumulator by value
  /// - `jmp` - Jump: Jump to repative address
  /// - `nop` - No-Op: Do nothing
  pub fn next (&mut self, verbose: bool) -> InstructionResult {
    // Check if steped just over last instruction
    if self._ip == self.instructions.len() {
      // Return program finished execution
      return InstructionResult::End(self._ip, self._acc);
    }

              // Loop prevention (shallow implementation)
    if self._loop_prevention {
      // Check if instruction has previous state
      if self._instruction_last_state.contains_key(&self._ip) {
        // Check if previous instruction state matches current state
        let state = self._instruction_last_state.get(&self._ip).expect("Fetching previous instruction state failed!");
        if *state == true {
          return InstructionResult::LoopDetectedError(self._ip, self._acc)
        }
      }
    }

    // Decode instruction (Check if out of bounds)
    match self.instructions.get(self._ip) {

      // Opcode decoded
      Some(instruction_code) => {

        // Process instruction
        let instruction_name = &instruction_code.0;
        let instruction_arg = instruction_code.1.clone();
  
        // Prompt instruction
        if verbose {
          println!("{}[acc: {}] {} {}", self._ip, self._acc, instruction_name, instruction_arg);
        }

        // Execute opcode
        match &instruction_name[..] {

          // Addition opcode
          "acc" => {
            // Update accumulator
            self._acc += instruction_arg;

            // Update instruction state for purposes of loop prevention
            if self._loop_prevention {
              self._instruction_last_state.insert(self._ip, true);
            }

            // Update Instruction pointer
            self._ip += 1;

            // Return instruction executed ok
            return InstructionResult::InstructionOk(self._ip, self._acc);
          },
    
          // Multiplication opcode
          "jmp"  => {
            // Update instruction state for purposes of loop prevention
            if self._loop_prevention {
              self._instruction_last_state.insert(self._ip, true);
            }

            // Update Instruction pointer
            self._ip = ((self._ip as isize) + instruction_arg) as usize;

            // Return instruction executed ok
            return InstructionResult::InstructionOk(self._ip, self._acc);
          },

          // Input to memory input opcode
          "nop" => {

            // Update instruction state for purposes of loop prevention
            if self._loop_prevention {
              self._instruction_last_state.insert(self._ip, true);
            }

            // Update Instruction pointer
            self._ip += 1;

            // Return instruction executed ok
            return InstructionResult::InstructionOk(self._ip, self._acc);
          },

          // Unknown instruction
          _ => {
            return InstructionResult::InstructionError(format!("Instruction not recognized: {}", instruction_name));
          }
        }
      },

      // Instruction pointer out of bounds
      _ => {
        // Return failed reading from memory
        return InstructionResult::InstructionError(format!("Unreachable memory location: {}", self._ip));
      }
    }
  }

  /// Resets intepreter to initial state, allowing for re-execution
  pub fn reset (&mut self) {
    self._ip = 0;
    self._acc = 0;
    self._instruction_last_state = HashMap::new();
  }

}
