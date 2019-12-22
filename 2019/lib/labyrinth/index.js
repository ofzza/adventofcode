// LABYRINTH SOLVER

// Import dependencies
const flags       = require('../../../lib').flags,
      logProgress = require('../../../lib').logProgress,
      readKey     = require('../../../lib').readKey,
      image       = require('../image');

// Defined palettes
module.exports.readPalette = {
  ' ': ' ',
  '#': '#',
  '.': '.',
  '@': '@',
  rail:     ':',
  portal:   '°',
  path:     '*',
  location: '!',
};
module.exports.defaultPalette = {
  ' ': '░',
  '#': '█',
  '.': ' ',
  '@': '@',
  [module.exports.readPalette.rail]:     module.exports.readPalette.rail,
  [module.exports.readPalette.portal]:   module.exports.readPalette.portal,
  [module.exports.readPalette.path]:     module.exports.readPalette.path,
  [module.exports.readPalette.location]: module.exports.readPalette.location
};

// Define default point types' selectors
module.exports.defaultTypes = { start: /@/, free: /[\.@\\:]/, wall: /[ #]/, door: /[A-Z]/, label: /[A-Z]/, portal: detectPortalType };

// Return a configured labyrinth file reader
module.exports.getReader = function fileReader (dirPath, { palette = module.exports.readPalette } = {}) {
  // Read labyrinth from file
  return (fileName) => {
    const filePath = require('path').join(dirPath, `./${ fileName }.txt`);
    return require('fs').readFileSync(filePath).toString().split('\n').reduce((points, l, y) => {
      return l.split('').reduce((points, c, x) => {
        points[`${ x }x${ y }`] = { coords: { x, y }, color: (palette[c] || c) };
        return points;
      }, points);
    }, {});
  };
};

// Analyzes a labyrinth and prepares it for solving
module.exports.analyze = function analyze (
  labyrinthPoints,
  {
    types           = module.exports.defaultTypes,
    fillInDeadEnds  = true,
    installRails    = true,
    connectPortals  = true,
    palette         = module.exports.defaultPalette
  } = {}
) {

  // If logging progress, ready renderer
  const render = (flags.PROGRESS && image.renderFieldFactory({ palette }));

  // Make a copy of labyrinth points
  const points = JSON.parse(JSON.stringify(labyrinthPoints));
  
  // Initialize types and keys
  for (let point of Object.values(points)) {
    if (!point.key) { point.key = pkey(point); }
    if (!point.type) { point.type = {}; }
  }

  // Fill in dead ends (modifying types, so needs to come before sorting)
  if (fillInDeadEnds) {
    module.exports.fillInDeadEnds(points, types);
  }

  // Install rails (modifying types, so needs to come before sorting)
  if (installRails) {
    module.exports.installRails(points, types);
  }

  // Sort points by color
  const byColor = {};
  for (let point of Object.values(points)) {
    if (!byColor[point.color]) { byColor[point.color] = []; }
    byColor[point.color].push(point);
  }

  // Sort points by type
  const byType = {}
  for (let type in types) {
    if (!byType[type]) { byType[type] = []; }
    for (let color in byColor) {
      // Check type instanceof ...
      if (types[type] instanceof Function) {

        // Check if each point of a color matches a type
        for (let point of byColor[color]) {
          let matched = matchType(color, types[type], { types, point, points });
          if (matched) {
            // Register point to matched type
            byType[type] = [...byType[type], point];
            // Set point's type info
            point.type[type] = matched;
          }
        }

      } else {

        // Check if color matches a type
        let matched = matchType(color, types[type]);
        if (matched) {
          // Register all points of same color to matched type
          byType[type] = [...byType[type], ...byColor[color]];
          // Set all points' type info
          for (let point of byColor[color]) {
            point.type[type] = matched;
          }
        }

      }
    }
  }
 
  // Connect portals
  if (connectPortals && byType['portal']) {
    module.exports.connectPortals(byType['portal']);
  }

  // If logging progress, prompt analyzed
  if (flags.PROGRESS) {
    // Render original
    logProgress([...render(image.drawPointsAsImage(Object.values(labyrinthPoints), { transparentColor: module.exports.readPalette[' '] }))].join(''));
    // Render processed
    logProgress([...render(image.drawPointsAsImage(Object.values(points),          { transparentColor: module.exports.readPalette[' '] }))].join(''));
    // Output found types
    logProgress(`Types: ${ Object.keys(byType).map((type) => `${ type } (${ byType[type].length })`).join(', ') }`);
    logProgress();
  }

  // Return analyzed labyrinth
  return { points, byColor, byType };

};

// Clears dead ends in a labyrinth
module.exports.fillInDeadEnds = function fillInDeadEnds (
  labyrinthPoints,
  types = module.exports.defaultTypes
) {

  // Close all dead-ends
  let foundDeadEnds;
  do {
    // Reset found flag
    foundDeadEnds = false;
    // Find dead ends
    for (let p of Object.values(labyrinthPoints)) {
      if (p.color.toString().match(types.free)) {
        let count = 0,
            nextPoints = [
              labyrinthPoints[pkey(p, 0, -1)],
              labyrinthPoints[pkey(p, 0, +1)],
              labyrinthPoints[pkey(p, -1, 0)],
              labyrinthPoints[pkey(p, +1, 0)]
            ];
        for (let nextPoint of nextPoints) {
          if (nextPoint && nextPoint.color.toString().match(types.wall)) { count++ }
        }
        // Fill in dead end
        if (count >= 3) {
        foundDeadEnds = true;
          // Update color
          p.color = module.exports.readPalette['#'];
        }
      }
    }
  } while (foundDeadEnds);

};

module.exports.installRails = function installRails (
  labyrinthPoints,
  types = module.exports.defaultTypes
) {

  // Find neighbours of a point not belonging to a current rail
  function findNeighbours (p, rail = []) {
    const neighbours = [],
          nextPoints = [
            labyrinthPoints[pkey(p, 0, -1)],
            labyrinthPoints[pkey(p, 0, +1)],
            labyrinthPoints[pkey(p, -1, 0)],
            labyrinthPoints[pkey(p, +1, 0)]
          ];
    for (let nextPoint of nextPoints) {
      if (nextPoint && (nextPoint.color.toString().match(types.free)) && (rail.indexOf(nextPoint) === -1)) {
        neighbours.push(nextPoint);
      }
    }
    return neighbours;
  }
  
  // Install rails
  let foundRailLocation;
  do {
    // Reset found flag
    foundRailLocation = false;
    // Find a free location for a rail
    for (let p of Object.values(labyrinthPoints)) {
      if (!p.type.rail && p.color.toString().match(types.free)) {
        let neighbours = findNeighbours(p);
        if ((neighbours.length === 2) && (findNeighbours(neighbours[0]).length === 2) && (findNeighbours(neighbours[1]).length === 2)) {
          foundRailLocation = {
            seeds: [p],
            endpoints:  [neighbours[0], neighbours[1]]
          };
          break;
        }
      }
    }
    // Install rail
    if (foundRailLocation) {
      // Find rail endpoints
      foundRailLocation.seeds[0].color = module.exports.readPalette.rail;
      foundRailLocation.seeds[0].type.rail = true;
      let expanding = false;
      do {
        // Look for endpoints
        expanding = false;
        for (let i in foundRailLocation.endpoints) {
          let endpointNeighbours = findNeighbours(foundRailLocation.endpoints[i], foundRailLocation.seeds);
          if (endpointNeighbours.length === 1) {
            expanding = true;
            foundRailLocation.endpoints[i].color = module.exports.readPalette.rail;
            foundRailLocation.endpoints[i].type.rail = true;
            foundRailLocation.seeds.push(foundRailLocation.endpoints[i]);
            foundRailLocation.endpoints.splice(i, 1, endpointNeighbours[0]);
          }
        }
        // When found endpoints, connect them togeather
        if (!expanding) {
          let railPointA = findNeighbours(foundRailLocation.endpoints[0]).find(p => (foundRailLocation.seeds.indexOf(p) !== -1)),
              railPointB = findNeighbours(foundRailLocation.endpoints[1]).find(p => (foundRailLocation.seeds.indexOf(p) !== -1));
          // railPointA.color = '!';
          railPointA.type.rail = {
            from:   foundRailLocation.endpoints[0].key,
            to:     railPointB.key,
            points: foundRailLocation.seeds.map(p => p.key)
          };
          // railPointB.color = '!';
          railPointB.type.rail = {
            from:   foundRailLocation.endpoints[1].key,
            to:     railPointA.key,
            points: foundRailLocation.seeds.map(p => p.key)
          };
        }
      } while (expanding);
    }
  } while (foundRailLocation);

};

// Connect pairs of portals
module.exports.connectPortals = function connectPortals (labyrinthPoints) {
  for (let source of Object.values(labyrinthPoints)) {
    if (!source.type['portal'].target) {
      for (let target of Object.values(labyrinthPoints)) {
        if ((source !== target) && !target.type['portal'].target && (source.type['portal'].key === target.type['portal'].key)) {
          source.color = module.exports.readPalette.portal;
          source.type['portal'].target = target.key;
          target.color = module.exports.readPalette.portal;
          target.type['portal'].target = source.key;
        }
      }
    }
  }
};

// Free roam and explore all paths available
module.exports.freeRoam = function * freeRoam (
  point,
  analysis,
  types = module.exports.defaultTypes,
  {
    beforeYield = null,
    allowMove   = (point) => (point.type && !point.type.wall && !point.type.door && !point.type.label),
  } = {},
  {
    palette     = module.exports.defaultPalette,
    pathLevel   = 0,
    pathPoints  = {}
  } = {}
) {

  // Render current state
  function renderProgress (level) {
    // Render progress
    const render = (flags.PROGRESS && image.renderFieldFactory({ palette }));
    if (flags.PROGRESS) {
      logProgress(`Level: ${ level || pathLevel }`);
      logProgress([...render(image.drawPointsAsImage([
        ...Object.values(analysis.points),
        ...Object.values(pathPoints[level || pathLevel]).map(p => colorPoint(p, module.exports.readPalette.path)),
        ...(pathLevel === (level || pathLevel) ? [point].map(p => colorPoint(p, module.exports.readPalette.location)) : [])
      ], { transparentColor: module.exports.readPalette[' '] }))].join(''));
      // If interactive, pause for key presses
      if (flags.INTERACTIVE) {
        const key = readKey('Press any key to continue, or "x" to quit ...');
        if (key === 'x') { process.exit(0); }
      }
    }  
  }

  // Before yield callback calling function
  function callBeforeYield (point, { portaled = false } = {}) {
    // Check if callback defined
    if (beforeYield) {
      // Initialize event
      const e = {
        point,
        pathLevel,
        pathPoints,
        renderProgress
      };
      // Execute callback
      beforeYield(e, { portaled })
      // Ingest changed event values      
      if (!pathPoints[pathLevel = e.pathLevel]) { pathPoints[pathLevel] =  {}; }
    }
    // Return point to yield
    return { point, level: pathLevel };
  }

  // Initialize path level
  if (!pathPoints[pathLevel]) { pathPoints[pathLevel] =  {}; }

  // Check if point is a portal
  let portaled = false;
  if (point.type.portal && point.type.portal.target) {
    // Add next point to path, yield it and teleport
    yield callBeforeYield(pathPoints[pathLevel][point.key] = point);
    point = analysis.points[point.type.portal.target];
    portaled = true;
    // Render progress
    renderProgress();
  }

  // Check if point is a rail
  if (point.type.rail) {
    // Add all rail points and proceed to end of rail
    for (let p of point.type.rail.points.map(key => analysis.points[key])) {
      pathPoints[pathLevel][p.key] = p;
    }
    point = analysis.points[point.type.rail.to];
    // Render progress
    renderProgress();
  }

  // Add current location to path
  pathPoints[pathLevel][point.key] = point;

  // Yield current location
  yield callBeforeYield(point, { portaled });
  // Render progress
  renderProgress();

  // Check options to move
  const nextPoints = [
    analysis.points[pkey(point, 0, -1)],
    analysis.points[pkey(point, 0, +1)],
    analysis.points[pkey(point, +1, 0)],
    analysis.points[pkey(point, -1, 0)]
  ];
  for (let nextPoint of nextPoints) {
    if (nextPoint && !pathPoints[pathLevel][nextPoint.key] && allowMove(nextPoint, pathLevel, pathPoints)) {
      // Continue roaming from next point
      let pathPointsCopy = Object.keys(pathPoints).reduce((pathPointsCopy, level) => {
            pathPointsCopy[level] = Object.keys(pathPoints[level]).reduce((pathPointsLevelCopy, key) => {
              pathPointsLevelCopy[key] = pathPoints[level][key];
              return pathPointsLevelCopy;
            }, {});
            return pathPointsCopy;
          }, {}),
          roam = freeRoam(nextPoint, analysis, types, { beforeYield, allowMove }, { pathLevel, pathPoints: pathPointsCopy }),
          result;
      while (!(result = roam.next()).done) {
        yield result.value;
      }      
    }
  }

};

// Finds a path between 2 points
module.exports.findPath = function (
  startPoint,
  endPoint,
  analysis,
  types = module.exports.defaultTypes,
  {
    beforeYield = null,
    allowMove   = (point) => (point.type && !point.type.wall),
  } = {},
  {
    palette = module.exports.defaultPalette
  } = {}
) {
  // Explore until finding the end
  let path,
      explore = module.exports.freeRoam(startPoint, analysis, types, {
        beforeYield: (e) => {
          // Run callback if set
          if (beforeYield) { beforeYield(e); }
          // Store path
          path = e.pathPoints;
        },
        allowMove
      }),
      result;
  while (!(result = explore.next()).done) {
    if ((result.value.point === endPoint) && (result.value.level === 0)) {

      // Compose full path
      let fullPath = Object.keys(path).reduce((fullPath, level) => {
        return [...fullPath, ...Object.values(path[level]).map(point => {
          return { level, point };
        })];
      }, []);

      // Render progress
      const render = (flags.PROGRESS && image.renderFieldFactory({ palette }));
      if (flags.PROGRESS) {
        logProgress([...render(image.drawPointsAsImage([
          ...Object.values(analysis.points),
          ...Object.values(path[0]).map(p => colorPoint(p, module.exports.readPalette.path)),
          ...[result.value.point].map(p => colorPoint(p, module.exports.readPalette.location))
        ], { transparentColor: module.exports.readPalette[' '] }))].join(''));
      }  
      logProgress(`Path length: ${ fullPath.length - 1 }`);
      logProgress();

      // Return full path
      return fullPath;
    }
  }

};

// Finds shortest path between 2 points
module.exports.findShortestPath = function (
  startPoint,
  endPoint,
  analysis,
  types = module.exports.defaultTypes,
  {
    beforeYield = null,
    allowMove   = (point) => (point.type && !point.type.wall),
  } = {},
  {
    palette = module.exports.defaultPalette
  } = {}
) {
  // Explore until explored everything
  let minPath = null,
      explore = module.exports.freeRoam(startPoint, analysis, types, {
        beforeYield: (e, { portaled = false } = {}) => {
          // Run callback if set
          if (beforeYield) { beforeYield(e, { portaled }); }
          // Store path
          path = e.pathPoints;
        },
        allowMove
      }),
      result;
  while (!(result = explore.next()).done) {
    if ((result.value.point === endPoint) && (result.value.level === 0)) {

      // Compose full path
      let fullPath = Object.keys(path).reduce((fullPath, level) => {
        return [...fullPath, ...Object.values(path[level]).map(point => {
          return { level, point };
        })];
      }, []);

      // Check if path shorter than current shortest
      if ((minPath === null) || (minPath.length > fullPath.length)) {
        minPath = fullPath;
      }

      // Render progress
      const render = (flags.PROGRESS && image.renderFieldFactory({ palette }));
      if (flags.PROGRESS) {
        for (let level in path) {
          logProgress(`Level: ${ level }`);
          logProgress([...render(image.drawPointsAsImage([
            ...Object.values(analysis.points),
            ...Object.values(path[level]).map(p => colorPoint(p, module.exports.readPalette.path)),
            ...(level === '0' ? [result.value.point].map(p => colorPoint(p, module.exports.readPalette.location)) : [])
          ], { transparentColor: module.exports.readPalette[' '] }))].join(''));
        }
        logProgress(`Path length: ${ fullPath.length - 1 } / ${ minPath.length - 1 }`);
        logProgress();
      }  

    }
  }
  // Return shortest found path
  return minPath;
};

// Point key generator
const pkey = (p, dx = 0, dy = 0) => (`${ p.coords.x + dx }x${ p .coords.y + dy }`);
// Returns a colored copy of a point
const colorPoint = (p, color) => { return { color, coords: p.coords, type: p.type }; };

// Checks if color matches a type
function matchType (color, type, options = {}) {
  if (type instanceof RegExp) {
    return !!color.toString().match(type);
  } else if (type instanceof Function) {
    return type(color.toString(), options);
  } else {
    return !!(type === color.toString());
  }
};

// Portal type detection function
function detectPortalType (color, { types, point, points } = {}) {
  // Find all portals
  if (point.color.toString().match(types.free)) {
    // Check neighbouring points for portal markings
    let matched,
        location;
    if (points[pkey(point, 0, -1)] && points[pkey(point, 0, -1)].color.toString().match(/[A-Z]/)
     && points[pkey(point, 0, -2)] && points[pkey(point, 0, -2)].color.toString().match(/[A-Z]/)) {
      matched = (points[pkey(point, 0, -2)] && points[pkey(point, 0, -2)].color)
              + (points[pkey(point, 0, -1)] && points[pkey(point, 0, -1)].color);
      location = (points[pkey(point, 0, -3)] === undefined ? 'outside' : 'inside');
    }
    if (points[pkey(point, 0, +1)] && points[pkey(point, 0, +1)].color.toString().match(/[A-Z]/)
     && points[pkey(point, 0, +2)] && points[pkey(point, 0, +2)].color.toString().match(/[A-Z]/)) {
      matched = (points[pkey(point, 0, +1)] && points[pkey(point, 0, +1)].color)
              + (points[pkey(point, 0, +2)] && points[pkey(point, 0, +2)].color);
      location = (points[pkey(point, 0, +3)] === undefined ? 'outside' : 'inside');
    }
    if (points[pkey(point, -1, 0)] && points[pkey(point, -1, 0)].color.toString().match(/[A-Z]/)
     && points[pkey(point, -2, 0)] && points[pkey(point, -2, 0)].color.toString().match(/[A-Z]/)) {
      matched = (points[pkey(point, -2, 0)] && points[pkey(point, -2, 0)].color)
              + (points[pkey(point, -1, 0)] && points[pkey(point, -1, 0)].color);
      location = (points[pkey(point, -3, 0)] === undefined ? 'outside' : 'inside');
    }
    if (points[pkey(point, +1, 0)] && points[pkey(point, +1, 0)].color.toString().match(/[A-Z]/)
     && points[pkey(point, +2, 0)] && points[pkey(point, +2, 0)].color.toString().match(/[A-Z]/)) {
      matched = (points[pkey(point, +1, 0)] && points[pkey(point, +1, 0)].color)
              + (points[pkey(point, +2, 0)] && points[pkey(point, +2, 0)].color);
      location = (points[pkey(point, +3, 0)] === undefined ? 'outside' : 'inside');
    }
    // Return portal matched
    return (matched ? { key: matched, location } : false);
  }
};
