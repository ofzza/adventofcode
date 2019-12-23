// DECK OF SPACE CARDS

// Import dependencies
const flags       = require('../../../lib').flags,
      logProgress = require('../../../lib').logProgress,
      turing      = require('../turing'),
      image       = require('../image');

// Generates a new deck of cards
module.exports.newDeck = function * newDeck (n) {
  for (let i = 0; i < n; i++) {
    yield i;
  }
};

// Shuffle deck
module.exports.shuffle = function * shuffle (cards, instructions) {
  for (let instruction of instructions) {
    yield (cards = [...(shuffles[instruction.method](...[cards, ...instruction.params]))]);
  }
};

// Track single card through a shuffle
module.exports.track = function * track (n, card, instructions, backwards = false) {
  for (let instruction of (!backwards ? [...instructions] : [...instructions].reverse())) {
    yield (card = tracking[!backwards ? 'forward' : 'backward'][instruction.method](...[n, card, ...instruction.params]));
  }
};
// // Track single card through multiple shuffles
// module.exports.repeatTrack = function * track (repetitions, n, card, instructions, backwards = false) {
//   // Track through shuffles looking for repetitions
//   let position = card;
//   for (let i = 0; i < repetitions; i++) {
//     // Track through a single shuffle
//     let track = module.exports.track(n, position, instructions, backwards),
//         result;
//     while (!(result = track.next()).done) {
//       position = result.value;
//     }
//     // Check if repeated position
//     if (position === card) {
//       break;
//     }
//   }
// };

// // Gets repetition period of an instruction
// module.exports.getRepetitionPeriod = function getRepetitionPeriod (cards, instruction) {
//   return repetitionPeriod[instruction.method](...[cards, ...instruction.params]);
// };

// Shuffles
const shuffles = {
  // Shuffle cards by dealing into a new deck
  dealIntoNewDeck: function * dealIntoNewDeck (cards) {
    cards = [...cards];
    for (let i = (cards.length - 1); i >= 0; i--) {
      yield cards[i];
    }
  },
  // Cut cards to given depth (negative depth is counted from the bottom)
  cutCards: function * cutCards (cards, depth) {
    cards = [...cards];
    for (let card of [...cards.splice(depth), ...cards]) {
      yield card;
    }
  },
  // Deal onto the table with given increment
  dealWithIncrement: function * dealWithIncrement(cards, increment) {
    cards = [...cards];
    let table = [],
          i = 0;
    for (let card of cards) {
      table[i % cards.length] = card;
      i += increment;
    }
    for (let card of table) {
      yield card;
    }
  }
};

// Track card through shuffles
const tracking = {};
// Track forwards through shuffles
tracking.forward = {
  // Shuffle cards by dealing into a new deck
  dealIntoNewDeck: function dealIntoNewDeck (n, card) {
    return ((n - 1) - card);
  },
  // Cut cards to given depth (negative depth is counted from the bottom)
  cutCards: function cutCards (n, card, depth) {
    depth = (depth > 0 ? depth : (n + depth));
    return (card < depth ? (card + (n - depth)) : (card - depth));
  },
  // Deal onto the table with given increment
  dealWithIncrement: function dealWithIncrement (n, card, increment) {
    return ((card * increment) % n);
  }
};
// Trace back through shuffles
tracking.backward = {
  // Shuffle cards by dealing into a new deck
  dealIntoNewDeck: (n, card) => tracking.forward.dealIntoNewDeck(n, card),
  // Cut cards to given depth (negative depth is counted from the bottom)
  cutCards: (n, card, depth) => tracking.forward.cutCards(n, card, (-1 * depth)),
  // Deal onto the table with given increment
  dealWithIncrement: function dealWithIncrement (n, card, increment) {
    // Find movement on every wrap-around
    const every = (n / increment),
          movesRight = increment - (n % increment);
    let x;
    // Find how many times moved
    for (let i = 0; true; i++) {
      if ((i * movesRight) % increment === (card % increment)) {
        // Original number?!
        x = Math.floor(i * every + (card / increment));
        break;
      }
    }
    for (let i = 0; true; i++) {
      if ((((x + i) * increment) % n) === card) {
        return x + i;
      }
      if ((((x - i) * increment) % n) === card) {
        return x - i;
      }
    }    
  }
};

// Calculate repetition period for a shuffle
// const repetitionPeriod = {
//   // Shuffle cards by dealing into a new deck
//   dealIntoNewDeck: function dealIntoNewDeck (cards) {
//     // Empirically test repetition
//     let period;
//     const history = {};
//     for (let i = 0; true; i++) {
//       // Check for repetition
//       const key = cards.join(',');
//       if (!history[key]) {
//         history[key] = { i };
//       } else {
//         period = i;
//         break;
//       }
//       // Shuffle
//       cards = [...shuffles.dealIntoNewDeck(cards)];
//     }
//     // Return calculated repetition period
//     if (period === 2) {
//       return 2;
//     } else {
//       throw new Error('Calculation broken!');
//     }
//   },
//   // Cut cards to given depth (negative depth is counted from the bottom)
//   cutCards: function cutCards (cards, depth) {
//     // Empirically test repetition
//     let period;
//     const history = {};
//     for (let i = 0; true; i++) {
//       // Check for repetition
//       const key = cards.join(',');
//       if (!history[key]) {
//         history[key] = { i };
//       } else {
//         period = i;
//         break;
//       }
//       // Shuffle
//       cards = [...shuffles.dealIntoNewDeck(cards)];
//     }
//     // Return calculated repetition period
//     if (period === 2) {
//       return 2;
//     } else {
//       throw new Error('Calculation broken!');
//     }
//   },
//   // Deal onto the table with given increment
//   dealWithIncrement: function dealWithIncrement (cards, increment) {
//     // Calculate repetition period
//     const n = cards.length,
//           every = (n / increment),
//           movesRight = increment - (n % increment);
//     let x = (n % increment === 2 ? (n - 1) : Math.floor(n / 2));
//     // Find how many times moved
//     // for (let i = 1; true; i++) {
//     //   if ((i * movesRight) % increment === 0) {
//     //     // Original number?!
//     //     x = (i * every) % increment;
//     //     break;
//     //   }
//     // }
//     // Empirically test repetition
//     let period;
//     const history = {};
//     for (let i = 0; true; i++) {
//       // Check for repetition
//       const key = cards.join(',');
//       if (!history[key]) {
//         history[key] = { i };
//       } else {
//         period = i;
//         break;
//       }
//       // Shuffle
//       cards = [...shuffles.dealWithIncrement(cards, increment)];
//     }
//     // Return calculated repetition period
//     if (period === x) {
//       console.log(`'> PERIOD: max ${n} inc ${increment} mod ${n % increment} -> period: ${period} (calculated: ${x})`);
//       return x;
//     } else {
//       console.log(`'> PERIOD: max ${n} inc ${increment} mod ${n % increment} -> period: ${period} (calculated: ${x})`);
//       throw new Error('Calculation broken!');
//     }
//   }
// };