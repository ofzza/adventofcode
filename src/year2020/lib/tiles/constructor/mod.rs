//! Image Tiles ::new() implementation
//! 
//! Image tiling constructor
// -----------------------------------------------------------------------------

// Include dependencies
use super::*;

/// Image Tiles ::new() implementation
/// 
/// Image tiling constructor
impl ImageTile {
  
  /// Instantiate a new ImageTile
  /// 
  /// # Arguments
  ///
  /// * `input` - Tiles in string representation
  pub fn new (input: &String) -> ImageTile {    
    // Parse lines
    let lines: Vec<&str>           = input.split("\n").collect();
    let id: usize                  = lines[0].replace("Tile", "").replace(":", "").trim().parse::<usize>().expect("Failed parsing tile ID!");
    let dimensions: (usize, usize) = (lines[1].len(), lines.len() - 1);
    let pixels: Vec<bool>          = lines[1..].join("").replace("\n", "").as_bytes().iter().map(|byte| byte.clone() as char == '#').collect();
    let borders = [
      // Top border (left-to-right)
      (0..dimensions.0)
        .map(|x| if pixels[0 * dimensions.1 + x] { "#" } else { " " }).collect::<Vec<&str>>()
        .join(""),
      // Right border (top-to-bottom)
      (0..dimensions.1)
        .map(|y| if pixels[y * dimensions.0 + (dimensions.0 - 1)] { "#" } else { " " }).collect::<Vec<&str>>()
        .join(""),
      // Bottom border (right-to-left)
      (0..dimensions.0)
        .map(|x| if pixels[(dimensions.1 - 1) * dimensions.0 + (dimensions.0 - x - 1)] { "#" } else { " " }).collect::<Vec<&str>>()
        .join(""),
      // Left border (bottom-to-top)
      (0..dimensions.1)
        .map(|y| if pixels[(dimensions.1 - y - 1) * dimensions.0 + 0] { "#" } else { " " }).collect::<Vec<&str>>()
        .join("")
    ];

    // Extract non-border part of the image
    let mut image: Vec<bool> = vec![];
    for y in 1..(dimensions.1 - 1) {
      for x in 1..(dimensions.0 - 1) {
        image.push(pixels[y * dimensions.0 + x]);
      }
    }

    // Instantiate and return a parsed tile
    return ImageTile {
      id,
      data: ImageData {
        dimensions: (dimensions.0 - 2, dimensions.1 - 2),
        pixels: image,
      },
      borders,
      bordering_tiles_ids: vec![None, None, None, None]
    };    
  }

}
