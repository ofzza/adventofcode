// DAY 15
// https://adventofcode.com/2019/day/15

// Import dependencies
const puzzle  = require('../../../lib').puzzle,
      robot   = require('../../lib/robots/repair-droid'),
      image   = require('../../lib/image');

// Set global inputs
const prog = require('fs').readFileSync(require('path').join(__dirname, './input.txt')).toString().trim().split(',').map(a => parseInt(a));

// Color palette to draw the map with
const palette = {
  0: '█',
  1: '░',
  2: 'O',
  3: 'X',
  4: '.',
  9: ' '
};

// Ready world
let world = {};

// 1st puzzle of the day
function puzzle01 (...prog) {
  return robot.explore(prog, world, { palette });
}
module.exports.puzzle01 = () => {
  puzzle('2019', '15', '01', puzzle01, [
    prog, { expected: 226, transform: r => r.distance, render: image.renderFieldFactory({ palette, transform: r => r.img }), example: false }
  ]);
};

// 2nd puzzle of the day
function puzzle02 (...prog) {
  return robot.fill(prog, world, { palette });
}
module.exports.puzzle02 = () => {
  puzzle('2019', '15', '02', puzzle02, [
    prog, { expected: 342, transform: r => r.time, render: image.renderFieldFactory({ palette, transform: r => r.img }), example: false }
  ]);
};
