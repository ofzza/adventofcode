// REPAIR DROID

// Import dependencies
const flags       = require('../../../../lib').flags,
      logProgress = require('../../../../lib').logProgress,
      readKey     = require('../../../../lib').readKey,
      turing      = require('../../turing'),
      image       = require('../../image');

// Run Repair Droid exploring the map
module.exports.explore = function explore (prog, world = {}, { palette } = {}) {

  // If logging progress, ready renderer
  const render = (flags.PROGRESS && image.renderFieldFactory({ palette }));

  // Initialize robot
  const inputs = [],        
        robot = turing.run(prog, inputs);
  let trail = {},
      distance = null;

  // Explore the entire world
  (function explore (robot, coords, world, path) {

    // Add starting position to world state
    if ((coords.x === 0 && coords.y === 0) && (world[`${ coords.x }x${ coords.y }`] === undefined)) {
      world[`${ coords.x }x${ coords.y }`] = {
        coords: { x: coords.x, y: coords.y },
        color:  3
      };
    }

    // Add current position to path state
    if (path[`${ coords.x }x${ coords.y }`] === undefined) {
      path[`${ coords.x }x${ coords.y }`] = {
        coords: { x: (coords.x), y: (coords.y) },
        color:  (coords.x === 0 && coords.y === 0 ? 3 : 4)
      };
    }

    // Define neighbouring points sampling function
    function sampleNeighbouringPoint (is, dx, dy) {
      if (path[`${ coords.x + dx }x${ coords.y + dy }`] === undefined) {
        // Move robot
        inputs.push(is[0]);
        const status = robot.next();
        // Update world state
        world[`${ coords.x + dx }x${ coords.y + dy }`] = {
          coords: { x: (coords.x + dx), y: (coords.y + dy) },
          color:  status.value
        };
        // If logging progress, draw world
        if (flags.PROGRESS) {
          logProgress([
            ...render(
              image.drawPointsAsImage([ ...Object.values(world), ...Object.values(path) ], { transparentColor: 9 })
            )
          ].join(''));
          // If interactive, pause for key presses
          if (flags.INTERACTIVE) {
            const key = readKey('Press any key to continue, or "x" to quit ...');
            if (key === 'x') { process.exit(0); }
          }
        }
        // If found destination, log trail
        if (status.value === 2) {
          if ((distance === null) || ((Object.values(path).length + 1) < distance)) {
            trail = {...path};
          }
        }
        // If robot moved to empty, continue exploring with updated coordinates
        if (status.value === 1 || status.value === 2) {
          // Continue exploring
          explore(robot, { x: (coords.x + dx), y: (coords.y + dy)}, world, {...path});
          // Return to previous position
          inputs.push(is[1]);
          robot.next();
        }
      }
    }

    // Explore neighbouring points (north)
    sampleNeighbouringPoint([1, 2], 0, -1);
    // Explore neighbouring points (south)
    sampleNeighbouringPoint([2, 1], 0, +1);
    // Explore neighbouring points (east)
    sampleNeighbouringPoint([4, 3], +1, 0);
    // Explore neighbouring points (west)
    sampleNeighbouringPoint([3, 4], -1, 0);

  })(robot, { x: 0, y: 0 }, world, {});

  // Return world, world, image, trail and minimal distance
  return {
    world,
    img: image.drawPointsAsImage([ ...Object.values(world), ...Object.values(trail) ], { transparentColor: 9 }),
    distance: (Object.values(trail).length)
  };

};

// Run fill simulation
module.exports.fill = function fill (prog, world = {}, { palette }) {

  // If logging progress, ready renderer
  const render = (flags.PROGRESS && image.renderFieldFactory({ palette }));

  // Find starting point
  const start = Object.values(world).find((p) => (p.color === 2));

  // Fill
  let time = 0;
  while (!!Object.values(world).find((p) => ((p.color === 1) || (p.color === 3)))) {

    // Get all seed points
    const seeds = Object.values(world).filter((p) => ((p.color === 2) && !p.done));

    // Spread from all seed points
    for (let seed of seeds) {
      if ([1, 3].indexOf(world[`${ seed.coords.x }x${ seed.coords.y - 1 }`].color) !== -1) {
        world[`${ seed.coords.x }x${ seed.coords.y - 1 }`].color = 2;
      }
      if ([1, 3].indexOf(world[`${ seed.coords.x }x${ seed.coords.y + 1 }`].color) !== -1) {
        world[`${ seed.coords.x }x${ seed.coords.y + 1 }`].color = 2;
      }
      if ([1, 3].indexOf(world[`${ seed.coords.x + 1}x${ seed.coords.y }`].color) !== -1) {
        world[`${ seed.coords.x + 1 }x${ seed.coords.y }`].color = 2;
      }
      if ([1, 3].indexOf(world[`${ seed.coords.x - 1}x${ seed.coords.y }`].color) !== -1) {
        world[`${ seed.coords.x - 1 }x${ seed.coords.y }`].color = 2;
      }
      seed.done = true;
    }

    // If logging progress, draw world
    if (flags.PROGRESS) {
      logProgress([
        ...render(
          image.drawPointsAsImage(Object.values(world), { transparentColor: 9 })
        )
      ].join(''));
      // If interactive, pause for key presses
      if (flags.INTERACTIVE) {
        const key = readKey('Press any key to continue, or "x" to quit ...');
        if (key === 'x') { process.exit(0); }
      }
    }

    // Update counter
    time++;

  }

  // Return world, world image and fill time
  return {
    world,
    img: image.drawPointsAsImage(Object.values(world), { transparentColor: 9 }),
    time
  };

};
