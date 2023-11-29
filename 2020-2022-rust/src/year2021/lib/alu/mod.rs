//! ALU module
//! 
//! Arithmetic Logic Unit implementation
// -----------------------------------------------------------------------------

// Include dependecies
use std::ops::*;

/// Arithmetic Logic Unit structure
pub struct ALU {
  registers: Range<char>,
  registers_len: usize,
  pub register_values: Vec<isize>,
  program: Vec<Vec<String>>
}

/// Arithmetic Logic Unit implementation
impl ALU {

  /// Constructor
  /// 
  /// # Arguments
  /// * first_register_name:  Name of the first register - all subsequent registers are assumed to be subsequent characters
  /// * registers_count:      Number of registers available
  pub fn new (registers: Range<char>) -> ALU {
    // Initialize registers
    let registers_len = ((registers.end as u8) - (registers.start as u8)) as usize;
    let mut register_values: Vec<isize> = Vec::with_capacity(registers_len);
    for _ in 0..registers_len {
      register_values.push(0);
    }
    // Return instance of ALU
    ALU {
      registers,
      registers_len,
      register_values,
      program: vec![]
    }
  }

  /// Loads a program into the ALU
  /// 
  /// # Arguments
  /// * program: Program to load
  pub fn load (&mut self, program: &Vec<Vec<String>>) {
    self.program = program.clone();
  }

  /// Executes a loaded program using given inputs
  /// 
  /// # Arguments
  /// * inputs:   Input values to be passed tot he program in order as requsted
  /// * callback: Callback function called after each instruction is executed and which is alloed to halt execution by returning a false
  /// 
  /// # Returns
  /// Value returned from the breaking callback execution, or None if executed to the end
  pub fn execute<T, F: Fn(usize, &Vec<String>, &ALU, usize, &Vec<usize>) -> Option<T>>(&mut self, inputs: &Vec<usize>, callback: F) -> Option<T> {
    // Reset register values
    for i in 0..self.registers_len {
      self.register_values[i] = 0;
    }
    // Initialize the input counter
    let mut ic: usize = 0;
    // Run program instruction by instruction
    for ec in 0..self.program.len() {
      // Decode the instruction
      let instruction = &self.program[ec];
      let instruction_call = &instruction[0];

      // Execute callback
      match callback(ec, &instruction, &self, ic, inputs) {
        Some(x) => { return Some(x); },
        _ => ()
      }

      // Execute any single argument instruction
      if instruction.len() == 2 {
        // Decode target register
        let ireg = self.decode_register(&instruction[1]);
        let ireg_index = match ireg {
          Some(ireg_index) => ireg_index,
          None => { panic!("Failed decoding register {:?}!", ireg); }
        };

        // Execute INP instruction
        if instruction_call == "inp" {
          if ic >= inputs.len() { break; }
          self.register_values[ireg_index] = inputs[ic] as isize;
          ic += 1;
        }        
      }

      // Execute any two argument instruction
      else if instruction.len() == 3 {
        // Decode target registers and/or values
        let ireg1 = self.decode_register(&instruction[1]);
        let ireg2 = self.decode_register(&instruction[2]);
        let ireg1_index = match ireg1 {
          Some(ireg1_index) => ireg1_index,
          None => { panic!("Failed decoding register {:?}!", ireg1); }
        };
        let ireg2_value = match ireg2 {
          Some(ireg2_index) => self.register_values[ireg2_index],
          None => instruction[2].parse::<isize>().unwrap()
        };

        // Execute ADD instruction
        if instruction_call == "add" {
          self.register_values[ireg1_index] = self.register_values[ireg1_index] + ireg2_value;
        }
        
        // Execute MUL instruction
        else if instruction_call == "mul" {
          self.register_values[ireg1_index] = self.register_values[ireg1_index] * ireg2_value;
        }
        
        // Execute DIV instruction
        else if instruction_call == "div" {
          self.register_values[ireg1_index] = self.register_values[ireg1_index] / ireg2_value;
        }
        
        // Execute MOD instruction
        else if instruction_call == "mod" {
          self.register_values[ireg1_index] = self.register_values[ireg1_index] % ireg2_value;
        }
        
        // Execute EQL instruction
        else if instruction_call == "eql" {
          self.register_values[ireg1_index] = if self.register_values[ireg1_index] == ireg2_value { 1 } else { 0 };
        }
      }
    }

    None
  }

  /// Decodes a register index from it's name
  /// 
  /// # Arguments
  /// * name: Name of the register to decode
  /// 
  /// # Returns option of register index if register name was successfully decoded
  fn decode_register (&self, name: &String) -> Option<usize> {
    let name_char = name.chars().next().unwrap();
    if self.registers.contains(&name_char) {
      Some(((name_char as u8) - (self.registers.start as u8)) as usize)
    } else {
      None
    }
  }

}

