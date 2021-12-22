//! 2021 day 20 puzzle
//! 
//! https://adventofcode.com/2021/day/20
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2021::lib::bits::*;
use crate::year2021::lib::matrix::*;

/// Parses input data
fn parse(data: &String) -> (Vec<bool>, Vec<Vec<bool>>) {
  let sections: Vec<&str> = Input::parse(data.trim(), "\n\n", |section| section.trim());
  let algorithm: Vec<bool> = sections[0].chars().map(|c| c == '#').collect();
  let image: Vec<Vec<bool>> = Input::parse(sections[1], "\n", |line| {
    Input::parse(line.trim(), "", |c| c.chars().next().unwrap() == '#')
  });
  (algorithm, image)
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 20,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Calculate and return result
      String::from(format!("{:?}", enhance(&data.1, &data.0, 2)))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 20,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);
      
      // Calculate and return result
      String::from(format!("{:?}", enhance(&data.1, &data.0, 50)))
    }

  );

  // Return registry ownership
  registry
}

/// Enhances the image a numbe r of consequitive times
/// 
/// # Arguments
/// * original_image: Input image data
/// * algorithm:      Image enhancing algorithm definition
/// * n:              Number of consequitive times to enhance the image
/// 
/// # Returns
/// Number of "on" pixels on the enhanced image
fn enhance (original_image: &Vec<Vec<bool>>, algorithm: &Vec<bool>, n: usize) -> usize {
  // Initialize the image matrix
  let mut image = Matrix::new(vec![original_image[0].len(), original_image.len()]);
  let mut image_data: Vec<bool> = image.create();
  let mut image_points: Vec<(usize, usize)> = Vec::with_capacity(image_data.len());
  for y in 0..original_image.len() {
    for x in 0..original_image[y].len() {
      image_data.push(original_image[y][x]);
      if original_image[y][x] { image_points.push((x, y)); }
    }
  }
  let mut image_points_len = image_points.len();

  // Draw image
  // DotDisplay::print(image_points);
  // println!("");

  // CEnhance the image a requested number of consequitive times
  for i in 0..n {
    // Initialize enhanced image
    let image_enhanced = Matrix::new(vec![image.dimensions[0] + 2, image.dimensions[1] + 2]);
    let mut image_data_enhanced: Vec<bool> = image_enhanced.create();
    let mut image_points_enhanced: Vec<(usize, usize)> = Vec::with_capacity(image_data_enhanced.len());
    
    // Compose the enhanced image pixel by pixel
    for y in 0..image_enhanced.dimensions[1] {
      for x in 0..image_enhanced.dimensions[0] {
        
        // Get surrounding pixels
        let mut values: Vec<bool> = Vec::with_capacity(9);
        for dy in -1..2 {
          for dx in -1..2 {
            // Get relative coordinates to the original image
            let py = y as isize + dy - 1;
            let px = x as isize + dx - 1;
            if px >= 0 && px < image.dimensions[0] as isize && py >= 0 && py < image.dimensions[1] as isize {
              values.push(image_data[image.coords_to_index(&vec![px as usize, py as usize]).unwrap()]);
            } else {
              values.push(if !algorithm[0] { false } else if i % 2 == 0 { false } else { true });
            }

          }
        }

        // Decode pixel value
        let value = BITS::decode_binary_number(&values);
        image_data_enhanced.push(algorithm[value]);
        if algorithm[value] { image_points_enhanced.push((x, y)); }
      }
    }

    // Replace image with enhanced image
    image = image_enhanced;
    image_data = image_data_enhanced;
    image_points = image_points_enhanced;
    image_points_len = image_points.len();

    // Draw image
    // DotDisplay::print(image_points);
    // println!("");
  }

  image_points_len
}
