//! Special Numeral-Analogue Fuel Units module
//! 
//! Special Numeral-Analogue Fuel Units module
// -----------------------------------------------------------------------------

// Include dependecies
// ...

/// Special Numeral-Analogue Fuel Units structure
pub struct SNAFU {
}

/// Special Numeral-Analogue Fuel Units implementation
impl SNAFU {

  /// Encode a decomally encoded number
  /// 
  /// # Arguments
  /// * value: Decomal value to encode
  /// 
  /// # Returns
  /// A SNAFU value encoded from the decomally encoded value provided
  pub fn encode (value: isize) -> String {
    // Initialize encoeded value
    let mut encoded: Vec<isize> = vec![];

    // Encoded digits of typical base-5 system
    let mut value: isize = value;
    let mut pow: isize = (value as f64).log(5.0).floor() as isize ;
    while pow >= 0 {
      // Calculate POW of 5 value
      let pow_value = (5 as usize).pow(pow as u32) as f64;
      // Calculate next digit
      let digit = (value as f64 / pow_value).floor() as isize;
      value -= digit * pow_value as isize;
      encoded.push(digit);
      // Next digit POW of 5
      pow -= 1;
    }

    // Re-encode digits as SNAFU digits
    let mut reencoded = encoded.clone();
    reencoded.splice(0..0, [0 as isize]);
    for i in 0..(reencoded.len() - 1) {
      // Reencode digit
      if reencoded[i + 1] > 2 {
        // Reencode and carry ...
        for j in (0..(i + 1)).rev() {
          if reencoded[j + 1] > 2 {
            reencoded[j] += 1;
            reencoded[j + 1] = reencoded[j + 1] - 5;
          }
        }
      }
    }

    // Return digits
    let mut rereencoded: String = "".to_string();
    for digit in reencoded {
      if digit != 0 || rereencoded.len() > 0 {
        rereencoded = rereencoded + match digit {
           2 => "2",
           1 => "1",
           0 => "0",
          -1 => "-",
          -2 => "=",
          _ => panic!("'{}' Digit is not recognized as a SNAFU digit!", digit)
        };
      }
    }
    rereencoded
  }

  /// Decoes a SNAFU encoded number
  /// 
  /// # Arguments
  /// * value: Encoded value to decode
  /// 
  /// # Returns
  /// A decimal value decoded from the SNAFU encoded value provided
  pub fn decode (value: &str) -> isize {
    // Initialieze decoded value
    let mut decoded: isize = 0;

    // Decode values
    let chars = value.chars().rev().collect::<Vec<char>>();
    for i in 0..chars.len() {
      // Get character and process into the encoded value
      match chars[i] {
        '2' => decoded +=  2 as isize * ((5 as  usize).pow(i as u32) as isize),
        '1' => decoded +=  1 as isize * ((5 as  usize).pow(i as u32) as isize),
        '0' => decoded +=  0 as isize,
        '-' => decoded += -1 as isize * ((5 as  usize).pow(i as u32) as isize),
        '=' => decoded += -2 as isize * ((5 as  usize).pow(i as u32) as isize),
        _ => panic!("'{}' Digit is not recognized as a SNAFU digit!", chars[i])
      }
    }
    
    // Return decoded value
    decoded
  }

}
