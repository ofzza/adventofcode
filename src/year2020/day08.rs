//! 2020/08 puzzle
//! 
//! https://adventofcode.com/2020/day/8
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::inputs::*;
use crate::lib::puzzle::*;
use super::asm::*;

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
        String::from("nop +0"),
        String::from("acc +1"),
        String::from("jmp +4"),
        String::from("acc +3"),
        String::from("jmp -3"),
        String::from("acc -99"),
        String::from("acc +1"),
        String::from("jmp -4"),
        String::from("acc +6")
      ]);
      stats.update(
        Puzzle::new(2020, 8, 1, "test", input, implementation1, |r| (r, Some(5)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<String>(load_input("./src/year2020/data/day08input.txt"), "\n");
      stats.update(
        Puzzle::new(2020, 8, 1, "solution", input, implementation1, |r| (r, Some(2051)))
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
        String::from("nop +0"),
        String::from("acc +1"),
        String::from("jmp +4"),
        String::from("acc +3"),
        String::from("jmp -3"),
        String::from("acc -99"),
        String::from("acc +1"),
        String::from("jmp -4"),
        String::from("acc +6")
      ]);
      stats.update(
        Puzzle::new(2020, 8, 2, "test", input, implementation2, |r| (r, Some(8)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<String>(load_input("./src/year2020/data/day08input.txt"), "\n");
      stats.update(
        Puzzle::new(2020, 8, 2, "solution", input, implementation2, |r| (r, Some(2304)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<String, isize, isize>, verbose: bool) -> Result<isize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(syntax) => {
      // Instantiate an ASM intepreter
      let mut interpreter = AssemblerInterpreter::new(syntax, true);
      // Check if interpeter terminates
      match check_if_terminates(&mut interpreter, verbose) {
        Ok(result) => {
          match result {
            InstructionResult::LoopDetectedError(_ip, registries) => Ok(registries.acc),
            _ => Err("Loop expected!")
          }
        },
        Err(_) => Err("Error detecting if program loops!")
      }
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<String, isize, isize>, verbose: bool) -> Result<isize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(syntax) => {
      // Instantiate an ASM intepreter
      let mut interpreter = AssemblerInterpreter::new(syntax, true);
      let mut instructions = interpreter.instructions.clone();
      // Find all JMP and NOP instructions and try replacing them
      for i in 0..instructions.len() {
        // Get and process instruction
        let instruction = &instructions.get(i).expect("Failed fetching instruction!").clone();
        let instruction_name = &instruction.0;
        // Check if instruction is to be tested as replaced
        if instruction_name == "jmp" || instruction_name == "nop" {
          // Swap instructions
          instructions[i].0 = if instruction_name == "jmp" { String::from("nop") } else { String::from("jmp") };
          interpreter.instructions = instructions.clone();
          // Prompt replaced instructions
          if verbose {
            println!("Testing looping with instruction {} replaced: {} ...", i, &instructions.get(i).expect("Failed fetching instruction!").clone().0);
          }
          // Check if interpeter terminates with swapped instructions
          interpreter.reset();
          match check_if_terminates(&mut interpreter, false) {
            Ok(result) => {
              match result {
                InstructionResult::End(_ip, registries) => { return Ok(registries.acc); },
                _ => ()
              }
            },
            Err(_) => { return Err("Error detecting if program loops!"); }
          };
          // Swap instructions back
          instructions[i].0 = instruction_name.clone();
          interpreter.instructions = instructions.clone();
        }
      }

      // Throw error
      Err("No non-looping variant found!")
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

/// Checks if an Assembler interpreter instance will terminate execution or loop for ever
/// 
/// # Arguments
/// * `interpreter` - Interpreter instance to test
/// * `verbose`     - Outputs executing output of the puzzle to the console 
fn check_if_terminates (interpreter: &mut AssemblerInterpreter, verbose: bool) -> Result<InstructionResult, &str> {
  // Run interpreter
  loop {
    match interpreter.next(verbose) {
      InstructionResult::InstructionOk(_ip, _registry_acc) => {
        // Proceed with execution
        continue;
      },
      InstructionResult::InstructionError(_err) => {
        // Execution error
        return Err("Intepreter errored executing an instruction!");
      },
      InstructionResult::LoopDetectedError(ip, registries) => {
        // Loop detected
        return Ok(InstructionResult::LoopDetectedError(ip.clone(), registries.clone()));
      },
      InstructionResult::End(ip, registries) => {
        // Execution error
        return Ok(InstructionResult::End(ip.clone(), registries.clone()));
      },
    }
  }
}
