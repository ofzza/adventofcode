//! Segment display module
//! 
//! Implements segmented display decoding functionality
// -----------------------------------------------------------------------------

/// Segment display struct
pub struct SegmentDisplay {
  signal_mapping: Vec<usize>
}
/// Segment display implementation
impl SegmentDisplay {

  /// Constructor
  pub fn new () -> SegmentDisplay {
    SegmentDisplay {
      signal_mapping: Vec::with_capacity(10)
    }
  }

  /// Converts a signal from a character to a numeric representation
  /// 
  /// # Arguments
  /// * c: Character representation of a signal
  /// 
  /// # Returns
  /// A numeric representation of a signal
  pub fn char_to_num (c: &char) -> usize {
    c.clone() as usize - 'a' as usize
  }
  /// Converts a signal from a number to a character representation
  /// 
  /// # Arguments
  /// * n: Numeric representation of a signal
  /// 
  /// # Returns
  /// A character representation of a signal
  pub fn num_to_char (n: &usize) -> char {
    (n.clone() + 'a' as usize) as u8 as char
  }

  /// Programs the display by providing it all digits to decode signals from
  /// 
  /// # Arguments
  /// * digits: Vector of vectors of signals for all possible digits
  pub fn program (&mut self, digits: &Vec<Vec<char>>) {
    // Reset programed digits
    self.signal_mapping = Vec::with_capacity(7);
    for _ in 0..7 {
      self.signal_mapping.push(25);
    }

    // Program "1"
    let digit_1_signals: &Vec<char> = digits.iter().find(|&signs| {
      signs.len() == 2
    }).unwrap();

    // Program "7" (and get segment 0)
    let digit_7_signals: &Vec<char> = digits.iter().find(|&signs| {
      signs.len() == 3
    }).unwrap();
    let segment_0: &char = &digit_7_signals.iter().find(|&sign| {
      digit_1_signals.iter().find(|&digit_1_signal| { &sign == &digit_1_signal }) == None
    }).unwrap().clone();
    self.signal_mapping[SegmentDisplay::char_to_num(&segment_0)] = 0;

    // Program "4"
    let digit_4_signals: &Vec<char> = digits.iter().find(|&signs| {
      signs.len() == 4
    }).unwrap();
    
    // Program "3" (and get segments 3 and 6)
    let digit_3_signals: &Vec<char> = digits.iter().find(|&signs| {
      signs.len() == 5
      && signs.iter().find(|&sign| { sign == &digit_1_signals[0] }) != None
      && signs.iter().find(|&sign| { sign == &digit_1_signals[1] }) != None
    }).unwrap();
    let segment_3_options: Vec<&char> = digit_3_signals.iter().filter(|&sign| {
      digit_1_signals.iter().find(|&digit_1_signal| { &sign == &digit_1_signal }) == None
    }).collect::<Vec<&char>>().clone();
    let segment_3: &char = segment_3_options.iter().find(|&sign| {
      digit_4_signals.iter().find(|&digit_4_signal| { sign == &digit_4_signal }) != None
    }).unwrap().clone();
    self.signal_mapping[SegmentDisplay::char_to_num(segment_3)] = 3;
    let segment_6_options: Vec<&char> = segment_3_options.clone();
    let segment_6 = segment_6_options.iter().find(|&sign| {
      sign != &segment_0 && sign != &segment_3
    }).unwrap().clone();
    self.signal_mapping[SegmentDisplay::char_to_num(segment_6)] = 6;

    // Program 2 (and get signals 2 and 4)
    let digit_2_signals: &Vec<char> = digits.iter().find(|&signs| {
      signs.len() == 5
      && signs.iter().filter(|&sign| { digit_4_signals.iter().find(|&digit_4_signal| { sign == digit_4_signal }) != None }).collect::<Vec<&char>>().len() == 2
    }).unwrap();
    let segment_2: &char = &digit_2_signals.iter().find(|&sign| {
      &sign != &segment_0 && &sign != &segment_3 && &sign != &segment_6 && digit_1_signals.iter().find(|&digit_1_signal| { sign == digit_1_signal }) != None
    }).unwrap().clone();
    self.signal_mapping[SegmentDisplay::char_to_num(segment_2)] = 2;
    let segment_4: &char = &digit_2_signals.iter().find(|&sign| {
      &sign != &segment_0 && &sign != &segment_3 && &sign != &segment_6 && &sign != &segment_2
    }).unwrap().clone();
    self.signal_mapping[SegmentDisplay::char_to_num(segment_4)] = 4;

    // Program 5 (and get signals 1 and 5)
    let digit_5_signals: &Vec<char> = digits.iter().find(|&signs| {
      signs.len() == 5
      && signs.iter().find(|&sign| { sign == segment_2 }) == None
      && signs.iter().find(|&sign| { sign == segment_4 }) == None
    }).unwrap();
    let segment_5: &char = &digit_5_signals.iter().find(|&sign| {
      &sign != &segment_0 && &sign != &segment_3 && &sign != &segment_6 && digit_1_signals.iter().find(|&digit_1_signal| { sign == digit_1_signal }) != None
    }).unwrap().clone();
    self.signal_mapping[SegmentDisplay::char_to_num(segment_5)] = 5;
    let segment_1: &char = &digit_5_signals.iter().find(|&sign| {
      &sign != &segment_0 && &sign != &segment_3 && &sign != &segment_6 && &sign != &segment_5
    }).unwrap().clone();
    self.signal_mapping[SegmentDisplay::char_to_num(segment_1)] = 1;

    if self.signal_mapping.iter().find(|x| x == &&25usize) != None {
      print!("");
    }
  }
  
  /// Decodes digits based on previous programming
  /// 
  /// # Arguments
  /// * digits: Vector of vectors of signals for digits to decode
  /// 
  /// # Returns
  /// Vector of decoded digits
  pub fn decode (&self, digits: &Vec<Vec<char>>) -> Vec<usize> {
    digits.iter().map(|signals| {
      // Translate signals
      let mut mapped: Vec<usize> = signals.iter()
        .map(|signal| {
          self.signal_mapping[SegmentDisplay::char_to_num(signal)]
        })
        .collect::<Vec<usize>>();
      mapped.sort();
      let translated: String = mapped
        .iter()
        .map(|n| {
          SegmentDisplay::num_to_char(&n)
        }).collect();
      // Decode digit
      if translated == "cf" {
        1
      } else if translated == "acdeg" {
        2
      } else if translated == "acdfg" {
        3
      } else if translated == "bcdf" {
        4
      } else if translated == "abdfg" {
        5
      } else if translated == "abdefg" {
        6
      } else if translated == "acf" {
        7
      } else if translated == "abcdefg" {
        8
      } else if translated == "abcdfg" {
        9
      } else {
        0
      }
    }).collect()
  }

}
