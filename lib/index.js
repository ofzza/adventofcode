// HELPER FUNCTIONS FOR RUNNING PUZZLES

// Import dependencies
const colors = require('colors');

// Runs multiple tests for a puzzle
module.exports.puzzle = (year, day, puzzle, puzzleFn, tests) => {
  // Prompt starting tests
  if (!colors) {
    // Output monochrome
    console.log(`> Year ${ year }, Day ${ day }, Puzzle ${ puzzle }`);
  } else {
    // Output color
    console.log(`${ '>'.yellow } Year ${ year.toString().yellow }, Day ${ day.toString().yellow }, Puzzle ${ puzzle.toString().yellow }`);
  }
  // Run all tests
  for (let i=0; i<Math.ceil(tests.length / 2); i++) {
    module.exports.test(puzzleFn, tests[2 * i], tests[2 * i + 1]);
  }
  // Done testing
  console.log('');
};

// Run a puzzle test
module.exports.test = (puzzle, args, { test = true, expected, extract = r => r } = {}) => {
  // Run test
  const result = puzzle(...args);
  // Check if result equals expected result
  const error = ((expected !== undefined) && (JSON.stringify(extract(result)) !== JSON.stringify(expected)));
  // Stringify and crop arguments
  const argsAsString = (function shortenArgs (args) {
    return args.slice(0, 5).map(arg => {
      return JSON.stringify(typeof arg === 'object' && arg.length ? `[${ shortenArgs(arg) }]` : (typeof arg === 'string' ? `'${ arg }'` : arg));
    }).join(',') + (args.length > 5 ? ',...' : '');
  })(JSON.parse(JSON.stringify(args)))
    .replace(/\\"/g, '')
    .replace(/"/g, '');
  const argsString = args.slice(0, 5).map(arg => JSON.stringify(arg)).join(',');  
  // Output test
  if (!colors) {
    // Output monochrome
    console.log(`  ... ${ test ? 'EXAMPLE: ' : 'MY INPUT:' }`
                + ` (${ argsAsString }) => ${ result }`
                + ` => ${ extract(result) } ${ !error ? (expected !== undefined ? '[OK]' : result) : '[ERROR]' }`);
  } else {
    // Output color
    console.log(`  ... ${ test ? 'EXAMPLE: ' : 'MY INPUT:'.yellow }`
                + ` (${ argsAsString }) => ${ result.toString() }`.gray
                + ` => ${ extract(result) } ${ !error ? (expected !== undefined ? '[OK]'.green : '') : '[ERROR]'.red }`);
  }
  // If error, throw error
  if (error) {
    console.log('');
    console.log('');
    throw new Error(`ERROR: result of "${ extract(result) }" does not match expected result of "${ expected }"`);
  }
};
