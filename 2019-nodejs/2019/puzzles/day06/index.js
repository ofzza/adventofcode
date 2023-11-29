// DAY 06
// https://adventofcode.com/2019/day/6

// Import dependencies
const puzzle          = require('../../../lib').puzzle,
      orbitsIntoGraph = require('../../lib/orbits').orbitsIntoGraph;

// Set global inputs
const input = require('fs').readFileSync(require('path').join(__dirname, './input.txt')).toString().trim().split('\n').map(a => a.trim());;

// 1st puzzle of the day
function puzzle01 (...args) {
  const orbits = orbitsIntoGraph(...args);
  return Object.values(orbits).reduce((sum, orbit) => { return (sum + orbit.depth); }, 0);
}
module.exports.puzzle01 = () => {
  puzzle('2019', '06', '01', puzzle01, [
    ['COM)B','B)C','C)D','D)E','E)F','B)G','G)H','D)I','E)J','J)K','K)L'],  { expected: 42 },
    input,                                                                  { expected: 204521, example: false },
  ]);
};

// 2nd puzzle of the day
function puzzle02 (...args) {
  const orbits = orbitsIntoGraph(...args);
  let you = orbits['YOU'],
      youCount = 0;
  do {
    let san = orbits['SAN'],
        sanCount = 0
    do {
      if (you === san) { return (youCount + sanCount - 2); }
      sanCount++;
    } while (san = san.parent);
    youCount++;
  } while (you = you.parent);
  return Object.values(orbits).reduce((sum, orbit) => { return (sum + orbit.depth); }, 0);
}
module.exports.puzzle02 = () => {
  puzzle('2019', '06', '02', puzzle02, [
    ['COM)B','B)C','C)D','D)E','E)F','B)G','G)H','D)I','E)J','J)K','K)L','K)YOU','I)SAN'],  { expected: 4 },
    input,                                                                                  { expected: 307, example: false },
  ]);
};
