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
module.exports.test = (puzzle, args, { test = true, expected, extract = r => r, render } = {}) => {
  // Run test
  const result = puzzle(...args);
  // Check if result equals expected result
  const error = ((expected !== undefined) && (JSON.stringify(extract(result)) !== JSON.stringify(expected)));
  // Output test
  if (!colors) {
    // Output monochrome
    console.log(`  ... ${ test ? 'EXAMPLE: ' : 'MY INPUT:' }`
                + ` (${ stringify(args) }) => ${ stringify(result) }`
                + ` => ${ stringify(extract(result)) } ${ !error ? (expected !== undefined ? '[OK]' : result) : '[ERROR]' }`);
  } else {
    // Output color
    console.log(`  ... ${ test ? 'EXAMPLE: ' : 'MY INPUT:'.yellow }`
                + ` (${ stringify(args) }) => ${ stringify(result) }`.gray
                + ` => ${ stringify(extract(result)) } ${ !error ? (expected !== undefined ? '[OK]'.green : '') : '[ERROR]'.red }`);
  }
  // Render output
  if (render) { console.log(`\n${ [...render(result)].join('').split('\n').map(r => `      ${ r }`).join('\n').yellow }`); }
  // If error, throw error
  if (error) {
    console.log('');
    console.log('');
    throw new Error(`ERROR: result of "${ stringify(extract(result)) }" does not match expected result of "${ expected }"`);
  }
};

// Converts to string and shortens a value
function stringify (value) {
  return (function shortenArgs (value) {
    if (typeof value === 'object' && value.length !== undefined) {
      return `[${
        value.slice(0, 5).map((value) => {
          return shortenArgs(value);
        }).join(',') + (value.length > 5 ? ',...' : '')
      }]`;
    } else {
      return `${ value.toString().slice(0, 20) }${ value.toString().length > 20 ? '...' : '' }`;
    }
  })(value)
    .replace(/\\"/g, '')
    .replace(/"/g, '');
}
