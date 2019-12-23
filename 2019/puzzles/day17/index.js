// DAY 17
// https://adventofcode.com/2019/day/17

// Import dependencies
const flags       = require('../../../lib').flags,
      readKey     = require('../../../lib').readKey,
      puzzle      = require('../../../lib').puzzle,
      logProgress = require('../../../lib').logProgress,
      turing      = require('../../lib/turing'),
      camera      = require('../../lib/camera'),
      image       = require('../../lib/image');

// Set global inputs
const prog = require('fs').readFileSync(require('path').join(__dirname, './input.txt')).toString().trim().split(',').map(a => parseInt(a));

// Color palette to draw the screen with
const palette = {
  0:    ' ',
  35:   '#',
  46:   '.',
  60:   '<',
  62:   '>',
  94:   '^',
  118:  'v'
};

// 1st puzzle of the day
function puzzle01 (...args) {
  // Initialize robot
  const robot = turing.run(args, []);
  // Pipe robot output to camera
  const points      = camera.shoot([...robot]),
        calibration = camera.calibrate(points);
  // Return result
  const img = image.drawPointsAsImage(Object.values(points), { transparentColor: 0 });
  return {
    calibration,
    size: img.length * (img[0].length + 1),
    img
  };
}
module.exports.puzzle01 = () => {
  puzzle('2019', '17', '01', puzzle01, [
    prog, { expected: 5680, transform: r => r.calibration, render: image.renderFieldFactory({ transform: r => r.img, palette }), example: false }
  ]);
};

// 2nd puzzle of the day
function puzzle02 (...args) {

  // Initialize robot input  
  // R12,R4,R10,R12,R6,L8,R10,R12,R4,R10,R12,L8,R4,R4,R6,R12,R4,R10,R12,R6,L8,R10,L8,R4,R4,R6,R12,R4,R10,R12,R6,L8,R10,L8,R4,R4,R6
  // A------------- B-------- A------------- C---------- A------------- B-------- C---------- A------------- B-------- C----------
  const inputs = [
    //...................]
    'A,B,A,C,A,B,C,A,B,C',
    'R,12,R,4,R,10,R,12',
    'R,6,L,8,R,10',
    'L,8,R,4,R,4,R,6',
    (flags.INTERACTIVE ? 'y' : 'n')
  ]
    .reduce((inputs, s) => [...inputs, ...s.split(''), '\n'], [])
    .map(c => c.charCodeAt(0));

  // Initialize robot
  const robot = turing.run(args, inputs);

  // Run robot
  let points = [],
      line = '',
      lineCount = 0,
      output,
    result;
  while (!(result = robot.next()).done) {

    // Get output
    output = result.value;
    points.push(output);  
    line += String.fromCharCode(output);

    // Write line when line complete
    if (line[line.length - 1] === '\n') {
      // Reset line count if new frame
      if ((lineCount >= 45) && (line.substr(0, 3) === '...')) {
        lineCount = 0;  
      }
        // If interactive, pause for key presses on full frames or single lines after full frames
        if (lineCount >= 45) {
        const key = readKey('Press any key to continue, or "x" to quit ...');
        if (key === 'x') { process.exit(0); }
      }
      // If logging progress, log progress
      if (flags.PROGRESS) {
        logProgress(line.substr(0, line.length - 1));
      }
      // Update  line and line count
      line = '';
      lineCount++;
    }
    

  }

  // Return result TODO: Get final value from robot
  return output;

}
module.exports.puzzle02 = () => {
  puzzle('2019', '17', '02', puzzle02, [
    [2, ...prog.slice(1)],  { expected: 895965, example: false }
  ]);
};
