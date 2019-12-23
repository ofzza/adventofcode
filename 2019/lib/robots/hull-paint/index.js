// EMERGENCY HULL PAINTING ROBOT

// Import dependencies
const turing = require('../../turing');

// Run Emergency Hull Painting (EHP) robot
module.exports.run = function * run (prog, surface = {}) {
  const coords = { x: 0, y: 0 },
        inputs = [],        
        robot = turing.run(prog, inputs);
  let direction = 0,
      result;
  do {

    // Read current coords color into robot input
    const coordsKey = `${ coords.x }x${ coords.y }`;
    inputs.push(surface[coordsKey] || 0);
    
    // Get new color
    const colorChange = (result = robot.next()).value;
    if (result.done) { break; }

    // Write trail
    surface[coordsKey] = colorChange;
    yield { coords: {...coords}, color: colorChange };

    // Get new direction
    const directionChange = (result = robot.next()).value;
    if (result.done) { break; }
    direction = (direction + (directionChange ? 90 : -90) + 360) % 360;

    // Move robot
    if (direction === 0) {
      coords.y += -1;
    } else if (direction === 90) {
      coords.x += +1;
    } else if (direction === 180) {
      coords.y += +1;
    } else if (direction === 270) {
      coords.x += -1;
    }

  } while (!result.done);
};
