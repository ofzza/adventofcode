//! Monkey Math module
//! 
//! Monkey Math module
// -----------------------------------------------------------------------------

// Include dependencies
use std::rc::Rc;
use std::borrow::Borrow;
use std::collections::hash_map::HashMap;

/// Equasion enum
#[derive(Debug)]
pub enum MonkeyMathEquation {
  Unknown,
  Constant(f64),
  Calculation(String, char, String)
}

/// Equation result
#[derive(Debug)]
pub enum MonkeyMathExpandedEquation {
  Unknown,
  Constant(f64),
  Addition(Rc<MonkeyMathExpandedEquation>, Rc<MonkeyMathExpandedEquation>),
  Subtraction(Rc<MonkeyMathExpandedEquation>, Rc<MonkeyMathExpandedEquation>),
  Product(Rc<MonkeyMathExpandedEquation>, Rc<MonkeyMathExpandedEquation>),
  Division(Rc<MonkeyMathExpandedEquation>, Rc<MonkeyMathExpandedEquation>),
  Equality(Rc<MonkeyMathExpandedEquation>, Rc<MonkeyMathExpandedEquation>),
}

/// Monkey Math structure
pub struct MonkeyMath {
  equations: HashMap<String, MonkeyMathEquation>
}

/// Monkey Math implementation
impl MonkeyMath {

  /// Constructor
  pub fn new (equations:  Vec<(&str, &str)>, unknown_key: Option<&str>) -> MonkeyMath {
    // Initialize hashmap
    let mut hash: HashMap<String, MonkeyMathEquation> = HashMap::with_capacity(equations.len());
    // Parse equasions into the hashmap
    for equation in equations {
      let parsed = equation.1.split(' ').collect::<Vec<&str>>();
      let value =
        if parsed.len() == 1 {
          MonkeyMathEquation::Constant(
            parsed[0].parse::<f64>().unwrap()
          )
        }
        else if parsed.len() == 3 {
          MonkeyMathEquation::Calculation(
            parsed[0].to_string(),
            parsed[1].chars().nth(0).unwrap(),
            parsed[2].to_string()
          )
        } else {
          panic!("Can't parse equation!!!");
        };
      hash.insert(
        equation.0.to_string(),
        match unknown_key {
          Option::None => value,
          Option::Some(unknown_key) => if equation.0 == unknown_key { MonkeyMathEquation::Unknown } else { value }
        }
      );
    }
    // Return instance
    MonkeyMath {
      equations: hash
    }
  }

  /// Calculates a requested node
  /// 
  /// # Arguments
  /// * key: Key of the node to calculate
  /// 
  /// # Returns
  /// Result of the calculation
  pub fn expand (&self, key: String) -> MonkeyMathExpandedEquation {
    // Get node equation
    let equation = self.equations.get(&key).unwrap();

    // Resolve node equasion
    match equation {
      // If node equation is am unknown
      MonkeyMathEquation::Unknown => MonkeyMathExpandedEquation::Unknown,
      // If node equation is a constant, return the constant value
      MonkeyMathEquation::Constant(value) => MonkeyMathExpandedEquation::Constant(value.clone()),
      // If node equation is a calculation, perform the calculation
      MonkeyMathEquation::Calculation(a, operation, b) => {

        // Calculate first operand value
        let value_a = self.expand(a.clone());
        // Calculate second operand value
        let value_b = self.expand(b.clone());

        // Calculate
        if operation.clone() == '+' {
          MonkeyMathExpandedEquation::Addition(Rc::new(value_a), Rc::new(value_b))
        }
        else if operation.clone() == '-' {
          MonkeyMathExpandedEquation::Subtraction(Rc::new(value_a), Rc::new(value_b))
        }
        else if operation.clone() == '*' {
          MonkeyMathExpandedEquation::Product(Rc::new(value_a), Rc::new(value_b))
        }
        else if operation.clone() == '/' {
          MonkeyMathExpandedEquation::Division(Rc::new(value_a), Rc::new(value_b))
        } else {
          panic!("Unknown operation: {}", operation);
        }

      }
    }
  }

  /// Counts all unknowns used within the equation
  /// 
  /// # Arguments
  /// * equation: Equation to check for unknowns
  /// 
  /// # Returns
  /// Number of unknowns that were found used in the equation
  fn count_unknowns (equation: &MonkeyMathExpandedEquation) -> usize {
    match equation {
      MonkeyMathExpandedEquation::Unknown => 1,
      MonkeyMathExpandedEquation::Constant(_) => 0,
      MonkeyMathExpandedEquation::Equality(a, b) |
      MonkeyMathExpandedEquation::Addition(a, b) |
      MonkeyMathExpandedEquation::Subtraction(a, b) |
      MonkeyMathExpandedEquation::Product(a, b) |
      MonkeyMathExpandedEquation::Division(a, b) => MonkeyMath::count_unknowns(a.borrow()) + MonkeyMath::count_unknowns(b.borrow())
    }
  }

  /// Resolves an equasion with no unknowns
  /// 
  /// # Arguments
  /// * equation: Equation to resolve
  /// 
  /// # Returns
  /// Value produced by resolving the equation
  pub fn resolve_without_unknowns (equation: &MonkeyMathExpandedEquation) -> f64 {
    match equation {
      MonkeyMathExpandedEquation::Unknown => panic!("The method .resolve_without_unknowns() can only be used to resolve equations with no unknowns and no equalities. MonkeyMathExpandedEquation::Unknown found!"),
      MonkeyMathExpandedEquation::Constant(value) => value.clone(),
      MonkeyMathExpandedEquation::Equality(_, _) => panic!("The method .resolve_without_unknowns() can only be used to resolve equations with no unknowns and no equalities. MonkeyMathExpandedEquation::Equality found!"),
      MonkeyMathExpandedEquation::Addition(a, b) => MonkeyMath::resolve_without_unknowns(a.borrow()) + MonkeyMath::resolve_without_unknowns(b.borrow()),
      MonkeyMathExpandedEquation::Subtraction(a, b) => MonkeyMath::resolve_without_unknowns(a.borrow()) - MonkeyMath::resolve_without_unknowns(b.borrow()),
      MonkeyMathExpandedEquation::Product(a, b) => MonkeyMath::resolve_without_unknowns(a.borrow()) * MonkeyMath::resolve_without_unknowns(b.borrow()),
      MonkeyMathExpandedEquation::Division(a, b) => MonkeyMath::resolve_without_unknowns(a.borrow()) / MonkeyMath::resolve_without_unknowns(b.borrow())
    }
  }

  pub fn resolve_equality_with_single_unknown (equation: &MonkeyMathExpandedEquation) -> f64 {
    // Unpack equation as an equality
    match equation {      
      MonkeyMathExpandedEquation::Equality(a, b) => {
        // Count unknowns used in each of the operands
        let unknowns_count_a = MonkeyMath::count_unknowns(a.borrow());
        let unknowns_count_b = MonkeyMath::count_unknowns(b.borrow());

        // Check if single unknown
        if unknowns_count_a + unknowns_count_b != 1 {
          panic!("The method .resolve_equality_with_single_unknown() can only be used to resolve equalities with a single unknown!")
        }

        // Sort equality sides by which side contains the unknown
        let (unknown_side, calculation_side) = if unknowns_count_a == 1 { (a, b) } else { (b, a) };

        // Unpack up the equality until the unknown side is "naked"
        match unknown_side.borrow() {
          // If "naked" unknown, done unpacking and should calculate the other side
          MonkeyMathExpandedEquation::Unknown => MonkeyMath::resolve_without_unknowns(calculation_side.borrow()),
          
          // Unpack operations
          MonkeyMathExpandedEquation::Addition(a, b) => {
            let (unknown_side_unknown_side, unknown_side_calculation_side) = if MonkeyMath::count_unknowns(a) == 1 { (a, b) } else { (b, a) };
            MonkeyMath::resolve_equality_with_single_unknown(
              &MonkeyMathExpandedEquation::Equality(
                unknown_side_unknown_side.clone(),
                Rc::new(MonkeyMathExpandedEquation::Subtraction(calculation_side.clone(), unknown_side_calculation_side.clone()))
              )
            )
          },
          MonkeyMathExpandedEquation::Subtraction(a, b) => {
            if MonkeyMath::count_unknowns(a) == 1 {
              MonkeyMath::resolve_equality_with_single_unknown(
                &MonkeyMathExpandedEquation::Equality(
                  a.clone(),
                  Rc::new(MonkeyMathExpandedEquation::Addition(calculation_side.clone(), b.clone()))
                )
              )
            } else {
              MonkeyMath::resolve_equality_with_single_unknown(
                &MonkeyMathExpandedEquation::Equality(
                  b.clone(),
                  Rc::new(MonkeyMathExpandedEquation::Subtraction(a.clone(), calculation_side.clone()))
                )
              )
            }
          },
          MonkeyMathExpandedEquation::Product(a, b) => {
            let (unknown_side_unknown_side, unknown_side_calculation_side) = if MonkeyMath::count_unknowns(a) == 1 { (a, b) } else { (b, a) };
            MonkeyMath::resolve_equality_with_single_unknown(
              &MonkeyMathExpandedEquation::Equality(
                unknown_side_unknown_side.clone(),
                Rc::new(MonkeyMathExpandedEquation::Division(calculation_side.clone(), unknown_side_calculation_side.clone()))
              )
            )
          },
          MonkeyMathExpandedEquation::Division(a, b) => {
            if MonkeyMath::count_unknowns(a) == 1 {
              MonkeyMath::resolve_equality_with_single_unknown(
                &MonkeyMathExpandedEquation::Equality(
                  a.clone(),
                  Rc::new(MonkeyMathExpandedEquation::Product(calculation_side.clone(), b.clone()))
                )
              )
            } else {
              MonkeyMath::resolve_equality_with_single_unknown(
                &MonkeyMathExpandedEquation::Equality(
                  b.clone(),
                  Rc::new(MonkeyMathExpandedEquation::Division(a.clone(), calculation_side.clone()))
                )
              )
            }
          },

          // Should not be able to contain a constant or equality
          MonkeyMathExpandedEquation::Constant(_) => panic!(""),
          MonkeyMathExpandedEquation::Equality(_, _) => panic!("")
        }
      },
      _ => panic!("The method .resolve_equality_with_single_unknown() can only be used to resolve equalities with a single unknown!")
    }
  }
}
