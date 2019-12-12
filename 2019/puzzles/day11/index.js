// DAY 11
// https://adventofcode.com/2019/day/11

// Import dependencies
const puzzle              = require('../../../lib').puzzle,
      robot               = require('../../lib/ehprobot'),
      renderFieldFactory  = require('../../lib/image').renderFieldFactory;

// Set global inputs
const prog = require('fs').readFileSync(require('path').join(__dirname, './input.txt')).toString().trim().split(',').map(a => parseInt(a));

// 1st puzzle of the day
function puzzle01 (...args) {
  // Calculate trail
  const trail = [...robot.run(args)],
        surface = { };
  for (let point of trail) {
    const key = `${ point.coords.x }x${ point.coords.y }`; 
    surface[key] = point.color;
  }
  // Return count and drawn image
  return { count: Object.values(surface).length, image: robot.drawTrailAsImage(trail) }
}
module.exports.puzzle01 = () => {
  puzzle('2019', '11', '01', puzzle01, [
    prog, { expected: 1709, extract: r => r.count, render: renderFieldFactory({ extract: r => r.image }), test: false }
  ]);
};

// 2nd puzzle of the day
function puzzle02 (...args) {
  // Calculate trail
  const trail = [...robot.run(args, { ['0x0']: 1 })],
        surface = { };
  for (let point of trail) {
    const key = `${ point.coords.x }x${ point.coords.y }`; 
    surface[key] = point.color;
  }
  // Return count and drawn image
  return { count: Object.values(surface).length, image: robot.drawTrailAsImage(trail) }
}
module.exports.puzzle02 = () => {
  puzzle('2019', '11', '02', puzzle02, [
    prog, { expected: 249, extract: r => r.count, render: renderFieldFactory({ extract: r => r.image }), test: false }
  ]);
};
