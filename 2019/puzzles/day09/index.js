// DAY 09
// https://adventofcode.com/2019/day/9

// Import dependencies
const puzzle = require('../../../lib').puzzle,
      turing = require('../../lib/turing');

// Set global inputs
const prog = require('fs').readFileSync(require('path').join(__dirname, './input.txt')).toString().trim().split(',').map(a => parseInt(a));;

// 1st puzzle of the day
function puzzle01 (...args) {
  const prog = args[0],
        inputs = args[1];
  return [
    ...turing.run(prog, inputs)
  ];
}
module.exports.puzzle01 = () => {
  puzzle('2019', '09', '01', puzzle01, [
    [[109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99], []],  { expected: [109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99] },
    [[1102,34915192,34915192,7,4,7,99,0], []],                          { expected: [1219070632396864] },
    [[104,1125899906842624,99], []],                                    { expected: [1125899906842624] },
    [[...prog], [1]],                                                   { expected: [3601950151], example: false },
  ]);
};

// 2nd puzzle of the day
function puzzle02 (...args) {
  return puzzle01(...args);
}
module.exports.puzzle02 = () => {
  puzzle('2019', '09', '02', puzzle02, [
    [[...prog], [2]], { expected: [64236], example: false },
  ]);
};
