//! 2021 day 16 puzzle
//! 
//! https://adventofcode.com/2021/day/16
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::year2021::lib::bits::*;

/// Parses input data
fn parse(data: &String) -> Vec<char> {
  data.chars().collect()
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 16,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Parse data into packets
      let bin = BITS::decode_hex(&data);
      let root_packet = BITS::parse(&bin);

      // Walk the nested packages
      let mut output: usize = 0;
      BITS::map(&root_packet, &mut output, |p, output| {
        *output += p.info.packet_version;
      });

      // Calculate and return result
      String::from(format!("{:?}", output))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 16,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Parse data into packets
      let bin = BITS::decode_hex(&data);
      let root_packet = BITS::parse(&bin);

      // Calculate and return result
      String::from(format!("{:?}", root_packet.content.content_value))
    }

  );

  // Return registry ownership
  registry
}
