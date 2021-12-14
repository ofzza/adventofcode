//! 2021 day 12 puzzle
//! 
//! https://adventofcode.com/2021/day/12
// -----------------------------------------------------------------------------

// Include dependencies
use std::collections::hash_map::*;
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2021::lib::path_finding::*;

/// Parses input data
fn parse(data: &String) -> Vec<(&str, &str)> {
  Input::parse(data.trim(), "\n", |line| {
    let parsed = Input::parse(line.trim(), "-", |n| { n.trim() });
    (parsed[0], parsed[1])
  })
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 12,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize path finding
      let finding = PathFinding::new(data);
      let mut cache: HashMap<&str, bool> = HashMap::new();
      let mut path: Vec<&str> = Vec::new();
      let mut paths: Vec<Vec<&str>> = Vec::new();
      finding.find_all_paths("start", "end", &mut cache , &mut path, &mut paths, |key: &&str, path: &mut Vec<&str>, _cache: &mut HashMap<&str, bool>| {
        // Check if key is upper-case
        if key != &key.to_lowercase() { return true; }
        // Check if previously visited
        let previously_visited = path.iter().find(|k| k == &key) != None;
        if !previously_visited { return true; } 
        // Default to false
        false
      });

      // Calculate and return result
      String::from(format!("{:?}", paths.len()))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 12,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize path finding
      let finding = PathFinding::new(data);
      let mut cache: HashMap<&str, bool> = HashMap::new();
      let mut path: Vec<&str> = Vec::new();
      let mut paths: Vec<Vec<&str>> = Vec::new();
      finding.find_all_paths("start", "end", &mut cache , &mut path, &mut paths, |key: &&str, path: &mut Vec<&str>, _cache: &mut HashMap<&str, bool>| {        
        // Check if key is upper-case
        if key != &key.to_lowercase() { return true; }
        // Check if previously visited
        let previously_visited = path.iter().find(|k| k == &key) != None;
        if !previously_visited { return true; } 
        // Check if "start"
        if key == &"start" { return false; }
        // Check if no doubled lower-cased keys already in path
        let mut lc = path.iter().filter(|k| k == &&&k.to_lowercase()).collect::<Vec<&&str>>();
        lc.sort();
        let mut lc_dedup = lc.clone();
        lc_dedup.dedup();
        let has_doubled_lc = lc_dedup.len() != lc.len();
        if !has_doubled_lc { return true; }
        // Default to false
        false
      });

      // Calculate and return result
      String::from(format!("{:?}", paths.len()))
    }

  );

  // Return registry ownership
  registry
}
