// DAY 22
// https://adventofcode.com/2019/day/22

// Import dependencies
const flags       = require('../../../lib').flags,
      logProgress = require('../../../lib').logProgress,
      puzzle      = require('../../../lib').puzzle,
      cards       = require('../../lib/cards');

// Read input from file
function read (name) { return require('fs').readFileSync(require('path').join(__dirname, `./${ name }.txt`)).toString().trim().split('\n').map(l => l.trim()); }

// 1st puzzle of the day
function puzzle01 (instructions, n) {
  const deck  = cards.processShuffleInstructions(instructions, n),
        order = [...Array(n)].map((undef, i) => deck.shuffleIteratively(i));
  return order.reduce((deck, n, x) => {
          deck[n] = x;
          return deck;
        }, []);
}
module.exports.puzzle01 = () => {
  puzzle('2019', '22', '01', puzzle01, [
    [read('example101'), 10],  { expected: [0, 3, 6, 9, 2, 5, 8, 1, 4, 7] },
    [read('example102'), 10],  { expected: [3, 0, 7, 4, 1, 8, 5, 2, 9, 6] },
    [read('example103'), 10],  { expected: [6, 3, 0, 7, 4, 1, 8, 5, 2, 9] },
    [read('example104'), 10],  { expected: [9, 2, 5, 8, 1, 4, 7, 0, 3, 6] },
    [read('input'),   10007],  { expected: 3377, example: false, transform: r => r.indexOf(2019) }
  ]);
};

// 2nd puzzle of the day
function puzzle02 (instructions, n, reps, order, backwards) {
  // Calculate ordering by analytical means
  const deck  = cards.processShuffleInstructions(instructions, n);
  return order.map((i) => deck[!backwards ? 'shuffleAnalytically' : 'unshuffleAnalytically'](i, reps));
}
module.exports.puzzle02 = () => {
  // Run examples
  puzzle('2019', '22', '02', puzzle02, [
    [read('input'),       10007,            1,                [2019],                         false], { expected: [3377] },
    [read('input'),       10007,            1,                [3377],                         true],  { expected: [2019] },
    [read('input'),       119315717514047,  101741582076661,  [2188371448886],                false], { expected: undefined,
      warning: 'Can\'t get it to work correctly within JS overflow constraints - see index.cpp and/or run "gcc index.cpp -std=c++11 -o binary && ./binary"!' },
    [read('input'),       119315717514047,  101741582076661,  [2020],                         true],  { expected: undefined, example: false,
      warning: 'Can\'t get it to work correctly within JS overflow constraints - see index.cpp and/or run "gcc index.cpp -std=c++11 -o binary && ./binary"!' }
  ]);
};
