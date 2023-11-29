// SPRING DROID

// Import dependencies
const flags       = require('../../../../lib').flags,
      logProgress = require('../../../../lib').logProgress,
      turing      = require('../../turing');

// Run Spring Droid
// ...
// Actions:   NOT IN OUT    Output !IN into OUT
//            AND IN OUT    Output (IN && OUT) into OUT
//            OR  IN OUT    Output (IN || OUT) into OUT
//            WALK          Start executing with sensor range 4
//            RUN           Start executing with sensor range 9
// Registers: [T = 0]       Temporary register
// Inputs:    [A ... I]     Inputs, floor seen on distance 1, 2, 3, 4
// Output:    [J = 0]       Jump over 3 blocks (J == 1) or not (J = 0)
module.exports.run = function run (prog, code, { palette } = {}) {

  // Initialize robot
  const assembler = [...code, 'WALK\n'],
        inputs    = assembler.join('\n').split('').map(c => c.charCodeAt(0)),
        robot     = turing.run(prog, inputs);

  // Run robot
  let output = [],
      result;
  while (!(result = robot.next()).done) {
    output.push(result.value);
  }

  // If logging progress, log output
  if (flags.PROGRESS) {
    logProgress(
      output.slice(0, output.length - 1)
            .map(c => String.fromCharCode(c))
            .join('')
            .replace(
              'Input instructions:\n',
              `Input instructions:\n${ assembler.join('\n') }`
            )
    );
  }

  // Return last output
  return output[output.length - 1];
  
};
