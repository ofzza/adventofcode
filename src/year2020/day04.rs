//! 2020/04 puzzle
//! 
//! https://adventofcode.com/2020/day/4
// -----------------------------------------------------------------------------

// Include dependencies
use std::collections::HashMap;
use regex::Regex;
use crate::lib::inputs::*;
use crate::lib::puzzle::*;

/// Registers puzzles for the day
pub fn run (index: u32, key: &str, verbose: bool) -> PuzzleExecutionStatitics {

  // Initialize stats
  let mut stats = PuzzleExecutionStatitics{
    ..Default::default()
  };

  // Run puzzle
  if (index == 0) || (index == 1) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector1D(vec![
        String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm"),
        String::from("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929"),
        String::from("hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm"),
        String::from("hcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in"),
      ]);
      stats.update(
        Puzzle::new(2020, 4, 1, "test", input, implementation1, |r| (r, Some(2)))
          .run(verbose)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<String>(load_input("./src/year2020/data/day04input.txt"), "\n\n");
      stats.update(
        Puzzle::new(2020, 4, 1, "solution", input, implementation1, |r| (r, Some(237)))
          .run(verbose)
      );
    }
  }

  // Run puzzle
  if (index == 0) || (index == 2) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector1D(vec![
        String::from("eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"),
        String::from("iyr:2019\nhcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:1946"),
        String::from("hcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"),
        String::from("hgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007"),
      ]);
      stats.update(
        Puzzle::new(2020, 4, 2, "test", input, implementation2, |r| (r, Some(0)))
          .run(verbose)
      );
    }
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector1D(vec![
        String::from("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f"),
        String::from("eyr:2029 ecl:blu cid:129 byr:1989\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"),
        String::from("hcl:#888785\nhgt:164cm byr:2001 iyr:2015 cid:88\npid:545766238 ecl:hzl\neyr:2022"),
        String::from("iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"),
      ]);
      stats.update(
        Puzzle::new(2020, 4, 2, "test", input, implementation2, |r| (r, Some(4)))
          .run(verbose)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<String>(load_input("./src/year2020/data/day04input.txt"), "\n\n");
      stats.update(
        Puzzle::new(2020, 4, 2, "solution", input, implementation2, |r| (r, Some(172)))
          .run(verbose)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<String, usize, usize>, _verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(lines) => {
      // Parse passports
      let passports = parse_passports(lines);
      // Count valid passports
      let mut count = 0;
      for passport in passports {
        if simple_validate_passport(passport) {
          count += 1;
        }
      }
      return Ok(count);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<String, usize, usize>, _verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(lines) => {
      // Parse passports
      let passports = parse_passports(lines);
      // Count valid passports
      let color_regex = Regex::new(r"^#[0-9a-f]{6}$").expect("Failed composing RegExp!");
      let passport_id_regex = Regex::new(r"^[0-9]{9}$").expect("Failed composing RegExp!");
      let mut count = 0;
      for passport in passports {
        if advanced_validate_passport(passport, &color_regex, &passport_id_regex) {
          count += 1;
        }
      }
      return Ok(count);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

/// Validates a password by checking for existance of mandatory fields
/// 
/// # Arguments
/// * `passport` - Hashmap with passport fields and values
fn simple_validate_passport (passport: HashMap<String, String>) -> bool {
  // Check values
  return passport.contains_key("byr")
      && passport.contains_key("iyr")
      && passport.contains_key("eyr")
      && passport.contains_key("hgt")
      && passport.contains_key("hcl")
      && passport.contains_key("ecl")
      && passport.contains_key("pid");
}

/// Validates a password by checking for existance of mandatory fields and the validity of their values
/// 
/// # Arguments
/// * `passport` - Hashmap with passport fields and values
/// * `color_regex` - Regular expression for checking if value is a valid color
/// * `passport_id_regex` - Regular expression for checking if value is a valid passpord ID number
fn advanced_validate_passport (passport: HashMap<String, String>, color_regex: &Regex, passport_id_regex: &Regex) -> bool {
  // Check byr
  if !passport.contains_key("byr") { return false; }
  let byr: u32 = passport.get_key_value("byr").expect("Failed finding 'byr'!").1.parse::<u32>().expect("Failed parsing 'byr'!");
  if byr < 1920 || byr > 2002 { return false; }
  // Check iyr
  if !passport.contains_key("iyr") { return false; }
  let iyr: u32 = passport.get_key_value("iyr").expect("Failed finding 'iyr'!").1.parse::<u32>().expect("Failed parsing 'iyr'!");
  if iyr < 2010 || iyr > 2020 { return false; }
  // Check eyr
  if !passport.contains_key("eyr") { return false; }
  let eyr: u32 = passport.get_key_value("eyr").expect("Failed finding 'eyr'!").1.parse::<u32>().expect("Failed parsing 'eyr'!");
  if eyr < 2020 || eyr > 2030 { return false; }
  // Check hgt
  if !passport.contains_key("hgt") { return false; }
  let hgt: &str = passport.get_key_value("hgt").expect("Failed finding 'hgt'!").1;
  let hgt_units: String = String::from(&hgt[(hgt.len() - 2)..]);
  let hgt_value: u32 = String::from(&hgt[..(hgt.len() - 2)]).parse::<u32>().expect("Failed parsing 'hgt'!");
  if hgt_units != "cm" && hgt_units != "in" { return false; }
  if hgt_units == "cm" && (hgt_value < 150 || hgt_value > 193) { return false; }
  if hgt_units == "in" && (hgt_value < 59 || hgt_value > 76) { return false; }
  // Check hcl
  if !passport.contains_key("hcl") { return false; }
  let hcl: &str = passport.get_key_value("hcl").expect("Failed finding 'hcl'!").1;
  if !color_regex.is_match(hcl) { return false; }
  // Check ecl
  if !passport.contains_key("ecl") { return false; }
  let ecl: &str = passport.get_key_value("ecl").expect("Failed finding 'ecl'!").1;
  if ecl != "amb" && ecl !=  "blu" && ecl !=  "brn" && ecl !=  "gry" && ecl !=  "grn" && ecl !=  "hzl" && ecl !=  "oth" { return false; }
  // Check pid
  if !passport.contains_key("pid") { return false; }
  let pid: &str = passport.get_key_value("pid").expect("Failed finding 'pid'!").1;
  if !passport_id_regex.is_match(pid) { return false; }

  // Return default
  return true;
}

/// Parses password raw text data into a vector of hashmaps
/// 
/// # Arguments
/// * `lines` - Raw text password data
fn parse_passports (lines: &Vec<String>) -> Vec<HashMap<String, String>> {
  // Initialize passports
  let mut passports: Vec<HashMap<String, String>> = Vec::new();
  
  // Process lines
  for line in lines {    
    // Initialize hashmap
    let mut hashmap: HashMap<String, String> = HashMap::new();    
    // Parse line
    let line_cleared = line.replace("\n", " ");
    let fields: Vec<&str> = line_cleared.trim().split(' ').collect();
    // Process line fields
    for field in fields {
      // Parse field
      let parsed: Vec<&str> = field.trim().split(':').collect();
      // Add to Hashmap
      &hashmap.insert(String::from(parsed[0]), String::from(parsed[1]));
    }
    // Add hashmap to result
    passports.push(hashmap);
  }
  
  // Return parsed passports
  return passports;
}
