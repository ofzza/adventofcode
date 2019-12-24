// DAY 16
// https://adventofcode.com/2019/day/16

// Import dependencies
const puzzle  = require('../../../lib').puzzle,
      fft     = require('../../lib/transmission').fft;

// Set global inputs
const input = require('fs').readFileSync(require('path').join(__dirname, './input.txt')).toString().trim().split('').map(a => parseInt(a));

// 1st puzzle of the day
function puzzle01 (input, n) {
  let processing = fft(input, { startIndex: 0, endIndex: 8 }),
      output;
  for (let i = 0 ; i < n; i++) {
    output = processing.next();
  }
  return output.value;
}
module.exports.puzzle01 = () => {
  puzzle('2019', '16', '01', puzzle01, [
    // ['12345678'.split('').map(a => parseInt(a)), 4],                            { expected: '01029498'.split('').map(a => parseInt(a)) },
    ['80871224585914546619083218645595'.split('').map(a => parseInt(a)), 100],  { expected: '24176176'.split('').map(a => parseInt(a)), transform: r => r.slice(0, 8) },
    ['19617804207202209144916044189917'.split('').map(a => parseInt(a)), 100],  { expected: '73745418'.split('').map(a => parseInt(a)), transform: r => r.slice(0, 8) },
    ['69317163492948606335995924319873'.split('').map(a => parseInt(a)), 100],  { expected: '52432133'.split('').map(a => parseInt(a)), transform: r => r.slice(0, 8) },
    [input, 100],                                                               { expected: '76795888'.split('').map(a => parseInt(a)), transform: r => r.slice(0, 8), example: false,
                                                                                  warning: 'Slow! Needs some way to optimize?!' }
  ]);
};

// 2nd puzzle of the day
function puzzle02 (input, repetitions, n) {
  let startIndex  = parseInt(input.slice(0, 7).join('')),
      endIndex    = startIndex + 8,
      processing  = fft(input, { repeatInput: repetitions, startIndex, endIndex }),
      output;
  for (let i = 0 ; i < n; i++) {
    output = processing.next();
  }
  return output.value.slice(startIndex, endIndex);
}
module.exports.puzzle02 = () => {
  puzzle('2019', '16', '02', puzzle02, [
    ['03036732577212944063491565474664'.split('').map(a => parseInt(a)), 10000, 100],  { expected: '84462026'.split('').map(a => parseInt(a)), transform: r => r.slice(0, 8),
                                                                                         warning: 'Slow! Needs some way to optimize?!' },
    ['02935109699940807407585447034323'.split('').map(a => parseInt(a)), 10000, 100],  { expected: '78725270'.split('').map(a => parseInt(a)), transform: r => r.slice(0, 8),
                                                                                         warning: 'Slow! Needs some way to optimize?!' },
    ['03081770884921959731165446850517'.split('').map(a => parseInt(a)), 10000, 100],  { expected: '53553731'.split('').map(a => parseInt(a)), transform: r => r.slice(0, 8),
                                                                                         warning: 'Slow! Needs some way to optimize?!' },
    [input, 10000, 100],                                                               { expected: '84024125'.split('').map(a => parseInt(a)), transform: r => r.slice(0, 8), example: false,
                                                                                         warning: 'Slow! Needs some way to optimize?!' }
  ]);
};
