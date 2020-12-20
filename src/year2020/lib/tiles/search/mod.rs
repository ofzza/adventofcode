//! Image Tiles ::search() implementation
//! 
//! Image tiling searching implementation
// -----------------------------------------------------------------------------

// Include dependencies
use super::*;

/// Image Tiles ::search() implementation
/// 
/// Image tiling searching implementation
impl ImageData {
  
  /// Searches for a needle image in the haystack image
  /// 
  /// # Arguments
  ///
  /// * `needle`   - Image to search for
  pub fn search (&mut self, needle: ImageData) -> usize {
    // Initialize max number of mathed pattern pixels
    let mut max = 0;
    // Search all orientations and flips
    for _ in 0..2 {
      for _ in 0..4 {
        // Search for needle in all possible starting positions
        let mut found_needle_pixels_count = 0;
        for start_y in 0..(self.dimensions.1 - needle.dimensions.1) {
          for start_x in 0..(self.dimensions.0 - needle.dimensions.0) {
            // Compare images
            let mut comparison_failed = false;
            let mut comparison_found_pixels = 0;
            for y in 0..needle.dimensions.1 {
              for x in 0..needle.dimensions.0 {
                // If pixel is on, compare it on on both images
                if needle.get_pixel(x, y) && !self.get_pixel(start_x + x, start_y + y) {
                  comparison_failed = true;
                  break;
                } else if needle.get_pixel(x, y) && self.get_pixel(start_x + x, start_y + y) {
                  comparison_found_pixels += 1;
                }
              }
              if comparison_failed { break; }
            }
            if !comparison_failed {
              found_needle_pixels_count += comparison_found_pixels;
            }
          }
        }

        // Check if matched needle
        if found_needle_pixels_count > max {
          max = found_needle_pixels_count;
        }

        // Rotate to next orientation
        self.rotate(90);
      }
      // Flip image
      self.flip_vertically();
    }

    // Return min remaining unmatched pixels    
    let mut image_pixel_count = 0;
    for i in 0..self.pixels.len() {
      if self.pixels[i] {
        image_pixel_count += 1;
      }
    }
    return image_pixel_count - max;
  }

}
