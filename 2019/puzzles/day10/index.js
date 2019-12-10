// DAY 10
// https://adventofcode.com/2019/day/10

// Import dependencies
const puzzle      = require('../../../lib').puzzle,
      lineofsight = require('../../lib/lineofsight');

// Set global inputs
const input = require('fs').readFileSync(require('path').join(__dirname, './input.txt')).toString().trim().split('\n').map((row) => {
  return row.trim().split('');
});;

// 1st puzzle of the day
function puzzle01 (...args) {
  const result = lineofsight.getBestVantagePoint(args);
  return [result.point, result.visible];
}
module.exports.puzzle01 = () => {
  puzzle('2019', '10', '01', puzzle01, [
    ['.#..#','.....','#####','....#','...##'].map(a => a.split('')),                                                      { expected: [{x:3,y:4},8] },
    ['......#.#.','#..#.#....','..#######.','.#.#.###..','.#..#.....','..#....#.#','#..#....#.',
     '.##.#..###','##...#..#.','.#....####'].map(a => a.split('')),                                                       { expected: [{x:5,y:8},33] },
    ['#.#...#.#.','.###....#.','.#....#...','##.#.#.#.#','....#.#.#.','.##..###.#','..#...##..',
     '..##....##','......#...','.####.###.'].map(a => a.split('')),                                                       { expected: [{x:1,y:2},35] },
    ['.#..#..###','####.###.#','....###.#.','..###.##.#','##.##.#.#.','....###..#','..#.#..#.#',
     '#..#.#.###','.##...##.#','.....#.#..'].map(a => a.split('')),                                                       { expected: [{x:6,y:3},41] },
    ['.#..##.###...#######','##.############..##.','.#.######.########.#','.###.#######.####.#.',
     '#####.##.#.##.###.##','..#####..#.#########','####################','#.####....###.#.#.##',
     '##.#################','#####.##.###..####..','..######..##.#######','####.##.####...##..#',
     '.#####..#.######.###','##...#.##########...','#.##########.#######','.####.#.###.###.#.##',
     '....##.##.###..#####','.#.#.###########.###','#.#.#.#####.####.###','###.##.####.##.#..##'].map(a => a.split('')),  { expected: [{x:11,y:13},210] },
    input,                                                                                                                { expected: [{x:20,y:18},280], test: false },
  ]);
};

// 2nd puzzle of the day
function puzzle02 (...args) {
  const enumerated = [...lineofsight.enumerateFromVantagePoint(args[0], args[1])],
        point = enumerated[args[2]];
  return (100 * point.x + point.y);
}
module.exports.puzzle02 = () => {
  puzzle('2019', '10', '02', puzzle02, [
    [['.#....#####...#..','##...##.#####..##','##...#...#.#####.','..#.....X...###..',
     '..#.#.....#....##'].map(a => a.split('')), {x:8,y:3}, 8],                                                           { expected: 1501 },
    [input, {x:20,y:18}, 199],                                                                                            { expected: 706, test: false },
  ]);
};
