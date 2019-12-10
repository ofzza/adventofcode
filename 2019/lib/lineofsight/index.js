// LINE-OF-SIGHT PROCESSING

// Get coordinates of all occupied point on the map
module.exports.getOccupiedCoords = function * getOccupiedCoords (map) {
  // Check all coordinates
  for (let y = 0; y < map.length; y++) {
    for (let x = 0; x < map[y].length; x++) {
      if (map[y][x] !== '.') {
        // Yield occupied coordinate
        yield { x, y };
      }
    }
  }
};

// Find occupied with most occupied points in unbroken line-of-siht
module.exports.getBestVantagePoint = function getBestVantagePoint (map) {
  // Get occupied points
  const occupied = [...module.exports.getOccupiedCoords(map)];
  // Initialize variables
  let coords = null, max = 0;
  // Analyse all points and find unique angles
  for (let p of occupied) {
    const analysis = [...module.exports.analyseFromVantagePoint(map, p, occupied)],
          angles = Object.values(analysis).reduce((angles, point) => {
            angles[point.angleCwDeg] = true;
            return angles;
          }, {}),
          visible = Object.keys(angles).length;
    if (visible > max) {
      coords = p;
      max = visible;
    }
  }
  // Return best vantage point
  return { point: coords, visible: max };
}

// Enumerates all occupied points from a vantage point
module.exports.enumerateFromVantagePoint = function * enumerateFromVantagePoint (map, vp, occupied = null) {
  // If not provided, find occupied points
  if (!occupied) { occupied = [...module.exports.getOccupiedCoords(map)]; }
  // Analyse from vantage point and find unique angles
  const analysis = [...module.exports.analyseFromVantagePoint(map, vp, occupied)],
        pointsByAngle = Object.values(analysis).reduce((angles, point) => {
          if (!angles[point.angleCwDeg]) { angles[point.angleCwDeg] = []; };
          angles[point.angleCwDeg].push(point);
          return angles;
        }, {});
  // Sort each unique angle by distance
  for (let a in pointsByAngle) {
    pointsByAngle[a] = pointsByAngle[a].sort((a, b) => {
      if (a.distance < b.distance) {
        return -1
      } else if (a.distance > b.distance) {
        return +1;
      } else {
        return 0;
      }
    });
  }
  // Sort angles
  const angles = Object.keys(pointsByAngle).map(a => parseFloat(a)).sort((a, b) => {
    if (a < b) {
      return -1
    } else if (a > b) {
      return +1;
    } else {
      return 0;
    }
  });
  // Circle angles and enumerate
  let count = 0;
  for (let i=0; count < (occupied.length - 1); i++) {
    points = pointsByAngle[angles[i % angles.length]];
    if (points.length) {
      count++;
      yield points.splice(0, 1)[0];
    }
  }
};

// Gets analysis for all occupied points from a given vantage point
module.exports.analyseFromVantagePoint = function * analyseFromVantagePoint (map, vp, occupied = null) {
  // If not provided, find occupied points
  if (!occupied) { occupied = [...module.exports.getOccupiedCoords(map)]; }
  // Analyse occupied points
  for (let p of occupied) {
    if ((p.x !== vp.x) || (p.y !== vp.y)) {
      // Calculate angle
      let angleRad;
      if (p.x === vp.x) {
        angleRad = (p.y < vp.y ? 0.5 * Math.PI : 1.5 * Math.PI);
      } else if (p.y === vp.y) {
        angleRad = (p.x > vp.x ? 0 : Math.PI);
      } else {
        angleRad = Math.atan(Math.abs(p.y - vp.y) / Math.abs(p.x - vp.x));
        if (p.y < vp.y) {
          if (p.x > vp.x) {
            // 1st quadrant
            angleRad = (angleRad >= 0 ? angleRad : (angleRad + (2 * Math.PI)));
          } else {
            // 2nd quadrant
            angleRad = Math.PI - (angleRad >= 0 ? angleRad : (angleRad + (2 * Math.PI)));
          }
        } else {
          if (p.x < vp.x) {
            // 3rd quadrant
            angleRad = Math.PI + (angleRad >= 0 ? angleRad : (angleRad + (2 * Math.PI)));
          } else {
            // 4th quadrant
            angleRad = (2 * Math.PI) - (angleRad >= 0 ? angleRad : (angleRad + (2 * Math.PI)));
          }
        }
      }
      const angleCwRad = radToClockwiseRad(angleRad),
            angleCwDeg = radToDeg(angleCwRad);
      // Yield point
      yield {
        x: p.x,
        y: p.y,
        angleRad,
        angleCwRad,
        angleCwDeg,
        distance: Math.sqrt(Math.pow((p.x - vp.x), 2) + Math.pow((p.y - vp.y), 2))
      };
    }
  }
};

// Converts from math coordinate plain angle (in radians) into a clockwise angle (in radians)
function radToClockwiseRad (angle) {
  return (0.5 * Math.PI - angle) + (angle > (0.5 * Math.PI) ? (2 * Math.PI) : 0);
}

// Converts from radians into degrees
function radToDeg (angle) {
  return (180 / Math.PI) * angle;
}
