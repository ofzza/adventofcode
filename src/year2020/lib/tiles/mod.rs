//! Image Tiles module
//! 
//! Implements Image tiling functionality
// -----------------------------------------------------------------------------

// Import child modules
mod constructor;
mod pixels;
mod compose;
mod search;

// (re)Export child modules
pub use constructor::*;
pub use pixels::*;
pub use compose::*;
pub use search::*;

/// Image Tile struct
#[derive(Debug, Clone)]
pub struct ImageTile {
  // Tile ID
  pub id: usize,
  // Tile image data
  pub data: ImageData,
  // 4 borders (top, right, bottom, left)
  pub borders: [String; 4],
  // 4 bordering tiles' IDs (top, right, bottom, left) if they exist
  pub bordering_tiles_ids: Vec<Option<usize>>
}

/// Image Data struct
#[derive(Debug, Clone)]
pub struct ImageData {
  // Tile content width and height
  pub dimensions: (usize, usize),
  // Tile content pixels
  pub pixels: Vec<bool>
}
