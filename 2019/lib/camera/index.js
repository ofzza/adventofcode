// CAMERA IMIGING

// Import dependencies
const turing  = require('../turing'),
      image   = require('../image');

// Capture camera output
module.exports.shoot = function shoot (inputs) {

  // Translate to image path
  const points = {}
  let x = 0;    
      y = 0;
  for (let i = 0; i < inputs.length; i++) {
    // Check for line-break
    if (inputs[i] !== 10) {
      // Update points
      points[`${ x }x${ y }`] = {
        coords: { x, y },
        color: inputs[i]
      };
      // Update coordinates
      x++;
    } else {
      // Update coordinates
      x = 0;
      y++;
    }
  }

  // Output image path
  return points;
  
};

// Get calibration 
module.exports.calibrate = function calibrate (points) {
  return Object.values(points).reduce((calibration, point) => {
    const ptop    = points[`${ point.coords.x }x${ point.coords.y - 1 }`],
          pright  = points[`${ point.coords.x + 1 }x${ point.coords.y }`],
          pbottom = points[`${ point.coords.x }x${ point.coords.y + 1 }`],
          pleft   = points[`${ point.coords.x - 1 }x${ point.coords.y }`];
    if ((point.color === 35) && (ptop && ptop.color === 35) && (pright && pright.color === 35) && (pbottom && pbottom.color === 35) && (pleft && pleft.color === 35)) {
      calibration += (point.coords.x * point.coords.y);
    }
    return calibration;
  }, 0);
};
