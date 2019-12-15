// DAY 12
// https://adventofcode.com/2019/day/12

// Import dependencies
const puzzle  = require('../../../lib').puzzle,
      newton  = require('../../lib/newton');

// Set global inputs
const input = require('fs').readFileSync(require('path').join(__dirname, './input.txt')).toString().trim().split('\n').map((b) => {
  return b.replace('<', '').replace('>', '').split(',').map((c) => {
    return parseFloat(c.trim().split('=')[1].trim());
  });
});

// 1st puzzle of the day
function puzzle01 (...args) {
  let simulation = newton.nbody(args[0], { coords: [0, 1, 2] }),
      state;
  for (let i = 0; i < args[1]; i++) { state = simulation.next(); }
  return state.value.reduce((E, a) => (E + a.E), 0);
}
module.exports.puzzle01 = () => {
  puzzle('2019', '12', '01', puzzle01, [
    [[[-1,0,2],[2,-10,-7],[4,-8,8],[3,5,-1]], 10],    { expected: 179 },
    [[[-8,-10,0],[5,5,10],[2,-7,3],[9,-8,-3]], 100],  { expected: 1940 },
    [input, 1000],                                    { expected: 9441, example: false }
  ]);
};

// 2nd puzzle of the day
function puzzle02 (...args) {
  return newton.nbodyFindPeriod(args, { coords: [0, 1, 2] });
}
module.exports.puzzle02 = () => {
  puzzle('2019', '12', '02', puzzle02, [
    [[-1,0,2],[2,-10,-7],[4,-8,8],[3,5,-1]],  { expected: 2772 },
    [[-8,-10,0],[5,5,10],[2,-7,3],[9,-8,-3]], { expected: 4686774924 },
    input,                                    { expected: 503560201099704, example: false }
  ]);
};
