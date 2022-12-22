//! 2022 day 21 puzzle
//! 
//! https://adventofcode.com/2022/day/21
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2022::lib::monkey_math::MonkeyMath;
use crate::year2022::lib::monkey_math::MonkeyMathExpandedEquation;

/// Parses input data
fn parse<'a>(data: &'a String) -> Vec<(&str, &str)> {
  Input::parse(data.as_str().trim(), "\n", |x| {
    let parsed = x.split(':').map(|x| x.trim()).collect::<Vec<&str>>();
    (parsed[0], parsed[1])
  })
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 21,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize monkey math
      let math = MonkeyMath::new(data, Option::None);

      // Calculate root
      let root = math.expand("root".to_string());

      // Return result
      String::from(format!("{:?}", MonkeyMath::resolve_without_unknowns(&root) as isize))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 21,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);
      
      // Initialize monkey math with "humn" node being an unknown variable value
      let math = MonkeyMath::new(data, Option::Some("humn"));

      // Calculate root
      let root = math.expand("root".to_string());

      // Rewrite root equation as an equality
      match root {
        MonkeyMathExpandedEquation::Addition(a, b) |
        MonkeyMathExpandedEquation::Subtraction(a, b) |
        MonkeyMathExpandedEquation::Product(a, b) |
        MonkeyMathExpandedEquation::Division(a, b) => {
          // Return result
          let result = MonkeyMath::resolve_equality_with_single_unknown(&MonkeyMathExpandedEquation::Equality(a, b));
          String::from(format!("{:?}", result as isize))
        },
        _ => panic!("Root equation should be an operation!")
      }
      
    }

  );

  // Return registry ownership
  registry
}
