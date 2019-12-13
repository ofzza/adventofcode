// DAY 08
// https://adventofcode.com/2019/day/8

// Import dependencies
const puzzle              = require('../../../lib').puzzle,
      processLayers       = require('../../lib/image').processLayers,
      renderLinearFactory = require('../../lib/image').renderLinearFactory;

// Set global inputs
const input = require('fs').readFileSync(require('path').join(__dirname, './input.txt')).toString().trim().split('').map(a => parseInt(a));;

// 1st puzzle of the day
function puzzle01 (...args) {
  let digits = null,
      layers = processLayers(args[0][0], args[0][1], args[1]);
  for (let layer of layers) {
    if (!digits || (layer.digits[0] <= digits[0])) {
      digits = layer.digits;
    }
  }
  return (digits[1] * digits[2]);
}
module.exports.puzzle01 = () => {
  puzzle('2019', '08', '01', puzzle01, [
    [[3,2], [1,2,3,4,5,6,7,8,9,0,1,2]], { expected: 1 },
    [[25,6], input],                    { expected: 2250, example: false },
  ]);
};

// 2nd puzzle of the day
function puzzle02 (...args) {
  let width = args[0][0],
      height = args[0][1],
      flat = [],
      layers = processLayers(args[0][0], args[0][1], args[1]);
  for (let i = 0; i < layers.length; i++) {
    const layer = layers[i];
    for (let j=0; j<(width * height); j++) {
      flat[j] = (flat[j] === undefined || flat[j] === 2 ? layer.data[j] : flat[j]);
    }
  }
  return flat;
}
module.exports.puzzle02 = () => {
  puzzle('2019', '08', '02', puzzle02, [
    [[2,2], [0,2,2,2,1,1,2,2,2,2,1,2,0,0,0,0]], { expected: [0,1,1,0], render: renderLinearFactory({ width: 2, height: 2 }) },
    [[25,6], input],                            { expected: [1,1,1,1,0,1,0,0,1,0,0,0,1,1,0,1,0,0,1,0,1,0,0,0,0,1,0,0,0,0,1,0,0,
                                                             1,0,0,0,0,1,0,1,0,0,1,0,1,0,0,0,0,1,1,1,0,0,1,1,1,1,0,0,0,0,1,0,1,
                                                             0,0,1,0,1,0,0,0,0,1,0,0,0,0,1,0,0,1,0,0,0,0,1,0,1,0,0,1,0,1,0,0,0,
                                                             0,1,0,0,0,0,1,0,0,1,0,1,0,0,1,0,1,0,0,1,0,1,0,0,0,0,1,0,0,0,0,1,0,
                                                             0,1,0,0,1,1,0,0,0,1,1,0,0,1,1,1,1,0], render: renderLinearFactory({ width: 25, height: 6 }), example: false },
  ]);
};
