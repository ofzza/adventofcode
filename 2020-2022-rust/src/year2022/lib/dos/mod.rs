//! DOS module
//! 
//! Device Operating System module
// -----------------------------------------------------------------------------

// Load (sub)modules
pub mod gpu;
pub mod screen;
pub mod fs;

// Include dependecies
use self::gpu::GpuController;
use self::screen::Screen;
use self::fs::FS;

/// Device Operating System structure
pub struct DOS<'a> {
  // GPU controller
  pub gpu: GpuController<'a>,
  // Screen controller
  pub screen: Screen,
  // Filesystem
  pub fs: FS<'a>,
}

/// Device Operating System implementation
impl<'a> DOS<'a> {

  /// Constructor
  pub fn new<'b> () -> DOS<'b> {
    // Initialize GPU
    let gpu: GpuController<'b> = GpuController::new();
    // Initialize Screen
    let screen: Screen = Screen::new();
    // Initialize FS
    let fs: FS<'b> = FS::new();
    // Initialize DOS
    DOS {
      gpu,
      screen,
      fs
    }
  }

  /// Process terminal input/output and execute all found commands
  /// 
  /// # Arguments
  /// * lines: Terminal lines to process
  pub fn process_terminal_stdout(&mut self, lines: &Vec<&'a str>) {
    // Parse out individial commands
    let mut command: Vec<&str> = vec![];
    for line in lines {
      // New command found
      if line.starts_with("$") {
        // Execute previous command
        if command.len() > 0 {
          self.exec_terminal_stdout_section(&command);
        }
        // Clear previous command
        command.clear();
      }
      // Append line to command
      command.push(line);
    }
    // Execute final command
    if command.len() > 0 {
      self.exec_terminal_stdout_section(&command);
    }

    // Prompt empty prompt to show final CWD
    self.fs.prompt(Option::None);
  }

  /// Executes an OS command
  /// 
  /// # Arguments
  /// * cmd: Command to execute
  fn exec_terminal_stdout_section(&mut self, cmd: &Vec<&'a str>) {
    // Support execution of FS commands
    if self.fs.exec_terminal_stdout_section(cmd) { return; }
  }

}

