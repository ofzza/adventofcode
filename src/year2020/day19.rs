//! 2020/19 puzzle
//! 
//! https://adventofcode.com/2020/day/19
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
      stats.update(
        Puzzle::new(2020, 19, 1, "solution", input, implementation1, |r| (r, Some(178)))
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
      stats.update(
        Puzzle::new(2020, 19, 2, "test", input, implementation2, |r| (r, Some(3)))
          .run(verbose, obfuscate)
      );
    }
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
        Puzzle::new(2020, 19, 2, "solution", input, implementation2, |r| (r, None))
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
      let mut rules = parsed.0;
      let msgs = parsed.1;

      // Get rules' lengths
      rules = get_rule_lengths(&mut rules, verbose);

      // Count matching messages
      let mut count = 0;
      for i in 0..msgs.len() {
        // Validate message
        let offsets = validate_msg(&msgs[i][..], 0, &rules, verbose, 0);
        if offsets.len() == 1 && offsets[0] == "" {
          count += 1;
        }
        // Prompt
        if verbose {
          println!("{} of {} messages matched!", count, msgs.len());
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
      let mut rules = parsed.0;
      let msgs = parsed.1;

      // Get rules' lengths
      rules = get_rule_lengths(&mut rules, verbose);
      rules = simplify_rules(&mut rules, verbose);

      // Count matching messages
      let mut count = 0;
      for i in 0..msgs.len() {
        // Validate message
        let offsets = validate_msg(&msgs[i][..], 0, &rules, verbose, 0);
        if offsets.len() == 1 && offsets[0] == "" {
          count += 1;
        }
        // Prompt
        if verbose {
          println!("{} of {} messages matched!", count, msgs.len());
        }
      }
      
      // Return count
      return Ok(count);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

/// TODO: ...
fn parse_input (lines: &Vec<String>) -> (HashMap<usize, (usize, Vec<Vec<Value>>)>, Vec<String>) {
  // Initialize rules and messages
  let mut rules: HashMap<usize, (usize, Vec<Vec<Value>>)> = HashMap::new();
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
      rules.insert(parsed_line[0].parse::<usize>().expect("Failed parsing rule index!"), (0, vec![vec![Value::Direct(option.to_string())]]));
    }

    // Parse complex rule, referencing other rules
    else {
      let options: Vec<Vec<Value>> = parsed_line[1]
        .trim()
        .split("|")
        .map(
          |indices| indices.trim()
            .split(" ")
            .map(|index| Value::Indirect(index.parse::<usize>().expect("Failed parsing rule index!")))
            .collect()
        ).collect();
        rules.insert(parsed_line[0].parse::<usize>().expect("Failed parsing rule index!"), (0, options));
    }
  }

  // Return parsed input
  return (rules, msgs);
}

// TODO: ...
fn get_rule_lengths (rules: &mut HashMap<usize, (usize, Vec<Vec<Value>>)>, _verbose: bool) -> HashMap<usize, (usize, Vec<Vec<Value>>)> {
  // Initialize new_rules
  let mut new_rules: HashMap<usize, (usize, Vec<Vec<Value>>)> = rules.clone();
  // Detect rule lengths
  for rule_index in rules.keys() {
    // Get rule
    let rule = rules.get(&rule_index).expect("Failed fetching rule!");
    // Update rule size
    let length = get_rule_length(rule_index.clone(), &new_rules);
    new_rules.insert(rule_index.clone(), (length, rule.1.clone()));
  }
  // Return updated rules
  return new_rules;
}

/// TODO: ...
fn get_rule_length (rule_index: usize, rules: &HashMap<usize, (usize, Vec<Vec<Value>>)>) -> usize {
  // Get rule
  let rule = rules.get(&rule_index).expect("Failed fetching rule!");
  // Check if rule already has calculated length
  if rule.0 > 0 {
    return rule.0.clone();
  }
  // Get option lengths
  let mut min_option_length: Option<usize> = None;
  for option in &rule.1 {
    // Sum option values lengths
    let mut option_length: usize = 0;
    for value in option {
      option_length += match value {
        Value::Direct(_) => 1,
        Value::Indirect(index) => {
          if index.clone() == rule_index {
            0
          } else {
            get_rule_length(index.clone(), &rules)
          }
        }
      };
    }
    // Check if new min
    match min_option_length {
      Some(min) => {
        if option_length < min {
          min_option_length = Some(option_length.clone());
        }
      },
      None => {
        min_option_length = Some(option_length.clone());
      }
    }
  }
  // Return min option length
  return min_option_length.unwrap();
}

/// TODO: ...
fn simplify_rules (rules: &mut HashMap<usize, (usize, Vec<Vec<Value>>)>, _verbose: bool) -> HashMap<usize, (usize, Vec<Vec<Value>>)> {
  // Initialize new_rules
  let mut new_rules: HashMap<usize, (usize, Vec<Vec<Value>>)> = rules.clone();
  // Keep simplifying until no simplifications are found
  loop {
    // Simplify rule options
    let mut is_simplified = false;
    for rule_index in rules.keys() {
      // Get rule
      let rule = new_rules.get(&rule_index).expect("Failed fetching rule!").clone();
      // Update rule size
      let simplified_rule = (0..rule.1.len())
        .map(|i| {

          // Try converting to simplified option form
          let simplified = simplify_option(rule.1[i].clone(), &rules);
          is_simplified = is_simplified || simplified.0;

          // Check if all values in the option are direct values
          let mut all_direct_values = true;
          for value in &simplified.1.clone() {
            match value {
              Value::Indirect(_) => {
                all_direct_values = false;
                break;
              },
              _ => ()
            }
          }

          // If all direct values, concatenate
          if all_direct_values {
            let joined_direct_values = simplified.1
              .iter()
              .map(|value| match value {
                Value::Direct(value) => value.clone(),
                _ => panic!("Failed fetching direct value!")
              })
              .collect::<Vec<String>>()
              .join("");
            return vec![Value::Direct(joined_direct_values.clone())];
          } else {
            // Return smplified option
            return simplified.1.clone();
          }
        })
        .collect();
      new_rules.insert(rule_index.clone(), (rule.0, simplified_rule));
    }
    // If no simplification made, break out
    if !is_simplified {
      break;
    }
  }
  // Return updated rules
  return new_rules;
}

/// TODO: ...
fn simplify_option (option: Vec<Value>, rules: &HashMap<usize, (usize, Vec<Vec<Value>>)>) -> (bool, Vec<Value>) {
  // Simplify by replacing single-step indirections to direct values
  let mut is_simplified = false;
  let single_step_indirection_simplified_option: Vec<Value> = option.iter()
    .map(|value| match value {
      Value::Direct(value) => Value::Direct(value.clone()),
      Value::Indirect(rule_index) => {
        // Get rule
        let rule = rules.get(&rule_index).expect("Failed fetching rule!");
        
        // Check if value is referencing direct rule
        if rule.1.len() == 1 && rule.1[0].len() == 1 {
          return match &rule.1[0][0] {
            Value::Direct(value) => {
              is_simplified = true;
              Value::Direct(value.clone())
            },
            _ => {
              Value::Indirect(rule_index.clone())
            }
          }
        }

        // Return as is
        return Value::Indirect(rule_index.clone());
      }
    })
    .collect();

  // Return simplified option
  return (is_simplified, single_step_indirection_simplified_option);
}

/// TODO: ...
fn validate_msg<'msg> (msg: &'msg str, rule_index: usize, rules: &HashMap<usize, (usize, Vec<Vec<Value>>)>, verbose: bool, depth: usize) -> Vec<&'msg str> {
  // Get rule
  let rule = rules.get(&rule_index).expect("Failed fetching rule!");

  // Check if message is long enough to match
  if rule.0 > 0 && msg.len() < rule.0 {
    return vec![];
  }

  // Prompt
  let padding = vec!["| "; depth + 1].join("");
  if verbose {
    println!("  {} > Matching {}. rule ({:?}) against \"{}\" ... ", padding, rule_index, rule, msg);
  }

  // Validate message matching rule by validating all it's options
  let mut matching_options_msg_remaining: Vec<&'msg str> = vec![];
  for option_index in 0..rule.1.len() {
    let option = &rule.1[option_index];

    // Prompt
    if verbose {
      println!("  {}  > Matching {} option ({:?}) against \"{}\" ... ", padding, option_index, option, msg);
    }

    // Validate each value in the rule
    let mut matched_values: Vec<&Value> = vec![];
    let mut matching_values_msg_remaining: Vec<&'msg str> = vec![];
    for value_index in 0..option.len() {
      let value = &option[value_index];

      // Clear superfluous message remainders
      matching_values_msg_remaining.sort();
      matching_values_msg_remaining.dedup();
      matching_values_msg_remaining = matching_values_msg_remaining
        .iter()
        .map(|msg| msg.clone())
        .filter(|msg| msg.len() > 0)
        .collect();

      // Check if any message remaining
      if msg.len() == 0 && matching_values_msg_remaining.len() == 0 {
        // Stop comparison, failed matching direct rule
        break;
      }

      // Prompt
      if verbose {
        match value {
          Value::Direct(rule_char) =>
            println!("  {}   > Matching {} VALUE::DIRECT({}) against \"{:?}\" ... ", padding, value_index, rule_char, matching_values_msg_remaining),
          Value::Indirect(rule_index,) =>
            println!("  {}   > Matching {} VALUE::INDIRECT({}) against \"{:?}\" ... ", padding, value_index, rule_index, matching_values_msg_remaining)
        };
      }

      // Match value against all possible remaining messages
      let remaining_msgs = if matching_values_msg_remaining.len() == 0 { vec![msg] } else { matching_values_msg_remaining.clone() };
      matching_values_msg_remaining = vec![];
      for remaining_msg in remaining_msgs {

        // Match value
        match value {
          Value::Direct(rule_str) => {

            // Check message remainder length
            if remaining_msg.len() < rule_str.len() {
              break;
            }

            // Match direct value
            let msg_str = remaining_msg[0..rule_str.len()].to_string();

            // If match
            if rule_str.clone() == msg_str {
              // Register as matched
              matched_values.push(value);
              // Reduce remaining message left for comparison
              matching_values_msg_remaining.push(&remaining_msg[rule_str.len()..]);
            }

            // Stop comparison, failed matching direct rule
            else {
              break;
            }

          },
          Value::Indirect(rule_index) => {

            // Match nested rule (indirect value)
            let new_remaining_msgs = validate_msg(remaining_msg, rule_index.clone(), &rules, verbose, depth + 1);
            
            // Check if any matches
            if new_remaining_msgs.len() > 0 {
              // Register as matched
              matched_values.push(value);
              // Set remaining message left for comparison
              for new_remaining_msg in new_remaining_msgs {
                matching_values_msg_remaining.push(new_remaining_msg);
              }
            }

            // Stop comparison, failed matching indirect rule
            else {
              break;
            }

          }
        }

      }
    }

    // Check if option successfully matched
    if option.len() == matched_values.len() {

      // Store remaining message for matched option
      matching_values_msg_remaining.sort();
      matching_values_msg_remaining.dedup();
      for msg in matching_values_msg_remaining {
        matching_options_msg_remaining.push(msg);
      }

      // Prompt success
      if verbose {
        println!("  {}  > Matching {} option ({:?}) against \"{}\": SUCCESS! (Remaining messages {:?})", padding, option_index, option, msg, matching_options_msg_remaining);
      }

    } else {
      
      // Prompt failure
      if verbose {
        println!("  {}  > Matching {} option ({:?}) against \"{}\": FAILED!", padding, option_index, option, msg);
      }

    }

  }

  // Deduplicate remaining messages
  matching_options_msg_remaining.sort();
  matching_options_msg_remaining.dedup();
  
  // Check for matching options
  if matching_options_msg_remaining.len() > 0 {

    // Prompt success
    if verbose {
      println!("  {} > Matching {}. rule ({:?}) against \"{}\": SUCCESS! (Remaining message: {:?})", padding, rule_index, rule, msg, matching_options_msg_remaining);
    }

  } else {

    // Prompt failure
    if verbose {
      println!("  {} > Matching {}. rule ({:?}) against \"{}\": FAILED! ", padding, rule_index, rule, msg);
    }

  }

  // Return remaining message slices
  return matching_options_msg_remaining;
}

// TODO: ...
#[derive(Debug, Clone)]
enum Value {
  Direct(String),
  Indirect(usize)
}
