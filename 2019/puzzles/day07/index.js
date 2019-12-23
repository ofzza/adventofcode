// DAY 07
// https://adventofcode.com/2019/day/7

// Import dependencies
const puzzle        = require('../../../lib').puzzle,
      permutations  = require('../../lib/sequences').permutations,
      turing        = require('../../lib/turing');

// Set global inputs
const prog = require('fs').readFileSync(require('path').join(__dirname, './input.txt')).toString().trim().split(',').map(a => parseInt(a));;

// 1st puzzle of the day
function puzzle01 (...args) {
  const prog = args,
        settingsPermutations = permutations([0, 1, 2, 3, 4]);
  let maxThrust = null,
      maxSettings = null;
  for (let settings of settingsPermutations) {
    let value = 0;
    for (let setting of settings) {
      const output = [
        ...turing.run([...prog], [setting, value], {
          actions: [
            turing.actions.turingAdd,
            turing.actions.turingMultiply,
            turing.actions.turingInput,
            turing.actions.turingOutput,
            turing.actions.turingJumpTrue,
            turing.actions.turingJumpFalse,
            turing.actions.turingLessThan,
            turing.actions.turingEquals
          ]
        })
      ];
      value = output[0];
    }
    if (!maxThrust || (maxThrust < value)) {
      maxThrust = value;
      maxSettings = settings;
    }
  }
  return [ maxThrust, maxSettings ];
}
module.exports.puzzle01 = () => {
  puzzle('2019', '07', '01', puzzle01, [
    [3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0],                                                       { expected: 43210, transform: r => r[0] },
    [3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0],                             { expected: 54321, transform: r => r[0] },
    [3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0],  { expected: 65210, transform: r => r[0] },
    prog,                                                                                                   { expected: 34686, transform: r => r[0], example: false },
  ]);
};

// 2nd puzzle of the day
function puzzle02 (...args) {
  const prog = args,
        settingsPermutations = permutations([5, 6, 7, 8, 9]);
  let maxThrust = null,
      maxSettings = null;
  for (const settings of settingsPermutations) {
    // Initialize generators and generator inputs for this settings permutation
    const inputs = [...Array(5)].map((x, i) => [settings[i]]),
          amplifiers = [...Array(5)].map((x, i) => turing.run([...prog], inputs[i], {
            actions: [
              turing.actions.turingAdd,
              turing.actions.turingMultiply,
              turing.actions.turingInput,
              turing.actions.turingOutput,
              turing.actions.turingJumpTrue,
              turing.actions.turingJumpFalse,
              turing.actions.turingLessThan,
              turing.actions.turingEquals
            ]
          }));
    // Run for settings permutation
    let output = 0,
        i = 0,
        result;
    do {
      // Update input with previous output
      inputs[i % 5].push(output);
      // Run next generator
      result = amplifiers[i % 5].next();
      if (!result.done) {
        output = result.value;
        i++;
      }
    } while (false && (i % 5 !== 4) || (!result.done));
    // Check if max output
    if (!maxThrust || (maxThrust < output)) {
      maxThrust = output;
      maxSettings = settings;
    }
  }
  return [maxThrust, ...maxSettings];
}
module.exports.puzzle02 = () => {
  puzzle('2019', '07', '02', puzzle02, [
    [3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5],  { expected: 139629729, transform: r => r[0] },
    [3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,
     54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10],  { expected: 18216, transform: r => r[0] },
    prog,                                                                                     { expected: 36384144, transform: r => r[0], example: false }
  ]);
};
