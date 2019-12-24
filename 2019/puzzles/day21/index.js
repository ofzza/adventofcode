// DAY 21
// https://adventofcode.com/2019/day/21

// Import dependencies
const puzzle  = require('../../../lib').puzzle,
      robot   = require('../../lib/robots/spring-droid').run;

// Set global inputs
const input = require('fs').readFileSync(require('path').join(__dirname, './input.txt')).toString().trim().split(',').map((a) => parseInt(a));

// 1st puzzle of the day
function puzzle01 (...prog) {
  // Run robot with custom assembler code
  // J = (!A || !B || !C) && D
  return robot(prog, [
    // ... (!A || !B || !C) && D
    'NOT A T',
    'OR  T J',
    'NOT B T',
    'OR  T J',
    'NOT C T',
    'OR  T J',
    // ... && D
    'AND D J',
    // ... walk!
    'WALK'
  ]);
}
module.exports.puzzle01 = () => {
  puzzle('2019', '21', '01', puzzle01, [
    input,  { expected: 19361023, example: false }
  ]);
};

// 2nd puzzle of the day
function puzzle02 (...prog) {
  // Run robot with custom assembler code
  // J =     D && H && (
  //               (!A)
  //            || (A && !B && (!E || !I))
  //            || (A && B && !C && (!F || (!E || !I)))
  //         )
  //   = (!A || (!B && (!E || !I)) || (B && !C && (!F || !E || !I))     && D && H
  //   = (!A || (!B && !(E && I)) || (B && !C && !(F && E && I))        && D && H
  //   = (!A || !(B || (E && I)) || (B && !(C || (F && E && I)))        && D && H
  //   = ((!(C || (F && E && I)) && B && H) || !((E && I) || B) || !A)  && D && H
  return robot(prog, [
    // (B && !(C || (F && E && I))
    'OR  F J',
    'AND E J',
    'AND I J',
    'OR  C J',
    'NOT J J',
    'AND B J',
    'AND H J',
    // ... || !(B || (E && I))
    'OR  E T',
    'AND I T',
    'OR  B T',
    'NOT T T',
    'OR  T J',
    // ... || !A
    'NOT A T',
    'OR  T J',
    // ... && D && H
    'AND D J',
    // ... run!
    'RUN'
  ]);
}
module.exports.puzzle02 = () => {
  puzzle('2019', '21', '02', puzzle02, [
    input,  { expected: 1141457530, example: false }
  ]);
};
