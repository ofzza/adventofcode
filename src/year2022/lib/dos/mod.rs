//! DOS module
//! 
//! Device Operating System module
// -----------------------------------------------------------------------------

// Load (sub)modules
pub mod fs;

// Include dependecies
use self::fs::directory::FsDirectory;
use self::fs::file::FsFile;

/// Device Operating System structure
pub struct DOS<'a> {
  // Filesystem
  pub fs: FsDirectory<'a>,
  // CWD 
  pub cwd_path: Vec<&'a str>
}

/// Device Operating System implementation
impl<'a> DOS<'a> {

  /// Constructor
  pub fn new<'b> () -> DOS<'b> {
    // Initialize FS
    let fs: FsDirectory<'b> = FsDirectory::new("", vec![]);
    // Initialize DOS
    DOS {
      fs,
      cwd_path: vec![]
    }
  }

  /// Makes sure a directory exists for a given path and returns a nutable reference to it
  /// 
  /// # Arguments
  /// * path: Path of a directory
  fn get_dir (&mut self, path: Vec<&'a str>) {
    // Get starting directory
    let mut dir: &mut FsDirectory = &mut self.fs;
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
    let mut dir: &mut FsDirectory = &mut self.fs;
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

  /// Process terminal input/output and execute all found commands
  /// 
  /// # Arguments
  /// * lines: Terminal lines to process
  pub fn terminal(&mut self, lines: &Vec<&'a str>) {
    // Parse out individial commands
    let mut command: Vec<&str> = vec![];
    for line in lines {
      // New command found
      if line.starts_with("$") {
        // Execute previous command
        if command.len() > 0 {
          self.exec(&command);
        }
        // Clear previous command
        command.clear();
      }
      // Append line to command
      command.push(line);
    }
    // Execute final command
    if command.len() > 0 {
      self.exec(&command);
    }

    // Prompt empty prompt
    self.prompt(Option::None);
  }
  /// Executes an OS command
  /// 
  /// # Arguments
  /// * cmd: Command to execute
  fn exec(&mut self, cmd: &Vec<&'a str>) {
    // Support execution of FS commands
    self.exec_fs(cmd);
  }
  /// Executes an OS FS command
  /// 
  /// # Arguments
  /// * cmd: Command to execute
  fn exec_fs(&mut self, cmd: &Vec<&'a str>) {

    // Extract 1st line of the command
    if cmd.len() == 0 {
      return;
    }
    let command: Vec<&str> = cmd[0].split(" ").collect();    
    let output: Vec<Vec<&str>> = if cmd.len() == 1 {
      vec![]
    } else {
      cmd[1..].iter().map(|line| line.split(" ").collect()).collect()
    };

    // Prompt executing command
    self.prompt(Option::Some((&command, &output)));

    // Process "cd" command
    if command.len() > 1 && command[0] == "$" && command[1] == "cd" {
      self.exec_fs_cd(&command);
    }

    // Process "ls" command
    if command.len() > 1 && command[0] == "$" && command[1] == "ls" {
      self.exec_fs_ls(&output);
    }

  }
  /// Executes an OS FS "CD" command
  /// 
  /// # Arguments
  /// * command: Command to execute
  fn exec_fs_cd(&mut self, command: &Vec<&'a str>) {
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
      if self.cwd_path.len() > 0 {
        cwd_updated = true;
        self.cwd_path.pop();
      }
    }
    // Switch to @ root directory
    else if arg == "/" {
      cwd_updated = self.cwd_path.len() > 0;
      self.cwd_path = vec![];
    }
    // Switch to a named subdirectory
    else {
      cwd_updated = true;
      self.cwd_path.push(arg);
    }

    // Make sure FS reflects the updated CWD
    if cwd_updated {
      let path: Vec<&str> = self.cwd_path.clone();
      self.get_dir(path);
    }

  }
  /// Executes an OS FS "LS" command
  /// 
  /// # Arguments
  /// * output: Command output
  fn exec_fs_ls(&mut self, output: &Vec<Vec<&'a str>>) {
    // Process output
    for line in output {
      // Verify line
      if line.len() != 2 {
        panic!("LS command output should only contain lines with 2 items!");
      }
      // Process directory
      if line[0] == "dir" {
        let path: Vec<&str> = self.cwd_path.clone();
        let dirname = line[1];
        self.get_dir([path.clone(), vec![dirname]].concat());
      }
      // Process file
      else {
        let path: Vec<&str> = self.cwd_path.clone();
        let size = line[0].parse::<usize>().unwrap();
        let filename = line[1];
        self.get_file(path, filename, size);
      }
    }
  }

  /// Refreshes directory sizes after a changes has been made to the directory or any of its children
  pub fn refresh_fs_sizes (&mut self) {
    self.fs.refresh_sizes();
  }

  /// Tracerses the entire FS and invokes a callback with every directory and file along the way
  /// 
  /// # Arguments
  /// * callback: Callback function to be called on every directory and file as they are being traversed
  pub fn traverse_fs<T> (&self, callback: fn(directory: Option<&FsDirectory>, file: Option<&FsFile>, aggregate: T) -> T, aggregate: T) -> T {
    self.traverse_directory(&self.fs, callback, aggregate)
  }
  /// Tracerses the entire FS and invokes a callback with every directory and file along the way
  /// 
  /// # Arguments
  /// * callback: Callback function to be called on every directory and file as they are being traversed
  fn traverse_directory<T> (&self, directory: &FsDirectory, callback: fn(directory: Option<&FsDirectory>, file: Option<&FsFile>, aggregate: T) -> T, aggregate: T) -> T {
    let mut aggregate = aggregate;
    // Callback for current directory
    aggregate = callback(Option::Some(directory), Option::None, aggregate);
    // Callback for current directory files
    for file in directory.files.values() {
      aggregate = callback(Option::None, Option::Some(&file), aggregate);
    }
    // Traverse (sub)directories
    for dir in directory.directories.values() {
      aggregate = self.traverse_directory(dir, callback, aggregate);
    }
    // Return edited aggregate
    aggregate
  }

  /// Prompts the currently executing command, or just empty terminal prompt
  /// 
  /// # Arguments
  /// * cmd: Option of command and command output to prompt
  fn prompt (&self, _cmd: Option<(&Vec<&str>, &Vec<Vec<&str>>)>) {
    // // Prompt executing command
    // match _cmd {
    //   Option::Some((command, output)) => {
    //     println!("[/{}]: {}", self.cwd_path.join("/"), command.join(" "));
    //     println!("{}", output.iter().map(|line| line.join(" ")).collect::<Vec<String>>().join("\n"));
    //     println!();
    //   },
    //   Option::None => {
    //     println!("[/{}]: $", self.cwd_path.join("/"));
    //     println!();
    //   }
    // }
  }

}

