// TRACTOR BEAM INSPECTING SHIP DRONE

// Import dependencies
const image       = require('../../image'),
      turing      = require('../../turing');

// Scan space for traction beam
module.exports.scanForTraction = function * scanForTraction (prog) {
  let count = 0,
      offset = 0,
      points = {};
      // Expand area
  for (let r = 0; true; r++) {
    // Expand scan area
    let previousMoving = null,
        line = [];
    for (let x = offset; x <= r; x++) {
      if (!points[`${ x }x${ r }`]) {
        const moving = checkTraction([...prog], x, r);
        count += moving;
        line.push(points[`${ x }x${ r }`] = {
          coords: { x: x, y: r },
          color: moving
        });
        if ((previousMoving !== 1) && (moving === 1)) { offset = x; }
        if ((previousMoving === 1) && (moving === 0)) { break; }
        previousMoving = moving;
      }
    }
    // Yield scanned
    yield {
      radius: r,
      count,
      offset,
      line,
      points,
      img: image.drawPointsAsImage(Object.values(points), { transparentColor: 2 })
    };
  }
}

// Got to a requested coordinates
function checkTraction (prog, x, y) {
  // Initialize robot
  const robot = turing.run(prog, [x, y]);
  // Get robot output
  return [...robot][0];
};
