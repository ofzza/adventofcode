//! Image Tiles pixels manipulation implementation
//! 
//! Image tiling pixels manipulation implementation
// -----------------------------------------------------------------------------

// Include dependencies
use super::*;

/// Image Tiles pixels manipulation implementation
/// 
/// Image tiling pixels manipulation implementation
impl ImageTile {

  /// Gets a pixel from the content image
  /// 
  /// # Arguments
  /// * `x` - X coordinate of the pixel
  /// * `y` - Y coordinate of the pixel
  pub fn get_pixel (&mut self, x: usize, y: usize) -> bool {
    return self.data.get_pixel(x, y);
  }

  /// Sets a pixel to the content image
  /// 
  /// # Arguments
  /// * `x`     - X coordinate of the pixel
  /// * `y`     - Y coordinate of the pixel
  /// * `value` - Pixel state to set
  pub fn _set_pixel (&mut self, x: usize, y: usize, value: bool) {
    self.data._set_pixel(x, y, value);
  }

  /// Rotates image by an angle
  /// 
  /// # Arguments
  /// * `angle` - Angle to rotate clock-wise for (Allowed values: 0, 90, 180, 270)
  pub fn rotate (&mut self, angle: usize) {
    // Rotate image
    self.data.rotate(angle);
    // Rotate borders
    let borders = self.borders.clone();
    let bordering_tiles_ids = self.bordering_tiles_ids.clone();
    for i in 0..4 {
      let offset = angle / 90;
      self.borders[i] = borders[(4 + i - offset) % 4].clone();
      self.bordering_tiles_ids[i] = bordering_tiles_ids[(4 + i - offset) % 4].clone();
    }
  }

  /// Flips horizontally
  pub fn flip_horizontally (&mut self) {
    // Flip image
    self.data.flip_horizontally();
    // Flip borders
    let borders = self.borders.clone();
    self.borders = [
      borders[0].chars().rev().collect::<String>(),
      borders[3].chars().rev().collect::<String>(),
      borders[2].chars().rev().collect::<String>(),
      borders[1].chars().rev().collect::<String>()
    ];
    let bordering_tiles_ids = self.bordering_tiles_ids.clone();
    self.bordering_tiles_ids = vec![
      bordering_tiles_ids[0].clone(),
      bordering_tiles_ids[3].clone(),
      bordering_tiles_ids[2].clone(),
      bordering_tiles_ids[1].clone()
    ];
  }
  
  /// Flips vertically
  pub fn flip_vertically (&mut self) {
    // Flip image
    self.data.flip_vertically();
    // Flip borders
    let borders = self.borders.clone();
    self.borders = [
      borders[2].chars().rev().collect::<String>(),
      borders[1].chars().rev().collect::<String>(),
      borders[0].chars().rev().collect::<String>(),
      borders[3].chars().rev().collect::<String>()
    ];
    let bordering_tiles_ids = self.bordering_tiles_ids.clone();
    self.bordering_tiles_ids = vec![
      bordering_tiles_ids[2].clone(),
      bordering_tiles_ids[1].clone(),
      bordering_tiles_ids[0].clone(),
      bordering_tiles_ids[3].clone()
    ];
  }
}

/// Image Data pixels manipulation implementation
/// 
/// Image tiling pixels manipulation implementation
impl ImageData {

  /// Calculates linear pixel coordinates for the image
  /// 
  /// # Arguments
  /// * `x` - X coordinate of the pixel
  /// * `y` - Y coordinate of the pixel
  fn pixel_coords (&self, x: usize, y: usize) -> usize{
    return y * self.dimensions.0 + x;
  }
  
  /// Gets a pixel from the content image
  /// 
  /// # Arguments
  /// * `x` - X coordinate of the pixel
  /// * `y` - Y coordinate of the pixel
  pub fn get_pixel (&self, x: usize, y: usize) -> bool {
    let source = self.pixel_coords(x, y);
    return self.pixels[source];
  }

  /// Sets a pixel to the content image
  /// 
  /// # Arguments
  /// * `x`     - X coordinate of the pixel
  /// * `y`     - Y coordinate of the pixel
  /// * `value` - Pixel state to set
  pub fn _set_pixel (&mut self, x: usize, y: usize, value: bool) {
    let target = self.pixel_coords(x, y);
    self.pixels[target] = value;
  }

  /// Rotates image by an angle
  /// 
  /// # Arguments
  /// * `angle` - Angle to rotate clock-wise for (Allowed values: 0, 90, 180, 270)
  pub fn rotate (&mut self, angle: usize) {
    // Initialize new image
    let mut new: Vec<bool> = self.pixels.clone();
    // Transform data
    match angle {
      90 => {
        for x in 0..self.dimensions.0 {
          for y in 0..self.dimensions.1 {
            let source = self.pixel_coords(x, y);
            let target = self.pixel_coords(self.dimensions.1 - y - 1, x);
            new[target] = self.pixels[source];
          }
        }
      },
      180 => {
        for x in 0..self.dimensions.0 {
          for y in 0..self.dimensions.1 {
            let source = self.pixel_coords(x, y);
            let target = self.pixel_coords(self.dimensions.0 - x - 1, self.dimensions.1 - y - 1);
            new[target] = self.pixels[source];
          }
        }
      },
      270 => {
        for x in 0..self.dimensions.0 {
          for y in 0..self.dimensions.1 {
            let source = self.pixel_coords(x, y);
            let target = self.pixel_coords(y, self.dimensions.0 - x - 1);
            new[target] = self.pixels[source];
          }
        }
      },
      _ => {}
    }
    // Set image data
    self.pixels = new;
  }

  /// Flips horizontally
  pub fn flip_horizontally (&mut self) {
    // Initialize new image
    let mut new: Vec<bool> = self.pixels.clone();
    // Transform data
    for x in 0..self.dimensions.0 {
      for y in 0..self.dimensions.1 {
        let source = self.pixel_coords(x, y);
        let target = self.pixel_coords(self.dimensions.0 - x - 1, y);
        new[target] = self.pixels[source];
      }
    }
    // Set image data
    self.pixels = new;
  }
  
  /// Flips vertically
  pub fn flip_vertically (&mut self) {
    // Initialize new image
    let mut new: Vec<bool> = self.pixels.clone();
    // Transform data
    for x in 0..self.dimensions.0 {
      for y in 0..self.dimensions.1 {
        let source = self.pixel_coords(x, y);
        let target = self.pixel_coords(x, self.dimensions.1 - y - 1);
        new[target] = self.pixels[source];
      }
    }
    // Set image data
    self.pixels = new;
  }

  /// Returns a string representation of the image
  pub fn to_string (&self) -> String {
    let mut output: Vec<&str> = vec![];
    for i in 0..self.pixels.len() {
      output.push(if self.pixels[i] { "#" } else { "." });
      if i > 0 && (i + 1) % self.dimensions.0 == 0 {
        output.push("\n");
      }
    }
    return output.join("");
  }
  
}
