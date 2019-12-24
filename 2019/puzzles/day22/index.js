// DAY 22
// https://adventofcode.com/2019/day/22

// Import dependencies
const puzzle  = require('../../../lib').puzzle,
      cards   = require('../../lib/cards');

// Read input from file
function read (name) {
  return require('fs').readFileSync(require('path').join(__dirname, `./${ name }.txt`)).toString().trim().split('\n').map((l) => {
    if (l === 'deal into new stack') {
      return {
        method: 'dealIntoNewDeck',
        params: []
      };
    }  else if (l.substr(0, 3) === 'cut') {
      return {
        method: 'cutCards',
        params: [parseInt(l.substr(3))]
      };
    } else if (l.substr(0, 19) === 'deal with increment') {
      return {
        method: 'dealWithIncrement',
        params: [parseInt(l.substr(19))]
      };
    }
  })
}

// 1st puzzle of the day
function puzzle01 (n, instructions) {
  // Ready new deck
  let deck    = cards.newDeck(n),
      shuffle = cards.shuffle(deck, instructions),
      order,
      result;
  while (!(result = shuffle.next()).done) { order = result.value; }
  return order;
}
module.exports.puzzle01 = () => {
  puzzle('2019', '22', '01', puzzle01, [
    [10,    read('example101')],  { expected: [0, 3, 6, 9, 2, 5, 8, 1, 4, 7] },
    [10,    read('example102')],  { expected: [3, 0, 7, 4, 1, 8, 5, 2, 9, 6] },
    [10,    read('example103')],  { expected: [6, 3, 0, 7, 4, 1, 8, 5, 2, 9] },
    [10,    read('example104')],  { expected: [9, 2, 5, 8, 1, 4, 7, 0, 3, 6] },
    [10007, read('input')],       { expected: 3377, example: false, transform: r => r.indexOf(2019) }
  ]);
};

// 2nd puzzle of the day
function puzzle02 (repetitions, n, instructions, positions, backwards) {
  return positions.map(position => {
    let track = cards.repeatTrack(repetitions, n, position, instructions, backwards),
        next,
        result;
    while (!(result = track.next()).done) {
      next = result.value;
    };
    return next;
  });
}
module.exports.puzzle02 = () => {
  puzzle('2019', '22', '02', puzzle02, [
    // [1,               10,               read('example101'), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9], false], { expected: [0, 7, 4, 1, 8, 5, 2, 9, 6, 3] },
    // [1,               10,               read('example101'), [0, 7, 4, 1, 8, 5, 2, 9, 6, 3], true],  { expected: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] },
    // [1,               10,               read('example102'), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9], false], { expected: [1, 4, 7, 0, 3, 6, 9, 2, 5, 8] },
    // [1,               10,               read('example102'), [1, 4, 7, 0, 3, 6, 9, 2, 5, 8], true],  { expected: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] },
    // [1,               10,               read('example103'), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9], false], { expected: [2, 5, 8, 1, 4, 7, 0, 3, 6, 9] },
    // [1,               10,               read('example103'), [2, 5, 8, 1, 4, 7, 0, 3, 6, 9], true],  { expected: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] },
    // [1,               10,               read('example104'), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9], false], { expected: [7, 4, 1, 8, 5, 2, 9, 6, 3, 0] },
    // [1,               10,               read('example104'), [7, 4, 1, 8, 5, 2, 9, 6, 3, 0], true],  { expected: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] },
    // [1,               10007,            read('input'),      [2019],                         false], { expected: [3377] },
    // [1,               10007,            read('input'),      [3377],                         true],  { expected: [2019] },
    // [101741582076661, 119315717514047,  read('input'),      [2020],                         true],  { expected: undefined, example: false }
  ]);
};
