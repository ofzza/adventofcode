// DAY 20
// https://adventofcode.com/2019/day/20

// Import dependencies
const puzzle              = require('../../../lib').puzzle,
      labyrinth           = require('../../lib/labyrinth');

// Input reader
const read = labyrinth.getReader(__dirname);

// 1st puzzle of the day
function puzzle01 (...args) {
  const analysis    = labyrinth.analyze(args[0]),
        startPoint  = analysis.byType.portal.find(p => p.type.portal.key == 'AA'),
        endPoint    = analysis.byType.portal.find(p => p.type.portal.key == 'ZZ');
  return Object.values(
    labyrinth.findShortestPath(startPoint, endPoint, analysis)
  ).length - 1;
}
module.exports.puzzle01 = () => {
  puzzle('2019', '20', '01', puzzle01, [
    [read('example101')], { expected: 23 },
    [read('example102')], { expected: 58 },
    [read('input')],      { expected: 516, example: false }
  ]);
};

// 2nd puzzle of the day
function puzzle02 (...args) {
  const analysis    = labyrinth.analyze(args[0]),
        startPoint  = analysis.byType.portal.find(p => p.type.portal.key == 'AA'),
        endPoint    = analysis.byType.portal.find(p => p.type.portal.key == 'ZZ');
  function searchToDepth (depth) {
        return labyrinth.findShortestPath(startPoint, endPoint, analysis, undefined, {
          allowMove: (point, level, path) => {
            if ((point === endPoint) && (level === 0)) {
              // Make sure to allow finalizing!!!
              return true;
            } else if (point.type.wall || point.type.wall || point.type.label) {
              // Don't allow movinng through walls
              return false;
            } else if (point.type.portal && (point.type.portal.location === 'outside') && (level <= 0)) {
              // Don't allow teleporting outside of outmost level
              return false;
            } else if (point.type.portal && (point.type.portal.location === 'inside') && (level > depth)) {
              // Don't allow going deeper than level 10
              return false;
            } else {
              // Allow movement
              return true;
            }
          },
          beforeYield: (e, { portaled = false } = {}) => {
            if (e.point.type.portal && e.point.type.portal.target && !portaled) {
              if (e.point.type.portal.location ==='outside') {
                e.pathLevel -= 1;  
              } else if (e.point.type.portal.location ==='inside') {
                e.pathLevel += 1;
              }
            }
          }
        });
      };
  // Search deeper until found a result
  let depth = 1,
      path;
  while (!(path = searchToDepth(depth++))) { }
  // Return result path length
  return (path ? Object.values(path).length - 1 : -1);
}
module.exports.puzzle02 = () => {
  puzzle('2019', '20', '02', puzzle02, [
    [read('example101')], { expected: 26 },
    [read('example201')], { expected: 396 },
    [read('input')],  { expected: undefined, example: false }
  ]);
};
