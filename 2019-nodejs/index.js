// ADVENT of CODE 2019
// https://adventofcode.com/2019

// Import dependencies
const fs    = require('fs'),
      path  = require('path'),
      yargs = require('yargs'),
      flags = require('./lib').flags;

// Check if yargs installed
if (!yargs) {

  // Run all available tests
  try {
    runTests();
  } catch (err) {
    console.error(err.message);
  }

} else {

  // Define yargs arguments
  const argv = yargs
    .usage('Usage: node $0 -y [num] -d [num] -p [num] -r [boolean] -i [boolean]')
    .example('node ./ --year 2019 --day 1 --puzzle 1')
    .alias('y', 'year')
    .describe('y', 'Run only puzzles from a single year')
    .alias('d', 'day')
    .describe('d', 'Run only puzzles from a single day')
    .alias('p', 'puzzle')
    .describe('p', 'Run only single puzzle')
    .alias('r', 'progress')
    .describe('r', 'Log progress when available')
    .alias('i', 'interactive')
    .describe('i', 'Run in interactive mode (take user input) when available')
    .demandOption([])
    .help('h')
    .alias('h', 'help')
    .argv;

  // Use yargs to determine which tests to run
  runTests({
    year:         argv.year,
    day:          argv.day,
    puzzle:       argv.puzzle,
    progress:     argv.progress,
    interactive:  argv.interactive,
  });

}

// Search and run all tests
function runTests ({ year, day, puzzle, progress, interactive } = {}) {

  // Set startup arguments
  flags.argv = { year, day, puzzle, progress, interactive };
  // Set logging progress flag
  flags.PROGRESS = progress;
  // Set interactive mode flag
  flags.INTERACTIVE = interactive;

  // Find all year directories
  for (let yearDir of fs.readdirSync('./')) {
    if (fs.lstatSync(yearDir).isDirectory()) {
      const parsedYear = parseInt(yearDir);
      if (parsedYear > 0 && (!year || (parsedYear === year))) {

        // Find all day directories
        for (let dayDir of fs.readdirSync(path.join('./', yearDir, 'puzzles'))) {
          if (fs.lstatSync(path.join('./', yearDir, 'puzzles', dayDir, 'index.js')).isFile()) {
            const parsedDay = parseInt(dayDir.replace(/day/g, ''));
            if (parsedDay > 0 && (!day || (parsedDay === day))) {

              // Load daily puzzle
              const dayPuzzles = require(`./${ path.join('./', yearDir, 'puzzles', dayDir) }`);
              for (let puzzleName in dayPuzzles) {
                const parsedPuzzle = parseInt(puzzleName.replace(/puzzle/g, ''));
                if (parsedPuzzle > 0 && (!puzzle || (parsedPuzzle === puzzle))) {

                  // Run puzzle
                  dayPuzzles[puzzleName]();

                }
              }

            }
          }
        }

      }
    }
  }

}
