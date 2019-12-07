// DAY 02
// https://adventofcode.com/2019/day/2

// Import dependencies
const puzzle = require('../../../lib').puzzle,
      turing = require('../../lib/turing');

// Set global inputs
const program = require('fs').readFileSync(require('path').join(__dirname, './input.txt')).toString().trim().split(',').map(a => parseInt(a))

// 1st puzzle of the day
function puzzle01 (...program) {
  [
    ...turing.run(program, [], [
      turing.actions.turingAdd,
      turing.actions.turingMultiply
    ])
  ];
  return program[0];
};
module.exports.puzzle01 = () => {
  puzzle('2019', '02', '01', puzzle01, [
    [1,9,10,3,2,3,11,0,99,30,40,50], { expected: 3500 },
    [1,0,0,0,99],                             { expected: 2 },
    [2,3,0,3,99],                             { expected: 2 },
    [2,4,4,5,99,0],                           { expected: 2 },
    [1,1,1,4,99,5,6,0,99],                    { expected: 30 },
    [program[0], 12, 2, ...program.slice(3)], { expected: 3101878, test: false }
  ]);
}

// 2nd puzzle of the day
function puzzle02 (target) {
  for (var noun=0; noun<=99; noun++) {
    for (var verb=0; verb<=99; verb++) {
      if (puzzle01(program[0], noun, verb, ...program.slice(3)) === target) {
        return (100 * noun) + verb;
      }
    }
  }
}
module.exports.puzzle02 = () => {
  puzzle('2019', '02', '02', puzzle02, [
    [3101878],  { expected: 1202 },
    [19690720], { expected: 8444, test: false }
  ]);
};
