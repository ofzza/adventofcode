//! Keep-Away module
//! 
//! Keep-Away module
// -----------------------------------------------------------------------------

// Include dependecies
// ...

/// Keep-Away structure
pub struct KeepAway<'a> {
  pub players: Vec<PlayerState<'a>>,
  worry_divisor: usize,
  turn_count: usize
}

// Keep-Away player state
pub struct PlayerState<'a> {
  pub items: Vec<usize>,
  pub operation: (&'a str, Option<usize>),
  pub test_divisibility: (usize, usize, usize)
}

/// Keep-Away implementation
impl<'a> KeepAway<'a> {

  /// Constructor
  /// 
  /// # Arguments
  /// * players: Starting player states
  pub fn new<'b> (players: Vec<PlayerState<'b>>, worry_divisor: usize) -> KeepAway<'b> {
    KeepAway {
      players,
      worry_divisor,
      turn_count: 0
    }
  }

  // Execute next player turn
  pub fn execute_turn (&mut self) {
    // Find multiple of all test divisors as a safe number to apply modulo arithmetic for
    let mut modulo = 1;
    for player in &self.players {
      modulo *= player.test_divisibility.0;
    }
    // Execute player round
    self.execute_player_turn(self.turn_count % self.players.len(), modulo);
    // Forward player turn count
    self.turn_count += 1;
  }

  /// Executes requested player turn
  /// 
  /// # Arguments
  /// * i: Index of the player to execute
  /// * modulo: Modulo to apply to every worry value to keep values from overflowing
  fn execute_player_turn (&mut self, i: usize, modulo: usize) {
    // Get player
    let player = &self.players[i];
    
    // Process all items
    let mut throws: Vec<(usize, usize)> = vec![];
    for j in 0..player.items.len() {
      // Takes item for inspection
      let mut item_worry_level = player.items[j];

      // Worry while inspecting item recalculated by "operation" formula      
      item_worry_level =
        if player.operation.0 == "+" {
          item_worry_level + (match player.operation.1 {
            Option::None => item_worry_level,
            Option::Some(value) => value
          })
        }
        else if player.operation.0 == "*" {
          item_worry_level * (match player.operation.1 {
            Option::None => item_worry_level,
            Option::Some(value) => value
          })
        }
        else {
          panic!("Operation operator {:?} not supported!", player.operation.0);
        };

      // Worry while bored with item reduced: old / 3
      if self.worry_divisor > 0 {
        item_worry_level = (item_worry_level as f64 / self.worry_divisor as f64).floor() as usize;
      }

      // Keep value in check via modulo arithmetic
      if self.worry_divisor == 0 && modulo > 0 {
        item_worry_level = item_worry_level % modulo;
      }

      // Tests divisibility to decide target and throws item
      let throw_divisibility_test_result = item_worry_level % player.test_divisibility.0 == 0;
      let throw_target_index = if throw_divisibility_test_result { player.test_divisibility.1 } else { player.test_divisibility.2 };
      
      // Schedule throw item to another player
      throws.push((throw_target_index, item_worry_level));
    }

    // Remove thrown items
    self.players[i].items = vec![];
    // Catch all thrown items
    for throw in throws {
      self.players[throw.0].items.push(throw.1);
    }
  }

}
