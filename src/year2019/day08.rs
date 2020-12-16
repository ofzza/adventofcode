//! 2019/08 puzzle
//! 
//! https://adventofcode.com/2019/day/8
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::inputs::*;
use crate::lib::puzzle::*;
use super::image::*;

/// Registers puzzles for the day
pub fn run (index: u32, key: &str, verbose: bool, obfuscate: bool) -> PuzzleExecutionStatistics {

  // Initialize stats
  let mut stats = PuzzleExecutionStatistics{
    ..Default::default()
  };
  
  // Run puzzle
  if (index == 0) || (index == 1) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::ParamsVector1D(vec![3, 2], "123456789012".as_bytes().to_vec());
      stats.update(
        Puzzle::new(2019, 8, 1, "test", input, implementation1, |n| (n, Some(1)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = PuzzleInput::ParamsVector1D(vec![25, 6], load_input("./src/year2019/data/day08input.txt").as_bytes().to_vec());
      stats.update(
        Puzzle::new(2019, 8, 1, "solution", input, implementation1, |n| (n, Some(2250)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Run puzzle
  if (index == 0) || (index == 2) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::ParamsVector1D(vec![2, 2, 0], "0222112222120000".as_bytes().to_vec());
      stats.update(
        Puzzle::new(2019, 8, 2, "test", input, implementation2, |img| (img.calculate_hash(), Some(441979580009008673)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = PuzzleInput::ParamsVector1D(vec![25, 6, 1], load_input("./src/year2019/data/day08input.txt").as_bytes().to_vec());
      stats.update(
        Puzzle::new(2019, 8, 2, "solution", input, implementation2, |img| (img.calculate_hash(), Some(10283588279939085619)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<u8, i32, i32>, verbose: bool) -> Result<i32, &str> {
  match &puzzle.input {
    PuzzleInput::ParamsVector1D(params, data) => {
      // Instantiate the image
      let image = SpaceImage::new(params[0] as usize, params[1] as usize, data.clone());
      // If verbose, print image layers
      if verbose {
        for i in 0..image.get_layers_count() {
          println!("Layer {}:", i);
          println!("{}", image.to_string(i, None));
        }
      }
      // Find min-zeros layer and count pixel characters
      let mut min_zeros_counts = vec![std::u32::MAX, 0, 0, 0, 0, 0, 0, 0, 0, 0];
      for layer in 0..image.get_layers_count() {
        // Initialize layer counts
        let mut counts = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        // Count diffferent pixel characters for a layer
        for index in 0..(image.get_height() * image.get_width()) {
          // Read pixel
          let pixel = image.read_by_index(layer, index);
          // Count pixel
          counts[(pixel - ('0' as u8)) as usize] += 1;
        }
        // Compare to current min count
        if counts[0] < min_zeros_counts[0] {
          min_zeros_counts = counts;
        }
      }
      // Output checksum
      Ok((min_zeros_counts[1] * min_zeros_counts[2]) as i32)
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<u8, SpaceImage, u64>, verbose: bool) -> Result<SpaceImage, &str> {
  match &puzzle.input {
    PuzzleInput::ParamsVector1D(params, data) => {
      // Instantiate the image
      let image = SpaceImage::new(params[0] as usize, params[1] as usize, data.clone());
      let image_mapping = SpaceImage::get_default_mapping();
      let image_mapping_option = if params[2] == 1 { Some(&image_mapping) } else { None };
      // If verbose, print image layers
      if verbose {
        for i in 0..image.get_layers_count() {
          println!("Layer {}:", i);
          println!("{}", image.to_string(i, image_mapping_option));
        }
      }
      // Collapse image
      let collapsed_image = image.collapse_layers(50);
      // If verbose, print collapsed image layers
      if verbose {
        println!("Collapsed:");
        println!("{}", collapsed_image.to_string(0, image_mapping_option));
      }
      // Return collapsed image
      Ok(collapsed_image)
    },
    _ => panic!("This shouldn't ever happen!")
  }
}
