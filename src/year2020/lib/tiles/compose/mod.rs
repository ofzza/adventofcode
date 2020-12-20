//! Image Tiles ::link() and ::compose() implementations
//! 
//! Image tiling linking and composition implementation
// -----------------------------------------------------------------------------

// Include dependencies
use super::*;
use std::collections::HashMap;

/// Image Tiles ::link() and ::compose() implementations
/// 
/// Image tiling linking and composition implementation
impl ImageTile {
  
  /// Links tiles together by their borders
  /// 
  /// # Arguments
  ///
  /// * `tiles` - All tiles available for linking
  pub fn link (mut tiles: Vec<ImageTile>) -> Vec<ImageTile> {    
    // Organize tiles by their borders
    let borders_hashmap: HashMap<String, Vec<usize>> = ImageTile::borders(&tiles);

    // Find bordering tiles for every tile
    for tile_index in 0..tiles.len() {
      // Get tile by index
      let tile = &mut tiles[tile_index];
      tile.bordering_tiles_ids.clear();
      // Find bordering tiles
      for border_index in 0..4 {
        // Get border
        let border = tile.borders[border_index].clone();
        // Search for bordering tiles
        let mut found_border = false;
        let found_tile_ids = borders_hashmap.get(&border).unwrap();
        for found_tile_id in found_tile_ids {
          if found_tile_id.clone() != tile.id {
            tile.bordering_tiles_ids.push(Some(found_tile_id.clone()));
            found_border = true;
            break;
          }
        }
        // If border item not found, add empty
        if !found_border {
          tile.bordering_tiles_ids.push(None);
        }
      }
    }

    // Return composed tiles
    return tiles;
  }

  /// Composes linked tiles into an image
  /// 
  /// # Arguments
  /// 
  /// * `tiles`   - Linked tiles available for composition
  /// * `verbose` - Outputs executing output of the puzzle to the console
  pub fn compose (tiles: Vec<ImageTile>, verbose: bool) -> ImageData {
    // Initialize transformation codes
    let mut transformation_codes: Vec<&str> = vec![];

    // Get tiles by tile ID
    let mut tiles_by_id: HashMap<usize, ImageTile> = HashMap::new();
    for tile in tiles.clone() {
      tiles_by_id.insert(tile.id, tile);
    }
    
    // Organize tiles by their borders
    let borders_hashmap: HashMap<String, Vec<usize>> = ImageTile::borders(&tiles);

    // Find a corner tile
    let mut seed_tile: ImageTile = tiles.iter().find(|tile| {
      let borders_count = if !tile.bordering_tiles_ids[0].is_none() { 1 } else { 0 }
        + if !tile.bordering_tiles_ids[1].is_none() { 1 } else { 0 }
        + if !tile.bordering_tiles_ids[2].is_none() { 1 } else { 0 }
        + if !tile.bordering_tiles_ids[3].is_none() { 1 } else { 0 };
      return borders_count == 2;
    }).unwrap().clone();

    // Rotate into top-left position
    if !seed_tile.bordering_tiles_ids[0].is_none() && !seed_tile.bordering_tiles_ids[1].is_none() {
      seed_tile.rotate(90);
      transformation_codes.push("90");
    }
    if !seed_tile.bordering_tiles_ids[1].is_none() && !seed_tile.bordering_tiles_ids[2].is_none() {
      transformation_codes.push("0");
    }
    if !seed_tile.bordering_tiles_ids[2].is_none() && !seed_tile.bordering_tiles_ids[3].is_none() {
      seed_tile.rotate(270);
      transformation_codes.push("270");
    }
    if !seed_tile.bordering_tiles_ids[3].is_none() && !seed_tile.bordering_tiles_ids[0].is_none() {
      seed_tile.rotate(180);
      transformation_codes.push("180");
    }

    // Initialize image
    let mut tiled_image: Vec<Vec<ImageTile>> = vec![];
    let mut pixels: Vec<bool> = vec![];
    loop {

      // Initialize image row
      let mut tiled_image_row: Vec<ImageTile> = if tiled_image.len() == 0 { vec![seed_tile.clone()] } else { vec![] };
      
      // If not first row, find first item based on item above
      if tiled_image_row.len() == 0 {
        // Get anchor tile
        let anchor_tile = tiled_image.last().unwrap()[0].clone();
        // Find next tile
        let bottom_border = anchor_tile.borders[2].clone();
        match borders_hashmap.get(&bottom_border) {
          Some(bottom_tile_ids) => {

            // Get bottom tile
            let bottom_tile_id = match bottom_tile_ids.iter().find(|id| id.clone().clone() != anchor_tile.id) {
              Some(id) => id,
              None => break
            };
            let mut bottom_tile = tiles_by_id.get(bottom_tile_id).unwrap().clone();

            // Find matching border position
            let reversed: bool;
            let position: usize;
            if bottom_tile.borders.contains(&bottom_border) {
              // Find matching border position
              reversed = false;
              position = bottom_tile.borders.iter().position(|border| border.clone() == bottom_border).unwrap();
            } else {
              // Find matching reverse border position
              reversed = true;
              position = bottom_tile.borders.iter().position(|border| border.chars().rev().collect::<String>() == bottom_border).unwrap();
            };

            // Transform image to fit
            if position == 0 && !reversed {
              bottom_tile.flip_horizontally();
              transformation_codes.push("H");
            } else if position == 0 && reversed {
              transformation_codes.push("");
            } else if position == 1 && !reversed {
              bottom_tile.flip_vertically();
              bottom_tile.rotate(270);
              transformation_codes.push("V,270");
            } else if position == 1 && reversed {
              bottom_tile.rotate(270);
              transformation_codes.push("270");
            } else if position == 2 && !reversed {
              bottom_tile.flip_horizontally();
              bottom_tile.rotate(180);
              transformation_codes.push("H,180");
            } else if position == 2 && reversed {
              bottom_tile.rotate(180);
              transformation_codes.push("180");
            } else if position == 3 && !reversed {
              bottom_tile.flip_vertically();
              bottom_tile.rotate(90);
              transformation_codes.push("V,90");
            } else if position == 3 && reversed {
              bottom_tile.rotate(90);
              transformation_codes.push("90");
            }

            // Append tile to image
            tiled_image_row.push(bottom_tile);
          },
          None => break
        }
      }

      // Fill image row
      loop {
        // Get anchor tile
        let anchor_tile = tiled_image_row.last().unwrap();
        // Find next tile
        let right_border = anchor_tile.borders[1].clone();
        match borders_hashmap.get(&right_border) {
          Some(right_tile_ids) => {

            // Get right tile
            let right_tile_id = match right_tile_ids.iter().find(|id| id.clone().clone() != anchor_tile.id) {
              Some(id) => id,
              None => break
            };
            let mut right_tile = tiles_by_id.get(right_tile_id).unwrap().clone();

            // Find matching border position
            let reversed: bool;
            let position: usize;
            if right_tile.borders.contains(&right_border) {
              // Find matching border position
              reversed = false;
              position = right_tile.borders.iter().position(|border| border.clone() == right_border).unwrap();
            } else {
              // Find matching reverse border position
              reversed = true;
              position = right_tile.borders.iter().position(|border| border.chars().rev().collect::<String>() == right_border).unwrap();
            };

            // Transform image to fit
            if position == 0 && !reversed {
              right_tile.flip_horizontally();
              right_tile.rotate(270);
              transformation_codes.push("H,270");
            } else if position == 0 && reversed {
              right_tile.rotate(270);
              transformation_codes.push("270");
            } else if position == 1 && !reversed {
              right_tile.flip_vertically();
              right_tile.rotate(180);
              transformation_codes.push("V,180");
            } else if position == 1 && reversed {
              right_tile.rotate(180);
              transformation_codes.push("180");
            } else if position == 2 && !reversed {
              right_tile.flip_horizontally();
              right_tile.rotate(90);
              transformation_codes.push("H,90");
            } else if position == 2 && reversed {
              right_tile.rotate(90);
              transformation_codes.push("90");
            } else if position == 3 && !reversed {
              right_tile.flip_vertically();
              transformation_codes.push("V");
            } else if position == 3 && reversed {
              transformation_codes.push("");
            }

            // Append tile to image
            tiled_image_row.push(right_tile);
          },
          None => break
        }
      }

      // Append row to image
      tiled_image.push(tiled_image_row);

    }

    // Compose single image data
    let image_width = tiled_image[0].len() * tiled_image[0][0].data.dimensions.0;
    let image_height = tiled_image.len() * tiled_image[0][0].data.dimensions.1;
    let tile_width = tiled_image[0][0].data.dimensions.0;
    let tile_height = tiled_image[0][0].data.dimensions.1;
    for y in 0..(tiled_image.len() * tile_height) {

      // Calculate coordinates
      let tile_coords_y = y / tile_height;
      let pixel_coords_y = y % tile_height;

      // Prompt border
      if verbose && pixel_coords_y == 0 {
        for i in 0..tiled_image[0].len() {
          let border = tiled_image[tile_coords_y][i].borders[0].clone();
          print!("{}|{}|{}", border[..1].to_string(), border[1..(border.len() - 1)].to_string(), border[(border.len() - 1)..].to_string());
        }
        println!("");
        for _ in 0..tiled_image[0].len() {
          print!("-+{}+-", vec!["-"; tile_width].join(""));
        }
        println!("");
        for i in 0..tiled_image[0].len() {
          let code = transformation_codes[tile_coords_y * tiled_image[0].len() + i];
          print!("-+ {} {} +-", code, vec!["-"; tile_width - code.len() - 3].join(""))
        }
        println!("");
      }

      // Process line
      for x in 0..(tiled_image[0].len() * tile_width) {

        // Calculate coordinates
        let tile_coords_x = x / tile_width;
        let pixel_coords_x = x % tile_width;
        let pixel = tiled_image[tile_coords_y][tile_coords_x].get_pixel(pixel_coords_x,pixel_coords_y);

        // Add pixel to image
        pixels.push(pixel);

        // Prompt border
        if verbose && pixel_coords_x == 0 {
          print!("{}|", tiled_image[tile_coords_y][tile_coords_x].borders[3].as_bytes()[tile_height - pixel_coords_y] as char);
        }
        // Prompt pixel
        if verbose {
          print!("{}", if pixel { "#" } else { "." });
        }
        // Prompt border
        if verbose && pixel_coords_x == tile_width - 1 {
          print!("|{}", tiled_image[tile_coords_y][tile_coords_x].borders[1].as_bytes()[pixel_coords_y + 1] as char);
        }
      }

      // Prompt line-end
      if verbose {
        println!("");
      }

      // Prompt border
      if verbose && pixel_coords_y == tile_height - 1 {
        for _ in 0..tiled_image[0].len() {
          print!("-+{}+-", vec!["-"; tile_width].join(""));
        }
        println!("");
        for i in 0..tiled_image[0].len() {
          let border = tiled_image[tile_coords_y][i].borders[2].chars().rev().collect::<String>();
          print!("{}|{}|{}", border[..1].to_string(), border[1..(border.len() - 1)].to_string(), border[(border.len() - 1)..].to_string());
        }
        println!("");
      }

    }
    // Prompt line-end
    if verbose {
      println!("");
    }
    // Compose a single image
    let image = ImageData {
      dimensions: (image_width, image_height),
      pixels
    };

    // Prompt image
    if verbose {
      println!("{}", image.to_string());
    }

    // Return composed single image
    return image;
  }

  /// Organizes tiles into a hashmap by their borders (and reversed borders)
  /// 
  /// # Arguments
  /// 
  /// * `tiles` - All tiles to be organized
  fn borders (tiles: &Vec<ImageTile>) -> HashMap<String, Vec<usize>> {
    // Initialize borders hashmap
    let mut borders_hashmap: HashMap<String, Vec<usize>> = HashMap::new();
    
    // Organize tiles by their borders
    for tile in tiles {
      for border in tile.borders.iter() {
        // Compose border and reverse border strings
        let border_string = border.clone();
        let border_string_reversed = border.chars().rev().collect::<String>();
        // Update hashmap for border and border reversed values
        for border in vec![border_string, border_string_reversed] {
          // Initialize hashmap
          if !borders_hashmap.contains_key(&border) {
            borders_hashmap.insert(border.clone(), vec![]);
          }
          // Take tile collection from hashmap
          let mut tiles = borders_hashmap.remove(&border).unwrap();
          // Append collection
          tiles.push(tile.id);
          // Put collection back to hashmap
          borders_hashmap.insert(border.clone(), tiles);
        }
      }
    }

    // Return borders hashmap
    return borders_hashmap;
  }

}
