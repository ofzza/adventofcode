//! Assembler ::new() implementation
//! 
//! Assembler interpreter constructor
// -----------------------------------------------------------------------------

// Include dependencies
use super::*;

/// Assembler interpreter ::new() implementation
/// 
/// Assembler interpreter constructor
impl AssemblerInterpreter {
  
  /// Instantiate a new Assembler interpreter
  /// 
  /// # Arguments
  ///
  /// * `input`           - Assembler code syntax
  /// * `loop_prevention` - If loop prevention should be used
  pub fn new (input: &Vec<String>, loop_prevention: bool) -> AssemblerInterpreter {
    
    // Initialize diagram
    let mut interpreter = AssemblerInterpreter {
      _loop_prevention: loop_prevention,
      instructions: vec![],
      ..Default::default()
    };

    // Decode instructions
    for line in input  {
      // Parse instructions
      let line_parsed: Vec<&str> = line.split(' ').collect();
      let instruction_name = String::from(line_parsed[0]);
      let instruction_args = line_parsed[1..].iter().map(|arg| {
        return arg.parse::<isize>().expect("Failed parsing instruction argument value!");
      }).collect();
      // Store instruction
      interpreter.instructions.push((instruction_name, instruction_args));
    }
    
    // Return diagram
    return interpreter;
    
  }

}
