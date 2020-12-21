//! Deck of cards module
//! 
//! Implements a Deck of cards functionality
// -----------------------------------------------------------------------------

/// Deck of cards struct
#[derive(Debug)]
pub struct DeckOfCards {
  // Holds cards
  pub cards: Vec<usize>,
  // Index of the first card
  pub head: usize,
  // Size of deck
  pub length: usize,
  // Size of underlying array holding the deck
  length_real: usize
}

// Implements basic functionality of a deck of cards
impl DeckOfCards {

  /// Creates a new deck of cards
  /// 
  /// # Arguments
  /// * `cards`     - Card values to store in the deck
  /// * `deck_size` - Maximum number of cards the deck can hold
  pub fn new (cards: Vec<usize>, max_deck_size: usize) -> DeckOfCards {
    // Get real length of the expanded deck
    let cards_count = cards.len();
    // Expand cards to max langth
    let mut cards_expanded: Vec<usize> = vec![0; max_deck_size];
    for i in 0..cards.len() {
      cards_expanded[i] = cards[i];
    }
    // Initialize and return a new deck of cards
    return DeckOfCards {
      cards: cards_expanded,
      head: 0,
      length: cards_count,
      length_real: max_deck_size
    };
  }

  /// Checks if deck is empty
  pub fn is_empty (&self) -> bool {
    return self.length == 0;
  }

  /// Gets a single card form the deck by its index
  /// 
  /// # Arguments
  /// * `index` - Index of the card to get
  pub fn get (&self, index: usize) -> usize {
    // Return card
    return self.cards[(self.head + index) % self.length_real];
  }

  /// Gets the top card from the deck
  pub fn pop (&mut self) -> &usize {
    // Update head and length to drop the first card
    self.head = (self.head + 1) % self.length_real;
    self.length -= 1;
    // Return popped card
    return &self.cards[(self.length_real + self.head - 1) % self.length_real];
  }

  /// Pushes card to the bottom of the deck
  /// 
  /// # Arguments
  /// * `card` - Card to push to the botton of the deck
  pub fn push (&mut self, card: usize) -> () {
    // Store card
    self.cards[(self.head + self.length) % self.length_real] = card;
    // Update deck length
    self.length += 1;
  }

  /// Generates a subset deck from the current deck
  /// 
  /// # Arguments
  /// * `start`  - Starting position in the current deck to extract the new deck from
  /// * `length` - Length of the new deck
  pub fn subset (&mut self, start: usize, length: usize) -> DeckOfCards {
    return DeckOfCards {
      cards: self.cards.clone(),
      head: (self.head + start) % self.length_real,
      length: length,
      length_real: self.length_real
    };
  }

  /// Returns all cards from the deck
  pub fn get_cards (&self) -> Vec<usize> {
    let mut cards: Vec<usize> = vec![];
    for i in self.head..(self.head + self.length) {
      cards.push(self.cards[i % self.length_real]);
    }
    return cards;
  }

  /// Generates a string representation of the deck
  pub fn to_string (&self) -> String {
    return self.get_cards().iter().map(|card| card.to_string()).collect::<Vec<String>>().join(":");
  }

}
