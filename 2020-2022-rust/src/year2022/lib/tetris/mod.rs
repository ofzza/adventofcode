//! Tetris module
//! 
//! Tetris module
// -----------------------------------------------------------------------------

// Define field size
pub const FIELD_SIZE: usize = 2usize.pow(16);
// Define tetris shapes
pub const SHAPES: [([[u8; 9]; 4], usize); 5] = [
  // "-" tetrimino
  // -4 offset    -3 offset    -2 offser    -1 offset    default      +1 offset    +2 offset    +3 offset    +4 offset
  ([
    [0b0_0000000, 0b0_0000000, 0b1_1111000, 0b1_0111100, 0b1_0011110, 0b1_0001111, 0b0_0000000, 0b0_0000000, 0b0_0000000],
    [0b0_0000000, 0b0_0000000, 0b1_0000000, 0b1_0000000, 0b1_0000000, 0b1_0000000, 0b0_0000000, 0b0_0000000, 0b0_0000000],
    [0b0_0000000, 0b0_0000000, 0b1_0000000, 0b1_0000000, 0b1_0000000, 0b1_0000000, 0b0_0000000, 0b0_0000000, 0b0_0000000],
    [0b0_0000000, 0b0_0000000, 0b1_0000000, 0b1_0000000, 0b1_0000000, 0b1_0000000, 0b0_0000000, 0b0_0000000, 0b0_0000000]
  ], 1),
  // "+" tetrimino
  // -4 offset    -3 offset    -2 offser    -1 offset    default      +1 offset    +2 offset    +3 offset    +4 offset
  ([
    [0b0_0000000, 0b0_0000000, 0b1_0100000, 0b1_0010000, 0b1_0001000, 0b1_0000100, 0b1_0000010, 0b0_0000000, 0b0_0000000],
    [0b0_0000000, 0b0_0000000, 0b1_1110000, 0b1_0111000, 0b1_0011100, 0b1_0001110, 0b1_0000111, 0b0_0000000, 0b0_0000000],
    [0b0_0000000, 0b0_0000000, 0b1_0100000, 0b1_0010000, 0b1_0001000, 0b1_0000100, 0b1_0000010, 0b0_0000000, 0b0_0000000],
    [0b0_0000000, 0b0_0000000, 0b1_0000000, 0b1_0000000, 0b1_0000000, 0b1_0000000, 0b1_0000000, 0b0_0000000, 0b0_0000000]
  ], 3),
  // "L" tetrimino
  // -4 offset    -3 offset    -2 offser    -1 offset    default      +1 offset    +2 offset    +3 offset    +4 offset
  ([
    [0b0_0000000, 0b0_0000000, 0b1_0010000, 0b1_0001000, 0b1_0000100, 0b1_0000010, 0b1_0000001, 0b0_0000000, 0b0_0000000],
    [0b0_0000000, 0b0_0000000, 0b1_0010000, 0b1_0001000, 0b1_0000100, 0b1_0000010, 0b1_0000001, 0b0_0000000, 0b0_0000000],
    [0b0_0000000, 0b0_0000000, 0b1_1110000, 0b1_0111000, 0b1_0011100, 0b1_0001110, 0b1_0000111, 0b0_0000000, 0b0_0000000],
    [0b0_0000000, 0b0_0000000, 0b1_0000000, 0b1_0000000, 0b1_0000000, 0b1_0000000, 0b1_0000000, 0b0_0000000, 0b0_0000000]
  ], 3),
  // "|" tetrimino
  // -4 offset    -3 offset    -2 offser    -1 offset    default      +1 offset    +2 offset    +3 offset    +4 offset
  ([
    [0b0_0000000, 0b0_0000000, 0b1_1000000, 0b1_0100000, 0b1_0010000, 0b1_0001000, 0b1_0000100, 0b1_0000010, 0b1_0000001],
    [0b0_0000000, 0b0_0000000, 0b1_1000000, 0b1_0100000, 0b1_0010000, 0b1_0001000, 0b1_0000100, 0b1_0000010, 0b1_0000001],
    [0b0_0000000, 0b0_0000000, 0b1_1000000, 0b1_0100000, 0b1_0010000, 0b1_0001000, 0b1_0000100, 0b1_0000010, 0b1_0000001],
    [0b0_0000000, 0b0_0000000, 0b1_1000000, 0b1_0100000, 0b1_0010000, 0b1_0001000, 0b1_0000100, 0b1_0000010, 0b1_0000001]
  ], 4),
  // "o" tetrimino
  // -4 offset    -3 offset    -2 offser    -1 offset    default      +1 offset    +2 offset    +3 offset    +4 offset
  ([
    [0b0_0000000, 0b0_0000000, 0b1_1100000, 0b1_0110000, 0b1_0011000, 0b1_0001100, 0b1_0000110, 0b1_0000011, 0b0_0000000],
    [0b0_0000000, 0b0_0000000, 0b1_1100000, 0b1_0110000, 0b1_0011000, 0b1_0001100, 0b1_0000110, 0b1_0000011, 0b0_0000000],
    [0b0_0000000, 0b0_0000000, 0b1_0000000, 0b1_0000000, 0b1_0000000, 0b1_0000000, 0b1_0000000, 0b1_0000000, 0b0_0000000],
    [0b0_0000000, 0b0_0000000, 0b1_0000000, 0b1_0000000, 0b1_0000000, 0b1_0000000, 0b1_0000000, 0b1_0000000, 0b0_0000000]
  ], 2)
];

/// Tetris structure
pub struct Tetris {
  // Current tetrimino index
  tetrimino_index: usize,
  // Wind directions
  wind: Vec<isize>,
  // Current wind direction index
  wind_index: usize,
  // Field of still tetriminos
  pub field: [u8; FIELD_SIZE],
  // Number of empty field lines (needed for rendering)
  pub field_empty: usize,
  // Number of lines cleared to save space
  pub field_cleared: u64,
  // Number of tetriminos dropped
  pub field_dropped: u64
}

/// Tetris implementation
impl Tetris {

  /// Constructor
  pub fn new (wind: Vec<char>) -> Tetris {
    Tetris {
      // Wind directions (Translate from characters to relative coordinates)
      wind: wind.iter().map(|wind| if wind == &'<' { -1 } else if wind == &'>' { 1 } else { 0 }).collect::<Vec<isize>>(),
      // Current wind direction index
      wind_index: 0,
      // Field of still tetriminos
      field: [0; FIELD_SIZE],
      // Number of empty field lines (needed for rendering)
      field_empty: FIELD_SIZE,
      // Number of lines cleared to save space
      field_cleared: 0,
      // Current tetrimino index
      tetrimino_index: 0,
      // Number of tetriminos dropped
      field_dropped: 0
    }
  }

  /// Drop next shape
  pub fn drop_next (&mut self) -> Option<u64> {
    // Select a shape
    let next = SHAPES[self.tetrimino_index];
    self.tetrimino_index = (self.tetrimino_index + 1) % SHAPES.len();
    // Drop a shape
    if self.drop_shape(&next.0, next.1) {
      return Option::Some(self.field_dropped);
    }
    else {
      return Option::None;
    }
  }
  /// Drop a shape
  /// 
  /// # Arguments
  /// * shape: Definition of a shape to drop
  /// 
  /// # Returns
  /// If after dropping the shape the game is back to initial position
  pub fn drop_shape (&mut self, shape: &[[u8; 9]; 4], shape_rows: usize) -> bool {
    // Get a mutable reference to the shape
    let mut shape_shift = 4;

    // Calculate initial spawning offset
    let mut offset_diff = 0;
    let mut offset = self.field_empty - shape_rows - 3;

    // Prompt field and staged tetrimino
    // self._prompt((shape, shape_shift), offset);
  
    // Drop shape until contact with floor or locked tetrimino is made    
    loop {
      // Get current wind and forweard wind pointer
      let wind = self.wind[self.wind_index];
      self.wind_index = (self.wind_index + 1) % self.wind.len();
      // Shift shape left/right by wind
      let shape_shift_candidate = shape_shift as isize + wind;      
      shape_shift =
        if (shape_shift_candidate >= 0) // MINIMIZING BRANCHING
        && ((shape_shift_candidate as usize) < shape[0].len())
        && (shape[0][shape_shift_candidate as usize] >= 128)
        && (offset_diff < 3 || !self.shape_check_collision((shape, shape_shift_candidate as usize), offset)) {
          shape_shift_candidate as usize
        } else {
          shape_shift
        };

      // Prompt field and falling tetrimino
      // self._prompt((shape, shape_shift), offset);
  
      // Fall down if shape not settled after being shifted by the wind
      if offset_diff < 3 || !self.shape_check_collision((shape, shape_shift), offset + 1) {
        offset += 1;
        offset_diff += 1;
      }
      // Or lock in place and stop falling
      else {
        self.shape_lock((shape, shape_shift), offset);
        break;
      }
      
      // Prompt field and falling tetrimino
      // self._prompt((shape, shape_shift), offset);
    }

    // Update empty rows
    let empty_fields_diff: isize = 3 as isize + shape_rows as isize - offset_diff as isize;
    // self.field_empty -= if empty_fields_diff > 0 { empty_fields_diff as usize } else { 0 };
    self.field_empty -= (empty_fields_diff + empty_fields_diff.abs()) as usize / 2; // NO BRANCH VERSION

    // Update number of tetriminos dropped
    self.field_dropped += 1;

    // Prompt field and falling tetrimino
    // self._prompt((shape, shape_shift), offset);

    // Check if more than 50% field full and needs partial clearing
    if self.field_empty < FIELD_SIZE / 2 {
      // Find 2 subsequent rows making a tetris(ish)
      let mut cleared = false;
      for i in self.field_empty..(FIELD_SIZE - 1) {
        if (self.field[i] | self.field[i + 1]) == 127 {
          cleared = true;
          let cleared_count = FIELD_SIZE - (i + 2);
          self.field_empty += cleared_count;
          self.field_cleared += cleared_count as u64;
          self.field.rotate_right(cleared_count);
          for j in 0..self.field_empty { self.field[j] = 0; }
          break;
        }
      }
      if !cleared {
        panic!("Failed to clear the field at 90% full!");
      }
    }

    // Default to returning no repetition
    false
  }

  /// Checks if shape fits at the propposed offset
  /// 
  /// # Arguments
  /// * shape: Definition of a shape currently dropping and its current shift
  /// * offset: Distance from the top the shape has dropped
  /// 
  /// # Returns
  /// If the shape can fit at the proposed offset
  fn shape_check_collision (&self, shape: (&[[u8; 9]; 4], usize), offset: usize) -> bool {
    // Check each shape line for collision
    
    // for i in offset..offset + shape.0.len() {
    //   // No check needed if within empty section of the field
    //   if i < self.field_empty {
    //     continue;
    //   }
    //   // If overlap between field and shape
    //   if i < self.field.len() && (self.field[i] & shape.0[i - offset][shape.1]) != 0
    //   // If shape's non-empty line on last line of the field
    //   || i == self.field.len() && shape.0[i - offset][shape.1] != 128 {
    //     return true;
    //   }
    // }
    // // No collision found
    // false

    // NO BRANCH VERSION
       (((offset + 0) >= self.field_empty) && (((offset + 0) < self.field.len() && (self.field[(offset + 0)] & shape.0[(offset + 0) - offset][shape.1]) != 0) || ((offset + 0) == self.field.len() && shape.0[(offset + 0) - offset][shape.1] != 128)))
    || (((offset + 1) >= self.field_empty) && (((offset + 1) < self.field.len() && (self.field[(offset + 1)] & shape.0[(offset + 1) - offset][shape.1]) != 0) || ((offset + 1) == self.field.len() && shape.0[(offset + 1) - offset][shape.1] != 128)))
    || (((offset + 2) >= self.field_empty) && (((offset + 2) < self.field.len() && (self.field[(offset + 2)] & shape.0[(offset + 2) - offset][shape.1]) != 0) || ((offset + 2) == self.field.len() && shape.0[(offset + 2) - offset][shape.1] != 128)))
    || (((offset + 3) >= self.field_empty) && (((offset + 3) < self.field.len() && (self.field[(offset + 3)] & shape.0[(offset + 3) - offset][shape.1]) != 0) || ((offset + 3) == self.field.len() && shape.0[(offset + 3) - offset][shape.1] != 128)))
  }

  /// Locks a shape into place
  /// 
  /// # Arguments
  /// * shape: Definition of a shape currently dropping and its current shift
  /// * offset: Distance from the top the shape has dropped
  fn shape_lock (&mut self, shape: (&[[u8; 9]; 4], usize), offset: usize) {
    // Check each chape line for collision
    // for i in offset..(offset + shape.0.len()) {
    //   if i < self.field.len() {
    //     let field_line = if i < self.field_empty { 0b0_0000000 } else { self.field[i] };
    //     self.field[i] = field_line | shape.0[i - offset][shape.1] & 0b0_1111111;
    //   }
    // }

    // NO BRANCH VERSION
    let field_len = self.field.len();
    self.field[(offset + 0) % field_len] = self.field[(offset + 0) % field_len] | shape.0[0][shape.1] & 0b0_1111111;
    self.field[(offset + 1) % field_len] = self.field[(offset + 1) % field_len] | shape.0[1][shape.1] & 0b0_1111111;
    self.field[(offset + 2) % field_len] = self.field[(offset + 2) % field_len] | shape.0[2][shape.1] & 0b0_1111111;
    self.field[(offset + 3) % field_len] = self.field[(offset + 3) % field_len] | shape.0[3][shape.1] & 0b0_1111111;
  }

  /// Prompts current state of the field and any faling tetriminos
  /// 
  /// # Arguments
  /// * shape: Definition of a shape currently dropping and its current shift
  /// * offset: Distance from the top the shape has dropped
  pub fn _prompt (&self, shape: (&[[u8; 9]; 4], usize), offset: usize) {
    println!();
    for i in (self.field_empty - 7)..self.field.len() {
      // Prompt field line with the currently falling tetrimino superimposed
      let field_line = if i < self.field_empty { 0b0_0000000 } else { self.field[i] };
      let line_combined: u8 = if i >= offset && i < (offset + shape.0.len()) { field_line | shape.0[i - offset][shape.1] } else { field_line };
      let line_combined_string = (format!("{:#10b}", line_combined)).chars().map(|c| if c == '1' { '#' } else { '.' } ).collect::<String>()[3..].to_string();
      println!("{}", line_combined_string);
    }
    return;
  }
}
