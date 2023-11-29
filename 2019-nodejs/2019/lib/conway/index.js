// CONWAY's GAME OF BUGS

// Import dependencies
const flags       = require('../../../lib').flags
      logProgress = require('../../../lib').logProgress

// Plays the GAME OF BUGS
module.exports.play = function * play (world2d, { recursive = false} = {}) {

  // Initialize world and world measures
  let world = {
        [-1]: [...Array(world2d.length)].map(() => [...Array(world2d[0].length)].map(() => 0)),
        [0]:  world2d,
        [+1]: [...Array(world2d.length)].map(() => [...Array(world2d[0].length)].map(() => 0))
      },
      miny = 0,
      maxy = (world2d.length - 1),
      minx = 0,
      maxx = (world2d[0].length - 1),
      center = {
        x: Math.floor(world2d[0].length / 2),
        y: Math.floor(world2d.length / 2)
      };

  // Log world state
  if (flags.PROGRESS) { logProgress(`${ worldMap(world) }\n`); }

  // Play game
  while (true) {

    // Initialize all world levels
    const next = Object.keys(world).reduce((next, level) => {
      next[level] = [...Array(world2d.length)].map(() => [...Array(world2d[0].length)].map(() => 0));
      return next;
    }, {})
    
    // Process all levels
    for (let level of Object.keys(world).map(i => parseInt(i))) {

      // If world level has any bugs, neighbouring levels need to be added
      const levelScore = world[level].reduce((score, line) => {
        return score + line.reduce((score, bug) => (score + bug), score)
      }, 0);
      if (levelScore) {
        if (!world[level - 1]) { next[level - 1] = [...Array(world2d.length)].map(() => [...Array(world2d[0].length)].map(() => 0)); }
        if (!world[level + 1]) { next[level + 1] = [...Array(world2d.length)].map(() => [...Array(world2d[0].length)].map(() => 0)); }
      }

      // Progress world state for this level
      for (let y = 0; y < world[level].length; y++) {
        for (let x = 0; x < world[level][y].length; x++) {

          // Check if center
          if (recursive & (x === center.x) && (y === center.y)) { continue; }

          // Count number of bugs around on this level
          let count = 0,
              points = [
                { x: 0, y: -1 },
                { x: 0, y: +1 },
                { x: -1, y: 0 },
                { x: +1, y: 0 }
              ];
          for (let p of points) {
            // Check if edge point
            if (recursive && ((world[level][y + p.y] === undefined) || (world[level][y + p.y][x + p.x] === undefined))) {
              // Point over the edge, calculate based on outside level bugs
              if (world[level - 1]) {
                if ((p.y === -1) && (p.x === 0)) { count += world[level - 1][center.y - 1][center.x + 0]; }
                if ((p.y === +1) && (p.x === 0)) { count += world[level - 1][center.y + 1][center.x + 0]; }
                if ((p.y === 0) && (p.x === -1)) { count += world[level - 1][center.y + 0][center.x - 1]; }
                if ((p.y === 0) && (p.x === +1)) { count += world[level - 1][center.y + 0][center.x + 1]; }
              }
            } else if (recursive && (((y + p.y) === center.y) && ((x + p.x) === center.x))) {
              // Center point, calculate based on inner level bugs
              if (world[level + 1]) {
                if ((p.y === -1) && (p.x === 0)) { count += world[level + 1][maxy].reduce((sum, bug) => (sum + bug), 0); }
                if ((p.y === +1) && (p.x === 0)) { count += world[level + 1][miny].reduce((sum, bug) => (sum + bug), 0); }
                if ((p.y === 0) && (p.x === -1)) { count += world[level + 1].reduce((sum, line) => (sum + line[maxx]), 0); }
                if ((p.y === 0) && (p.x === +1)) { count += world[level + 1].reduce((sum, line) => (sum + line[minx]), 0); }
              }
            } else {
              // Same level point, check for bugs
              if (world[level][y + p.y] && world[level][y + p.y][x + p.x] && world[level][y + p.y][x + p.x] === 1) { count++; }
            }
          }

          // Process field based on neighbour count
          if (world[level][y][x] === 1) {
            // Bug is alive, check if it dies
            next[level][y][x] = (count === 1 ? 1 : 0);
          } else if (world[level][y][x] === 0) {
            // Space is empty, check if it spawns a bug
            next[level][y][x] = ((count === 1) || (count === 2) ? 1 : 0);
          }

        }
      }
     
    }
    // Update world state for this level
    world = next;

    // Log world state
    const map = worldMap(world);
    if (flags.PROGRESS) { logProgress(`${ map }\n`); }

    // Yield updated world state
    yield { world, map, hash: worldHash(world) };

  }

};

// Calculate biodiversity of a world
module.exports.calculateBiodiversity = function calculateBiodiversity (world) {
  let score = 0;
  for (let level of Object.keys(world)) {
    for (let y = 0; y < world[level].length; y++) {
      for (let x = 0; x < world[level][y].length; x++) {
        if (world[level][y][x] === 1) {
          const e = (y * world[level][y].length) + x;
          score += Math.pow(2, e);
        }
      }
    }
  }
  return score;
};

// Represent world as a printable map
function worldMap (world) {
  const maps = Object.keys(world).sort((a, b) => {
      a = parseInt(a);
      b = parseInt(b);
      return (a !== b ? (a < b) ? -1 : +1 : 0);
    })
    .map(level => [
      `L ${ level }`.padEnd(world[level][0].length, ' '),
      ...world[level].map(((l, y) => l.map((c, x) => ((!c && (y === Math.floor(world[level].length / 2)) && (x === Math.floor(l.length / 2))) ? '?' : ['.', '#'][c])).join('')))
    ]);
  return [null, ...world[0]].map((l, y) => {
    return maps.map(m => m[y]).join(' ');
  }).join('\n');
}

// Represent world as unique hash
function worldHash (world) {
  return Object.values(world).map(w => w.map((l => l.join(''))).join('/')).join('|');
}
