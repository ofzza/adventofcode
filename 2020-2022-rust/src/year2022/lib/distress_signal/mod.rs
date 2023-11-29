//! Distress Signal Packet Data module
//! 
//! Distress Signal Packet Data module
// -----------------------------------------------------------------------------

// Include dependencies
use std::cmp::Ordering;

/// Distress Signal Packet Data item enum
#[derive(Debug)]
pub enum DistressSignalPacketValue {
  Number(usize),
  List(Vec<DistressSignalPacketValue>)
}

/// Distress Signal Packet Data structure
pub struct DistressSignalPacket {}

/// Distress Signal Packet Data implementation
impl DistressSignalPacket {

  /// Parses a string representation of a distress signal packet into a data model
  /// 
  /// # Arguments
  /// * data: String representation of a distress signal packet
  /// 
  /// # Returns 
  /// * Parsed DistressSignalPacketValue instance
  pub fn parse (data: &str) -> DistressSignalPacketValue {
    // Parse data
    let (result, remainder) = DistressSignalPacket::parse_section(data.to_string());
    // Verify data remainder
    if remainder.len() != 0 && remainder != "]" {
      panic!("Unable to fully parse data!");
    }
    // Return parsed data
    result
  }

  /// Parses a partial string representation of a distress signal packet into a data model and remainder
  /// 
  /// # Arguments
  /// * data: PArtial string representation of a distress signal packet (starting with a "[" character)
  /// 
  /// # Returns 
  /// * Tuple of parsed DistressSignalPacketValue instance and the string representation remainder
  fn parse_section (data: String) -> (DistressSignalPacketValue, String) {
    // Make sure data being parsed is a list starting with a '[' character
    if data.len() == 0 || !data.starts_with('[') {
      panic!(".parse_section(data) should only be called with data starting with a '[' character!");
    }
    // Make data mutable
    let mut data = data.to_string();
    // Clear superfluous leading '[' char
    data = data[1..].to_string();        
    
    // Initialize resulting list
    let mut result: Vec<DistressSignalPacketValue> = vec![];

      // Parse item-by-item until the end or unmatched cosing brackets
    while data.len() > 0 {
      
      // Clear superfluous leading ',' char
      if data.starts_with(',') {
        data = data[1..].to_string();        
      }

      // Check if currently parsing list is at an end
      if data.starts_with(']') {
        return (DistressSignalPacketValue::List(result), data[1..].to_string())
      }
      // Parse next value in list as a list
      else if data.len() > 0 && data.starts_with('[') {
        // Parse next value as a list
        let (list, remainder) = DistressSignalPacket::parse_section(data);
        // Store next item
        result.push(list);
        // Use remainer as data for continuation
        data = remainder;
      }
      // Parse next value in list as a number
      else if data.len() > 0 {
        // Find end of value
        match data.find(|c| (c == ',') || (c == ']')) {
          // Only numeric value remaining
          None => {
            // Parse number
            let num = data.trim().parse::<usize>().unwrap();
            result.push(DistressSignalPacketValue::Number(num));
            // No data remaining
            data = "".to_string();
          },
          // Extract and parse next numeric value
          Some(i) => {
            // Parse number
            let num = data[..i].trim().parse::<usize>().unwrap();
            result.push(DistressSignalPacketValue::Number(num));
            // Set remaining data
            data = data[i..].to_string();
          }
        }
      }

    }

    // Return collected result
    panic!("Never should happen! SHould have returned when finding a ']' character!");
    // (DistressSignalPacketValue::List(result), "".to_string())
  }

  /// Compares two instances of DistressSignalPacketValue
  /// 
  /// # Arguments
  /// * low: First instance to compare, expected to be lower
  /// * high: Second instance to compare, expected to be higher
  /// 
  /// # Returns
  /// * +1 if first value is lower than the second value
  /// * -1 if first value is higher than the second value
  /// *  0 if first value is equal to the second value
  pub fn compare (low: &DistressSignalPacketValue, high: &DistressSignalPacketValue) -> Ordering {
    // Check first value's type
    match low {
      DistressSignalPacketValue::Number(low_num) => match high {
        // Match 2 numbers
        DistressSignalPacketValue::Number(high_num) => {
          if low_num < high_num { Ordering::Less } else if low_num > high_num { Ordering::Greater } else { Ordering::Equal }
        },
        // Match first value being a number to second value being a list
        DistressSignalPacketValue::List(_) => {
          DistressSignalPacket::compare(&DistressSignalPacketValue::List(vec![DistressSignalPacketValue::Number(low_num.clone())]), high)
        }
      },
      DistressSignalPacketValue::List(low_vec) => match high {
        // Match first value being a list to second value being a number
        DistressSignalPacketValue::Number(high_num) => {
          DistressSignalPacket::compare(low, &DistressSignalPacketValue::List(vec![DistressSignalPacketValue::Number(high_num.clone())]))
        },
        // Match 2 lists
        DistressSignalPacketValue::List(high_vec) => {
          // Compare members
          for i in 0..low_vec.len().min(high_vec.len()) {
            let comprison = DistressSignalPacket::compare(&low_vec[i], &high_vec[i]);
            if comprison != Ordering::Equal {
              return comprison;
            }
          }
          // Equal matching values, judge by length
          if low_vec.len() < high_vec.len() { Ordering::Less } else if low_vec.len() > high_vec.len() { Ordering::Greater } else { Ordering::Equal }
        }
      }
    }
  }
}
