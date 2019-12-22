// DAY 22
// https://adventofcode.com/2019/day/22

// Import dependencies
const puzzle  = require('../../../lib').puzzle;

// Set global inputs
const input = require('fs').readFileSync(require('path').join(__dirname, './input.txt')).toString().trim().split(',').map((a) => parseInt(a));

// 1st puzzle of the day
function puzzle01 (...args) {
  return 0;
}
module.exports.puzzle01 = () => {
  puzzle('2019', '22', '01', puzzle01, [
    [],     { expected: undefined },
    input,  { expected: undefined, example: false }
  ]);
};

// 2nd puzzle of the day
function puzzle02 (...args) {
  return 0;
}
module.exports.puzzle02 = () => {
  puzzle('2019', '22', '02', puzzle02, [
    [],     { expected: undefined },
    input,  { expected: undefined, example: false }
  ]);
};
