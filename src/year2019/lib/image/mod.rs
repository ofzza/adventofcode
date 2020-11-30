//! Space Image module
//! 
//! Implements a "Space image format"
// -----------------------------------------------------------------------------

// Include dependencies
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use crate::lib::console::*;

/// Space Image module
/// 
/// TODO: more details ...
#[derive(Hash)]
pub struct SpaceImage {
  _width: usize,
  _height: usize,
  _data: Vec<u8>
}

/// SpaceImage implementation
impl SpaceImage {

  /// Creates a new instance of SpaceImage
  /// 
  /// # Arguments
  /// 
  /// - `width`  - Image width
  /// - `height` - Image height
  /// - `data`   - Space image encoded data
  pub fn new (width: usize, height: usize, data: Vec<u8>) -> SpaceImage {
    SpaceImage{
      _width: width,
      _height: height,
      _data: data
    }
  }

  /// Composes default mapping to (optionally) use when converting to string
  pub fn get_default_mapping () -> HashMap<u8, String> {
    // Define an image mapping
    let mut mapping: HashMap<u8, String> = HashMap::new();
    mapping.insert('0' as u8, format!("{}{}O{}", CONSOLE_VERBOSE_EMPTY_BG, CONSOLE_VERBOSE_FG, CONSOLE_RESET));
    mapping.insert('1' as u8, format!("{}{}O{}", CONSOLE_VERBOSE_FULL_BG,  CONSOLE_VERBOSE_FG, CONSOLE_RESET));
    mapping.insert('2' as u8, format!("{}{} {}", CONSOLE_VERBOSE_EMPTY_BG, CONSOLE_VERBOSE_FG, CONSOLE_RESET));
    // Return image mapping
    mapping
  }

  /// Get image width
  pub fn get_width (&self) -> usize {
    self._width
  }
  /// Get image height
  pub fn get_height (&self) -> usize {
    self._height
  }  
  /// Get image layers count
  pub fn get_layers_count (&self) -> usize {
    (self._data.len() as usize) / (self._width * self._height)
  }

  /// Gets byte at a position of a layer
  /// 
  /// # Arguments
  /// 
  /// - `layer` - Layer to read from
  /// - `index` - Index coordinate on the layer to read from
  pub fn read_by_index (&self, layer: usize, index: usize) -> u8 {
    self._data[(layer * self._height * self._width) + index]
  }
  /// Gets byte at a position of a layer
  /// 
  /// # Arguments
  /// 
  /// - `layer` - Layer to read from
  /// - `x`     - X coordinate on the layer to read from
  /// - `y`     - > coordinate on the layer to read from
  pub fn _read_by_coordinates (&self, layer: usize, x: usize, y: usize) -> u8 {
    self._data[(layer * self._height * self._width) + (y * self._width) + x]
  }
  /// Gets layer's data
  /// 
  /// # Arguments
  /// 
  /// - `layer` - Layer to get data for
  pub fn read_layer (&self, layer: usize) -> &[u8] {
    &self._data[(layer * self._width * self._height)..((layer + 1) * self._width * self._height)]
  }

  /// Collapses all image layers and returns a collapsed single layer image
  /// 
  /// # Arguments
  /// 
  /// - `transparent_byte` - Value of pixel to be considered transparent
  pub fn collapse_layers (&self, transparent_byte: u8) -> SpaceImage {
    // Initialize collapsed data
    let mut data: Vec<u8> = vec![];
    // Initialize collapsed data and tracking of still transparent parts of the image
    let mut transparent_indices: Vec<usize> = vec![];
    for i in 0..(self._height * self._width) {
      data.push(transparent_byte);
      transparent_indices.push(i);
    }
    // Process layers
    for layer in 0..self.get_layers_count() {
      // Stack next layer onto transparent parts
      let mut filled_indices: Vec<usize> = vec![];
      for i in 0..(&transparent_indices).len() {
        let index = transparent_indices[i].clone();
        let pixel = self.read_by_index(layer, index);
        if pixel != transparent_byte {
          data[index] = pixel;
          filled_indices.push(i);
        }
      }
      // Unregister newly filled parts as no longer transparent
      for i in (0..filled_indices.len()).rev() {
        transparent_indices.remove(filled_indices[i]);
      }
      // If no transparent parts remaining, return early
      if transparent_indices.len() == 0 {
        return SpaceImage{
          _width: self._width,
          _height: self._height,
          _data: data
        };
      }
    }
    // Return single layer image
    SpaceImage{
      _width: self._width,
      _height: self._height,
      _data: data
    }
  }

  /// Calculates a unique image hash
  pub fn calculate_hash(&self) -> u64 {
    let mut s = DefaultHasher::new();
    self._data.hash(&mut s);
    s.finish()
  }

  /// Returns a layer of data as a string
  /// 
  /// # Arguments
  /// 
  /// - `layer`   - Layer to stringify
  /// - `pallete` - Optional mapping to use when representing values as strings
  pub fn to_string (&self, layer: usize, mapping: Option<&HashMap<u8, String>>) -> String {
    // Initialize output string
    let mut result = String::new();
    // Get layer data
    let data = self.read_layer(layer);
    let mut line: String;
    // Process lines
    for i in 0..self._height {
      // Get line (with or without mapping pixels)
      let line = match mapping {
        None => {
          // Get line with no mapping
          match std::str::from_utf8(&data[(i * self._width)..((i + 1) * self._width)]) {
            Ok(output) => output,
            Err(err) => panic!("Invalid UTF-8 sequence: {}", err)
          }
        },
        Some(mapping) => {
          // Map line characters
          line = String::new();
          for x in 0..self._width {
            let pixel = match mapping.get(&data[(i * self._width) + x]) {
              Some(pixel) => pixel.clone(),
              None => format!("?")
            };
            line.push_str(&pixel);
          }
          &line[..]
        }
      };
      // Append line
      result.push_str(line);
      result.push_str("\n");
    }
    // Return composed output string
    result
  }

}
