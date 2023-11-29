// LABYRINTH SOLVER

// Import dependencies
const flags       = require('../../../../lib').flags,
      logProgress = require('../../../../lib').logProgress,
      readKey     = require('../../../../lib').readKey,
      image       = require('../../image');

// Find shortest path to collect all keys in a tree-like labyrinth
module.exports.findShortestABPath = function findShortestABPath (
  labyrinth,
  aKey,
  bKey,
  {
    keysRegexp            = /[a-z]/,
    doorsRegExp           = /[A-Z]/,
    mapKeyToDoor          = (key) => key.toUpperCase(),
    portalsRegExp         = /\!/,
    mapPortalToKey        = () => null,
    distanceCorrection,
    closeOffDeadEnds      = true,
    questionableShortcuts = false,
    palette               = {...image.defaultPalette}
  } = {}
) {

  // If logging progress, ready renderer
  const render = (flags.PROGRESS && image.renderFieldFactory({ palette }));

  // Initialize shared cache  
  const cache = {};

  // Analyze labyrinth
  const { points, keys } = analyze(labyrinth, {
    startChar: aKey,
    keysRegexp,
    doorsRegExp,
    mapKeyToDoor,
    portalsRegExp,
    mapPortalToKey,
    distanceCorrection,
    closeOffDeadEnds,
    questionableShortcuts,
    palette,
    render,
    cache
  });

  // Measure all possible key permutations
  if (flags.PROGRESS) {
    logProgress(`Finding shortest path:`);
  }
  let distance = null;
  explore (
    labyrinth,
    keys[aKey],
    {
      keysRegexp,
      doorsRegExp,
      palette,
      allowMovementFn:  (location) => {
        return (location.color === 2 || location.color.toString().match(keysRegexp));
      },
      onMovementFn:     ({ location, path }) => {
        // Check for destination
        if (location.color === bKey) {
          // Get distance
          const d = Object.values(path).length;
          // If logging progress, prompt new minimum
          if (flags.PROGRESS && ((distance === null) || (d < distance))) {
            logProgress(`Found new min distance: ${ d }`)
          }
          // Compare to current shortest distance
          distance = ((distance === null) || (d < distance) ? d : distance);
          return;
        }
        // Check for portal
         if (location.portal) {
          const target = points.find(p => ((p !== location) && (p.portal === location.portal)));
          if (target) {
            path[`${ location.coords.x }x${ location.coords.y }`] = location;
            return target;
          }
        }
      },
      PROGRESS:         false,
      INTERACTIVE:      false
    }
  );

  return (distance || -1);
};

// Find shortest path to collect all keys in a tree-like labyrinth
module.exports.collectAllKeys = function collectAllKeys (
  labyrinth,
  {
    startChar             = '@',
    keysRegexp            = /[a-z]/,
    doorsRegExp           = /[A-Z]/,
    mapKeyToDoor          = (key) => key.toUpperCase(),
    portalsRegExp         = /\!/,
    mapPortalToKey        = () => null,
    distanceCorrection,
    closeOffDeadEnds      = true,
    questionableShortcuts = false,
    palette               = {...image.defaultPalette}
  } = {}
) {

  // If logging progress, ready renderer
  const render = (flags.PROGRESS && image.renderFieldFactory({ palette }));

  // Initialize shared cache  
  const cache = {};

  // Analyze labyrinth
  const { start, keys } = analyze(labyrinth, {
    startChar,
    keysRegexp,
    doorsRegExp,
    mapKeyToDoor,
    portalsRegExp,
    mapPortalToKey,
    distanceCorrection,
    closeOffDeadEnds,
    questionableShortcuts,
    palette,
    render,
    cache
  });

  // Measure all possible key permutations
  if (flags.PROGRESS) {
    logProgress(`Finding shortest path between keys:`);
  }
  return measureKeySequences(labyrinth, start, keys, {
    cache,
    keysRegexp,
    doorsRegExp,
    portalsRegExp,
    palette,
    mapKeyToDoor,
    distanceCorrection,
    questionableShortcuts
  });

};

// Analyzes a labyrinth
function analyze (
  labyrinth,
  {
    startChar,
    keysRegexp            = /\!/,
    doorsRegExp           = /\!/,
    mapKeyToDoor          = (key) => key.toUpperCase(),
    portalsRegExp         = /\!/,
    mapPortalToKey        = () => null,
    closeOffDeadEnds      = true,
    palette               = {...image.defaultPalette},
    render
  } = {}
) {

  // Find relevant objects
  const points  = Object.values(labyrinth),
        keys    = points.filter(p => p.color.toString().match(keysRegexp))
                    .reduce((keys, p) => {
                      keys[p.color] = p;
                      p.doors = [];
                      return keys;
                    }, {}),
        doors   = points.filter(p => p.color.toString().match(doorsRegExp))
                    .reduce((keys, p) => {
                      keys[p.color] = p;
                      return keys;
                    }, {}),
        portals = [];

  // Find all portals
  for (let point of points) {
    if (point.color === 2) {
      // Check neighbouring points for portal markings
      if (labyrinth[`${ point.coords.x }x${ point.coords.y - 1 }`] && labyrinth[`${ point.coords.x }x${ point.coords.y - 1 }`].color.toString().match(portalsRegExp)) {
        point.portal  = (labyrinth[`${ point.coords.x }x${ point.coords.y - 2 }`] && labyrinth[`${ point.coords.x }x${ point.coords.y - 2 }`].color)
                      + (labyrinth[`${ point.coords.x }x${ point.coords.y - 1 }`] && labyrinth[`${ point.coords.x }x${ point.coords.y - 1 }`].color);
      }
      if (labyrinth[`${ point.coords.x }x${ point.coords.y + 1 }`] && labyrinth[`${ point.coords.x }x${ point.coords.y + 1 }`].color.toString().match(portalsRegExp)) {
        point.portal  = (labyrinth[`${ point.coords.x }x${ point.coords.y + 1 }`] && labyrinth[`${ point.coords.x }x${ point.coords.y + 1 }`].color)
                      + (labyrinth[`${ point.coords.x }x${ point.coords.y + 2 }`] && labyrinth[`${ point.coords.x }x${ point.coords.y + 2 }`].color);
      }
      if (labyrinth[`${ point.coords.x - 1 }x${ point.coords.y }`] && labyrinth[`${ point.coords.x - 1 }x${ point.coords.y }`].color.toString().match(portalsRegExp)) {
        point.portal  = (labyrinth[`${ point.coords.x - 2 }x${ point.coords.y }`] && labyrinth[`${ point.coords.x - 2 }x${ point.coords.y }`].color)
                      + (labyrinth[`${ point.coords.x - 1 }x${ point.coords.y }`] && labyrinth[`${ point.coords.x - 1 }x${ point.coords.y }`].color);
      }
      if (labyrinth[`${ point.coords.x + 1}x${ point.coords.y }`] && labyrinth[`${ point.coords.x + 1}x${ point.coords.y }`].color.toString().match(portalsRegExp)) {
        point.portal  = (labyrinth[`${ point.coords.x + 1 }x${ point.coords.y }`] && labyrinth[`${ point.coords.x + 1 }x${ point.coords.y }`].color)
                      + (labyrinth[`${ point.coords.x + 2 }x${ point.coords.y }`] && labyrinth[`${ point.coords.x + 2 }x${ point.coords.y }`].color);
      }
      // Register portals and map portal to key
      if (point.portal) {
        // Register portal
        portals.push(point);
        // Map portal to key
        const key = mapPortalToKey(point.portal);
        if (key) {
          point.color = key;
          keys[key] = point;
          keys[key].doors = [];
        }
      }
    }
  }

  // Find start position
  const start = points.filter(p => (p.color === startChar));

  // Add starting position, keys and doors to palette
  palette[9] = '.';
  palette[startChar] = startChar;
  for (let p of [...Object.values(keys), ...Object.values(doors), ...Object.values(portals)]) {
    if (!palette[p.color]) {
      palette[p.color] = p.color;
    }
  }

  // Draw initial map state
  if (flags.PROGRESS) {
    logProgress([...render(image.drawPointsAsImage(points, { transparentColor: 2, palette }))].join(''));
  }

  // Close all dead-ends
  if (closeOffDeadEnds) {
    let foundDeadEnds = false;
    do {
      // Reset found flag
      foundDeadEnds = false;
      for (let p of points) {
        if (p.color === 2) {
          let count = 0;
          if (labyrinth[`${ p.coords.x }x${ p .coords.y - 1 }`] && labyrinth[`${ p.coords.x }x${ p .coords.y - 1 }`].color === 1) { count++ }
          if (labyrinth[`${ p.coords.x }x${ p .coords.y + 1 }`] && labyrinth[`${ p.coords.x }x${ p .coords.y + 1 }`].color === 1) { count++ }
          if (labyrinth[`${ p.coords.x - 1 }x${ p .coords.y }`] && labyrinth[`${ p.coords.x - 1 }x${ p .coords.y }`].color === 1) { count++ }
          if (labyrinth[`${ p.coords.x + 1 }x${ p .coords.y }`] && labyrinth[`${ p.coords.x + 1 }x${ p .coords.y }`].color === 1) { count++ }
          if (count >= 3) {
            p.color = 1;
            foundDeadEnds = true;
          }
        }
      }
    } while (foundDeadEnds);
    if (flags.PROGRESS) {
      logProgress([...render(image.drawPointsAsImage(points, { transparentColor: 0, palette }))].join(''));
    }
  }

  // Collect all keys locations and doors they are behind
  start.forEach((start, i) => {
    explore(labyrinth, start, {
      keysRegexp,
      doorsRegExp,
      portalsRegExp,
      palette,
      allowMovementFn:  () => true,
      onMovementFn:     ({ key, location, path }) => {
        if (key) {
          // Set which start was arrived from
          keys[location.color].startIndex = i;
          // Get doors (and keys as "automatic" doors) on the way
          keys[location.color].doors = Object.values(path)
            .filter((p) => (p.color.toString().match(doorsRegExp) || p.color.toString().match(keysRegexp)))
            .map((p) => (p.color.toString().match(doorsRegExp) ? p.color : mapKeyToDoor(p.color)));
        }
      },
      PROGRESS: false,
      INTERACTIVE: false
    });
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

  // Return analyzed data
  return { start, points, keys, doors, portals };

};

// Explores the labyrinth
function explore (
  labyrinth,
  location,
  {
    keysRegexp,
    doorsRegExp,
    portalsRegExp,
    palette,
    path = {},
    pathIsEmpty       = true,
    allowMovementFn   = null,
    onMovementFn      = null,
    PROGRESS          = flags.PROGRESS,
    INTERACTIVE       = flags.INTERACTIVE
  } = {}
) {

  // If logging progress, ready renderer
  const render = (PROGRESS && image.renderFieldFactory({ palette }));

  // Check if standing on key or door
  if (location.color.toString().match(keysRegexp)) {
    pathIsEmpty = false;
    if (onMovementFn) {
      location = onMovementFn({ key: true, location, path }) || location;
    }
  } else if (location.color.toString().match(doorsRegExp)) {
    pathIsEmpty = false;
    if (onMovementFn) {
      location = onMovementFn({ door: true, location, path }) || location;
    }
  } else if (location.portal) {
    if (onMovementFn) {
      location = onMovementFn({ portal: true, location, path }) || location;
    }
  } else {
    if (onMovementFn) {
      location = onMovementFn({ location, path }) || location;
    }
  }

  // Add current location to trail
  path[`${ location.coords.x }x${ location.coords.y }`] = { coords: location.coords, color: location.color };

  // If logging progress, draw world
  if (PROGRESS) {
    logProgress([
      ...render(
        image.drawPointsAsImage([ ...Object.values(labyrinth), ...Object.values(path).map(p => { return { coords: p.coords, color: 9 }}) ], { transparentColor: 0, palette })
      )
    ].join(''));
    // If interactive, pause for key presses
    if (INTERACTIVE) {
      const key = readKey('Press any key to continue, or "x" to quit ...');
      if (key === 'x') { process.exit(0); }
    }
  }

  // Move
  pathIsEmpty = sampleNeighbouringPoint(labyrinth, location, 0, -1, { keysRegexp, doorsRegExp, portalsRegExp, palette, path: {...path}, allowMovementFn, onMovementFn, PROGRESS, INTERACTIVE }) && pathIsEmpty;
  pathIsEmpty = sampleNeighbouringPoint(labyrinth, location, 0, +1, { keysRegexp, doorsRegExp, portalsRegExp, palette, path: {...path}, allowMovementFn, onMovementFn, PROGRESS, INTERACTIVE }) && pathIsEmpty;
  pathIsEmpty = sampleNeighbouringPoint(labyrinth, location, +1, 0, { keysRegexp, doorsRegExp, portalsRegExp, palette, path: {...path}, allowMovementFn, onMovementFn, PROGRESS, INTERACTIVE }) && pathIsEmpty;
  pathIsEmpty = sampleNeighbouringPoint(labyrinth, location, -1, 0, { keysRegexp, doorsRegExp, portalsRegExp, palette, path: {...path}, allowMovementFn, onMovementFn, PROGRESS, INTERACTIVE }) && pathIsEmpty;

  // If path and all subpaths are empty, run callback
  if (pathIsEmpty && onMovementFn) {
    onMovementFn({ empty: true, location, path });
  }

  // Return if path and all subpaths are empty
  return pathIsEmpty;

}

// Define neighbouring points sampling function
function sampleNeighbouringPoint (
  labyrinth,
  location,
  rx,
  ry,
  {
    keysRegexp,
    doorsRegExp,
    portalsRegExp,
    palette,
    path = {},
    pathIsEmpty = true,
    allowMovementFn = null,
    onMovementFn = null,
    PROGRESS = flags.PROGRESS,
    INTERACTIVE = flags.INTERACTIVE
  } = {}
) {
  // Check if direction is explorable
  const key = `${ location.coords.x + rx }x${ location.coords.y + ry }`;
  if ((path[key] === undefined) && (labyrinth[key] !== undefined) && (labyrinth[key].color !== 1)) {
    // Check if allowed to enter
    if ((!allowMovementFn || allowMovementFn(labyrinth[key]))) {
      // Keep exploring
      pathIsEmpty = explore(labyrinth, labyrinth[key], { keysRegexp, doorsRegExp, portalsRegExp, palette, path: {...path}, allowMovementFn, onMovementFn, PROGRESS, INTERACTIVE }) && pathIsEmpty;
    } else {
      // If not allowed, mark as not empty
      pathIsEmpty = false;
    }
  }
  // Return if path and all subpaths are empty
  return pathIsEmpty;
}

// Compose all possible permutations/sequences for collecting all keys and find the one with shortest distance
function measureKeySequences (
  labyrinth,
  start,
  keys,
  {
    cache = {},
    keysRegexp,
    doorsRegExp,
    portalsRegExp,
    mapKeyToDoor,
    distanceCorrection,
    questionableShortcuts,
    palette,
    distance = 0,
    minDistance = null,
    order = [],
    doors = []
  } = {}
) {
  // Get keys not used yet
  const remainingKeys = Object.values(keys).filter(key => !order.find(c => (c === key.color)));
  if (remainingKeys.length) {
    // If remaining keys, continue generating permutations
    const accessibleKeys = remainingKeys.filter(key => !key.totalDoors.filter(d => (doors.indexOf(d) === -1)).length);
    for (let key of accessibleKeys) {
      // Calculate optimistic distance to all other points and make sure it is not already too long
      if (questionableShortcuts) {
        let optimisticDistance = distance,
            optimisticDistances = {};
        start.forEach((start, i) => {
          const orderFilteredByStart = order.filter(k => (keys[k].startIndex === i)),
                remainingKeysByStart = remainingKeys.filter(key => (key.startIndex === i)),
                a = (!orderFilteredByStart.length ? start : keys[orderFilteredByStart[orderFilteredByStart.length - 1]]);
          optimisticDistances[i] = findOptimisticDistance(labyrinth, a, remainingKeysByStart, {
            cache,
            keysRegexp,
            doorsRegExp,
            portalsRegExp,
            distanceCorrection,
            palette
          });
          optimisticDistance += optimisticDistances[i];
        });
        if ((minDistance !== null) && (optimisticDistance > minDistance)) { continue; }
      }
      // Calculate distance to new point and make sure it is not already too long
      const orderFilteredByStart = order.filter(k => (keys[k].startIndex === key.startIndex)),
            a = (!orderFilteredByStart.length ? start[key.startIndex] : keys[orderFilteredByStart[orderFilteredByStart.length - 1]]);
      const d = findDistance(labyrinth, a, key, {
              cache,
              keysRegexp,
              doorsRegExp,
              portalsRegExp,
              distanceCorrection,
              palette
            }),
            nextDistance = distance + d;
      if ((minDistance !== null) && (nextDistance > minDistance)) { continue; }
      // Continue composing path
      minDistance = measureKeySequences(labyrinth, start, keys, {
        cache,
        keysRegexp,
        doorsRegExp,
        portalsRegExp,
        mapKeyToDoor,
        distanceCorrection,
        questionableShortcuts,
        palette,
        distance: nextDistance,
        minDistance,
        order: [...order, key.color],
        doors: [...doors, mapKeyToDoor(key.color)]
      });
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
function findOptimisticDistance (
  labyrinth,
  a,
  keys,
  {
    cache = {},
    keysRegexp,
    doorsRegExp,
    portalsRegExp,
    distanceCorrection,
    palette
  } = {}
) {
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
        let d = findDistance(labyrinth, a, remainingKeys[i], {
          cache,
          keysRegexp,
          doorsRegExp,
          portalsRegExp,
          distanceCorrection,
          palette
        });
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
function findDistance (
  labyrinth,
  a,
  b,
  {
    cache = {},
    keysRegexp,
    doorsRegExp,
    portalsRegExp,
    distanceCorrection,
    palette
  } = {}
) {
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
      portalsRegExp,
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
