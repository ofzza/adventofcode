//! 2022 day 19 puzzle
//! 
//! https://adventofcode.com/2022/day/19
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2022::lib::mining_blueprint::*;

/// Parses input data
fn parse<'a>(data: &'a String) -> Vec<Vec<(&str, Vec<(usize, &str)>)>> {
  Input::parse(data.as_str().trim(), "\n", |data| {
    data.split(':').collect::<Vec<&str>>()[1].split('.')
      .map(|recipe| recipe.trim())
      .filter(|recipe| recipe.len() > 0)
      .map(|recipe| {
        let parsed = recipe.split(" robot costs ").collect::<Vec<&str>>();
        let robot_type = parsed[0].split(' ').collect::<Vec<&str>>()[1];
        let robot_ingredients = parsed[1].split(" and ")
          .map(|ingredient| ingredient.trim())
          .map(|ingredient| {
            let parsed = ingredient.split(' ').collect::<Vec<&str>>();
            let ingredient_quantity = parsed[0].trim().parse::<usize>().unwrap();
            let ingredient_type = parsed[1].trim();
            (ingredient_quantity, ingredient_type)
          })
          .collect::<Vec<(usize, &str)>>();
        (robot_type, robot_ingredients)
      })
      .collect::<Vec<(&str, Vec<(usize, &str)>)>>()
  })
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 19,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize all the blueprints
      let blueprints = data.iter()
        .enumerate()
        .map(|(index, data)| MiningBlueprint::new(index + 1, data))
        .collect::<Vec<MiningBlueprint>>();

      // Evaluate all the blueprints and find max
      let max = blueprints.iter()
        .map(|blueprint| {
          let max = blueprint.evaluate("geode", 24);
          // println!("Evaluated blueprint #{}: {}", blueprint.index, max);
          blueprint.index * max
        })
        .sum::<usize>();

      // Return result
      String::from(format!("{:?}", max))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2022,
      day: 19,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Initialize all the blueprints
      let blueprints = data[0..(if data.len() >= 3 { 3 } else { data.len() })].iter()
        .enumerate()
        .map(|(index, data)| MiningBlueprint::new(index + 1, data))
        .collect::<Vec<MiningBlueprint>>();

      // Evaluate all the blueprints and find max
      let product = blueprints.iter()
        .map(|blueprint| {
          let max = blueprint.evaluate("geode", 32);
          // println!("Evaluated blueprint #{}: {}", blueprint.index, max);
          max
        })
        .reduce(|accum, item| accum * item)
        .unwrap();

      // Return result
      String::from(format!("{:?}", product))
    }

  );

  // Return registry ownership
  registry
}
