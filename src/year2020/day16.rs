//! 2020/16 puzzle
//! 
//! https://adventofcode.com/2020/day/16
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::inputs::*;
use crate::lib::puzzle::*;
use std::collections::HashMap;

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
        String::from("class: 1-3 or 5-7|row: 6-11 or 33-44|seat: 13-40 or 45-50").replace("|", "\n"),
        String::from("your ticket:|7,1,14").replace("|", "\n"),
        String::from("nearby tickets:|7,3,47|40,4,50|55,2,20|38,6,12").replace("|", "\n")
      ]);
      stats.update(
        Puzzle::new(2020, 16, 1, "test", input, implementation1, |r| (r, Some(71)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d(load_input("./src/year2020/data/day16input.txt"), "\n\n");
      stats.update(
        Puzzle::new(2020, 16, 1, "solution", input, implementation1, |r| (r, Some(22057)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Run puzzle
  if (index == 0) || (index == 2) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::ParamVector1D(String::from(""), vec![
        String::from("class: 0-1 or 4-19|row: 0-5 or 8-19|seat: 0-13 or 16-19").replace("|", "\n"),
        String::from("your ticket:|11,12,13").replace("|", "\n"),
        String::from("nearby tickets:|3,9,18|15,1,5|5,14,9").replace("|", "\n")
      ]);
      stats.update(
        Puzzle::new(2020, 16, 2, "test", input, implementation2, |r| (r, Some(1716)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = match parse_1d::<String>(load_input("./src/year2020/data/day16input.txt"), "\n\n") {
        PuzzleInput::Vector1D(code) => PuzzleInput::ParamVector1D(String::from("departure"), code),
        _ => panic!("This should never, ever happen!")
      };
      stats.update(
        Puzzle::new(2020, 16, 2, "solution", input, implementation2, |r| (r, Some(1093427331937)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<String, usize, usize>, _verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(input) => {
      // Parse input
      let (rules, _, other_tickets) = parse_input(&input);
      // Find invalid ticket values
      let mut checksum: usize = 0;
      for ticket in other_tickets {
        let invalid_values = validate_ticket(&ticket, &rules);
        for invalid_value in invalid_values {
          checksum += invalid_value;
        }
      }
      // Return checksum
      return Ok(checksum);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<String, usize, usize>, _verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::ParamVector1D(key_filter, input) => {
      // Parse input
      let (rules, your_ticket, other_tickets) = parse_input(&input);
      // Decode tickets
      let mut mapping = decode_ticket_fields(&other_tickets, &rules);
      let mapping = prune_decoded_fields(&mut mapping);
      // Filter mapping keys
      let keys: Vec<&str> = rules.keys()
        .map(|key| key.clone())
        .filter(|key| key.starts_with(key_filter))
        .collect();
      // Multiply values from ticket
      let mut checksum = 1;
      for key in keys {
        let index = mapping[key].clone();
        checksum *= your_ticket[index];
      }
      // Return checksum
      Ok(checksum)
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

/// Parses inputs into rules, your ticket and other tickets
/// 
/// # Arguments
/// * `input` - Input to parse split by sections
fn parse_input (input: &Vec<String>) -> (HashMap<&str, Vec<(usize, usize)>>, Vec<usize>, Vec<Vec<usize>>) {
  // Parse rules
  let parsed: Vec<&str> = input[0].split('\n').collect();
  let mut rules: HashMap<&str, Vec<(usize, usize)>> = HashMap::new();
  for rule in parsed {
    let parsed_rule: Vec<&str> = rule.split(':').collect();
    let rule_key: &str = parsed_rule[0];
    let rule_ranges:Vec<(usize, usize)> = parsed_rule[1]
      .split("or")
      .map(|range| {
        let parsed_range: Vec<usize> = range
          .trim()
          .split('-')
          .map(|value| value.parse::<usize>().expect("Failed parsing value!"))
          .collect::<Vec<usize>>();
        return (parsed_range[0], parsed_range[1]);
      })
      .collect();
    rules.insert(rule_key, rule_ranges);
  }

  // Parse your ticket
  let parsed: Vec<&str> = input[1].split('\n').collect();
  let your_ticket: Vec<usize> = parsed[1].trim().split(',')
    .map(|value| value.parse::<usize>().expect("Failed parsing value!"))
    .collect();

  // Parse other tickets
  let parsed: Vec<&str> = input[2].split('\n').collect();
  let mut other_tickets: Vec<Vec<usize>> = vec![];
  for i in 1..parsed.len() {
    other_tickets.push(
      parsed[i].trim().split(',')
        .map(|value| value.parse::<usize>().expect("Failed parsing value!"))
        .collect()
    );
  }

  // Return result
  return (rules, your_ticket, other_tickets);
}

/// Validates a ticket by matching all its values to all known rules and returns ticket values that were found to be invalid
/// 
/// # Arguments
/// * `ticket` - Ticket to validate
/// * `rules`  - All rules against whose ranges ticket values are being validated
fn validate_ticket (ticket: &Vec<usize>, rules: &HashMap<&str, Vec<(usize, usize)>>) -> Vec<usize> {
  // Initialize invalid vales
  let mut invalid: Vec<usize> = vec![];
  // Validate ticket
  for value in ticket {
    // Check if value value by any rule
    let mut valid_by_rule: bool = false;
    for rule in rules.values() {
      for range in rule {
        // Check if value valid by any range of the rule
        if value >= &range.0 && value <= &range.1 {
          valid_by_rule = true;
          break;
        }
      }
      if valid_by_rule { break; }
    }
    // If value invalid
    if !valid_by_rule {
      invalid.push(value.clone());
      break;
    }
  }
  // Return invalid values
  return invalid;
}

/// Decodes valid tickets by matching valid field indices for each rule and return all matching field indices for each rule key
/// 
/// # Arguments
/// * `tickets` - Tickets to decode
/// * `rules`   - All rules against whose ranges tickets are to be decoded
fn decode_ticket_fields (tickets: &Vec<Vec<usize>>, rules: &HashMap<&str, Vec<(usize, usize)>>) -> HashMap<String, Vec<usize>> {
  // Initialize mapping
  let mut mapping:HashMap<String, Vec<usize>> = HashMap::new();
  let length = rules.len();

  // Filter tickets
  let filtered_tickets: Vec<Vec<usize>> = tickets.iter()
    .filter(|ticket| {
      let invalid: Vec<usize> = validate_ticket(&ticket, &rules);
      return invalid.len() == 0;
    })
    .map(|ticket| ticket.clone())
    .collect();

  // Decode ticket fields by rules
  for field_index in 0..length {    
    for rule_key in rules.keys() {
      
      // Get ranges
      let ranges = rules[rule_key].clone();

      // Check rule against field of all tickets
      let mut all_tickets_matched_rule = true;
      for filtered_ticket in &filtered_tickets {
        let field_value = filtered_ticket[field_index];
        // Check rule ranges for match
        let mut ticket_matched_range = false;
        for range in &ranges {
          if field_value >= range.0 && field_value <= range.1 {
            ticket_matched_range = true;
            break;
          }
        }
        // Check if range matched
        if !ticket_matched_range {
          all_tickets_matched_rule = false;
          break;
        }
      }

      // Check if mapping found
      if all_tickets_matched_rule {
        // Check if key initialized
        let rule_key = String::from(rule_key.clone());
        if !mapping.contains_key(&rule_key) { mapping.insert(rule_key.clone(), vec![]); }
        mapping.get_mut(&rule_key).expect("Can't find hashmap key!").push(field_index);
      }

    }
  }

  // Return mapping
  return mapping;
}

/// Prunes out unique ticket field index to rule key mapping
/// 
/// # Arguments
/// * `mapping` - Decoded mapping of ticket fields to rule keys
fn prune_decoded_fields (mapping: &mut HashMap<String, Vec<usize>>) -> HashMap<String, usize> {
  // Prune mappings down to singular relationships
  let keys: Vec<String> = mapping.keys().map(|key| key.clone()).collect();
  let mut pruned: HashMap<String, usize> = HashMap::new();
  loop {
    // Search for singular mapping
    let mut matched_single = false;
    for key in &keys {
      // Get indices for key
      let indices: &mut Vec<usize> = mapping.get_mut(key).expect("Can't find hashmap key!");
      // Check if singular index
      if indices.len() == 1 {
        
        // Get value
        let value = indices[0].clone();

        // Register singular mapping
        pruned.insert(key.clone(), value);

        // Remove value from other mappings
        for key in &keys {   
          let indices: &mut Vec<usize> = mapping.get_mut(key).expect("Can't find hashmap key!");
          match indices.iter().position(|i| i.clone() == value) {
            Some(position) => { indices.remove(position); },
            None => ()
          };
        }

        // Singular mapping found
        matched_single = true;
        break;
      }
    }

    // No singular relationship found - done pruning
    if !matched_single { 
      break;
    }
  }

  // Return mapping
  return pruned;
}
