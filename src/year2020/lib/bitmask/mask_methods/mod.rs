//! BitMask .mask() and related implementations
//! 
//! Implements bit-masking functionality 
// -----------------------------------------------------------------------------

// Include dependencies
use super::*;

/// BitMask .mask() implementation
/// 
/// Implements bit-masking functionality 
impl BitMask {
  
  /// Masks a value using by applying bits from the mask except for positions where mask bit equals the transparent_character char 
  /// 
  /// # Arguments
  /// 
  /// * `value`                 - Value to mask
  /// * `mask`                  - Mask to apply
  /// * `transparent_character` - Character that will copy over bits from the value being masked
  pub fn mask (value: &Vec<u8>, mask: &Vec<u8>, transparent_character: char) -> Vec<u8> {
    return (0..value.len()).map(|i| {
      return if mask[i] == (transparent_character as u8) { value[i] } else { mask[i] }
    }).collect();
  }

  /// Generates all possible permutations of a mask with undefined bits
  /// 
  /// # Arguments
  /// * `masked_value`        - Mask value including a transparent character to use in generation of permutations
  /// * `undefined_character` - Character in the mask which will needs to be replaced with '0' and '1' when generating permutations
  pub fn permutate_mask (masked_value: Vec<u8>, undefined_character: char) -> Vec<Vec<u8>> {
    // Replace floating bits (recursively) with 0 and 1
    if masked_value.contains(&(undefined_character as u8)) {
      // Initialize addresses
      let mut values: Vec<Vec<u8>> = vec![];
      let index = masked_value.iter().position(|bit| bit == &(undefined_character as u8)).expect("Failed finding index of transparent char!");

      // Replace first remaining 'X' with 1 and recursively handle the rest
      let replaced = (0..masked_value.len()).map(|i| if i == index { '1' as u8 } else { masked_value[i] }).collect();
      for address in BitMask::permutate_mask(replaced, undefined_character) {
        values.push(address);
      }
      // Replace first remaining 'X' with 0 and recursively handle the rest
      let replaced = (0..masked_value.len()).map(|i| if i == index { '0' as u8 } else { masked_value[i] }).collect();
      for address in BitMask::permutate_mask(replaced, undefined_character) {
        values.push(address);
      }

      // Return generated addresses
      return values;
    }
    
    // Return fully composed value
    else {
      return vec![masked_value];
    }
  }

  /// Evaluates a binary number
  /// 
  /// # Arguments
  /// * `value` - Binary string to be evaluated as a number
  pub fn evaluate (value: Vec<u8>) -> usize {
    let binary: String = value.iter().map(|bit| bit.clone() as char).collect();
    return usize::from_str_radix(&binary[..], 2).expect("Failed parsing binary number!");
  }

}
