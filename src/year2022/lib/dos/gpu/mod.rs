//! DOS GPU module
//! 
//! Device Operating System GPU module
// -----------------------------------------------------------------------------

/// Device Operating System GPU structure
pub struct GpuController<'a> {
  // GPU clock
  pub clock: usize,
  // GPU registers
  pub registers: [isize; 1],
  // Command queue storing all commands queued for execution
  command_queue: Vec<(&'a str, Option<isize>)>,
  // Command pointer pointing to the next command in command queue to be executed
  command_pointer: usize,
  // If executing a microtask, here is the number of cycles left before it is executed
  microtask_remaning_cycles: usize,
  // If executing a microtask, here is the state of the registers after completion
  microtask_result: [isize; 1]
}

/// Device Operating System GPU implementation
impl<'a> GpuController<'a> {

  /// Constructor
  pub fn new () -> GpuController<'a> {
    GpuController {
      clock: 0,
      registers: [1],
      command_queue: vec![],
      command_pointer: 0,
      microtask_remaning_cycles: 0,
      microtask_result: [0]
    }
  }

  pub fn process_controller_stdin (&mut self, lines: &Vec<&'a str>) {
    // Process lines
    for line in lines {
      let cmd = self.parse_controller_stdin_command(line);
      self.command_queue.push(cmd);
    }
  }

  /// Executes an GPU controller command
  /// 
  /// # Arguments
  /// * cmd: Command to execute
  pub fn parse_controller_stdin_command(&mut self, line: &'a str) -> (&'a str, Option<isize>) {
    // Parse commnad
    let parsed: Vec<&str> = line.split(" ").collect();
    if parsed.len() < 2 {
      (parsed[0], Option::None)
    } else {
      (parsed[0], Option::Some(parsed[1].parse::<isize>().unwrap()))
    }
  }

  /// Executes a command
  /// 
  /// # Arguments
  /// * cmd: Command (name and opttional argument) to execute
  /// 
  /// # Returns
  /// A tuple of register x value after the command executes and number of cycles execution should take
  fn exec (&mut self, cmd: (&'a str, Option<isize>)) -> ([isize; 1], usize) {
    // Execute "NOOP" command
    if cmd.0 == "noop" {
      return self.exec_noop();
    }
    
    // Execute "ADDX" command
    else if cmd.0 == "addx" {
      return self.exec_addx(cmd.1.unwrap());
    }

    // Execute noting
    (self.registers, 0)
  }
  /// Executes a "NOOP" command
  /// 
  /// # Returns
  /// A tuple of register x value after the command executes and number of cycles execution should take
  fn exec_noop(&mut self) -> ([isize; 1], usize) {
    // Do nothing
    // ...
    // Return number of cycles required to execute
    (self.registers, 1)
  }
  /// Executes a "ADDX" command
  /// 
  /// # Arguments
  /// * value: Value argument for the "ADDX" command
  /// 
  /// # Returns
  /// A tuple of register x value after the command executes and number of cycles execution should take
  fn exec_addx(&mut self, value: isize) -> ([isize; 1], usize) {
    // Calculate updated register x value
    let mut registers = self.registers.clone();
    registers[0] += value;
    // Return number of cycles required to execute
    (registers, 2)
  }

  /// Prompts current state of the controllers
  /// 
  /// # Arguments
  /// * comment: Comment to prompt
  fn prompt (&mut self, _comment: &str) {
    // println!("GPU > clock: {}, registers: {:?}: {}", self.clock, self.registers, _comment);
  }
}

// Implement iterator trait for GpuController
impl<'a> Iterator for GpuController<'a> {
  // Define iterator item type
  type Item = (usize, [isize; 1]);

  // next() is the only required method
  fn next(&mut self) -> Option<Self::Item> {
    // If microtask still processing, forward clock
    if self.microtask_remaning_cycles > 1 {
      // Forward microtask counter
      self.microtask_remaning_cycles -= 1;
      // Forward clock
      self.clock += 1;
      self.prompt(format!("Executing microtask, cycles remaining: {}", self.microtask_remaning_cycles).as_str());
      return Option::Some((self.clock, self.registers));
    }
    // If microtask finished executing, set value and proceed to ingest next command
    else if self.microtask_remaning_cycles == 1 {
      // Forward microtask counter
      self.microtask_remaning_cycles -= 1;
      // Commit microtask result
      self.registers = self.microtask_result;
      self.prompt(format!("... done executing microtask, stored value {:?}", self.registers).as_str());
    }

    // Check if next command available
    if self.command_pointer >= self.command_queue.len() {
      // Refuse execution
      self.prompt(format!("No commands left in queue!").as_str());
      return Option::None;
    }
    
    // Simulate execution of next command
    let cp = self.command_pointer;
    let cmd = self.command_queue[cp];
    let (execution_result, execution_duration) = self.exec(cmd);
    self.prompt(format!("... ingested command from queue: {:?}", cmd).as_str());
    self.prompt(format!("... simulated command execution: result {:?} in {} cycles", execution_result, execution_duration).as_str());
    // Forward command pointer
    self.command_pointer += 1;

    // Check if command can't be executed
    if execution_duration == 0 {
      // Refuse execution
      self.prompt(format!("Command failed executing!").as_str());
      return Option::None;
    }
    // If command can be executed in single cycle, execute command
    else if execution_duration == 1 {
      // Commit execution result
      self.registers = execution_result;
      // Forward clock
      self.clock += 1;
      self.prompt(format!("Command executed in current cycle with result: {:?}", execution_result).as_str());
      return Option::Some((self.clock, self.registers));
    }
    // Store microtask to execute
    else {
      // Forward microtask counter
      self.microtask_remaning_cycles = execution_duration;
      // Store microtask result
      self.microtask_result = execution_result;
      // Forward clock
      self.clock += 1;
      self.prompt(format!("Command executing, microtask started: {} cycles will produce result {:?}", execution_duration, execution_result).as_str());
      return Option::Some((self.clock, self.registers));
    }
  }
}
