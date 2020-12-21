//! 2020/21 puzzle
//! 
//! https://adventofcode.com/2020/day/21
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
        String::from("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)"),
        String::from("trh fvjkl sbzzf mxmxvkd (contains dairy)"),
        String::from("sqjhc fvjkl (contains soy)"),
        String::from("sqjhc mxmxvkd sbzzf (contains fish)")
      ]);
      stats.update(
        Puzzle::new(2020, 21, 1, "test", input, implementation1, |r| (r, Some(5)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<String>(load_input("./src/year2020/data/day21input.txt"), "\n");
      stats.update(
        Puzzle::new(2020, 21, 1, "solution", input, implementation1, |r| (r, Some(2324)))
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
        String::from("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)"),
        String::from("trh fvjkl sbzzf mxmxvkd (contains dairy)"),
        String::from("sqjhc fvjkl (contains soy)"),
        String::from("sqjhc mxmxvkd sbzzf (contains fish)")
      ]);
      stats.update(
        Puzzle::new(2020, 21, 2, "test", input, implementation2, |r| (r, Some(String::from("mxmxvkd,sqjhc,fvjkl"))))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<String>(load_input("./src/year2020/data/day21input.txt"), "\n");

      stats.update(
        Puzzle::new(2020, 21, 2, "solution", input, implementation2, |r| (r, Some(String::from("bxjvzk,hqgqj,sp,spl,hsksz,qzzzf,fmpgn,tpnnkc"))))
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
      let items: Vec<(Vec<String>, Vec<String>)> = parse_input(input.clone());
      // Count ingredients per allergens
      let ingredients_per_allergens = get_ingredients_per_allergens(items.clone());
      // Map ingredients to allergens
      let ingredient_per_allergens = map_ingredients_to_allergens(ingredients_per_allergens, verbose);
      let allergenic_ingreedients: Vec<String> = ingredient_per_allergens.values().map(|value| value.0.clone()).collect();

      // Count usages of non allergenic ingredients
      let mut count = 0;
      for item in items {
        for ingredient in item.0 {
          if !allergenic_ingreedients.contains(&ingredient) {
            count += 1;
          }
        }
      }

      // Return count
      return Ok(count);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<String, String, String>, verbose: bool) -> Result<String, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(input) => {
      // Parse input
      let items: Vec<(Vec<String>, Vec<String>)> = parse_input(input.clone());
      // Count ingredients per allergens
      let ingredients_per_allergens = get_ingredients_per_allergens(items.clone());
      // Map ingredients to allergens
      let ingredient_per_allergens = map_ingredients_to_allergens(ingredients_per_allergens, verbose);

      // Compose a sorted allergenic ingredients list
      let mut allergen_list: Vec<String> = ingredient_per_allergens.keys()
        .map(|allergen| allergen.clone())
        .collect();
      allergen_list.sort_by(|a, b| a.partial_cmp(&b).unwrap());
      let ingredients_list: Vec<String> = allergen_list.iter()
        .map(|allergen| ingredient_per_allergens.get(allergen).unwrap().0.clone())
        .collect();

      // Return list
      Ok(ingredients_list.join(","))
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

/// Parses input into ingredients/allergens
/// 
/// # Arguments
/// * `lines` - Input lines to parse
fn parse_input (lines: Vec<String>) -> Vec<(Vec<String>, Vec<String>)> {
  // Ready parsed
  let mut result: Vec<(Vec<String>, Vec<String>)> = vec![];
  // Parse all lines
  for line in lines {
    let line_clean = line.replace("(", "|").replace(")", "").replace("contains", "");
    let parsed_line: Vec<&str>   = line_clean.split("|").collect();
    let ingredients: Vec<String> = parsed_line[0].trim().split(" ").map(|s| s.trim().to_string()).collect();
    let allergens: Vec<String>   = parsed_line[1].trim().split(",").map(|s| s.trim().to_string()).collect();
    result.push((ingredients, allergens));
  }
  // Return parsed
  return result;
}

/// Organizes ingredients per allergens
/// and returns a hashmap of ingredients per allergen and the number of mappings to each allergen
/// 
/// # Arguments
/// * `items` - Parsed items
fn get_ingredients_per_allergens (items: Vec<(Vec<String>, Vec<String>)>) -> HashMap<String, HashMap<String, usize>> {
  // Initialize ingredients per allergen hashmap
  let mut hashmap: HashMap<String, HashMap<String, usize>> = HashMap::new();
  // Process all items
  for item in items {
    for allergen in &item.1 {
      // Get allergen's collection of ingredients from the hashmap
      let mut ingredients_per_allergen: HashMap<String, usize> = if hashmap.contains_key(allergen) {
        hashmap.remove(allergen).unwrap()
      } else {
        HashMap::new()
      };
      // Update ingredients' counts for this allergen
      for ingredient in &item.0 {
        let updated_count = match ingredients_per_allergen.get(ingredient) {
          Some(count) => count.clone() + 1,
          None => 1
        };
        ingredients_per_allergen.insert(ingredient.clone(), updated_count);
      }
      // Update allergen's collection of ingredients from the hashmap
      hashmap.insert(allergen.clone(), ingredients_per_allergen);
    }
  }
  // Return ingredients per allergen
  return hashmap;
}

/// Filters out ingredients until there is only a single ingredient mapped to each allergen,
/// and returns a hashmap of ingredients per allergen and the number of mappings to the allergen
/// 
/// # Arguments
/// * `ingredients_per_allergens` - Organized ingredients per allergens
/// * `verbose`                   - Outputs executing output of the puzzle to the console
fn map_ingredients_to_allergens (ingredients_per_allergens: HashMap<String, HashMap<String, usize>>, verbose: bool) -> HashMap<String, (String, usize)> {
  // Initialize unique matches
  let mut ingredient_per_allergens: HashMap<String, (String, usize)> = HashMap::new();
  // Find a unique ingredient matching every allergen
  let mut matched_ingredients: Vec<String> = vec![];
  loop {

    // Search allergens for unique ingredient
    for allergen in ingredients_per_allergens.keys() {
      // Check if allergen already matched
      if !ingredient_per_allergens.contains_key(allergen) {
        // Get allergen's ingredient counts for only yet unmatched ingredients
        let ingredients_per_allergen = ingredients_per_allergens.get(allergen).unwrap();
        let mut ingredient_counts: Vec<(String, usize)> = ingredients_per_allergen
          .keys()
          .filter(|key| !matched_ingredients.contains(key))
          .map(|key| (key.clone(), ingredients_per_allergen.get(key).unwrap().clone()))
          .collect();
        ingredient_counts.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        // Check if unique top ingredient
        if ingredient_counts.len() == 1 || ingredient_counts[ingredient_counts.len() - 1].1 > ingredient_counts[ingredient_counts.len() - 2].1 {
          // Set as matched
          let ingredient = ingredient_counts.last().unwrap();
          matched_ingredients.push(ingredient.0.clone());
          ingredient_per_allergens.insert(allergen.clone(), ingredient.clone());
          // Prompt
          if verbose {
            println!("Matched allergen [{}] to [{}], across {} declarations", allergen, ingredient.0, ingredient.1);
          }
        }
      }
    }

    // If all matched, break
    if ingredients_per_allergens.len() == matched_ingredients.len() {
      break;
    }
  }
  // Return matched
  return ingredient_per_allergens;
}
