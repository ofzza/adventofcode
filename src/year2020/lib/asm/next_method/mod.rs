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
/// - `InstructionResult::InstructionError(String)`                                 - Execution error
/// - `InstructionResult::InstructionOk(usize, AssemblerInterpreterRegistries)`     - Instruction executed at given instruction pointer, with a registries state
/// - `InstructionResult::LoopDetectedError(usize, AssemblerInterpreterRegistries)` - Loop detected at given instruction pointer, with a registries state
/// - `InstructionResult::End(usize, AssemblerInterpreterRegistries)`               - Execution finished at given instruction pointer, with a registries state
pub enum InstructionResult {
  InstructionError(String),
  InstructionOk(usize, AssemblerInterpreterRegistries),
  LoopDetectedError(usize, AssemblerInterpreterRegistries),
  End(usize, AssemblerInterpreterRegistries)
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
      return InstructionResult::End(self._ip, self.registries.clone());
    }

              // Loop prevention (shallow implementation)
    if self._loop_prevention {
      // Check if instruction has previous state
      if self._loop_prevention_memory.contains_key(&self._ip) {
        // Check if previous instruction state matches current state
        let state = self._loop_prevention_memory.get(&self._ip).expect("Fetching previous instruction state failed!");
        if *state == true {
          return InstructionResult::LoopDetectedError(self._ip, self.registries.clone())
        }
      }
    }

    // Decode instruction (Check if out of bounds)
    match self.instructions.get(self._ip) {

      // Opcode decoded
      Some(instruction_code) => {

        // Process instruction
        let instruction_name = &instruction_code.0;
        let instruction_args = instruction_code.1.clone();
  
        // Prompt instruction
        if verbose {
          println!("{}[acc: {}] {} {:?}", self._ip, self.registries.acc, instruction_name, instruction_args);
        }

        // Execute opcode
        match &instruction_name[..] {

          // Addition opcode
          "acc" => {
            // Update accumulator
            self.registries.acc += instruction_args[0];

            // Update instruction state for purposes of loop prevention
            if self._loop_prevention {
              self._loop_prevention_memory.insert(self._ip, true);
            }

            // Update Instruction pointer
            self._ip += 1;

            // Return instruction executed ok
            return InstructionResult::InstructionOk(self._ip, self.registries.clone());
          },
    
          // Multiplication opcode
          "jmp"  => {
            // Update instruction state for purposes of loop prevention
            if self._loop_prevention {
              self._loop_prevention_memory.insert(self._ip, true);
            }

            // Update Instruction pointer
            self._ip = ((self._ip as isize) + instruction_args[0]) as usize;

            // Return instruction executed ok
            return InstructionResult::InstructionOk(self._ip, self.registries.clone());
          },

          // Input to memory input opcode
          "nop" => {

            // Update instruction state for purposes of loop prevention
            if self._loop_prevention {
              self._loop_prevention_memory.insert(self._ip, true);
            }

            // Update Instruction pointer
            self._ip += 1;

            // Return instruction executed ok
            return InstructionResult::InstructionOk(self._ip, self.registries.clone());
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
    self.registries.acc = 0;
    self._loop_prevention_memory = HashMap::new();
  }

}
