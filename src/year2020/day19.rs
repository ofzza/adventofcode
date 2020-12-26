//! 2020/19 puzzle
//! 
//! https://adventofcode.com/2020/day/19
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::inputs::*;
use crate::lib::puzzle::*;
use std::collections::HashMap;
use std::cmp::Ordering;

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
        String::from("0: 4 1 5"),
        String::from("1: 2 3 | 3 2"),
        String::from("2: 4 4 | 5 5"),
        String::from("3: 4 5 | 5 4"),
        String::from("4: \"a\""),
        String::from("5: \"b\""),
        String::from(""),
        String::from("ababbb"),
        String::from("bababa"),
        String::from("abbbab"),
        String::from("aaabbb"),
        String::from("aaaabbb"),
      ]);
      stats.update(
        Puzzle::new(2020, 19, 1, "test", input, implementation1, |r| (r, Some(2)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<String>(load_input("./src/year2020/data/day19input.txt"), "\n");
      if index != 0 {
        // Run SLOW puzzle only if explicitly called for by index
        stats.update(
          Puzzle::new(2020, 19, 1, "solution", input, implementation1, |r| (r, Some(178)))
            .run(verbose, obfuscate)
        );
      } else {
        // Prompt puzzle is SLOW!
        Puzzle::new(2020, 19, 1, "test", input, implementation2, |r| (r, Some(178)))
          .prompt("Puzzle is known to be slow to execute, please explicitly execute the puzzle with --year/--day/--index arguments if you want it to run anyway. Execute with --verbose to keep track of progress ...");
      }
    }
  }

  // Run puzzle
  if (index == 0) ||(index == 2) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector1D(vec![
        String::from("42: 9 14 | 10 1"),
        String::from("9: 14 27 | 1 26"),
        String::from("10: 23 14 | 28 1"),
        String::from("1: \"a\""),
        String::from("11: 42 31"),
        String::from("5: 1 14 | 15 1"),
        String::from("19: 14 1 | 14 14"),
        String::from("12: 24 14 | 19 1"),
        String::from("16: 15 1 | 14 14"),
        String::from("31: 14 17 | 1 13"),
        String::from("6: 14 14 | 1 14"),
        String::from("2: 1 24 | 14 4"),
        String::from("0: 8 11"),
        String::from("13: 14 3 | 1 12"),
        String::from("15: 1 | 14"),
        String::from("17: 14 2 | 1 7"),
        String::from("23: 25 1 | 22 14"),
        String::from("28: 16 1"),
        String::from("4: 1 1"),
        String::from("20: 14 14 | 1 15"),
        String::from("3: 5 14 | 16 1"),
        String::from("27: 1 6 | 14 18"),
        String::from("14: \"b\""),
        String::from("21: 14 1 | 1 14"),
        String::from("25: 1 1 | 1 14"),
        String::from("22: 14 14"),
        String::from("8: 42"),
        String::from("26: 14 22 | 1 20"),
        String::from("18: 15 15"),
        String::from("7: 14 5 | 1 21"),
        String::from("24: 14 1"),
        String::from(""),
        String::from("abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa"),
        String::from("bbabbbbaabaabba"),
        String::from("babbbbaabbbbbabbbbbbaabaaabaaa"),
        String::from("aaabbbbbbaaaabaababaabababbabaaabbababababaaa"),
        String::from("bbbbbbbaaaabbbbaaabbabaaa"),
        String::from("bbbababbbbaaaaaaaabbababaaababaabab"),
        String::from("ababaaaaaabaaab"),
        String::from("ababaaaaabbbaba"),
        String::from("baabbaaaabbaaaababbaababb"),
        String::from("abbbbabbbbaaaababbbbbbaaaababb"),
        String::from("aaaaabbaabaaaaababaa"),
        String::from("aaaabbaaaabbaaa"),
        String::from("aaaabbaabbaaaaaaabbbabbbaaabbaabaaa"),
        String::from("babaaabbbaaabaababbaabababaaab"),
        String::from("aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba")
      ]);
      let input = match input {
        // Replace rules #8 and #11
        PuzzleInput::Vector1D(input) => PuzzleInput::Vector1D(
          input
            .iter()
            .map(|line| {
              if line == "8: 42" {
                return String::from("8: 42 | 42 8");
              } else if line == "11: 42 31" {
                return String::from("11: 42 31 | 42 11 31");
              } else {
                return line.clone();
              }
            })
            .collect()
        ),
        _ => panic!("This should never, ever happen!")
      };
      stats.update(
        Puzzle::new(2020, 19, 2, "test", input, implementation2, |r| (r, Some(12)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<String>(load_input("./src/year2020/data/day19input.txt"), "\n");
      let input = match input {
        // Replace rules #8 and #11
        PuzzleInput::Vector1D(input) => PuzzleInput::Vector1D(
          input
            .iter()
            .map(|line| {
              if line == "8: 42" {
                return String::from("8: 42 | 42 8");
              } else if line == "11: 42 31" {
                return String::from("11: 42 31 | 42 11 31");
              } else {
                return line.clone();
              }
            })
            .collect()
        ),
        _ => panic!("This should never, ever happen!")
      };
      stats.update(
        Puzzle::new(2020, 19, 2, "solution", input, implementation2, |r| (r, Some(346)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<String, usize, usize>, verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(input) => {
      // Parse input
      let parsed = parse_input(&input);
      let rules = parsed.0;
      let msgs = parsed.1;  
      if verbose {
        rules_to_string("  Read rules:", &rules);
      }    

      // Optimize    
      let rules = optimize_rules(rules, vec![], verbose);
      if verbose {
        rules_to_string("  Pre-compiled rules:", &rules);
      }    
      
      // Count matching messages
      let mut count = 0;
      for i in 0..msgs.len() {
        // Validate message
        if validate_msg(msgs[i].clone(), 0, rules.clone(), false) {
          count += 1;
        }
        // Prompt
        if verbose {
          println!("  -> {}. {} of {} messages matched!", i, count, msgs.len());
        }
      }
      
      // Return count
      return Ok(count);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<String, usize, usize>, verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(input) => {
      // Parse input
      let parsed = parse_input(&input);
      let rules = parsed.0;
      let msgs = parsed.1;  
      if verbose {
        rules_to_string("  Read rules:", &rules);
      }    

      // Optimize    
      let rules = optimize_rules(rules, vec![0, 8, 11], verbose);
      if verbose {
        rules_to_string("  Pre-compiled rules:", &rules);
      }    

      // Count matching messages
      let mut count = 0;
      for i in 0..msgs.len() {
        // Validate message
        if validate_msg_cheatingish(msgs[i].clone(), rules.clone(), verbose) {
          count += 1;
        }
        // Prompt
        if verbose {
          println!("  -> {}. {} of {} messages matched!", i, count, msgs.len());
        }
      }
      
      // Return count
      return Ok(count);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

/// Parses input into rules and a list of messages
/// 
/// # Arguments
/// * `lines` - Lines of text to parse into rules and messages
fn parse_input (lines: &Vec<String>) -> (HashMap<usize, Rule>, Vec<String>) {
  // Initialize rules and messages
  let mut rules: HashMap<usize, Rule> = HashMap::new();
  let mut msgs: Vec<String> = vec![];
  
  // Parse
  for line in lines {
    // Check for empty lines
    if line.trim().len() == 0 {
      continue;
    }

    // Split line
    let parsed_line: Vec<&str> = line.split(":").collect();

    // Store message
    if parsed_line.len() == 1 {
      msgs.push(String::from(line.trim()));
    }
    
    // Register explicit allowed character rule
    else if parsed_line.len() > 1 && parsed_line[1].contains(&"\"") {
      let option = parsed_line[1].trim().replace("\"", "").as_bytes()[0] as char;
      rules.insert(parsed_line[0].parse::<usize>().expect("Failed parsing rule index!"), Rule::Options(vec![Option::Values(vec![Value::Direct(option.to_string())])]));
    }

    // Parse complex rule, referencing other rules
    else {
      let options: Vec<Option> = parsed_line[1]
        .trim()
        .split("|")
        .map(
          |indices| {
            let values = indices.trim()
              .split(" ")
              .map(|index| Value::Indirect(index.parse::<usize>().expect("Failed parsing rule index!")))
              .collect();
            return Option::Values(values);
          }
        ).collect();
      rules.insert(parsed_line[0].parse::<usize>().expect("Failed parsing rule index!"), Rule::Options(options));
    }
  }

  // Return parsed input
  return (rules, msgs);
}

/// Optimizes rules by recombining them
/// 
/// Arguments
/// * `rules`    - Rules to optimize
/// * `ignore`   - List of rule IDs to not optimize
/// * `verbose`  - Outputs executing output of the puzzle to the console
fn optimize_rules (mut rules: HashMap<usize, Rule>, ignore: Vec<usize>, _verbose: bool) -> HashMap<usize, Rule> {
  // Process rules until no changes left to be made
  loop {
    // Initialize optimization detection
    let mut optimized = false;
    let rules_cloned = &rules.clone();
    
    // Attempt optimizations
    for rule_id in rules_cloned.keys() {

      // Ignore rule if in ignore list
      if ignore.contains(rule_id) {
        continue;
      }

      // Get rule
      let rule = rules.get_mut(rule_id).unwrap();
      
      // Replace indirect values to direct valued rules with direct values
      match rule {
        Rule::Options(options) => {

          // Process options
          for option_index in 0..options.len() {
            let option = &mut options[option_index];
            match option {
              Option::Values(values) => {

                // Process option values
                for value_index in 0..values.len() {
                  let value = &values[value_index];
                  match value {
                    Value::Indirect(rule_id) => {

                      // Check if rule with single
                      let direct_value_rule = rules_cloned.get(rule_id).unwrap();
                      match direct_value_rule {
                        Rule::Options(direct_value_options) => {
                          if direct_value_options.len() == 1 {
                            let direct_value_option = &direct_value_options[0];
                            match direct_value_option {
                              Option::Values(direct_match_values) => {
                                if direct_match_values.len() == 1 {
                                  let direct_match_value = &direct_match_values[0];
                                  match direct_match_value {
                                    Value::Direct(c) => {

                                      // Replace indirect value in option with a direct one
                                      optimized = true;
                                      values[value_index] = Value::Direct(c.clone());

                                    },
                                    _ => ()
                                  }
                                }
                              },
                              _ => ()
                            }
                          }
                        },
                        _ => ()
                      }

                    },
                    _ => ()
                  }
                }

              },
              _ => ()
            }
          }

        },
        _ => ()
      }
      if optimized { break; }

      // Replace indirect values to pre-compiled rules with pre-compiled values
      match rule {
        Rule::Options(options) => {

          // Process options
          for option_index in 0..options.len() {
            let option = &mut options[option_index];
            match option {
              Option::Values(values) => {

                // Process option values
                for value_index in 0..values.len() {
                  let value = &values[value_index];
                  match value {
                    Value::Indirect(rule_id) => {

                      // Check if rule with single
                      let direct_value_rule = rules_cloned.get(rule_id).unwrap();
                      match direct_value_rule {
                        Rule::Precompiled(v) => {

                          // Replace indirect value in option with a pre-compiled one
                          optimized = true;
                          values[value_index] = Value::Precompiled(v.clone());

                        },
                        _ => ()
                      }

                    },
                    _ => ()
                  }
                }

              },
              _ => ()
            }
          }

        },
        _ => ()
      }
      if optimized { break; }

      // Concatenate direct values into pre-compiled options
      match rule {
        Rule::Options(options) => {

          // Process options
          for option_index in 0..options.len() {
            let option = &mut options[option_index];
            match option {
              Option::Values(values) => {

                // Check if all option values are direct values
                if values.iter().find(|value| match value { Value::Direct(_) => false, _ => true }).is_none() {
                  // Replace option with a precalculated option
                  optimized = true;
                  options[option_index] = Option::Precompiled(
                    vec![
                      values.iter()
                        .map(
                          |value| match value { Value::Direct(value) => value.clone(), _ => panic!() }
                        )
                        .collect::<Vec<String>>()
                        .join("")
                    ]
                  );
                }              

              },
              _ => ()
            }
          }

        },
        _ => ()
      }
      if optimized { break; }

      // Concatenate adjecent direct and pre-compiled values into pre-compiled values
      match rule {
        Rule::Options(options) => {

          // Process options
          for option_index in 0..options.len() {
            let option = &mut options[option_index];
            match option {
              Option::Values(values) => {
                if values.len() > 0 {

                  // Find adjecent values that can be merged
                  for i in 1..values.len() {
                    let index_a = i - 1;
                    let index_b = i;
                    // Confirm that both values are of direct or pre-compiled type
                    match values[index_a] {
                      Value::Indirect(_) => (),
                      _ => {
                        match values[index_b] {
                          Value::Indirect(_) => (),
                          _ => {
                            
                            // Extract values' pre-compileds
                            let precompiled_a = match &values[index_a] {
                              Value::Direct(v) => vec![v.clone()],
                              Value::Precompiled(vs) => vs.clone(),
                              _ => panic!()
                            };
                            let precompiled_b = match &values[index_b] {
                              Value::Direct(v) => vec![v.clone()],
                              Value::Precompiled(vs) => vs.clone(),
                              _ => panic!()
                            };
                            
                            // Combine extracted values
                            let mut combined: Vec<String> = vec![];
                            for a in precompiled_a {
                              for b in precompiled_b.clone() {
                                combined.push(vec![a.clone(), b.clone()].join(""));
                              }
                            }

                            // Remove previous values
                            optimized = true;
                            values.remove(index_a);
                            values.remove(index_a);
                            // ... and replace them with a combined pre-compiled value
                            values.insert(index_a, Value::Precompiled(combined));
                            break;

                          }
                        }
                      }
                    }
                  }

                }
              },
              _ => ()
            }
          }

        },
        _ => ()
      }
      if optimized { break; }

      // Concatenate pre-compiled single value into a pre-compiled option
      match rule {
        Rule::Options(options) => {

          // Process options
          for option_index in 0..options.len() {
            let option = &mut options[option_index];
            match option {
              Option::Values(values) => {

                // Check if option only has a single pre-compiled value
                if values.len() == 1 {
                  if values.iter().find(|value| match value { Value::Precompiled(_) => false, _ => true }).is_none() {
                    // Replace option with a precalculated option
                    optimized = true;
                    options[option_index] = Option::Precompiled(
                      match &values[0] { Value::Precompiled(vs) => vs.clone(), _ => panic!() }
                    );
                  }   
                }
              },
              _ => ()
            }
          }

        },
        _ => ()
      }
      if optimized { break; }

      // Concatenate pre-compiled options into a pre-compiled rule
      match rule {
        Rule::Options(options) => {
          if options.iter().find(|option| match option { Option::Precompiled(_) => false, _ => true }).is_none() {
            // Collect all precompiled options' values
            let mut all_options: Vec<String> = vec![];
            for option in options {
              match option {
                Option::Precompiled(values) => {
                  for value in values {
                    all_options.push(value.clone());
                  }
                },
                _ => panic!()
              }
            }
            // Insert as precompiled rule
            optimized = true;
            rules.insert(rule_id.clone(), Rule::Precompiled(all_options));
          }
        },
        _ => ()
      }
      if optimized { break; }

    }

    // Prune unused rules
    let mut used_rule_ids: Vec<usize> = vec![0, 8, 11, 31, 42];
    for rule_id in rules_cloned.keys() {
      let rule = rules.get(rule_id).unwrap();
      match rule {
        Rule::Options(options) => {
          for option in options {
            match option {
              Option::Values(values) => {
                for value in values {
                  match value {
                    Value::Indirect(rule_id) => {
                      used_rule_ids.push(rule_id.clone());
                    },
                    _ => ()
                  }
                }
              },
              _ => ()
            }
          }
        },
        _ => ()
      }
    }
    for rule_id in rules_cloned.keys() {
      if !used_rule_ids.contains(rule_id) {
        // Get rule
        match rules.get(rule_id).unwrap() {
          Rule::None => (),
          _ => {
            // Remove unused rule
            optimized = true;
            rules.insert(rule_id.clone(), Rule::None);
          }
        }
      }
    }

    // If no optimizations left, break
    if !optimized { break; }
  }
  // Return proecessed rules
  return rules;
}

/// Validates a message according to a single rule, assuming the rule was optimized to have no outside references
/// 
/// Arguments
/// * `msg`        - Message to validate
/// * `rule_index` - ID of the rule to verify the message against
/// * `rules`      - Optimized rules
/// * `verbose`    - Outputs executing output of the puzzle to the console
fn validate_msg (msg: String, rule_index: usize, rules: HashMap<usize, Rule>, verbose: bool) -> bool {
  // Prompt
  if verbose {
    println!("  Matching message: \"{}\"", msg);
  }

  // Get rules
  let mut rule = match rules.get(&rule_index).unwrap() { Rule::Precompiled(values) => values.clone(), _ => panic!() };
  rule.sort_by(|a, b| if a.len() < b.len() { Ordering::Less } else { Ordering::Greater });
  let _rule_length = rule[0].len();
  
  // Set starting indices
  let mut starting_indices = vec![0];
    
  // Match rule for as long as possible
  loop {
    // Try matching rule
    let mut matched_indices: Vec<usize> = starting_indices.iter()
      // Try matching rule at given index
      .map(|index| {
        for value in rule.clone() {
          let index_start = index.clone();
          let index_end = index_start + value.len();
          if msg.len() >= index_end && msg[index_start..index_end] == value {
            return Some(index + value.len())
          }
        }
        return None;
      })
      .filter(|index| {
        return match index { Some(_) => true, None => false };
      })
      .map(|index| index.unwrap())
      .collect();
    // Sort and deduplicate results
    matched_indices.sort();
    matched_indices.dedup();
    // Check results
    if matched_indices.len() == 0 {
      break;
    } else {
      starting_indices = matched_indices;
    }
  }

  // Check match
  let matched = starting_indices.len() == 1 && starting_indices[0] == msg.len();
  
  // Prompt match
  if verbose {
    println!("  -> {}", if matched { "MATCHED!!!" } else { "NOT matched!" });
  }

  // Return match
  return matched;
}

/// Validates a message according to a single, particular rule consisting of syntax: (Ax[42]) (Bx[42] Bx[31]), A > 1, B > 1
/// 
/// Arguments
/// * `msg`        - Message to validate
/// * `rules`      - Optimized rules
/// * `verbose`    - Outputs executing output of the puzzle to the console
fn validate_msg_cheatingish (msg: String, rules: HashMap<usize, Rule>, verbose: bool) -> bool {
  // Prompt
  if verbose {
    println!("  Matching message: \"{}\"", msg);
  }

  // Get rules
  let mut rule31 = match rules.get(&31).unwrap() { Rule::Precompiled(values) => values.clone(), _ => panic!() };
  rule31.sort_by(|a, b| if a.len() < b.len() { Ordering::Less } else { Ordering::Greater });
  let _rule31_length = rule31[0].len();
  let mut rule42 = match rules.get(&42).unwrap() { Rule::Precompiled(values) => values.clone(), _ => panic!() };
  rule42.sort_by(|a, b| if a.len() < b.len() { Ordering::Less } else { Ordering::Greater });
  let _rule42_length = rule42[0].len();
  
  // Set starting indices
  let mut starting_indices = vec![0];
    
  // Match rule 42 for as long as possible
  let mut count_42 = 0;
  loop {
    // Try matching rule
    let mut matched_indices: Vec<usize> = starting_indices.iter()
      // Try matching rule at given index
      .map(|index| {
        for value in rule42.clone() {
          let index_start = index.clone();
          let index_end = index_start + value.len();
          if msg.len() >= index_end && msg[index_start..index_end] == value {
            return Some(index + value.len())
          }
        }
        return None;
      })
      .filter(|index| {
        return match index { Some(_) => true, None => false };
      })
      .map(|index| index.unwrap())
      .collect();
    // Sort and deduplicate results
    matched_indices.sort();
    matched_indices.dedup();
    // Check results
    if matched_indices.len() == 0 {
      break;
    } else {
      starting_indices = matched_indices;
      count_42 += 1;
    }
  }

  // Check not fully matched by rule 42
  if starting_indices.len() < 1 || starting_indices[0] == msg.len() {
    return false;
  }

  // Match rule 31 for as long as possible
  let mut count_31 = 0;
  loop {
    // Try matching rule
    let mut matched_indices: Vec<usize> = starting_indices.iter()
      // Try matching rule at given index
      .map(|index| {
        for value in rule31.clone() {
          let index_start = index.clone();
          let index_end = index_start + value.len();
          if msg.len() >= index_end && msg[index_start..index_end] == value {
            return Some(index + value.len())
          }
        }
        return None;
      })
      .filter(|index| {
        return match index { Some(_) => true, None => false };
      })
      .map(|index| index.unwrap())
      .collect();
    // Sort and deduplicate results
    matched_indices.sort();
    matched_indices.dedup();
    // Check results
    if matched_indices.len() == 0 {
      break;
    } else {
      starting_indices = matched_indices;
      count_31 += 1;
    }
  }

  // Check match
  let matched = starting_indices.len() == 1 && starting_indices[0] == msg.len() && (count_42 > count_31);
  
  // Prompt match
  if verbose {
    println!("  -> {}", if matched { "MATCHED!!!" } else { "NOT matched!" });
  }

  // Return match
  return matched;
}

/// Returns a string representation of rules
/// 
/// # Arguments
/// * `msg`   - Caption message to concatenate before the rules
/// * `rules` - Rules to return a string representation of
fn rules_to_string (msg: &str, rules: &HashMap<usize, Rule>) -> () {
  println!("  {}", msg);
  for rule_id in rules.keys() {
    let rule = rules.get(&rule_id).unwrap();
    println!("  > {}.: {:?}", rule_id, rule);
  }
  println!("\n");
}

/// Types of rules
#[derive(Debug, Clone)]
enum Rule {
  Options(Vec<Option>),
  Precompiled(Vec<String>),
  None
}

/// Types of rule options
#[derive(Debug, Clone)]
enum Option {
  Values(Vec<Value>),
  Precompiled(Vec<String>)
}

/// Types of rule values
#[derive(Debug, Clone)]
enum Value {
  Direct(String),
  Indirect(usize),
  Precompiled(Vec<String>)
}
