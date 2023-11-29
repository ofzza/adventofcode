// NANO-FACTORY RECIPES

// Import dependencies
const flags       = require('../../../lib').flags,
      logProgress = require('../../../lib').logProgress;

// Parses recipe syntax
module.exports.parse = function parse (syntax) {
  const ins = syntax.trim().split('=>')[0].trim().split(','),
        out = syntax.trim().split('=>')[1].trim();
  return {
    clone: function () {
      return JSON.parse(JSON.stringify({
        ins: this.ins,
        out: this.out
      }));
    },
    get syntax () {
      return `${Object.values(this.ins).map(i => `${i.quantity} ${i.element}`).join(', ')} => ${this.out.quantity} ${this.out.element}`;
    },
    ins: ins.reduce((ins, a) => {
      const element   = (a.trim().split(' ').length > 1 ? a.trim().split(' ')[1] : a),
            quantity  = parseInt(a.trim().split(' ').length > 1 ? a.trim().split(' ')[0] : 1);
      ins[element] = { element, quantity };
      return ins;
    }, {}),
    out: {
      element:  (out.trim().split(' ').length > 1 ? out.trim().split(' ')[1] : out),
      quantity: parseInt(out.trim().split(' ').length > 1 ? out.trim().split(' ')[0] : 1)
    }
  }
};

// Gets raw ingredients for a given output
module.exports.getRawIngredients = function compose (element, recipes) {
  // Check if recipe has branching
  if (checkBranching(recipes)) {
    throw new Error('Multiple ways of composing the output found!');
  }
  // Precalculate optimizations
  const ingredients     = findIngredients(recipes),
        haltConditionFn = (recipe, leftovers) => {
          return !Object.keys(recipe.ins).find(element => (ingredients.raw.indexOf(element) === -1));
        };
  // Process
  const recipe = breakdown({ element, recipes, haltConditionFn, ingredients });
  return recipe.ins;
}

// Find quantity that leaves no leftovers
module.exports.getMaxQuantity = function compose (element, stock, recipes) {
  // Check if recipe has branching
  if (checkBranching(recipes)) {
    throw new Error('Multiple ways of composing the output found!');
  }
  // Precalculate optimizations
  const ingredients     = findIngredients(recipes),
        haltConditionFn = (recipe, leftovers) => {
          return !Object.keys(recipe.ins).find(element => (ingredients.raw.indexOf(element) === -1));
        };
  // Initialize
  let originalStock = {...stock},
      count = 0,
      leftovers = {};

  // Process
  while (true) {

    // Break down recipe with current leftovers
    const recipe = breakdown({ element, recipes, leftovers, haltConditionFn, ingredients });

    // Update stock
    for (let element of Object.keys(recipe.ins)) {
      if (stock[element] && (stock[element] >= recipe.ins[element].quantity)) {
        stock[element] -= recipe.ins[element].quantity;
      } else {
        return count;
      }
    }

    // Update output
    count++;

    // Output percentage done if logging progress
    if (flags.PROGRESS && (count % 10000 === 0)) {
      for (let raw of Object.keys(stock)) {
        logProgress(`... ${ element.padStart(8, ' ') } : ${ count.toString().padStart(10, ' ') }`
                  + ` => ${ raw.padStart(8, ' ') } : ${ stock[raw].toString().padStart(15, ' ') } / ${ originalStock[raw].toString().padStart(15, ' ') }`
                  + ` => ${ Math.ceil(10000 * (stock[raw] / originalStock[raw])) / 100 }%`);
      }
    }

    // If possible (all leftovers spent), accelerate based on found period
    if (!Object.keys(leftovers).find((key) => !!leftovers[key])) {
      // Find safe number of steps to skip ahead
      let countSafeSkipSteps = null;
      for (let element of Object.keys(stock)) {
        const elementCountSafeSkipSteps = Math.floor(originalStock[element] / (originalStock[element] - stock[element]));
        if ((countSafeSkipSteps === null) || (elementCountSafeSkipSteps < countSafeSkipSteps)) {
          countSafeSkipSteps = elementCountSafeSkipSteps;
        }
      }
      // Skip ahead
      if (countSafeSkipSteps) {
        for (let element of Object.keys(stock)) {
          stock[element] = originalStock[element] - (countSafeSkipSteps * (originalStock[element] - stock[element]));
        }
        count = countSafeSkipSteps * count;
      }
      // Skip other accelerations
      continue;
    }
    
  }
}

// Checks if recipe has multiple ways of being completed 
function checkBranching (recipes) {
  const counts = {};
  for (let recipe of recipes) {
    if (counts[recipe.out.element] !== undefined) {
      // Found branching
      return true;
    }
  }
  // No branching found
  return false;
};

// Find all ingredients used in provided recipes
function findIngredients (recipes) {
  const ingredients = recipes.reduce((ingredients, recipe) => {
    Object.values(recipe.ins).reduce((ingredients, i) => {
      ingredients[i.element] = (ingredients[i.element] === undefined ? 'RAW' : ingredients[i.element]);
      return ingredients;
    }, ingredients);
    ingredients[recipe.out.element] = 'COMPOSITE';
    return ingredients;
  }, {});
  return {
    all:        ingredients,
    raw:        Object.keys(ingredients).filter((element) => (ingredients[element] === 'RAW')),
    composite:  Object.keys(ingredients).filter((element) => (ingredients[element] === 'COMPOSITE'))
  };
}

// Deconstruct recipe to basic required ingredients
function breakdown ({ element, recipes, leftovers = {}, haltConditionFn, ingredients = null } = {}) {
  // If not provided, find ingredients
  if (ingredients === null) {
    ingredients = findIngredients(recipes);
  }
  // Organize recipes by output element
  const howto = recipes.reduce((howto, r) => {
    howto[r.out.element] = r;
    return howto;
  }, {});
  // Deconstruct the recipe
  const recipe    = howto[element].clone();
  while (!haltConditionFn(recipe, leftovers)) {
    // Find deconstruct-able input element
    const element = Object.keys(recipe.ins).find(element => (ingredients.raw.indexOf(element) === -1));
    // Calculate needed quantities
    const inElement     = recipe.ins[element].element,
          inQuantity    = recipe.ins[element].quantity - (leftovers[inElement] || 0),
          inRecipe      = howto[inElement],
          usedBatches   = Math.ceil((inQuantity > 0 ? inQuantity : 0) / inRecipe.out.quantity),
          usedQuantity  = usedBatches * inRecipe.out.quantity;
    // Update leftovers
    leftovers[inElement] = usedQuantity - inQuantity;
    // Update deconstructed recipe
    delete recipe.ins[element];
    for (let i of Object.values(inRecipe.ins)) {
      const quantity = (recipe.ins[i.element] !== undefined ? recipe.ins[i.element].quantity : 0) + i.quantity * usedBatches;
      if (quantity > 0) {
        recipe.ins[i.element] = {
          element: i.element,
          quantity
        };
      }
    }
  }
  // Return deconstructed recipe
  return recipe;
}
