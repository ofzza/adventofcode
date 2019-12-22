// DAY 18
// https://adventofcode.com/2019/day/18

// Import dependencies
const puzzle          = require('../../../lib').puzzle,
      collectAllKeys  = require('../../lib/labyrinth/neptune').collectAllKeys;

// Input reader
function read (name) {
  return require('fs').readFileSync(require('path').join(__dirname, `./${ name }.txt`)).toString().trim().split('\n').reduce((points, l, y) => {
    return l.split('').reduce((points, c, x) => {
      points[`${ x }x${ y }`] = {
        coords: { x, y },
        color: ({ '.': 2, '#': 1 }[c] !== undefined ? { '.': 2, '#': 1 }[c] : c)
      };
      return points;
    }, points);
  }, {});
}

// Set global inputs
const input   = read('input'),
      input01 = JSON.parse(JSON.stringify(input)),
      input02 = JSON.parse(JSON.stringify(input));

// Manually modify input for 1st puzzle to remove wide passages
input01['39x40'].color = input01['41x40'].color = 1;
function inputModificationDistanceCorrection (a, b) {
  // Check quadrants for A and B
  const quadA = (a.coords.x < 40 ? (a.coords.y < 40 ? 'A' : 'D') : (a.coords.y < 40 ? 'B' : 'C')),
        quadB = (b.coords.x < 40 ? (b.coords.y < 40 ? 'A' : 'D') : (b.coords.y < 40 ? 'B' : 'C'));
  if ((quadA === 'A' && quadB === 'D') || (quadA === 'D' && quadB === 'A') || (quadA === 'B' && quadB === 'C') || (quadA === 'C' && quadB === 'B')) {
    return -2;
  } else {
    return 0;
  }
};

// 1st puzzle of the day
function puzzle01 (...args) {
  return collectAllKeys(args[0], {
    questionableShortcuts:  args[1],
    distanceCorrection:     args[2]
  });
}
module.exports.puzzle01 = () => {
  puzzle('2019', '18', '01', puzzle01, [
    [read('example101'), true],                           { expected: 86 },
    [read('example102'), true],                           { expected: 132 },
    [read('example103'), true],                           { expected: 136 },
    [read('example104'), true],                           { expected: 81 },
    [input01, true, inputModificationDistanceCorrection], { expected: 6286, example: false,  // dtsnojqxregpzklmicwyuvabhf
                                                            warning: 'Slow! Needs to be reimplemented using the labyrinth lib ...' }
  ]);
};

// Manually modify input for 2nd puzzle to split the labyrinth
input02['40x40'].color = input02['39x40'].color = input02['41x40'].color = input02['40x39'].color = input02['40x41'].color = 1;
input02['39x39'].color = input02['39x41'].color = input02['41x39'].color = input02['41x41'].color = '@';

// 2nd puzzle of the day
function puzzle02 (...args) {
  return collectAllKeys(args[0], {
    questionableShortcuts: args[1]
  });
}
module.exports.puzzle02 = () => {
  puzzle('2019', '18', '02', puzzle02, [
    [read('example201'), true],  { expected: 8 },
    [read('example202'), true],  { expected: 24 },
    [read('example203'), true],  { expected: 32 },
    [read('example204'), true],  { expected: 72 },
    [input02, false],            { expected: 2140, example: false,  // (tdgsnoqxplzjmhicwyuerkvfab)
                                   warning: 'Slow! Needs to be reimplemented using the labyrinth lib ...' }
  ]);
};
