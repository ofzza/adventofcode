//! DOS FS module
//! 
//! Device Operating System Filesystem module
// -----------------------------------------------------------------------------

// Load (sub)modules
pub mod directory;
pub mod file;

// Include dependecies
use self::directory::FsDirectory;
use self::file::FsFile;

/// Device Operating System Filesystem structure
pub struct FS<'a> {
  // Filesystem
  pub root: FsDirectory<'a>,
  // CWD 
  pub cwd: Vec<&'a str>
}

/// Device Operating System Filesystem implementation
impl<'a> FS<'a> {

  /// Constructor
  pub fn new<'b> () -> FS<'b> {
    // Initialize FS
    let root: FsDirectory<'b> = FsDirectory::new("", vec![]);
    // Initialize DOS
    FS {
      root,
      cwd: vec![]
    }
  }

  /// Makes sure a directory exists for a given path and returns a nutable reference to it
  /// 
  /// # Arguments
  /// * path: Path of a directory
  fn get_dir (&mut self, path: Vec<&'a str>) {
    // Get starting directory
    let mut dir: &mut FsDirectory = &mut self.root;
    // Search for directory with given path and create if not exists
    for dirname in path {
      if !dir.directories.contains_key(dirname) {
        dir.directories.insert(dirname, FsDirectory::new(dirname, [dir.path.clone(), vec![dirname]].concat()));
      }
      dir = dir.directories.get_mut(dirname).unwrap();
    }
  }
  /// Makes sure a directory exists for a given path and filename and returns a nutable reference to it
  /// 
  /// # Arguments
  /// * path: Path of a parent directory
  /// * filename: Name of the file
  /// * size: Size of the file
  fn get_file(&mut self, path: Vec<&'a str>, filename: &'a str, size: usize) {
    // Get starting directory
    let mut dir: &mut FsDirectory = &mut self.root;
    // Search for directory with given path and create if not exists
    for dirname in path.clone() {
      if !dir.directories.contains_key(dirname) {
        dir.directories.insert(dirname, FsDirectory::new(dirname, [dir.path.clone(), vec![dirname]].concat()));
      }
      dir = dir.directories.get_mut(dirname).unwrap();
    }
    // Check if file exists
    if !dir.files.contains_key(filename) {
      dir.files.insert(filename, FsFile::new(dir.path.clone(), filename, size));
    }
  }

  /// Executes an OS FS command
  /// 
  /// # Arguments
  /// * cmd: Command to execute
  /// 
  /// # Returns
  /// If command was processed or not
  pub fn exec_terminal_stdout_section(&mut self, cmd: &Vec<&'a str>) -> bool {

    // Extract 1st line of the command
    if cmd.len() == 0 {
      return false;
    }
    let command: Vec<&str> = cmd[0].split(" ").collect();    
    let output: Vec<Vec<&str>> = if cmd.len() == 1 {
      vec![]
    } else {
      cmd[1..].iter().map(|line| line.split(" ").collect()).collect()
    };

    // Process "CD" command
    if command.len() > 1 && command[0] == "$" && command[1] == "cd" {
      self.exec_cd(&command);
      return true;
    }

    // Process "LS" command
    if command.len() > 1 && command[0] == "$" && command[1] == "ls" {
      self.exec_ls(&output);
      return true;
    }

    // Command was not processed
    false
  }
  /// Executes an OS FS "CD" command
  /// 
  /// # Arguments
  /// * command: Command to execute
  fn exec_cd(&mut self, command: &Vec<&'a str>) {
    // Validate command
    if command.len() != 3 {
      panic!("CD command should be prefaced with a '$' char and contain a single argument! {:?}", command);
    }

    // Update CWD
    let arg: &'a str = command[2];    
    let mut cwd_updated: bool = false;
    // Keep to current directory
    if arg == "." {
      // Do nothing!
    }
    // Switch to parent directory unless already @ root directory
    else if arg == ".." {
      if self.cwd.len() > 0 {
        cwd_updated = true;
        self.cwd.pop();
      }
    }
    // Switch to @ root directory
    else if arg == "/" {
      cwd_updated = self.cwd.len() > 0;
      self.cwd = vec![];
    }
    // Switch to a named subdirectory
    else {
      cwd_updated = true;
      self.cwd.push(arg);
    }

    // Make sure FS reflects the updated CWD
    if cwd_updated {
      let path: Vec<&str> = self.cwd.clone();
      self.get_dir(path);
    }

  }
  /// Executes an OS FS "LS" command
  /// 
  /// # Arguments
  /// * output: Command output
  fn exec_ls(&mut self, output: &Vec<Vec<&'a str>>) {
    // Process output
    for line in output {
      // Verify line
      if line.len() != 2 {
        panic!("LS command output should only contain lines with 2 items!");
      }
      // Process directory
      if line[0] == "dir" {
        let path: Vec<&str> = self.cwd.clone();
        let dirname = line[1];
        self.get_dir([path.clone(), vec![dirname]].concat());
      }
      // Process file
      else {
        let path: Vec<&str> = self.cwd.clone();
        let size = line[0].parse::<usize>().unwrap();
        let filename = line[1];
        self.get_file(path, filename, size);
      }
    }
  }

  /// Refreshes directory sizes after a changes has been made to the directory or any of its children
  pub fn refresh_sizes (&mut self) {
    self.root.refresh_sizes();
  }

  /// Tracerses the entire FS and invokes a callback with every directory and file along the way
  /// 
  /// # Arguments
  /// * callback: Callback function to be called on every directory and file as they are being traversed
  /// * aggregate: Aggregating value being passed between itnerations of the callback execution
  /// 
  /// # Returns
  /// Aggregating value being passed between itnerations of the callback execution
  pub fn traverse<T> (&self, callback: fn(directory: Option<&FsDirectory>, file: Option<&FsFile>, aggregate: T) -> T, aggregate: T) -> T {
    self.root.traverse(callback, aggregate)
  }

  /// Prompts the currently executing command, or just empty terminal prompt
  /// 
  /// # Arguments
  /// * cmd: Option of command and command output to prompt
  pub fn prompt (&self, _cmd: Option<(&Vec<&str>, &Vec<Vec<&str>>)>) {
    // // Prompt executing command
    // match _cmd {
    //   Option::Some((command, output)) => {
    //     println!("FS > /{} {}", self.cwd.join("/"), command.join(" "));
    //     println!("{}", output.iter().map(|line| line.join(" ")).collect::<Vec<String>>().join("\n"));
    //     println!();
    //   },
    //   Option::None => {
    //     println!("FS > /{} $", self.cwd.join("/"));
    //     println!();
    //   }
    // }
  }
}

