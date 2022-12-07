//! DOS FS directory module
//! 
//! Device Operating System Filesystem Directory module
// -----------------------------------------------------------------------------

// Include dependencies
use super::file::FsFile;
use std::collections::hash_map::HashMap;

/// Filesystem directory structure
#[derive(Debug)]
pub struct FsDirectory<'a> {
  // Name
  pub name: &'a str,
  // Full path
  pub path: Vec<&'a str>,
  // Size
  pub size: usize,
  // Subdirectories
  pub directories: HashMap<&'a str, FsDirectory<'a>>,
  // Files
  pub files: HashMap<&'a str, FsFile<'a>>  
}

/// Device Operating System Filesystem directory implementation
impl FsDirectory<'_> {

  /// Constructor
  /// 
  /// # Arguments
  /// * name: Name of the directory
  /// * path: Full path of the directory
  pub fn new<'a> (name: &'a str, path: Vec<&'a str>) -> FsDirectory<'a> {
    FsDirectory {
      name,
      path,
      size: 0,
      directories: HashMap::new(),
      files: HashMap::new()
    }
  }

  /// Refreshes directory sizes after a changes has been made to the directory or any of its children
  pub fn refresh_sizes (&mut self) {
    // Initialize directory size
    let mut size = 0;
    // Refresh child directories' sizes
    for dir in self.directories.values_mut() {
      dir.refresh_sizes();
      size += dir.size;
    }
    // Add file sizes
    for file in self.files.values() {
      size += file.size;
    }
    // Set directory size
    self.size = size;
  }
}

