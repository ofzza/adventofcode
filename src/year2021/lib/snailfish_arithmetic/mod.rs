//! Snail fish arithmetic module
//! 
//! Snail fish numbers arirhmetic implementation
// -----------------------------------------------------------------------------

// Include dependencies
use std::rc::Rc;
use std::cell::RefCell;

/// Snailfish arithmetic struct
pub struct SnailFishArithmetic {}
/// Snailfish arithmetic implementation
impl SnailFishArithmetic {

  /// Parses a string representation of the snailfish number into a snailfish number instance
  /// 
  /// # Arguments
  /// * data: string representation of the snailfish number
  /// 
  /// # Returns 
  /// Composed snailfish number instance
  pub fn parse (data: &Vec<char>) -> Rc<RefCell<SnailFishNumber>> {
    // Ready stack of currently processing numbers
    let mut stack: Vec<Rc<RefCell<SnailFishNumber>>> = vec![];
    // Process serialized snailfish number string    
    for i in 0..data.len() {
      let c = data[i];
      match c {

        // Begining of a new (nested) snailfish number
        '[' => {
          // Add new frame onto the stack of processing
          stack.push(Rc::new(RefCell::new(SnailFishNumber::new())));
        },

        // End of a new (nested) snailfish number
        ']' => {
          // Pop finished frame from stack and nest into previous frame
          let number = stack.pop().unwrap();
          if stack.len() > 0 {
            let mut current = stack.last().unwrap().borrow_mut();
            if !current.has_left() {
              current.left = Some(number);
            } else if !current.has_right() {
              current.right = Some(number);
            } else {
              panic!("Snail number parsing failed: Each number can contain only 2 nested values!")
            }
          }
          // ... or if no more frames on the stack, return number as fully parsed root number
          else {
            number.borrow_mut().reduce();
            return number;
          }
        }

        // Skip comma characters
        ',' => (),

        // Snailfish number digit
        _ => {
          // Add explicit value snailfish number to top frame on the stack
          let mut raw = SnailFishNumber::new();
          match c.to_string().parse::<usize>() {
            Ok(n) => {
              raw.value = Some(n);
              let number = Rc::new(RefCell::new(raw));
              let mut current = stack.last().unwrap().borrow_mut();
              match current.left {
                None => current.left = Some(number),
                Some(_) => current.right = Some(number)
              }    
            },
            _ => ()
          }
        }
      }
    }
    // 
    panic!("Snail number parsing failed: Non matching brackets!");
  }

  /// Sums two snailfish number together
  /// 
  /// # Arguments
  /// * first:  First number to sum
  /// * second: Second number to sum
  /// 
  /// # Returns
  /// Sum of the two numbers
  pub fn sum (first: SnailFishNumber, second: SnailFishNumber) -> SnailFishNumber {
    let mut result: SnailFishNumber = SnailFishNumber::new();
    result.left = Some(Rc::new(RefCell::new(first.clone())));
    result.right = Some(Rc::new(RefCell::new(second.clone())));
    result.reduce();
    result
  }

}

// Snailfish number struct
#[derive(Clone, Debug)]
pub struct SnailFishNumber {
  left: Option<Rc<RefCell<SnailFishNumber>>>,
  right: Option<Rc<RefCell<SnailFishNumber>>>,
  value: Option<usize>
}
// Snailfish  number implementation
impl SnailFishNumber {

  /// Constructor
  fn new () -> SnailFishNumber {
    SnailFishNumber {
      left: None,
      right: None,
      value: None
    }
  }

  /// Deep clones the snailfish number
  /// 
  /// # Returns
  /// A deep clone of the snailfish number
  pub fn clone (&self) -> SnailFishNumber {
    let mut number = SnailFishNumber::new();
    if self.has_value() {
      number.value = Some(self.get_value().clone());
    } else {
      number.left = Some(Rc::new(RefCell::new(self.get_left().borrow().clone())));
      number.right = Some(Rc::new(RefCell::new(self.get_right().borrow().clone())));
    }
    number
  }

  // Checks if left nested number exists
  fn has_left (&self) -> bool {
    match &self.left {
      Some(_) => true,
      None => false
    }
  }
  // Gets left nested number if it is set
  fn get_left (&self) -> &Rc<RefCell<SnailFishNumber>> {
    match &self.left {
      Some(n) => &n,
      None => panic!("Failed getting number.left!")
    }
  }
  // Checks if left nested number exists
  fn has_right (&self) -> bool {
    match &self.right {
      Some(_) => true,
      None => false
    }
  }
  // Gets right nested number if it is set
  fn get_right (&self) -> &Rc<RefCell<SnailFishNumber>> {
    match &self.right {
      Some(n) => &n,
      None => panic!("Failed getting number.right!")
    }
  }
  // Checks if explicit numeric value is set
  fn has_value (&self) -> bool {
    match &self.value {
      Some(_) => true,
      None => false
    }
  }
  // Gets value if it is set
  fn get_value (&self) -> &usize {
    match &self.value {
      Some(n) => &n,
      None => panic!("Failed getting number.value!")
    }
  }

  /// Adds a number to the first left nested member with an explicit value
  ///
  /// # Arguments
  /// * n: Number to add to first left-found value
  fn add_to_left(&mut self, n: usize) {
    if self.has_value() {
      self.value = Some(self.get_value() + n);
    } else {
      self.get_left().borrow_mut().add_to_left(n);
    }
  }
  /// Adds a number to the first right nested member with an explicit value
  ///
  /// # Arguments
  /// * n: Number to add to first right-found value
  fn add_to_right(&mut self, n: usize) {
    if self.has_value() {
      self.value = Some(self.get_value() + n);
    } else {
      self.get_right().borrow_mut().add_to_right(n);
    }
  }

  /// Processes the snailfish number to its reduced form
  fn reduce(&mut self) {
    // Search for nested snailfish numbers in need of reduction
    loop {
      match self.reduce_internal_nesting(0) {
        Some(_) => (),
        None => match self.reduce_internal_value_size(0) {
          Some(_) => (),
          None => { break; } 
        }
      }      
    }   
  }
  /// Processes a singe snailfish number reduction step, clearing up too deeply nested numbers
  /// 
  /// # Arguments
  /// * depth: Depth of currently analysed number in the overall hierarchy
  fn reduce_internal_nesting(&mut self, depth: usize) -> Option<SnailFishNumber> {
    // If number has nested numbers, check those numbers for too deep nesting
    if !self.has_value() {
      // Check if too deeply nested
      if depth >= 4 {
        // Clone nested values and reset number to 0
        let mut number: SnailFishNumber = SnailFishNumber::new();
        number.left = Some(Rc::clone(self.get_left()));
        number.right = Some(Rc::clone(self.get_right()));
        // Reset to 0
        self.left = None;
        self.right = None;
        self.value = Some(0);
        // Return number
        return Some(number);
      }

      // ... or, if not, check nested child numbers
      else {
        // Check left nested number
        match self.get_left().borrow_mut().reduce_internal_nesting(depth + 1) {
          Some(mut number) => {
            if number.has_right() {
              self.get_right().borrow_mut().add_to_left(number.get_right().borrow().get_value().clone());
              number.right = None;
            }
            return Some(number);
          },
          None => ()
        }
        // Check right nested number
        match self.get_right().borrow_mut().reduce_internal_nesting(depth + 1) {
          Some(mut number) => {
            if number.has_left() {
              self.get_left().borrow_mut().add_to_right(number.get_left().borrow().get_value().clone());
              number.left = None;
            }
            return Some(number);
          },
          None => ()
        }
      }
    }

    None
  }
  /// Processes a singe snailfish number reduction step, clearing up numbers with too large a value
  /// 
  /// # Arguments
  /// * depth: Depth of currently analysed number in the overall hierarchy
  fn reduce_internal_value_size(&mut self, depth: usize) -> Option<SnailFishNumber> {
    // If number has explicit value, check if value is too large
    if self.has_value() {
      let value = self.get_value().clone();
      if value >= 10 {
        self.value = None;
        let mut left = SnailFishNumber::new();
        left.value = Some((value as f64 / 2.0).floor() as usize);
        self.left = Some(Rc::new(RefCell::new(left)));
        let mut right = SnailFishNumber::new();
        right.value = Some((value as f64 / 2.0).ceil() as usize);
        self.right = Some(Rc::new(RefCell::new(right)));
        return Some(SnailFishNumber::new());
      }

    // If number has nested numbers, check those numbers for too deep nesting
    } else {
      // Check left nested number
      match self.get_left().borrow_mut().reduce_internal_value_size(depth + 1) {
        Some(number) => {
          return Some(number);
        },
        None => ()
      }
      // Check right nested number
      match self.get_right().borrow_mut().reduce_internal_value_size(depth + 1) {
        Some(number) => {
          return Some(number);
        },
        None => ()
      }
    }

    None
  }

  /// Calculates magnitude of a snailfish number
  /// 
  /// # Returns 
  /// Magnitude of a number
  pub fn get_magnitude(&self) -> usize {
    if self.has_value() {
      self.get_value().clone()
    } else {
      3 * self.get_left().borrow().get_magnitude() + 2 * self.get_right().borrow().get_magnitude()
    }
  }

  /// Serializes the snailfish number into a string representation
  /// 
  /// # Returns
  /// String representation of the snailfish number
  pub fn to_string(&self) -> String {
    if self.has_value() {
      format!("{}", &self.get_value())
    } else {
      format!("[{},{}]", &self.get_left().borrow().to_string(), &self.get_right().borrow().to_string())
    }
  }

}
