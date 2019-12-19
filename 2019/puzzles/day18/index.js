// DAY 18
// https://adventofcode.com/2019/day/18

// Import dependencies
const puzzle  = require('../../../lib').puzzle,
      image   = require('../../lib/image'),
      solve   = require('../../lib/labyrinth').solve;

// Input reader
function read (name) {
  return require('fs').readFileSync(require('path').join(__dirname, `./${ name }.txt`)).toString().trim().split('\n').reduce((points, l, y) => {
    return l.split('').reduce((points, c, x) => {
      points[`${ x }x${ y }`] = { coords: { x, y }, color: ({ '.': 2, '#': 1 }[c] !== undefined ? { '.': 2, '#': 1 }[c] : c) };
      return points;
    }, points);
  }, {});
}

// Set global inputs
const input = read('input');

// Manually modify input to remove wide passages
input['39x40'].color = 1;
input['41x40'].color = 1;
function inputModificationDistanceCorrection (a, b) {
  // Check quadrants for A and B
  const quadA = (a.coords.x < 40 ? (a.coords.y < 40 ? 'A' : 'D') : (a.coords.y < 40 ? 'C' : 'D')),
        quadB = (b.coords.x < 40 ? (b.coords.y < 40 ? 'A' : 'D') : (b.coords.y < 40 ? 'C' : 'D'));
  if ((quadA === 'A' && quadB === 'D') || (quadA === 'D' && quadB === 'A') || (quadA === 'B' && quadB === 'C') || (quadA === 'C' && quadB === 'B')) {
    return -2;
  } else {
    return 0;
  }
};

// 1st puzzle of the day
function puzzle01 (...args) {
  return solve(args[0], {
    distanceCorrection: args[1]
  });
}
module.exports.puzzle01 = () => {
  puzzle('2019', '18', '01', puzzle01, [
    // [read('example01')],  { expected: 86 },
    // [read('example02')],  { expected: 132 },
    // [read('example03')],  { expected: 136 },
    // [read('example04')],  { expected: 81 },
    [input, inputModificationDistanceCorrection], { expected: undefined, example: false } // 6282 (dtsnojqxregpzklmicwyuvabhf)
  ]);
};

// 2nd puzzle of the day
function puzzle02 (...args) {
  return 0;
}
module.exports.puzzle02 = () => {
  puzzle('2019', '18', '02', puzzle02, [
    [input],  { expected: undefined, example: false }
  ]);
};
