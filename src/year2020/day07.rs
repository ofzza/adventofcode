//! 2020/07 puzzle
//! 
//! https://adventofcode.com/2020/day/7
// -----------------------------------------------------------------------------

// Include dependencies
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::lib::inputs::*;
use crate::lib::puzzle::*;

/// Registers puzzles for the day
pub fn run (index: u32, key: &str, verbose: bool, obfuscate: bool) -> PuzzleExecutionStatistics {

  // Initialize stats
  let mut stats = PuzzleExecutionStatistics{
    ..Default::default()
  };

  // Run puzzle
  if (index == 0) || (index == 1) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector1D(vec![
        String::from("light red bags contain 1 bright white bag, 2 muted yellow bags."),
        String::from("dark orange bags contain 3 bright white bags, 4 muted yellow bags."),
        String::from("bright white bags contain 1 shiny gold bag."),
        String::from("muted yellow bags contain 2 shiny gold bags, 9 faded blue bags."),
        String::from("shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags."),
        String::from("dark olive bags contain 3 faded blue bags, 4 dotted black bags."),
        String::from("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags."),
        String::from("faded blue bags contain no other bags."),
        String::from("dotted black bags contain no other bags.")
      ]);
      stats.update(
        Puzzle::new(2020, 7, 1, "test", input, implementation1, |r| (r, Some(4)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<String>(load_input("./src/year2020/data/day07input.txt"), "\n");
      stats.update(
        Puzzle::new(2020, 7, 1, "solution", input, implementation1, |r| (r, Some(169)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Run puzzle
  if (index == 0) || (index == 2) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector1D(vec![
        String::from("shiny gold bags contain 2 dark red bags."),
        String::from("dark red bags contain 2 dark orange bags."),
        String::from("dark orange bags contain 2 dark yellow bags."),
        String::from("dark yellow bags contain 2 dark green bags."),
        String::from("dark green bags contain 2 dark blue bags."),
        String::from("dark blue bags contain 2 dark violet bags."),
        String::from("dark violet bags contain no other bags."),
      ]);
      stats.update(
        Puzzle::new(2020, 7, 2, "test", input, implementation2, |r| (r, Some(126)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<String>(load_input("./src/year2020/data/day07input.txt"), "\n");
      stats.update(
        Puzzle::new(2020, 7, 2, "solution", input, implementation2, |r| (r, Some(82372)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<String, usize, usize>, verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(rules) => {
      // Parse rules
      let rules = parse_rules(rules, false);
      // Process bags
      let bags = process_rules(rules, verbose);
      // Count children
      let mut counted = HashMap::new();
      match count_parents(String::from("shiny gold"), &bags, verbose, &mut counted) {
        Some(count) => Ok(count),
        None => Err("Failed counting bags!")
      }
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<String, usize, usize>, verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(rules) => {
      // Parse rules
      let rules = parse_rules(rules, false);
      // Process bags
      let bags = process_rules(rules, verbose);
      // Count children
      match count_children(String::from("shiny gold"), &bags, verbose) {
        Some(count) => Ok(count),
        None => Err("Failed counting bags!")
      }
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

/// Parses bag nesting rules
/// 
/// # Arguments
/// * `rules`   - All rules' syntax
/// * `verbose` - Outputs executing output of the puzzle to the console
fn parse_rules (rules: &Vec<String>, verbose: bool) -> Vec<(String, Vec<(String, usize)>)> {
  // If verbose, prompt
  if verbose {
    println!("Parsing rules ...");
  }

  // Initialize result
  let mut result: Vec<(String, Vec<(String, usize)>)> = vec![];

  // Parse lines
  for rule in rules {
    // Parse rule
    let rule_clean: String = rule.replace(" bags", "").replace(" bag", "").replace(".", "");
    let rule_parsed: Vec<&str> = rule_clean.trim().split("contain").collect();
    // Parse parent bag
    let rule_parent_name: String = String::from(rule_parsed[0].trim());
    // Parse child bags
    let rule_children_parsed: Vec<&str> = rule_parsed[1].trim().split(',').collect();
    let mut children: Vec<(String, usize)> = vec![];
    for rule_child in rule_children_parsed {
      if rule_child.trim() != "no other" {
        let rule_child_parsed: Vec<&str> = rule_child.trim().split(' ').collect();
        let rule_child_quantity: usize = match rule_child_parsed[0].trim().parse::<usize>() {
          Ok(n) => n,
          Err(_) => panic!("Could not parse child bag quantity for {}", rule)
        };
        let rule_child_name: String = String::from(rule_child_parsed[1..].join(" ").trim());
        children.push((rule_child_name, rule_child_quantity));
      }
    }
    result.push((rule_parent_name, children));

    // If verbose, prompt result
    if verbose {
      println!("{} -> {:?}", rule,  result);
    }
  }

  // Return result
  return result;
}

/// Processes parsed bag nesting rules into a graph structure of nested bag instances
/// 
/// # Arguments
/// * `parsed_rules` - Parsed bag rules
/// * `verbose`      - Outputs executing output of the puzzle to the console
fn process_rules (parsed_rules: Vec<(String, Vec<(String, usize)>)>, verbose: bool) -> HashMap<String, Rc<RefCell<Bag>>> {
  // If verbose, prompt
  if verbose {
    println!("Processing rules ...");
  }

  // Initialize result
  let mut hashmap: HashMap<String, Rc<RefCell<Bag>>> = HashMap::new();
  
  // Process bag rules
  for rule in parsed_rules {
    // Get or instantiate parent bag
    let key = String::from(rule.0);
    let parent_bag = if !hashmap.contains_key(&key) {
      Rc::new(
        RefCell::new(
          Bag {
            name: key.clone(),
            parents: vec![],
            children: vec![]
          }
        )
      )
    } else {
      hashmap.remove(&key).expect("Failed removing parent bag!")
    };

    // Process bag children
    for child in rule.1 {
      // Get or instantiate child bag
      let key = String::from(child.0);
      let child_bag = if !hashmap.contains_key(&key) {
        Rc::new(
          RefCell::new(
            Bag {
              name: key.clone(),
              parents: vec![],
              children: vec![]
            }
          )
        )
      } else {
        hashmap.remove(&key).expect("Failed removing child bag!")
      };
      
      // Add child to parent
      parent_bag.borrow_mut().children.push((child_bag.clone(), child.1));
      
      // Add parent to child
      child_bag.borrow_mut().parents.push(parent_bag.clone());

      // (Re)Insert child
      hashmap.insert(key.clone(), child_bag);
    }

    // If verbose, prompt result
    if verbose {
      println!("{} -> children: {}, parents: {}", parent_bag.borrow().name, parent_bag.borrow().children.len(), parent_bag.borrow().parents.len());
    }

    // (Re)Insert parent
    hashmap.insert(key.clone(), parent_bag);
  }

  // Return result
  return hashmap;
}

/// Counts direct and indirect parents of a bag
/// 
/// # Arguments
/// * `key`             - Name of the bag to count parents for
/// * `processed_rules` - Rules processed into a graph structure of nested bag instances
/// * `verbose`         - Outputs executing output of the puzzle to the console
/// * `counter`         - (Internal) Hashmap of already counter parents (used internally to prevent double counting of parents)
fn count_parents (key: String, processed_rules: &HashMap<String, Rc<RefCell<Bag>>>, verbose: bool, counted: &mut HashMap<String, bool>) -> Option<usize> {
  // Find initial bag
  match processed_rules.get(&key) {
    Some(bag) => {
      // Initialize count
      let mut recursive_count = 0;
      // Count parents
      for parent_bag in &bag.borrow().parents {
        // Check if parent already counted
        if !counted.contains_key(&parent_bag.borrow().name) {
          // Count parent
          recursive_count += 1;
          counted.insert(parent_bag.borrow().name.clone(), true);
          // Count parent's parents recursivelly
          match count_parents(parent_bag.borrow().name.clone(), processed_rules, verbose, counted) {
            Some(count) => { recursive_count += count; },
            None => {}
          }
        }
      }
      return Some(recursive_count);
    },
    None => {
      return None;
    }
  }
}

/// Counts total number of direct and indirect children of a bag
/// 
/// # Arguments
/// * `key`             - Name of the bag to count children for
/// * `processed_rules` - Rules processed into a graph structure of nested bag instances
/// * `verbose`         - Outputs executing output of the puzzle to the console
fn count_children (key: String, processed_rules: &HashMap<String, Rc<RefCell<Bag>>>, verbose: bool) -> Option<usize> {
  // Find initial bag
  match processed_rules.get(&key) {
    Some(bag) => {
      // Initialize count
      let mut recursive_count = 0;
      // Count parents
      if bag.borrow().children.len() == 0 {
        return Some(0);
      } else {
        for child_bag in &bag.borrow().children {
          // Count parent's parents recursivelly
          match count_children(child_bag.0.borrow().name.clone(), processed_rules, verbose) {
            Some(count) => {
              recursive_count += child_bag.1 * (1 + count);
            },
            None => {}
          }
        }
        return Some(recursive_count);
      }
    },
    None => {
      return None;
    }
  }
}

/// Bag structure holds bag information and references to parents and children
#[derive(Debug, Clone)]
struct Bag {
  name: String,
  parents: Vec<Rc<RefCell<Bag>>>,
  children: Vec<(Rc<RefCell<Bag>>, usize)>
}
