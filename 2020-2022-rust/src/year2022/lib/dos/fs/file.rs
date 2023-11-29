//! DOS FS file module
//! 
//! Device Operating System Filesystem File module
// -----------------------------------------------------------------------------

// Include dependecies
// ...

/// Device Operating System Filesystem file structure
#[derive(Debug)]
pub struct FsFile<'a> {
  // Name
  pub name: &'a str,
  // Full path
  pub path: Vec<&'a str>,
  // Size
  pub size: usize
}

/// Device Operating System Filesystem file implementation
impl FsFile<'_> {

  /// Constructor
  /// 
  /// # Arguments
  /// * path: Path of the parent directory
  /// * name: Name of the file
  /// * size: Size of the file
  pub fn new<'a> (path: Vec<&'a str>, name: &'a str, size: usize) -> FsFile<'a> {
    FsFile {
      name,
      path,
      size
    }
  }

}

