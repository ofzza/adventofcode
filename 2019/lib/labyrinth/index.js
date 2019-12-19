// LABYRINTH SOLVER

// Import dependencies
const flags       = require('../../../lib').flags,
      logProgress = require('../../../lib').logProgress,
      readKey     = require('../../../lib').readKey,
      image       = require('../image');

// Solves a labyrinth
module.exports.solve = function solve (labyrinth, { startChar = '@', keysRegexp = /[a-z]/, doorsRegExp = /[A-Z]/, mapKeyToDoor = (key) => key.toUpperCase(), distanceCorrection, palette = {...image.defaultPalette} } = {}) {

  // If logging progress, ready renderer
  const render = (flags.PROGRESS && image.renderFieldFactory({ palette }));
  
  // Initialize shared cache  
  const cache = {};

  // Find relevant objects
  const start = Object.values(labyrinth).find(p => (p.color === startChar)),
        keys  = Object.values(labyrinth).filter(p => p.color.toString().match(keysRegexp))
                  .reduce((keys, p) => {
                    keys[p.color] = p;
                    return keys;
                  }, {}),
        doors = Object.values(labyrinth).filter(p => p.color.toString().match(doorsRegExp))
                  .reduce((keys, p) => {
                    keys[p.color] = p;
                    return keys;
                  }, {});

  // Add starting position, keys and doors to palette
  palette[startChar] = startChar;
  for (let p of [...Object.values(keys), ...Object.values(doors)]) {
    palette[p.color] = p.color;
  }

  // Close all dead-ends
  explore(labyrinth, start, {
    keysRegexp,
    doorsRegExp,
    palette,
    allowMovementFn:  (location) => true,
    onMovementFn:     ({ empty, location, path }) => {
      // Close up dead-end
      if (empty) { location.color = '1'; }
    },
    PROGRESS: false,
    INTERACTIVE: false
  });
  if (flags.PROGRESS) {
    logProgress([...render(image.drawPointsAsImage(Object.values(labyrinth), { transparentColor: 0, palette }))].join(''));
  }

  // Collect all keys locations and doors they are behind
  explore(labyrinth, start, {
    keysRegexp,
    doorsRegExp,
    palette,
    allowMovementFn:  () => true,
    onMovementFn:     ({ key, location, path }) => {
      if (key) {
        // Get doors (and keys as "automatic" doors) on the way
        keys[location.color].doors = Object.values(path)
          .filter((p) => (p.color.toString().match(doorsRegExp) || p.color.toString().match(keysRegexp)))
          .map((p) => (p.color.toString().match(doorsRegExp) ? p.color : mapKeyToDoor(p.color)));
      }
    },
    PROGRESS: false,
    INTERACTIVE: false
  });
  for (let key of Object.values(keys)) {
    key.directDoors = [...key.doors];
  }
  const sortedKeysByDirectDoorsCount = Object.values(keys).sort((a, b) => (a.directDoors.length === b.directDoors.length ? 0 : (a.directDoors.length < b.directDoors.length ? -1 : +1)));
  if (flags.PROGRESS) {
    logProgress(`Sorted keys by direct occlusion depth:`);
    for (let key of sortedKeysByDirectDoorsCount) {
      logProgress(`> KEY "${ key.color }" behind ${ key.directDoors.length ? key.directDoors.join(', ') : 'no doors' }`);
    }
    logProgress('');
  }

  // For all doors, add implied doors (needed to get keys, needed to get doors, needed to get keys, ...)
  for (let key of Object.values(keys)) {
    key.totalDoors = [...key.directDoors];
  }
  while (Object.values(keys).find(key => key.doors.length)) {
    for (let parent of sortedKeysByDirectDoorsCount) {
      if (!parent.doors.length) {
        for (let child of sortedKeysByDirectDoorsCount) {
          if ((child !== parent) && (child.doors.find(d => (d === mapKeyToDoor(parent.color))))) {
            child.totalDoors = [...child.totalDoors, ...parent.totalDoors];
            child.doors.splice(child.doors.findIndex(d => (d === mapKeyToDoor(parent.color))), 1);
          }
        }
      }
    }
  }
  for (let key of Object.values(keys)) {
    key.totalDoors = Object.keys(
      key.totalDoors.reduce((doors, d) => { doors[d] = true; return doors; }, {})
    ).sort();
  }
  const sortedKeysByTotalDoorsCount = Object.values(keys).sort((a, b) => (a.totalDoors.length === b.totalDoors.length ? 0 : (a.totalDoors.length < b.totalDoors.length ? -1 : +1)));
  if (flags.PROGRESS) {
    logProgress(`Sorted keys by total occlusion depth:`);
    for (let key of sortedKeysByTotalDoorsCount) {
      logProgress(`> KEY "${ key.color }" behind ${ key.totalDoors.length ? key.totalDoors.join(', ') : 'nothing!' }`);
    }
    logProgress('');
  }

  // Calculate all ab distances
  // logProgress(`Finding shortest path between any 2 keys ...`);
  // const allKeys = [start, ...Object.values(keys)];
  // for (let i in allKeys) {
  //   for (let j in allKeys) {
  //     findDistance (labyrinth, allKeys[i], allKeys[j], { cache, keysRegexp, doorsRegExp, distanceCorrection });
  //   }
  //   if (flags.PROGRESS) {
  //     logProgress(`... done ${ ~~(100 * ((i * allKeys.length) + j) / (allKeys.length * allKeys.length)) / 100 }%`);
  //   }
  // }

  // Measure all possible key permutations
  if (flags.PROGRESS) {
    logProgress(`Finding shortest path between keys:`);
  }
  return measureKeySequences(labyrinth, start, keys, { cache, keysRegexp, doorsRegExp, palette, mapKeyToDoor, distanceCorrection });

};

// Explores the labyrinth
function explore (labyrinth, location, { keysRegexp, doorsRegExp, palette, path = {}, pathIsEmpty = true, allowMovementFn = null, onMovementFn = null, PROGRESS = flags.PROGRESS, INTERACTIVE = flags.INTERACTIVE } = {}) {

  // If logging progress, ready renderer
  const render = (PROGRESS && image.renderFieldFactory({ palette }));

  // Check if standing on key or door
  if (location.color.toString().match(keysRegexp)) {
    pathIsEmpty = false;
    if (onMovementFn) {
      onMovementFn({ key: true, location, path });
    }
  } else if (location.color.toString().match(doorsRegExp)) {
    pathIsEmpty = false;
    if (onMovementFn) {
      onMovementFn({ door: true, location, path });
    }
  }

  // Add current location to trail
  path[`${ location.coords.x }x${ location.coords.y }`] = { coords: location.coords, color: (location.color !== 2 ? location.color : 0) };

  // If logging progress, draw world
  if (PROGRESS) {
    logProgress([
      ...render(
        image.drawPointsAsImage([ ...Object.values(labyrinth), ...Object.values(path) ], { transparentColor: 0, palette })
      )
    ].join(''));
    // If interactive, pause for key presses
    if (INTERACTIVE) {
      const key = readKey('Press any key to continue, or "x" to quit ...');
      if (key === 'x') { process.exit(0); }
    }
  }

  // Move
  pathIsEmpty = sampleNeighbouringPoint(labyrinth, location, 0, -1, { keysRegexp, doorsRegExp, palette, path: {...path}, allowMovementFn, onMovementFn, PROGRESS, INTERACTIVE }) && pathIsEmpty;
  pathIsEmpty = sampleNeighbouringPoint(labyrinth, location, 0, +1, { keysRegexp, doorsRegExp, palette, path: {...path}, allowMovementFn, onMovementFn, PROGRESS, INTERACTIVE }) && pathIsEmpty;
  pathIsEmpty = sampleNeighbouringPoint(labyrinth, location, +1, 0, { keysRegexp, doorsRegExp, palette, path: {...path}, allowMovementFn, onMovementFn, PROGRESS, INTERACTIVE }) && pathIsEmpty;
  pathIsEmpty = sampleNeighbouringPoint(labyrinth, location, -1, 0, { keysRegexp, doorsRegExp, palette, path: {...path}, allowMovementFn, onMovementFn, PROGRESS, INTERACTIVE }) && pathIsEmpty;

  // If path and all subpaths are empty, run callback
  if (pathIsEmpty && onMovementFn) {
    onMovementFn({ empty: true, location, path });
  }

  // Return if path and all subpaths are empty
  return pathIsEmpty;

}

// Define neighbouring points sampling function
function sampleNeighbouringPoint (labyrinth, location, rx, ry, { keysRegexp, doorsRegExp, palette, path = {}, pathIsEmpty = true, allowMovementFn = null, onMovementFn = null, PROGRESS = flags.PROGRESS, INTERACTIVE = flags.INTERACTIVE } = {}) {
  // Check if direction is explorable
  const key = `${ location.coords.x + rx }x${ location.coords.y + ry }`;
  if ((path[key] === undefined) && (labyrinth[key] !== undefined) && (labyrinth[key].color !== 1)) {
    // Check if allowed to enter
    if ((!allowMovementFn || allowMovementFn(labyrinth[key]))) {
      // Keep exploring
      pathIsEmpty = explore(labyrinth, labyrinth[key], { keysRegexp, doorsRegExp, palette, path: {...path}, allowMovementFn, onMovementFn, PROGRESS, INTERACTIVE }) && pathIsEmpty;
    } else {
      // If not allowed, mark as not empty
      pathIsEmpty = false;
    }
  }
  // Return if path and all subpaths are empty
  return pathIsEmpty;
}

// Compose all possible permutations/sequences for collecting all keys and find the one with shortest distance
function measureKeySequences (labyrinth, start, keys, { distance = 0, minDistance = null, cache = {}, order = [], keysRegexp, doorsRegExp, palette, mapKeyToDoor, doors = [], distanceCorrection } = {}) {
  // Get keys not used yet
  const remainingKeys = Object.values(keys).filter(key => !order.find(c => (c === key.color)));
  if (remainingKeys.length) {
    // If remaining keys, continue generating permutations
    const accessibleKeys = remainingKeys.filter(key => !key.totalDoors.filter(d => (doors.indexOf(d) === -1)).length);
    for (let key of accessibleKeys) {
      // Calculate optimistic distance to all other points and make sure it is not already too long
      const optimisticTotalDistance = distance + findOptimisticDistance(labyrinth, (!order.length ? start : keys[order[order.length - 1]]), remainingKeys, { cache, keysRegexp, doorsRegExp, distanceCorrection, palette });
      if ((minDistance !== null) && (optimisticTotalDistance > minDistance)) { continue; }
      // Calculate distance to new point and make sure it is not already too long
      const nextDistance = distance + findDistance(labyrinth, (!order.length ? start : keys[order[order.length - 1]]), key, { cache, keysRegexp, doorsRegExp, distanceCorrection, palette });
      if ((minDistance !== null) && (nextDistance > minDistance)) { continue; }
      // Continue composing path
      minDistance = measureKeySequences(labyrinth, start, keys, { distance: nextDistance, minDistance, cache, order: [...order, key.color], keysRegexp, doorsRegExp, palette, mapKeyToDoor, doors: [...doors, mapKeyToDoor(key.color)], distanceCorrection});
    }
    return minDistance;
  } else {
    // If logging progress, prompt new minimum
    if (flags.PROGRESS && ((minDistance === null) || (distance < minDistance))) {
      logProgress(`Found new min distance: ${ distance } (${ order.join('') })`)
    }
    // If no more remaining keys, compare to current min distance
    return ((minDistance === null) || (distance < minDistance) ? distance : minDistance);
  }
}

// Finds shortest possible distance between current location and a number of other locations
function findOptimisticDistance (labyrinth, a, keys, { cache = {}, keysRegexp, doorsRegExp, distanceCorrection, palette } = {}) {
  // Initialize cache
  if (!cache.optimisticDistance) { cache.optimisticDistance = {}; }
  
  // Generate unique key
  const unique = `${ a.color }${ keys.map(k => k.color).sort().join('') }`;

  // Check cache
  if (cache.optimisticDistance[unique]) {

    // Return cached value
    return cache.optimisticDistance[unique]

  } else {

    // Calculate optimistic distance
    let remainingKeys = [...keys],
        optimisticDistance = 0;
    while (remainingKeys.length) {
      let minAbDistance = null,
          minAbDistanceKeyIndex = null
      for (let i = 0; i < remainingKeys.length; i++) {
        let d = findDistance(labyrinth, a, remainingKeys[i], { cache, keysRegexp, doorsRegExp, distanceCorrection, palette})
        if ((minAbDistance === null) || (d < minAbDistance)) {
          minAbDistance = d;
          minAbDistanceKeyIndex = i;
        }
      }
      a = remainingKeys[minAbDistanceKeyIndex];
      remainingKeys.splice(minAbDistanceKeyIndex, 1);
      optimisticDistance += minAbDistance;
    }

    // Cache and return distance
    return (cache.optimisticDistance[unique] = optimisticDistance);

  }
}

// Finds distance between 2 locations
function findDistance (labyrinth, a, b, { cache = {}, keysRegexp, doorsRegExp, distanceCorrection, palette } = {}) {
  // Check if same point
  if (a === b) { return 0; }
  
  // Initialize cache
  if (!cache.abDistance) { cache.abDistance = {}; }
  
  // Check cache
  if (cache.abDistance[`${ a.color }${ b.color }`] || cache.abDistance[`${ b.color }${ a.color }`]) {

    // Return cached value
    return cache.abDistance[`${ a.color }${ b.color }`] || cache.abDistance[`${ b.color }${ a.color }`];

  } else {

    // Calculate distance
    let abPath;
    explore(labyrinth, a, {
      keysRegexp,
      doorsRegExp,
      palette,
      allowMovementFn:  () => !abPath,
      onMovementFn:     ({ key, location, path }) => {
        // Check if done, and get path
        if (key && (location.color === b.color)) {
          abPath = path;
        }
      },
      PROGRESS: false,
      INTERACTIVE: false
    });

    // Check distance corrections
    const pathDistance = Object.keys(abPath).length + (distanceCorrection ? distanceCorrection(a, b) : 0) - 1;

    // Cache and return distance
    return (cache.abDistance[`${ a.color }${ b.color }`] = pathDistance);

  }
}
