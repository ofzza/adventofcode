// DAY 19
// https://adventofcode.com/2019/day/19

// Import dependencies
const flags       = require('../../../lib').flags,
      logProgress = require('../../../lib').logProgress,
      puzzle      = require('../../../lib').puzzle,
      image       = require('../../lib/image'),
      robot       = require('../../lib/robots/tractor-inspector');

// Set global inputs
const prog = require('fs').readFileSync(require('path').join(__dirname, `./input.txt`)).toString().trim().split(',').map(a => parseInt(a));

// 1st puzzle of the day
function puzzle01 (prog, limit) {
  let scanner = robot.scanForTraction(prog),
      result;
  while ((result = scanner.next()).value.radius < (limit - 1)) {}
  return result.value;
}
module.exports.puzzle01 = () => {
  puzzle('2019', '19', '01', puzzle01, [
    [prog, 10], { expected: 6,    transform: r => r.count, render: image.renderFieldFactory({ transform: r => r.img }) },
    [prog, 50], { expected: 160,  transform: r => r.count, render: image.renderFieldFactory({ transform: r => r.img }), example: false }
  ]);
};

// 2nd puzzle of the day
function puzzle02 (prog, limit) {
  // Initialize renderer
  const render = (flags.PROGRESS && image.renderFieldFactory());
  // Initialize the scanner
  let scanner = robot.scanForTraction(prog),
      result;
  while (!(result = scanner.next()).done) {
    // Draw last line
    if (flags.PROGRESS) {
      logProgress(
        [
          ...[...Array(result.value.offset)].map(() => 2),
          ...result.value.line
        ]
          .map(c => image.defaultPalette[c.color || c])
          .join('')
      );
    }
    // Check if target area fits
    const blPoint = result.value.points[`${ result.value.offset }x${ result.value.radius }`],
          brPoint = result.value.points[`${ result.value.offset + (limit - 1) }x${ result.value.radius }`],
          tlPoint = result.value.points[`${ result.value.offset }x${ result.value.radius - (limit - 1) }`],
          trPoint = result.value.points[`${ result.value.offset + (limit - 1) }x${ result.value.radius - (limit - 1) }`];
    if (blPoint && blPoint.color && brPoint && brPoint.color && tlPoint && tlPoint.color && trPoint && trPoint.color) {
      return (10000 * tlPoint.coords.x + tlPoint.coords.y);
    }
  }
}
module.exports.puzzle02 = () => {
  puzzle('2019', '19', '02', puzzle02, [
    [prog, 2],    { expected: 110015 },
    [prog, 3],    { expected: 220030 },
    [prog, 10],   { expected: 870118 },
    [prog, 100],  { expected: 9441282, example: false,
                    warning: 'Slow! Needs some way to optimize?!' }
  ]);
};
