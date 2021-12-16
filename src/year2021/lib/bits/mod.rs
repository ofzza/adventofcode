//! Buoyancy Interchange Transmission System (BITS) module
//! 
//! Implements Buoyancy Interchange Transmission System (BITS) decoding and processing
// -----------------------------------------------------------------------------

// Import dependencies
use std::rc::Rc;
use std::cell::RefCell;

/// Implements Buoyancy Interchange Transmission System (BITS) structure
pub struct BITS { }
/// Implements Buoyancy Interchange Transmission System (BITS) implementation
impl BITS {

  /// Parses packet content
  /// 
  /// # Arguments
  /// * bin: Binary data to parse from
  pub fn parse (bin: &Vec<bool>) -> BitsPackage {
    let mut instance = BitsPackage::new();
    instance.parse(bin);
    instance
  }

  /// Iterates through all of the nested packages
  ///
  /// # Arguments
  /// * packet:   Packet to iterate through
  /// * output:   Common entity to share between all invokations of the callback function
  /// * callback: Callback function that will get executed for every iterated packet
  pub fn map<T> (packet: &BitsPackage, output: &mut T, callback: fn(p: &BitsPackage, output: &mut T) -> ()) {
    // Execute callback on packet
    callback(packet, output);
    // Recurse into children
    let children = &packet.content.content_packets;
    for i in 0..children.len() {
      BITS::map(&children[i].borrow() , output, callback);
    }
  }

  /// Decodes a hex stream of BITS data into a binary stream of BITS data
  pub fn decode_hex(hex: &Vec<char>) -> Vec<bool> {
    // Initialize binary data
    let mut bin: Vec<bool> = Vec::with_capacity(hex.len() * 4);
    // Translate hex into binary
    for i in 0..hex.len() {
      match hex[i].to_ascii_uppercase() {
        '0' => { bin.push(false); bin.push(false); bin.push(false); bin.push(false); },
        '1' => { bin.push(false); bin.push(false); bin.push(false); bin.push(true); },
        '2' => { bin.push(false); bin.push(false); bin.push(true); bin.push(false); },
        '3' => { bin.push(false); bin.push(false); bin.push(true); bin.push(true); },
        '4' => { bin.push(false); bin.push(true); bin.push(false); bin.push(false); },
        '5' => { bin.push(false); bin.push(true); bin.push(false); bin.push(true); },
        '6' => { bin.push(false); bin.push(true); bin.push(true); bin.push(false); },
        '7' => { bin.push(false); bin.push(true); bin.push(true); bin.push(true); },
        '8' => { bin.push(true); bin.push(false); bin.push(false); bin.push(false); },
        '9' => { bin.push(true); bin.push(false); bin.push(false); bin.push(true); },
        'A' => { bin.push(true); bin.push(false); bin.push(true); bin.push(false); },
        'B' => { bin.push(true); bin.push(false); bin.push(true); bin.push(true); },
        'C' => { bin.push(true); bin.push(true); bin.push(false); bin.push(false); },
        'D' => { bin.push(true); bin.push(true); bin.push(false); bin.push(true); },
        'E' => { bin.push(true); bin.push(true); bin.push(true); bin.push(false); },
        'F' => { bin.push(true); bin.push(true); bin.push(true); bin.push(true); },
        _ => ()
      }
    }
    // Return binary data
    bin
  }

  /// Decodes a binary representation of a number
  /// 
  /// # Arguments
  /// * bin: Binary representation of a number
  /// 
  /// # Returns
  /// Parsed integer value
  pub fn decode_binary_number(bin: &Vec<bool>) -> usize {
    let mut result: usize = 0;
    for i in (0..bin.len()).rev() {
      if bin[i] { result += 2usize.pow(bin.len() as u32 - i as u32 - 1u32); }
    }
    result
  }

}

/// BITS Packet binary data structure
pub struct BitsPacketBinary {
  head: Option<Vec<bool>>,
  body: Option<Vec<bool>>
}
/// BITS Packet binary data implementation
impl BitsPacketBinary {
  /// Constructor
  pub fn new () -> BitsPacketBinary {
    BitsPacketBinary {
      head: None,
      body: None
    }
  }
  /// Parses packet content
  /// 
  /// # Arguments
  /// * bin: Binary data to parse from
  fn parse (&mut self, bin: &Vec<bool>) -> Option<()> {
    // Check if packet long enough to extract at least the header
    if bin.len() >= 6 {
      self.head = Some(bin[0..6].to_vec());
      self.body = Some(bin[6..].to_vec());
      Some(())
    } else {
      None
    }
  }
}

/// BITS Packet information structure
pub struct BitsPacketInfo {
  pub packet_version: usize,
  pub packet_type: usize,
}
/// BITS Packet information implementation
impl BitsPacketInfo {  
  /// Constructor
  pub fn new () -> BitsPacketInfo {
    BitsPacketInfo {
      packet_version: 0,
      packet_type: 0
    }
  }
  /// Parses packet content
  /// 
  /// # Arguments
  /// * bin: Binary data to parse from
  fn parse (&mut self, bin: & Vec<bool>) -> Option<usize> {
    // Extract version
    self.packet_version = (if bin[0] { 4 } else { 0 }) + (if bin[1] { 2 } else { 0 }) + (if bin[2] { 1 } else { 0 });
    // Extract type
    self.packet_type = (if bin[3] { 4 } else { 0 }) + (if bin[4] { 2 } else { 0 }) + (if bin[5] { 1 } else { 0 });
    // Return consumed size
    Some(6)
  }
}

/// BITS Packet parsed content structure
pub struct BitsPacketContent {
  content_length: usize,
  content_length_type: bool,
  pub content_value: usize,
  pub content_packets: Vec<Rc<RefCell<BitsPackage>>>
}
/// BITS Packet parsed content implementation
impl BitsPacketContent {
  /// Constructor
  pub fn new () -> BitsPacketContent {
    BitsPacketContent {
      content_length: 0,
      content_length_type: false,
      content_value: 0,
      content_packets: vec![]
    }
  }
  /// Parses packet content
  /// 
  /// # Arguments
  /// * bin: Binary data to parse from
  fn parse (&mut self, packet_type: usize, bin: &Vec<bool>) -> Option<usize> {
    // Check if bin empty
    if bin.len() == 0 { return None; }

    // Parse of TYPE 4: Literal value packet
    if packet_type == 4 {
      let mut value_bin: Vec<bool> = Vec::new();
      let mut total_consumed_content_length = 0;
      for i in 0..(bin.len() / 5) {
        total_consumed_content_length += 5;
        value_bin.push(bin[i * 5 + 1]);
        value_bin.push(bin[i * 5 + 2]);
        value_bin.push(bin[i * 5 + 3]);
        value_bin.push(bin[i * 5 + 4]);
        if bin[i * 5] == false { break; }
      }
      // Decode stored value
      self.content_value = BITS::decode_binary_number(&value_bin);
      // Return consumed size
      return Some(total_consumed_content_length);
    }

    // Parse of TYPE !4: Oprator with nested packets
    if packet_type != 4 {
      self.content_length_type = bin[0];    
      let content_length_encoding_length = (if self.content_length_type { 11 } else { 15 }) + 1;
      let content_length_encoding_length = if content_length_encoding_length > bin.len() { bin.len() } else { content_length_encoding_length };
      self.content_length = BITS::decode_binary_number(&bin[1..content_length_encoding_length].to_vec());
      let mut content_packets_bin = if content_length_encoding_length > bin.len() { vec![] } else { bin[content_length_encoding_length..].to_vec() };
      
      // If content length provided in bits, limit input number of bits
      if self.content_length_type == false && content_packets_bin.len() > self.content_length {
        content_packets_bin = content_packets_bin[0..self.content_length].to_vec();
      }

      // Parse sub-packets
      let mut total_consumed_content_length = content_length_encoding_length;
      loop {
        // Parse a subpacket
        let mut subpacket = BitsPackage::new();
        let consumed_content_length_option = subpacket.parse(&content_packets_bin);
        match consumed_content_length_option {
          // Parsed subpacket
          Some(consumed_content_length) => {
            // Add subpacket
            self.content_packets.push(Rc::new(RefCell::new(subpacket)));
            // Remove consumed data from parent packet content being processed
            content_packets_bin = content_packets_bin[consumed_content_length..].to_vec();
            total_consumed_content_length += consumed_content_length;
          },
          // Done parssing subpackets
          None => { break; }
        }

        // If content length provided in number of packets, limit number of packets
        if self.content_length_type == true {
          if self.content_packets.len() == self.content_length {
            break;
          }
        }
      }

      // Return consumed size
      if total_consumed_content_length > 0 {
        return Some(total_consumed_content_length);
      } else {
        return None;
      }
    }

    // Return parsing failed
    None
  }
}

/// Generic packet with no specific type structure
pub struct BitsPackage {
  bin: BitsPacketBinary,
  pub info: BitsPacketInfo,
  pub content: BitsPacketContent,
}
/// Generic packet with no specific type implementation
impl BitsPackage {
  /// Constructor
  pub fn new () -> BitsPackage {
    BitsPackage {
      bin: BitsPacketBinary::new(),
      info: BitsPacketInfo::new(),
      content: BitsPacketContent::new(),
    }
  }
}
/// Shared BitsPacket trait implementation for Generic packet
impl BitsPackage {
  /// Parses packet content
  /// 
  /// # Arguments
  /// * bin: Binary data to parse from
  fn parse (&mut self, bin: &Vec<bool>) -> Option<usize> {
    // Parse binary
    let binary = &mut self.bin;
    match binary.parse(bin) {
      Some(_) => {

        // Get bonary head and body
        let head: &Option<Vec<bool>> = &self.bin.head;
        let head_bin: Vec<bool> = match head { Some(bin) => bin.clone(), _ => vec![] };
        let body: &Option<Vec<bool>> = &self.bin.body;
        let body_bin: Vec<bool> = match body { Some(bin) => bin.clone(), _ => vec![] };

        // Parse info
        let info_consumed_size_option = self.info.parse(&head_bin);
        match info_consumed_size_option {
          Some(info_consumed_size) => {
            // Parse content
            let packet_type = self.info.packet_type;
            let content_consumed_size_option = self.content.parse(packet_type, &body_bin);
            match content_consumed_size_option {
              Some(content_consumed_size) => {                
                // Calculate packet value
                self.content.content_value = match packet_type {
                  0 => { // Sum packet
                    let mut value: usize = 0;
                    for i in 0..self.content.content_packets.len() {
                      value += self.content.content_packets[i].borrow().content.content_value;
                    } 
                    value
                  },
                  1 => { // Product packet
                    let mut value: usize = 1;
                    for i in 0..self.content.content_packets.len() {
                      value *= self.content.content_packets[i].borrow().content.content_value;
                    } 
                    value
                  },
                  2 => { // Minimum packet
                    let mut value: usize = usize::MAX;
                    for i in 0..self.content.content_packets.len() {
                      if self.content.content_packets[i].borrow().content.content_value < value {
                        value = self.content.content_packets[i].borrow().content.content_value;
                      }
                    } 
                    value
                  },
                  3 => { // Maximum packet
                    let mut value: usize = 0;
                    for i in 0..self.content.content_packets.len() {
                      if self.content.content_packets[i].borrow().content.content_value > value {
                        value = self.content.content_packets[i].borrow().content.content_value;
                      }
                    } 
                    value
                  },
                  4 => { // Literal value
                    self.content.content_value
                  },
                  5 => { // Greater than
                    if self.content.content_packets[0].borrow().content.content_value > self.content.content_packets[1].borrow().content.content_value {
                      1
                    } else {
                      0
                    }
                  },
                  6 => { // Less than
                    if self.content.content_packets[0].borrow().content.content_value < self.content.content_packets[1].borrow().content.content_value {
                      1
                    } else {
                      0
                    }
                  },
                  7 => { // Equal to
                    if self.content.content_packets[0].borrow().content.content_value == self.content.content_packets[1].borrow().content.content_value {
                      1
                    } else {
                      0
                    }
                  },
                  _ => 0
                };
                // Return total consumed size
                return Some(info_consumed_size + content_consumed_size);
              },
              _ => ()
            }
          },
          _ => ()
        }
      },
      _ => ()
    }

    // Return parse failed
    None
  }
}
