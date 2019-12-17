// GAME ARCADE

// Import dependencies
const flags       = require('../../../lib').flags,
      logProgress = require('../../../lib').logProgress,
      turing      = require('../turing'),
      image       = require('../image');

// Runs and plays arcade game program
module.exports.play = function * play (prog, inputs, { overlayFn = null, palette } = {}) {

  // Ready renderer for logging progress
  const render = (flags.PROGRESS && image.renderFieldFactory({ palette }));

  // Initialize screen and score
  let screen = {},
      overlay = [],
      score = 0;

  // Initialize a computer
  const computer = turing.run(prog, inputs, [
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

  // Run game
  let output, outputs = [];
  while (!(output = computer.next()).done) {

    // Append output
    outputs.push(output.value);
    if (outputs.length !== 3) { continue; }

    // Update screen and score
    if (outputs[0] === -1 && outputs[1] === 0) {   

      // Update score
      score = outputs[2];

    } else {

      // Add point to screen
      const point = {
        coords: { x: outputs[0], y: outputs[1] },
        color: outputs[2]
      };
      screen[`${ outputs[0] }x${ outputs[1] }`] = point;

      // If ball updated, draw
      if (point.color === 4) {

        // If ball updated, draw trail
        if (overlayFn) {
          const points = overlayFn(screen, overlay, point);
          if (points && points.length) {
            for (let point of points) { overlay.push(point); }
          }
        }

        // If logging progress, draw screen and capture inputs
        if (flags.PROGRESS) {
          logProgress([...render(image.drawPointsAsImage(Object.values(screen), { transparentColor: 2 }))].join(''));
        }
        
      }    

    }

    // Reset outputs
    outputs = [];

    // Yield screen update
    yield { screen: [...Object.values(screen), ...overlay], score };

  }
};
