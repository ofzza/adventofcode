//! 2020/20 puzzle
//! 
//! https://adventofcode.com/2020/day/20
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::inputs::*;
use crate::lib::puzzle::*;
use super::ImageTile;
use super::ImageData;

/// Registers puzzles for the day
pub fn run (index: u32, key: &str, verbose: bool, obfuscate: bool) -> PuzzleExecutionStatistics {

  // Initialize stats
  let mut stats = PuzzleExecutionStatistics{
    ..Default::default()
  };

  // Initialize input
  let input = vec![
    String::from("Tile 2311:|..##.#..#.|##..#.....|#...##..#.|####.#...#|##.##.###.|##...#.###|.#.#.#..##|..#....#..|###...#.#.|..###..###").replace("|", "\n"),
    String::from("Tile 1951:|#.##...##.|#.####...#|.....#..##|#...######|.##.#....#|.###.#####|###.##.##.|.###....#.|..#.#..#.#|#...##.#..").replace("|", "\n"),
    String::from("Tile 1171:|####...##.|#..##.#..#|##.#..#.#.|.###.####.|..###.####|.##....##.|.#...####.|#.##.####.|####..#...|.....##...").replace("|", "\n"),
    String::from("Tile 1427:|###.##.#..|.#..#.##..|.#.##.#..#|#.#.#.##.#|....#...##|...##..##.|...#.#####|.#.####.#.|..#..###.#|..##.#..#.").replace("|", "\n"),
    String::from("Tile 1489:|##.#.#....|..##...#..|.##..##...|..#...#...|#####...#.|#..#.#.#.#|...#.#.#..|##.#...##.|..##.##.##|###.##.#..").replace("|", "\n"),
    String::from("Tile 2473:|#....####.|#..#.##...|#.##..#...|######.#.#|.#...#.#.#|.#########|.###.#..#.|########.#|##...##.#.|..###.#.#.").replace("|", "\n"),
    String::from("Tile 2971:|..#.#....#|#...###...|#.#.###...|##.##..#..|.#####..##|.#..####.#|#..#.#..#.|..####.###|..#.#.###.|...#.#.#.#").replace("|", "\n"),
    String::from("Tile 2729:|...#.#.#.#|####.#....|..#.#.....|....#..#.#|.##..##.#.|.#.####...|####.#.#..|##.####...|##..#.##..|#.##...##.").replace("|", "\n"),
    String::from("Tile 3079:|#.#.#####.|.#..######|..#.......|######....|####.#..#.|.#...#.##.|#.#####.##|..#.###...|..#.......|..#.###...").replace("|", "\n")
  ];

  // Run puzzle
  if index == 9 {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector1D(vec![
        vec![
          String::from("Tile 0:"),
          String::from(".#........"),
          String::from(".#########"),
          String::from(".##......#"),
          String::from(".#.#......"),
          String::from(".#..#....."),
          String::from("##...#...."),
          String::from("##....#..."),
          String::from("##.....#.."),
          String::from("##......#."),
          String::from("......###.")
        ].join("\n")
      ]);
      stats.update(
        Puzzle::new(2020, 20, 0, "test", input, run_tests, |r| (r, Some(0)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Run puzzle
  if (index == 0) || (index == 1) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector1D(input.clone());
      stats.update(
        Puzzle::new(2020, 20, 1, "test", input, implementation1, |r| (r, Some(20899048083289)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<String>(load_input("./src/year2020/data/day20input.txt"), "\n\n");
      stats.update(
        Puzzle::new(2020, 20, 1, "solution", input, implementation1, |r| (r, Some(18449208814679)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Run puzzle
  if (index == 0) || (index == 2) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector1D(input.clone());
      stats.update(
        Puzzle::new(2020, 20, 2, "test", input, implementation2, |r| (r, Some(273)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<String>(load_input("./src/year2020/data/day20input.txt"), "\n\n");

      stats.update(
        Puzzle::new(2020, 20, 2, "solution", input, implementation2, |r| (r, Some(1559)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn run_tests (puzzle: &Puzzle<String, usize, usize>, verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(input) => {
      // Parse, link and compose tiles into an image
      let tiles: Vec<ImageTile> = input.iter().map(|input| ImageTile::new(input)).collect();

      // Output rotations
      if verbose {
        println!("Rotate:");
        for i in 0..4 {
          let mut tile = tiles[0].clone();
          tile.rotate(i * 90);
          println!("{}... borders: {:?}\n", tile.data.to_string(), tile.borders.iter().map(|border| border.replace(" ", ".")).collect::<Vec<String>>());
        }
        println!("Flip vertically:");
        {
          let mut tile = tiles[0].clone();
          tile.flip_vertically();
          println!("{}\n... borders: {:?}\n", tile.data.to_string(), tile.borders.iter().map(|border| border.replace(" ", ".")).collect::<Vec<String>>());
        }
        println!("Flip horizontally:");
        {
          let mut tile = tiles[0].clone();
          tile.flip_horizontally();
          println!("{}... borders: {:?}\n", tile.data.to_string(), tile.borders.iter().map(|border| border.replace(" ", ".")).collect::<Vec<String>>());
        }
      }

      // Return unmatched pixels
      return Ok(0);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation1 (puzzle: &Puzzle<String, usize, usize>, _verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(input) => {
      // Parse and link tiles
      let tiles: Vec<ImageTile> = input.iter().map(|input| ImageTile::new(input)).collect();
      let tiles: Vec<ImageTile> = ImageTile::link(tiles);

      // Find corners
      let mut checksum = 1;
      for tile in tiles {
        // Count bordering tiles
        let mut bordering_tiles_count = 0;
        for bordering_tile_id in tile.bordering_tiles_ids {
          match bordering_tile_id {
            Some(_) => bordering_tiles_count += 1,
            None => ()
          }
        }
        // Check if corner tile (found 2 bordering tiles)
        if bordering_tiles_count == 2 {
          checksum *= tile.id;
        }
      }

      // Return checksum
      return Ok(checksum);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<String, usize, usize>, verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(input) => {
      // Parse, link and compose tiles into an image
      let tiles: Vec<ImageTile> = input.iter().map(|input| ImageTile::new(input)).collect();
      let tiles: Vec<ImageTile> = ImageTile::link(tiles);
      let mut image = ImageTile::compose(tiles, verbose);
      
      // Search for needle pattern and count unmatched pixels
      let count = image.search(
        ImageData {
          dimensions: (20, 3),
          pixels: vec![
            String::from("                  # "),
            String::from("#    ##    ##    ###"),
            String::from(" #  #  #  #  #  #   ")
          ].join("").chars().map(|c| c.clone() == '#').collect::<Vec<bool>>()
        }
      );

      // Return unmatched pixels
      return Ok(count);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}
