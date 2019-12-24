// DAY 04
// https://adventofcode.com/2019/day/4

// Import dependencies
const puzzle        = require('../../../lib').puzzle,
      ascendingDigits = require('../../lib/sequences').ascendingDigits;

// Set global inputs
const input = [128392, 643281];

// 1st puzzle of the day
function puzzle01 (min, max) {
  return [
    ...ascendingDigits({
      min,
      max,
      validationFn: (n, digits) => {
        // Check for double-digits
        if (digits.length <= 1) { return false; }
        for (let i = 0; i < digits.length; i++) {
          if (digits[i] === digits[i - 1]) { return true; }
        }
        return false;
      }
    })
  ].length;
}
module.exports.puzzle01 = () => {
  puzzle('2019', '04', '01', puzzle01, [
    [0, 100],   { expected: 9 },
    [0, 1000],  { expected: 90 },
    [0, 10000], { expected: 459 },
    input,      { expected: 2050, example: false }
  ]);
};

// 2nd puzzle of the day
function puzzle02 (min, max) {
  return [
    ...ascendingDigits({
      min,
      max,
      validationFn: (n, digits) => {
        // Check for double-only digits (triples or more don't count ...)
        if (digits.length <= 1) { return false; }
        for (let i = 1; i < digits.length; i++) {
          if ((digits[i] === digits[i - 1]) && (digits[i - 1] !== digits[i - 2]) && (digits[i + 1] !== digits[i])) { return true; }
        }
        return false;
      }
    })
  ].length;
}
module.exports.puzzle02 = () => {
  puzzle('2019', '04', '02', puzzle02, [
    [0, 100],     { expected: 9 },
    [999, 1000],  { expected: 0 },
    input,        { expected: 1390, example: false }
  ]);
};
