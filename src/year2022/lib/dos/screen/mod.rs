//! DOS Screen module
//! 
//! Device Operating System Screen module
// -----------------------------------------------------------------------------

/// Device Operating System Screen structure
pub struct Screen {
  // Collection of points drawn in sequence by copying a matching location on the sprite
  pub screen_buffer: [bool; 240],
  // Location of the next point on the screen to be drawn
  screen_buffer_pointer: usize,
  // Sprite used to copy points to the screen when drawing
  sprite_buffer: [bool; 40]
}

/// Device Operating System Screen implementation
impl Screen {

  /// Constructor
  pub fn new () -> Screen {
    Screen {
      screen_buffer: [false; 240],
      screen_buffer_pointer: 0,
      sprite_buffer: [false; 40],
    }
  }

  /// Updates the sprite position to requested location
  /// 
  /// Arguments
  /// * position: New position of the middle pixel of the 3 pixel wide sprite
  pub fn update_sprite_position(&mut self, position: isize) {
    // Reset sprice
    self.sprite_buffer = [false; 40];
    // Draw sprite
    if position >=  1 { self.sprite_buffer[(position - 1) as usize] = true; }
    if position >=  0 { self.sprite_buffer[(position + 0) as usize] = true; }
    if position >= -1 { self.sprite_buffer[(position + 1) as usize] = true; }
  }

  /// Draws a pixel to the screen by copying the matching position from the sprite
  pub fn draw_pixel(&mut self) {
    self.screen_buffer[self.screen_buffer_pointer] = self.sprite_buffer[self.screen_buffer_pointer % self.sprite_buffer.len()];
    self.screen_buffer_pointer += 1;
  }

  /// Outputs contents of the screen in firndly format
  pub fn prompt(&mut self) {
    for i in 0..6 {
      println!("{}", self.screen_buffer[(i*40)..((i+1)*40)].iter().map(|x| if x.clone() { "#" } else { "." }).collect::<Vec<&str>>().join(""));
    }
  }

}
