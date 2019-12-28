// HELPER FUNCTIONS FOR RUNNING PUZZLES

// Import dependencies
const colors        = require('colors'),
      readlineSync  = require('readline-sync');

// Holds runtime flags
module.exports.flags = {
  // Startup arguments
  argv: null,  
  // Logging progress flag
  PROGRESS: false,
  // Interactive mode flag
  INTERACTIVE: false
};

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
module.exports.test = (puzzle, args, { example = true, expected, transform = r => r, render, warning } = {}) => {
  // Check if test has a warning
  if (warning) {
    if (!colors) {
      console.log(`      ${ example ? 'EXAMPLE ' : 'MY INPUT' } WARNING: ${ warning }`);
    } else {
      console.log(`      ${ example ? 'EXAMPLE ' : 'MY INPUT' } WARNING: ${ warning }`.red);
    }
    if (!module.exports.flags.argv.year || !module.exports.flags.argv.day || !module.exports.flags.argv.puzzle) {
      if (!colors) {
        console.log(`      (Please run particular year/day/puzzle individually - see "--help" for details)`);
      } else {
        console.log(`      (Please run particular year/day/puzzle individually - see "--help" for details)`.red);
      }
      return;
    }
  }
  // Run test
  const startTime = Date.now(),
        result = puzzle(...args),
        endTime = Date.now();
  // Check if result equals expected result
  const error = ((expected !== undefined) && (JSON.stringify(transform(result)) !== JSON.stringify(expected)));
  // Output test
  if (!colors) {
    // Output monochrome
    console.log(`  ... ${ example ? 'EXAMPLE: ' : 'MY INPUT:' }`
                + ` (${ stringify(args) }) => ${ stringify(result) }`
                + ` => ${ stringify(transform(result)) } ${ !error ? (expected !== undefined ? '[OK]' : result) : '[ERROR]' }`
                + ` (in ${Math.floor((endTime - startTime) / Math.pow(10, 3))} secs)`);
  } else {
    // Output color
    console.log(`  ... ${ example ? 'EXAMPLE: ' : 'MY INPUT:'.yellow }`
                + ` (${ stringify(args) }) => ${ stringify(result) }`.gray
                + ` => ${ stringify(transform(result)) } ${ !error ? (expected !== undefined ? '[OK]'.brightGreen : '') : '[ERROR]'.red }`
                + ` (in ${Math.floor((endTime - startTime) / 10) / 100} secs)`.gray);
  }
  // Render output
  if (render) { console.log(`\n${ [...render(result)].join('').split('\n').map(r => `      ${ r }`).join('\n').yellow }`); }
  // If error, throw error
  if (error) {
    console.log('');
    console.log('');
    throw new Error(`ERROR: result of "${ stringify(transform(result)) }" does not match expected result of "${ expected }"`);
  }
};

// Log progress
module.exports.logProgress = (output = '') => {
  if (this.flags.PROGRESS) {
    console.log(colors ? output.brightGreen : output);
  }
};

// Read interactive key
module.exports.readKey = (msg = '') => {
  if (this.flags.INTERACTIVE && readlineSync) {
    return readlineSync.keyIn((colors ? msg.magenta : msg), { hideEchoBack: true });
  }
};

// Read interactive test
module.exports.readText = (msg = '') => {
  if (this.flags.INTERACTIVE && readlineSync) {
    return readlineSync.question((colors ? msg.magenta : msg));
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
    } else if (typeof value === 'object') {
      const strval = JSON.stringify(value).replace(/\\"/g, '').replace(/"/g, '');
      return `${ strval.substr(0, 32) }${ strval.length > 32 ? '...' : '' }`;
    } else {
      const strval = (value !== undefined ? value : 'undefined').toString().replace(/\\"/g, '').replace(/"/g, '');
      return `${ strval.substr(0, 20) }${ strval.length > 20 ? '...' : '' }`;
    }
  })(value)
    .replace(/\\"/g, '')
    .replace(/"/g, '');
}
