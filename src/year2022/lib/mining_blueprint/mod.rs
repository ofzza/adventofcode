//! Mining Blueprint module
//! 
//! Mining Blueprint module
// -----------------------------------------------------------------------------

// Include dependencies
use std::collections::hash_map::HashMap;

/// Mining Recipe Ingredient structure
pub struct MiningRecipeIngredient<'a> {
  // Ingredient index
  index: usize,
  // Ingredient name
  _name: &'a str,
  // Ingredient quantity
  quantity: usize
}

/// Mining Recipe structure
pub struct MiningRecipe<'a> {
  // Recipe target index
  index: usize,
  // Recipe target name
  _name: &'a str,
  // Recipe ingredients
  ingredients: HashMap<usize, MiningRecipeIngredient<'a>>
}
/// Mining Blueprint structure
pub struct MiningBlueprint<'a> {
  // Blueprint index
  pub index: usize,
  // Holds recepies for this blueprint
  recipes: HashMap<usize, MiningRecipe<'a>>,
  // Enummerated ingrediant names
  index_by_ingrediant: HashMap<&'a str, usize>,
  _ingrediant_by_index: Vec<&'a str>
}

/// Mining Blueprint implementation
impl<'a> MiningBlueprint<'a> {

  /// Constructor
  pub fn new<'b> (index: usize, data: &Vec<(&'b str, Vec<(usize, &'b str)>)>) -> MiningBlueprint<'b> {
    // Translate types into indexes
    let mut index_by_ingrediant: HashMap<&str, usize> = HashMap::with_capacity(data.len());
    let mut ingrediant_by_index: Vec<&str> = vec![""; data.len()];
    // Build a hashmap of receipies
    let recipies: Vec<MiningRecipe> = data.iter()
      .map(|recipe| {
        // Build a hashmap of ingredients
        let ingredients: Vec<MiningRecipeIngredient> = recipe.1.iter()
          .map(|ingredient| MiningRecipeIngredient {
            // Ingredient index
            index: MiningBlueprint::index_from_name(&mut index_by_ingrediant, &mut ingrediant_by_index, ingredient.1),
            // Ingredient index
            _name: ingredient.1,
            // Ingredient quantity
            quantity: ingredient.0
          })
          .collect::<Vec<MiningRecipeIngredient>>();
        let mut ingredients_hashmap: HashMap<usize, MiningRecipeIngredient> = HashMap::new();
        for ingredient in ingredients {
          ingredients_hashmap.insert(ingredient.index, ingredient);
        }
        // Build a recipe instance
        return MiningRecipe {
          // Recipe target index
          index: MiningBlueprint::index_from_name(&mut index_by_ingrediant, &mut ingrediant_by_index, recipe.0),
          // Recipe target name
          _name: recipe.0,
          // Recipe ingredients
          ingredients: ingredients_hashmap
        };
      })
      .collect::<Vec<MiningRecipe>>();
    let mut recipes_hashmap: HashMap<usize, MiningRecipe> = HashMap::with_capacity(data.len());
    for recipe in recipies {
      recipes_hashmap.insert(recipe.index, recipe);
    }
    // Return instance
    MiningBlueprint {
      // Blueprint index
      index,
      // Holds recepies for thsi blueprint
      recipes: recipes_hashmap,
      // Enumerated types
      index_by_ingrediant,
      _ingrediant_by_index: ingrediant_by_index
    }
  }

  // Assigns a unique index to an ingredient
  fn index_from_name<'b> (index_by_ingrediant: &mut HashMap<&'b str, usize>, ingrediant_by_index: &mut Vec<&'b str>, name: &'b str) -> usize {
    if index_by_ingrediant.contains_key(name) {
       let index = index_by_ingrediant.get(name).unwrap().clone();
       ingrediant_by_index[index] = name;
       index
    } else {
      let index = index_by_ingrediant.len();
      index_by_ingrediant.insert(name, index);
      ingrediant_by_index[index] = name;
      index
    }
  }

  /// Evaluates maximum possible production within alotted time
  /// 
  /// # Arguments
  /// * target: Name of the target recipe to produce max quanntities of
  /// * time: Time alloted for production
  /// 
  /// # Returns
  /// Maximum number of produced resource possible in alloted time
  pub fn evaluate (&self, target: &str, time: usize) -> usize {
    // Initialize state with a single ore robot
    let mut state: Vec<(usize, usize, usize)> = vec![(0, 0, 0); self.index_by_ingrediant.len()];
    state[self.index_by_ingrediant.get("ore").unwrap().clone()] = (1, 0, 0);

    // Initialize quick early exit cache
    let mut naive_quick_early_exit_cache: HashMap<(usize, usize), usize> = HashMap::new();
    // Evaluate production over time
    self._evaluate(self.index_by_ingrediant.get(target).unwrap().clone(), time, state, 0, &mut naive_quick_early_exit_cache)
  }

  /// Internal, recursive method for evaluating maximum possible production within alotted time
  fn _evaluate (&self, target: usize, time: usize, state: Vec<(usize, usize, usize)>, max: usize, naive_quick_early_exit_cache: &mut HashMap<(usize, usize), usize>) -> usize {
    
    // Initialize tracked maximum production of target resource with no extra robots being constructed
    let mut max: usize = max.clone();
    // Initialize a mutable copy of the state to forward time for
    let mut state = state.clone();

    // Run testcases if any time left and if early exit is over current max
    if time > 0 && !self._naive_quick_test_for_early_exit(target, time, max, &state) {

      // Unless time still for building combinatorics, update state after a time-tick of production
      state = MiningBlueprint::forward_time(state, 1);

      // Try building robots per each of the recipes
      for recipe in self.recipes.values().collect::<Vec<&MiningRecipe>>().iter().rev() {

        // Get a separate, mutable state for this recipe
        let mut state = state.clone();

        // Check if ingredient for the recipe found
        let mut all_ingredients_found = true;
        for ingredient in recipe.ingredients.values() {
          // If inadequate ingredients, skip recipe
          if state[ingredient.index].2 < ingredient.quantity {
            all_ingredients_found = false;
            break;
          }
          // If adequate ingredient, consume ingredient
          else {
            state[ingredient.index].2 -= ingredient.quantity;
          }
        }

        // Check if ingredients found ...
        if all_ingredients_found {
          // If ingedients found, build a robot
          state[recipe.index].1 += 1;

          // ~Try building more with new robot built~
          // Build the robot and forward time while building the robot
          let max_candidate = self._evaluate(target, time - 1, state.clone(), max, naive_quick_early_exit_cache);
          if max_candidate > max {
            max = max_candidate;
            // If new max, prompt
            // println!("Blueprint #{} ::: Time left: {} | Max: {}", self.index, time, max);
          }
        }
      }

      // Try forwarding time, with no builds
      let max_candidate = self._evaluate(target, time - 1, state.clone(), max, naive_quick_early_exit_cache);
      if max_candidate > max {
        max = max_candidate;
        // If new max, prompt
        // println!("Blueprint #{} ::: Time left: {} | Max: {}", self.index, time, max);
      }

    }

    // Return found maximum producion of targeted resource
    if max < state[target].2 { state[target].2 } else { max }
  }

  /// Calculates naive, quick, upper production bound for an ingredient within the time alloted
  fn _naive_quick_test_for_early_exit (&self, target: usize, time: usize, found_max: usize, state: &Vec<(usize, usize, usize)>) -> bool {
    // TODO: Consider checking recipes for dependencies when calculating max number of robots possible
    // TODO: Consider caching results
    
    // Check max target production
    let max_target_production =
      // Already collected
        state[target].2
      // Colected by already built robots
      + (state[target].0 * time)
      // Colected by robots being built
      + (state[target].1 * (time - 1))
      // Collected by max possible robots to be built if building 1 per turn
      + (0.5 * ((time - 1) * time) as f64).ceil() as usize;

    // Early exit if impossible to do useful work in given time
    if max_target_production <= found_max {
      // Early exit
      true
    } else {
      // Keep crunching
      false
    }
  }

  /// Internal method forwards time, and mutates state accordingly
  /// 
  /// # Arguments
  /// * state: State to forwad time for
  /// * time: How much time to forward
  /// 
  /// # Returns
  /// State after time has passed
  fn forward_time (mut state: Vec<(usize, usize, usize)>, time: usize) -> Vec<(usize, usize, usize)> {
    // Forward time
    for _ in 0..time {
      for i in 0..state.len () {
        // Produce with existing robots
        state[i].2 += state[i].0;
        // Complete building robots under construction
        state[i].0 += state[i].1;
        state[i].1 = 0;
      }
    }
    // Return time-forwarded state
    state
  }
}
