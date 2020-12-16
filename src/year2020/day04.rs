//! 2020/04 puzzle
//! 
//! https://adventofcode.com/2020/day/4
// -----------------------------------------------------------------------------

// Include dependencies
use regex::Regex;
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
        String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm"),
        String::from("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929"),
        String::from("hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm"),
        String::from("hcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in"),
      ]);
      stats.update(
        Puzzle::new(2020, 4, 1, "test", input, implementation1, |r| (r, Some(2)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<String>(load_input("./src/year2020/data/day04input.txt"), "\n\n");
      stats.update(
        Puzzle::new(2020, 4, 1, "solution", input, implementation1, |r| (r, Some(237)))
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
        String::from("eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"),
        String::from("iyr:2019\nhcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:1946"),
        String::from("hcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"),
        String::from("hgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007"),
      ]);
      stats.update(
        Puzzle::new(2020, 4, 2, "test", input, implementation2, |r| (r, Some(0)))
          .run(verbose, obfuscate)
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
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<String>(load_input("./src/year2020/data/day04input.txt"), "\n\n");
      stats.update(
        Puzzle::new(2020, 4, 2, "solution", input, implementation2, |r| (r, Some(172)))
          .run(verbose, obfuscate)
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
fn simple_validate_passport (passport: Passport) -> bool {
  // Check values
  return match passport.byr { Some(_) => true, None => false }
      && match passport.iyr { Some(_) => true, None => false }
      && match passport.eyr { Some(_) => true, None => false }
      && match passport.hgt { Some(_) => true, None => false }
      && match passport.hcl { Some(_) => true, None => false }
      && match passport.ecl { Some(_) => true, None => false }
      && match passport.pid { Some(_) => true, None => false }
}

/// Validates a password by checking for existance of mandatory fields and the validity of their values
/// 
/// # Arguments
/// * `passport` - Hashmap with passport fields and values
/// * `color_regex` - Regular expression for checking if value is a valid color
/// * `passport_id_regex` - Regular expression for checking if value is a valid passpord ID number
fn advanced_validate_passport (passport: Passport, color_regex: &Regex, passport_id_regex: &Regex) -> bool {
  // Check byr
  match passport.byr {
    Some(value) => {
      let byr: u32 = value.parse::<u32>().expect("Failed parsing 'byr'!");
      if byr < 1920 || byr > 2002 { return false; }
    },
    None => { return false; }
  }
  // Check iyr
  match passport.iyr {
    Some(value) => {
      let iyr: u32 = value.parse::<u32>().expect("Failed parsing 'iyr'!");
      if iyr < 2010 || iyr > 2020 { return false; }
    },
    None => { return false; }
  }
  // Check eyr
  match passport.eyr {
    Some(value) => {
      let eyr: u32 = value.parse::<u32>().expect("Failed parsing 'eyr'!");
      if eyr < 2020 || eyr > 2030 { return false; }
    },
    None => { return false; }
  }
  // Check hgt
  match passport.hgt {
    Some(value) => {
      let hgt_units: String = String::from(&value[(value.len() - 2)..]);
      let hgt_value: u32 = String::from(&value[..(value.len() - 2)]).parse::<u32>().expect("Failed parsing 'hgt'!");
      if hgt_units != "cm" && hgt_units != "in" { return false; }
      if hgt_units == "cm" && (hgt_value < 150 || hgt_value > 193) { return false; }
      if hgt_units == "in" && (hgt_value < 59 || hgt_value > 76) { return false; }
    },
    None => { return false; }
  }
  // Check hcl
  match passport.hcl {
    Some(value) => {
      if !color_regex.is_match(&value) { return false; }
    },
    None => { return false; }
  }
  // Check ecl
  match passport.ecl {
    Some(value) => {
      if value != "amb" && value !=  "blu" && value !=  "brn" && value !=  "gry" && value !=  "grn" && value !=  "hzl" && value !=  "oth" { return false; }
    },
    None => { return false; }
  }
  // Check pid
  match passport.pid {
    Some(value) => {
      if !passport_id_regex.is_match(&value) { return false; }
    },
    None => { return false; }
  }
  // Return default
  return true;
}

/// Parses password raw text data into a vector of hashmaps
/// 
/// # Arguments
/// * `lines` - Raw text password data
fn parse_passports (lines: &Vec<String>) -> Vec<Passport> {
  // Initialize passports
  let mut passports: Vec<Passport> = Vec::new();
  
  // Process lines
  for line in lines {    
    // Initialize empty passport
    let mut passport: Passport = Passport {
      byr: None,
      iyr: None,
      eyr: None,
      hgt: None,
      hcl: None,
      ecl: None,
      pid: None,
      cid: None
    };    
    // Parse line
    let line_cleared = line.replace("\n", " ");
    let fields: Vec<&str> = line_cleared.trim().split(' ').collect();
    // Process line fields
    for field in fields {
      // Parse field
      let parsed: Vec<&str> = field.trim().split(':').collect();
      // Add to passport
      match parsed[0] {
        "byr" => passport.byr = Some(String::from(parsed[1])),
        "iyr" => passport.iyr = Some(String::from(parsed[1])),
        "eyr" => passport.eyr = Some(String::from(parsed[1])),
        "hgt" => passport.hgt = Some(String::from(parsed[1])),
        "hcl" => passport.hcl = Some(String::from(parsed[1])),
        "ecl" => passport.ecl = Some(String::from(parsed[1])),
        "pid" => passport.pid = Some(String::from(parsed[1])),
        "cid" => passport.cid = Some(String::from(parsed[1])),
        _ => {}
      }
    }
    // Add passport to result
    passports.push(passport);
  }
  
  // Return parsed passports
  return passports;
}

/// Passport structure holds passport information
struct Passport {  
  byr: Option<String>, // (Birth Year)
  iyr: Option<String>, // (Issue Year)
  eyr: Option<String>, // (Expiration Year)
  hgt: Option<String>, // (Height)
  hcl: Option<String>, // (Hair Color)
  ecl: Option<String>, // (Eye Color)
  pid: Option<String>, // (Passport ID)
  cid: Option<String>  // (Country ID)
}
