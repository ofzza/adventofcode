//! 2022 day 13 puzzle
//! 
//! https://adventofcode.com/2022/day/13
// -----------------------------------------------------------------------------

// Include dependencies
use std::cmp::Ordering;
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2022::lib::distress_signal::*;

/// Parses input data
fn parse<'a>(data: &'a String) -> Vec<Vec<&str>> {
  Input::parse(data.as_str().trim(), "\n\n", |data| {
    Input::parse(data, "\n", |data| data)
  })
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 13,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Parse and compare packet pairs
      let mut result = 0;
      for i in 0..data.len() {
        // Parse packets pair
        let a = DistressSignalPacket::parse(data[i][0]);
        let b = DistressSignalPacket::parse(data[i][1]);
        // Compare packets pair
        let correct = DistressSignalPacket::compare(&a, &b);
        if correct == Ordering::Less {
          result += i + 1;
        }
      }

      // Return result
      String::from(format!("{:?}", result))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 13,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);
      
      // Join all packets, regardless or pairs
      let mut packets: Vec<DistressSignalPacketValue> = vec![
        DistressSignalPacket::parse("[[2]]"),
        DistressSignalPacket::parse("[[6]]"),
      ];
      for i in 0..data.len() {
        // Parse packets pair
        packets.push(DistressSignalPacket::parse(data[i][0]));
        packets.push(DistressSignalPacket::parse(data[i][1]));
      }

      // Sort packets
      packets.sort_by(|a, b| DistressSignalPacket::compare(a, b));

      // Find inserted "separator" packets
      let mut a: usize = 0;
      let mut b: usize = 0;
      for i in 0..packets.len() {
        match &packets[i] {
          DistressSignalPacketValue::List(packets) => if packets.len() == 1 {
            match &packets[0] {
              DistressSignalPacketValue::List(packets) => if packets.len() == 1 {
                match &packets[0] {
                  DistressSignalPacketValue::Number(value) => {
                    if value == &2 { a = i; }
                    if value == &6 { b = i; }
                  },
                  _ => ()
                }
              },
              _ => ()
            }
          },
          _ => ()
        };
      }

      // Return result
      String::from(format!("{:?}", (a + 1) * (b + 1)))
    }

  );

  // Return registry ownership
  registry
}
