// DAY 01
// https://adventofcode.com/2019/day/1

// Import dependencies
const puzzle = require('../../../lib').puzzle;

// Read inputs
const input = require('fs').readFileSync(require('path').join(__dirname, './input.txt')).toString().trim().split('\n').map(a => a.trim());
                
// 1st puzzle of the day
function puzzle01 (...masses) {
  return masses.reduce((sum, mass) => {
    mass = Math.floor(mass / 3) - 2;
    return sum + (mass > 0 ? mass : 0);
  }, 0);
}
module.exports.puzzle01 = () => {
  puzzle('2019', '01', '01', puzzle01, [
    [12, 14, 1969, 100756], { expected: 34241 },
    input,                  { expected: 3262358, example: false }
  ]);
};

// 2nd puzzle of the day
function puzzle02 (...masses) {
  return masses.reduce((sum, mass) => {
    while (mass = puzzle01(mass)) { sum += mass; };
    return sum;
  }, 0);
}
module.exports.puzzle02 = () => {
  // Run puzzle
  puzzle('2019', '01', '02', puzzle02, [
    [14, 1969, 100756], { expected: 51314 },
    input,              { expected: 4890696, example: false }
  ]);
};
