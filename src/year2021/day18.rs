//! 2021 day 18 puzzle
//! 
//! https://adventofcode.com/2021/day/18
// -----------------------------------------------------------------------------

// Include dependencies
use std::rc::Rc;
use std::cell::RefCell;
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2021::lib::snailfish_arithmetic::*;

/// Parses input data
fn parse(data: &String) -> Vec<Vec<char>> {
  Input::parse(data.trim(), "\n", |line| {
    line.chars().collect()
  })
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 18,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize sum (from first number)
      let mut sum = SnailFishArithmetic::parse(&data[0]).borrow().clone();
      // Sum all numbers
      for i in 1..data.len() {
        // Initialize number
        let number = SnailFishArithmetic::parse(&data[i]).borrow().clone();
        // Add to existing sum
        sum = SnailFishArithmetic::sum(sum, number);
      }

      // Calculate and return result
      println!("{}", sum.to_string());
      String::from(format!("{:?}", sum.get_magnitude()))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 18,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);
      let numbers: Vec<Rc<RefCell<SnailFishNumber>>> = data.iter().map(|n| SnailFishArithmetic::parse(n)).collect();
      
      // Sum every pair of numbers and find max sum
      let mut max: usize = 0;
      for i in 0..numbers.len() {
        for j in 0..numbers.len() {
          if i != j {
            let a = SnailFishArithmetic::sum(numbers[i].borrow().clone(), numbers[j].borrow().clone()).get_magnitude();
            if a > max { max = a; }
            let b = SnailFishArithmetic::sum(numbers[j].borrow().clone(), numbers[i].borrow().clone()).get_magnitude();
            if b > max { max = b; }
          }
        }
      }
      
      // Calculate and return result
      String::from(format!("{:?}", max))
    }

  );

  // Return registry ownership
  registry
}
