// DAY 25
// https://adventofcode.com/2019/day/25

// Import dependencies
const puzzle  = require('../../../lib').puzzle,
      robot   = require('../../lib/robots/investigator-droid');

// Set global inputs
const prog = require('fs').readFileSync(require('path').join(__dirname, './input.txt')).toString().trim().split(',').map(a => parseInt(a));

// 1st puzzle of the day
function puzzle01 (...prog) {
  return robot.control(
    prog, 
    [
      'explore',                  // Get lay of the land and pick up safe items.
      'goto Security Checkpoint', // Go to Security checkpoint ... 
      'cheat pressure floor',     // ... and try to cheat the pressure sensors.
      'exit'
    ],
    {
      dontExploreRooms: ['Security Checkpoint'],
      dontPickUpItems:  ['giant electromagnet', 'infinite loop', 'photons', 'molten lava', 'escape pod']
    }
  );
}
module.exports.puzzle01 = () => {
  puzzle('2019', '25', '01', puzzle01, [
    prog, { expected: 67635328, example: false }
  ]);
};

// 2nd puzzle of the day
function puzzle02 (...prog) {
  return 'NEED ALL STARS!';
}
module.exports.puzzle02 = () => {
  puzzle('2019', '25', '02', puzzle02, [
    prog, { expected: 'NEED ALL STARS!', example: false }
  ]);
};
