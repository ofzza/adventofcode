//! Bingo module
//! 
//! Implements a game of bingo
// -----------------------------------------------------------------------------

// Import dependencies
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::hash_map::*;

/// Bing card line structure
pub struct BingoCardLine {
  pub card: Rc<RefCell<BingoCard>>,
  pub nums: Vec<u8>,
  pub drawn: usize
}
/// Bing card line implementation
impl BingoCardLine {

  /// Contructor
  /// 
  /// # Arguments
  /// * parent:   Parent card of this line
  /// * capacity: Count of numbers in the line
  pub fn new (card: Rc<RefCell<BingoCard>>, capacity: usize) -> BingoCardLine {
    BingoCardLine {
      card,
      nums: Vec::with_capacity(capacity),
      drawn: 0
    }
  }

  /// Draws a number and marks line with that number as drawn
  /// 
  /// # Arguments
  /// * number: Number to draw
  fn draw(&mut self, _: u8) {
    self.drawn += 1;
  }

  /// Checks if line has been completed
  fn is_won (&self) -> bool {
    self.nums.len() == self.drawn
  }
}

/// Bing card structure
pub struct BingoCard {
  _width: usize,
  _height: usize,
  _nums: Vec<u8>,
  pub lines: Vec<Rc<RefCell<BingoCardLine>>>,
  pub drawn: usize,
  pub drawn_sum: usize
}
/// Bing card implementation
impl BingoCard {

  /// Constructor
  /// 
  /// # Arguments
  /// * width:    Count of numbers in each row of the card
  /// * height:   Count of numbers in each column of the card
  fn new (width:usize, height: usize) -> Rc<RefCell<BingoCard>> {
    // Instatiate new card
    let card = Rc::new(
      RefCell::new(
        BingoCard {
          _width: width,
          _height: height,
          _nums: Vec::with_capacity(width * height),
          lines: Vec::with_capacity(width + height),
          drawn: 0,
          drawn_sum: 0
        }
      )
    );
    // Initialize lines
    for _ in 0..(width + height) {
      let line = BingoCardLine::new(Rc::clone(&card), width);
      card.borrow_mut().lines.push(Rc::new(RefCell::new(line)));
    }
    // Return instantiated card
    card
  }

  /// Sets number sof the card (not necessarily all at one time)
  /// 
  /// # Arguments
  /// * numbers:  Numbers on the card
  /// 
  /// # Returns
  /// Coordinates on the card of the added number
  fn set_number(&mut self, num: u8) -> (usize, usize) {
    // Get new number's position
    let x = self._nums.len() % self._width;
    let y = self._nums.len() / self._height;
    // Add number to the card
    self._nums.push(num);
    // Add number to lines
    self.lines[x].borrow_mut().nums.push(num);
    self.lines[self._width + y].borrow_mut().nums.push(num);
    // Return lines with this number
    (x, y)
  }

  /// Gets lines of numbers from the card containing the requested number
  /// 
  /// # Arguments
  /// * x: (x, y) x coordinate of numbers on the card
  /// * y: (x, y) y coordinate of numbers on the card
  /// 
  /// Returns
  /// Lines from the card containing the requested number
  fn get_lines_for_number (&self, x: usize, y: usize) -> (Rc<RefCell<BingoCardLine>>, Rc<RefCell<BingoCardLine>>) {
    (Rc::clone(&self.lines[x]), Rc::clone(&self.lines[self._width + y]))
  }

  /// Draws a number and marks line with that number as drawn
  /// 
  /// # Arguments
  /// * number: Number to draw
  fn draw(&mut self, num: u8) {
    self.drawn += 1;
    self.drawn_sum += num as usize;
  }

  // Check if card has won
  fn is_won (&self) -> bool {
    for i in 0..self.lines.len() {
      if self.lines[i].borrow().is_won() {
        return true;
      }
    }
    false
  }

  /// Gets sum of undrawn numbers
  /// 
  /// Returns
  /// Sum of all not-drawn numbers
  pub fn get_remaining_numbers_sum (&self) -> usize {
    // Sum all numbers
    let mut all_sum: usize = 0;
    for i in 0..self._nums.len() {
      all_sum += self._nums[i] as usize;
    }
    // Return sum of not-drawn numbers
    return all_sum - self.drawn_sum;
  }
}

/// References to cards and lines with a given number structure
struct BingoNumberRefs {
  pub cards: Vec<Rc<RefCell<BingoCard>>>,
  pub lines: Vec<Rc<RefCell<BingoCardLine>>>
}
/// References to cards and lines with a given number implementation
impl BingoNumberRefs {
  fn new () -> BingoNumberRefs {
    BingoNumberRefs {
      cards: vec![],
      lines: vec![]
    }
  }
}

/// Bingo structure
pub struct Bingo {
  _hashmap: HashMap<u8, BingoNumberRefs>,
  _width: usize,
  _height: usize,
  _cards: Vec<Rc<RefCell<BingoCard>>>
}
/// Bingo implementation
impl Bingo {

  /// Constructor
  /// 
  /// # Arguments
  /// * width:  Count of numbers in each row of the card
  /// * height: Count of numbers in each column of the card
  pub fn new (width: usize, height: usize) -> Bingo {
    Bingo {
      _hashmap: HashMap::new(),
      _width: width,
      _height: height,
      _cards: vec![]
    }
  }

  /// Parses data into a card and adds the card to the game
  /// 
  /// # Arguments
  /// * data: Numbers to add to the card
  pub fn add_card(&mut self, data: &Vec<Vec<u8>>) {
    // Create card
    let card = BingoCard::new(self._width, self._height);
    // Set card numbers
    let mut coordinates: Vec<(u8, usize, usize)> = Vec::with_capacity(self._width * self._height);
    for i in 0..data.len() {
      for j in 0..data[i].len() {
        // Add number to the card
        let num = data[i][j];
        let (x, y) = card.borrow_mut().set_number(num);
        coordinates.push((num, x, y));
      }
    }
    // Register card and lines with the number
    // let mut card = Rc::new(RefCell::new(card));
    for i in 0..coordinates.len() {
      let num = coordinates[i].0;
      let lines = card.borrow().get_lines_for_number(coordinates[i].1, coordinates[i].2);
      if self._hashmap.contains_key(&num) {
        // Update already registered number references
        let refs = self._hashmap.get_mut(&num).unwrap();
        refs.cards.push(Rc::clone(&card));
        refs.lines.push(Rc::clone(&lines.0));
        refs.lines.push(Rc::clone(&lines.1));
      } else {
        // Insert new number references
        let mut refs = BingoNumberRefs::new();
        refs.cards.push(Rc::clone(&card));
        refs.lines.push(Rc::clone(&lines.0));
        refs.lines.push(Rc::clone(&lines.1));
        self._hashmap.insert(num, refs);
      }
    }
    // Store card
    self._cards.push(Rc::clone(&card));
  }

  /// Draws a number and marks cards with that number
  /// 
  /// # Arguments
  /// * number: Number to draw
  /// 
  /// # Returns
  /// If drawing the number has produced a win
  pub fn draw(&mut self, num: u8) -> Option<Rc<RefCell<BingoCard>>> {
    // Get cards and lines with this number
    let refs = self._hashmap.get_mut(&num).unwrap();
    // Update cards
    for i in 0..refs.cards.len() {
      if !refs.cards[i].borrow().is_won() {
        refs.cards[i].borrow_mut().draw(num);
      }
    }
    // Update lines
    let mut winner: Option<Rc<RefCell<BingoCard>>> = None;
    for i in 0..refs.lines.len() {
      if !refs.lines[i].borrow().is_won() && !refs.lines[i].borrow().card.borrow().is_won() {
        refs.lines[i].borrow_mut().draw(num);
        if refs.lines[i].borrow().is_won() && refs.lines[i].borrow().card.borrow().is_won() {
          winner = Some(Rc::clone(&refs.lines[i].borrow().card));
        }
      }
    }
    // Return no win
    winner
  }  
}
