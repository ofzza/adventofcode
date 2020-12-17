//! Math(ish) module
//! 
//! Implements Math with some weird(ish) features interpreter
// -----------------------------------------------------------------------------

/// Math(ish) struct
#[derive(Debug, Default)]
pub struct Mathish {
}

/// Math(ish) implementation
impl Mathish {
  
  /// Evaluates a mathematical expression and returns a result
  /// 
  /// # Arguments
  /// * `expression`     - Expression to evaluate
  /// * `operator_order` - Priority order of operations (supported operations: [`+`, `*`]); Pass empty vector for no priority
  /// * `verbose`        - Outputs executing output of the puzzle to the console
  pub fn eval (expression: String, operator_order: Vec<char>, verbose: bool) -> usize {
    // Walk expression and collect linear operations
    let expression_bytes = expression.as_bytes();
    let mut expression_depth = 0;
    let mut read_buffer: Vec<u8> = vec![];
    let mut last_operator: char = '+';
    let mut collected_operations: Vec<(char, usize)> = vec![];
    for i in 0..(expression.len() + 1) {
      // Track expression depth
      if i < expression_bytes.len() && expression_bytes[i] as char == '(' {
        expression_depth += 1;
      }
      else if i < expression_bytes.len() && expression_bytes[i] as char == ')' {
        expression_depth -= 1;
      }
      
      // Parse character and perform previous operation before updating to next operation
      if expression_depth == 0 && (i >= expression_bytes.len() || expression_bytes[i] as char == '+' || expression_bytes[i] as char == '*') {
        // Parse buffered expression
        let parsed: usize = match std::str::from_utf8(&read_buffer) {
          Ok(value) => match value.parse::<usize>() {
            Ok(value) => value,
            Err(_) => Mathish::eval(value[1..(value.len() - 1)].to_string(), operator_order.clone(), verbose)
          },
          Err(_) => panic!("Failed collecting read buffer to string!")
        };
        // Clear buffer
        read_buffer = vec![];
        // Store linear operation
        collected_operations.push((last_operator, parsed));
        if i < expression_bytes.len() {
          last_operator = expression_bytes[i] as char;
        }
      }
      // Read to buffer
      else if i < expression_bytes.len() && expression_bytes[i] as char != ' ' {
        read_buffer.push(expression_bytes[i]);
      }
  
      // Check if index out of bounds
      if i >= expression_bytes.len() {
        break;
      }
    }
  
    // Collapse colleted operations into a single result
    for operator in if operator_order.len() > 0 { operator_order } else { vec!['?'] } {
      // Walk operations
      let len = collected_operations.len();
      let mut index = 0;
      for _ in 0..(len - 1) {
        // Calculate current operation arguments' indices
        let x = index;      
        let y = index + 1;
        // Depending on operator, collapse operation onto previous one
        if (operator == '?' || operator == '+') && collected_operations[y].0 == '+' {
          collected_operations[x].1 += collected_operations[y].1;
          collected_operations.remove(y);
        }
        else if (operator == '?' || operator == '*') && collected_operations[y].0 == '*' {
          collected_operations[x].1 *= collected_operations[y].1;
          collected_operations.remove(y);
        }
        else {
          // Proceed with index
          index += 1;
        }
      }
    }
  
    // Prompt
    if verbose {
      println!("  > {} = {}", expression.replace(" ", ""), collected_operations[0].1);
    }
  
    // Return result
    return collected_operations[0].1;
  }

}

