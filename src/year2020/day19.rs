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
        String::from("aaaabbb")
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
      // let rules = optimize_rules(rules, vec![0, 8, 11], verbose);
      // if verbose {
      //   rules_to_string("  Pre-compiled rules:", &rules);
      // }    
      
      // Count matching messages
      let mut count = 0;
      for i in 0..msgs.len() {
        // Validate message
        match validate_msg_by_rule(&msgs[i][..], 0, &rules, vec![], 0, verbose) {
          Some(_) => { count += 1; },
          None => {}
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
      // let rules = optimize_rules(rules, vec![0, 8, 11], verbose);
      // if verbose {
      //   rules_to_string("  Pre-compiled rules:", &rules);
      // }    

      // Count matching messages
      let mut count = 0;
      for i in 0..msgs.len() {
        // Validate message
        match validate_msg_by_rule(&msgs[i][..], 0, &rules, vec![], 0, verbose) {
          Some(_) => { count += 1; },
          None => {}
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
      rules.insert(parsed_line[0].parse::<usize>().expect("Failed parsing rule index!"), Rule::Options(vec![RuleOption::Values(vec![RuleValue::Direct(option.to_string())])]));
    }

    // Parse complex rule, referencing other rules
    else {
      let options: Vec<RuleOption> = parsed_line[1]
        .trim()
        .split("|")
        .map(
          |indices| {
            let values = indices.trim()
              .split(" ")
              .map(|index| RuleValue::Indirect(index.parse::<usize>().expect("Failed parsing rule index!")))
              .collect();
            return RuleOption::Values(values);
          }
        ).collect();
      rules.insert(parsed_line[0].parse::<usize>().expect("Failed parsing rule index!"), Rule::Options(options));
    }
  }

  // Return parsed input
  return (rules, msgs);
}

/// Validates if message matches a rule
/// 
/// # Argument
/// * `msg`             - Message to validate
/// * `rule_id`         - Id of the rule to validate by
/// * `rules`           - All rules as parsed
/// * `match_remainder` - (For internal use in recursive calls) Additional values to match after the rule 
/// * `depth`           - (For internal use in recursive calls) Recursive depth
/// * `verbose`         - Outputs executing output of the puzzle to the console
fn validate_msg_by_rule<'l> (msg: &'l str, rule_id: usize, rules: &HashMap<usize, Rule>, match_remainder: Vec<RuleValue>, depth: usize, verbose: bool) -> Option<()> {
  // Get requested rule
  let rule = rules.get(&rule_id).unwrap();

  // Prompt
  if verbose {
    println!("  > [{}] Validating \"{}\" against rule: {:?} / remainder: {:?}", depth, msg, rule, match_remainder);
  }
  
  // Validate by requested rule
  match rule {
    Rule::Options(options) => {

      // Check each option
      for option in options {
        match option {
          RuleOption::Values(values) => {

            // Prompt
            if verbose {
              println!("    > [{}] Validating \"{}\" against option: {:?} / remainder: {:?}", depth, msg, option, match_remainder);
            }

            let mut validation_values: Vec<RuleValue> = vec![];
            validation_values.extend(values.clone());
            validation_values.extend(match_remainder.clone());
            let validation_result = validate_msg_by_values(msg, validation_values, rules, depth + 1, verbose);
            match validation_result {
              // If validation successful, pass along success
              Some(_) => { return Some(()); },
              // If not successful, continue checking options
              None => {}
            }

          }
        }
      }

    }
  }

  // Rule not matched
  return None;
}

/// Validates if message matches a sequence of values
/// 
/// # Argument
/// * `msg`             - Message to validate
/// * `values`          - Values to validate the message by
/// * `rules`           - All rules as parsed
/// * `depth`           - (For internal use in recursive calls) Recursive depth
/// * `verbose`         - Outputs executing output of the puzzle to the console
fn validate_msg_by_values<'l> (msg: &'l str, values: Vec<RuleValue>, rules: &HashMap<usize, Rule>, depth: usize, verbose: bool) -> Option<()> {
  // Take currently validating message segment
  let mut msg = &msg[..];

  // Validate values
  for i in 0..values.len() {

    // Prompt
    if verbose {
      println!("  > [{}] Validating \"{}\" against values: {:?}", depth, msg, values[i..].to_vec());
    }

    // If no message left to validate, return failed validation
    if msg.len() == 0 {
      return None;
    }

    // Match value against remaining message segment
    let value = values[i].clone();
    match value {
      RuleValue::Direct(text) => {

        // If match, update remaining message segment
        if msg[0..text.len()] == text {
          msg = &msg[text.len()..];
        }
        
        // If no match, return failed validation
        else {
          return None;
        }

      },
      RuleValue::Indirect(rule_id) => {

        // Validate an indirect match
        match validate_msg_by_rule(msg, rule_id, rules, values[(i + 1)..].to_vec(), depth + 1, verbose) {
          // If validation successful, pass along success
          Some(_) => { return Some(()); },
          // If no match, return failed validation
          None => { return None; }
        }

      }
    }
  }

  // Return success if message fully matched
  if msg.len() == 0 {
    return Some(());
  }

  // Return validation failed 'cos remainder not allowed
  else {
    return None;
  }
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
  Options(Vec<RuleOption>)
}

/// Types of rule options
#[derive(Debug, Clone)]
enum RuleOption {
  Values(Vec<RuleValue>)
}

/// Types of rule values
#[derive(Debug, Clone)]
enum RuleValue {
  Direct(String),
  Indirect(usize)
}
