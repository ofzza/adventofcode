// EMERGENCY HULL PAINTING ROBOT

// Import dependencies
const turing = require('../turing');

// Run robot
module.exports.run = function * run (prog, surface = {}) {
  const coords = { x: 0, y: 0 },
        inputs = [],        
        robot = turing.run(prog, inputs, [
          turing.actions.turingAdd,
          turing.actions.turingMultiply,
          turing.actions.turingInput,
          turing.actions.turingOutput,
          turing.actions.turingJumpTrue,
          turing.actions.turingJumpFalse,
          turing.actions.turingLessThan,
          turing.actions.turingEquals,
          turing.actions.turingSetRelativeBase
        ]);
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
}

// Draw trail as image
module.exports.drawTrailAsImage = function drawTrailAsImage (trail) {
  // Normalize trail
  let dx = { min: null, max: null }, 
      dy = { min: null, max: null };
  for (const point of trail) {
    if (dx.min === null || point.coords.x < dx.min) { dx.min = point.coords.x; }
    if (dx.max === null || point.coords.x > dx.max) { dx.max = point.coords.x; }
    if (dy.min === null || point.coords.y < dy.min) { dy.min = point.coords.y; }
    if (dy.max === null || point.coords.y > dy.max) { dy.max = point.coords.y; }
  }
  for (const point of trail) {
    if (dx.min < 0) { point.coords.x += (-1 * dx.min); }
    if (dy.min < 0) { point.coords.y += (-1 * dy.min); }
  }
  // Initialize image
  const image = [...Array(dy.max - dy.min + 1)].map(() => {
    return [...Array(dx.max - dx.min + 1)].map(n => 2);
  });
  // Draw trail onto the image
  for (const point of trail) {
    image[point.coords.y][point.coords.x] = point.color;
  }
  // Return image
  return image;
}
