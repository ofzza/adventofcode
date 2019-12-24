// DAY 24
// https://adventofcode.com/2019/day/24

// Import dependencies
const flags       = require('../../../lib').flags
      logProgress = require('../../../lib').logProgress
      puzzle      = require('../../../lib').puzzle,
      conway      = require('../../lib/conway');

// Set global inputs
function read (name) {
  return require('fs').readFileSync(require('path').join(__dirname, `./${ name }.txt`)).toString().trim().split('\n').map((l) => {
    return l.split('').map(c => {
      return { '.': 0, '#': 1 }[c];
    });
  })
};

// 1st puzzle of the day
function puzzle01 (...world) {
  let play = conway.play(world),
      history = {},
      result;
  // Play until repeats
  while (!(result = play.next()).done) {
    // Check history
    if (!history[result.value.hash]) {
      history[result.value.hash] = true;
    } else {
      break;
    }
  }
  // Calculate biodiversity
  return conway.calculateBiodiversity(result.value.world);
}
module.exports.puzzle01 = () => {
  puzzle('2019', '24', '01', puzzle01, [
    read('example101'), { expected: 2129920 },
    read('input'),      { expected: 28903899, example: false }
  ]);
};

// 2nd puzzle of the day
function puzzle02 (world, n) {
  let play = conway.play(world, { recursive: true });
  // Play requested number of turns
  for (let i = 0; i < n; i++) {
    let result = play.next();;
    if (result.done) { return null; }
    world = result.value.world;
  }
  // Count bugs
  return Object.values(world).reduce((score, w2d) => {
    return score + w2d.reduce((score, line) => {
      return score + line.reduce((score, bug) => score + bug, 0)
    }, 0);
  }, 0);
}
module.exports.puzzle02 = () => {
  puzzle('2019', '24', '02', puzzle02, [
    [read('example101'), 10], { expected: 99, example: false },
    [read('input'), 200],     { expected: 1896, example: false }
  ]);
};
