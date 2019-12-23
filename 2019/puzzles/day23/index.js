// DAY 23
// https://adventofcode.com/2019/day/23

// Import dependencies
const puzzle  = require('../../../lib').puzzle,      
      network = require('../../lib/network');

// Set global inputs
const prog = require('fs').readFileSync(require('path').join(__dirname, './input.txt')).toString().trim().split(',').map(a => parseInt(a));

// 1st puzzle of the day
function puzzle01 (prog, n) {
  // Initialize computers
  const boxes = [...Array(n)].map((undef, i) => network.spawnNetworkedBox(prog, i));
  // Bootstrap network
  let traffic = network.bootstrap(boxes),
      packet;
  while (!(packet = traffic.next()).done) {
    if (packet.value[1] === 255) {
      return packet.value[3];
    }
  }
}
module.exports.puzzle01 = () => {
  puzzle('2019', '23', '01', puzzle01, [
    [prog, 50], { expected: 22829, example: false }
  ]);
};

// 2nd puzzle of the day
function puzzle02 (prog, n) {
  // Initialize computers
  const boxes = [...Array(n)].map((undef, i) => network.spawnNetworkedBox(prog, i));
  // Bootstrap network
  let traffic = network.bootstrap(boxes),
      packet,
      previousY;
  while (!(packet = traffic.next()).done) {
    if ((packet.value[0] === 255) && (packet.value[1] === 0)) {
      if (previousY === packet.value[3]) {
        return packet.value[3];
      } else {
        previousY = packet.value[3];
      }
    }
  }
}
module.exports.puzzle02 = () => {
  puzzle('2019', '23', '02', puzzle02, [
    [prog, 50], { expected: 15678, example: false }
  ]);
};
