//! Bit-masking module
//! 
//! Implements an bit masking functionality interpreter
// -----------------------------------------------------------------------------

// Import child modules
mod mask_methods;

// (re)Export child modules
pub use mask_methods::*;

/// BitMask struct
#[derive(Debug)]
pub struct BitMask {
  pub mask: Vec<u8>
}
