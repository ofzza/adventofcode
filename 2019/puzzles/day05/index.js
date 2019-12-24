// DAY 05
// https://adventofcode.com/2019/day/5

// Import dependencies
const puzzle  = require('../../../lib').puzzle,
      turing  = require('../../lib/turing');

// Set global inputs
const prog = require('fs').readFileSync(require('path').join(__dirname, './input.txt')).toString().trim().split(',').map(a => parseInt(a));

// 1st puzzle of the day
function puzzle01 (prog, inputs) {
  return [
    ...turing.run(prog, inputs, {
      actions: [
        turing.actions.turingAdd,
        turing.actions.turingMultiply,
        turing.actions.turingInput,
        turing.actions.turingOutput
      ]
    })
  ];
}
module.exports.puzzle01 = () => {
  // Set input
  const input = [1];
  // Run puzzle tests
  puzzle('2019', '05', '01', puzzle01, [
    [[...prog], input], { expected: 7286649, transform: r => r[r.length - 1], example: false }
  ]);
};

// 2nd puzzle of the day
function puzzle02 (prog, inputs) {
  return [
    ...turing.run(prog, inputs, {
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
}
module.exports.puzzle02 = () => {
  // Set input
  const input = [5];
  // Run puzzle tests
  puzzle('2019', '05', '02', puzzle02, [
    [[3,9,8,9,10,9,4,9,99,-1,8], [8]],  { expected: 1, transform: r => r[r.length - 1] },
    [[3,9,8,9,10,9,4,9,99,-1,8], [0]],  { expected: 0, transform: r => r[r.length - 1] },
    [[3,9,7,9,10,9,4,9,99,-1,8], [7]],  { expected: 1, transform: r => r[r.length - 1] },
    [[3,9,7,9,10,9,4,9,99,-1,8], [8]],  { expected: 0, transform: r => r[r.length - 1] },
    [[3,3,1108,-1,8,3,4,3,99], [8]],    { expected: 1, transform: r => r[r.length - 1] },
    [[3,3,1108,-1,8,3,4,3,99], [0]],    { expected: 0, transform: r => r[r.length - 1] },
    [[3,3,1107,-1,8,3,4,3,99], [7]],    { expected: 1, transform: r => r[r.length - 1] },
    [[3,3,1107,-1,8,3,4,3,99], [8]],    { expected: 0, transform: r => r[r.length - 1] },
    [[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], [7]],  { expected: 999, transform: r => r[r.length - 1] },
    [[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], [8]],  { expected: 1000, transform: r => r[r.length - 1] },
    [[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], [9]],  { expected: 1001, transform: r => r[r.length - 1] },
    [[...prog], input],                 { expected: 15724522, transform: r => r[r.length - 1], example: false }
  ]);
};
