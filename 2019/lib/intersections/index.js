// INTERSECTION DETECTION

// Decode instructions into movement
function decodeInstruction (code) {
  if (code[0] === 'U') {
      return { x: 0, y: parseInt(code.slice(1)) };
    } else if (code[0] === 'D') {
      return { x: 0, y: -1 * parseInt(code.slice(1)) };
    } else if (code[0] === 'R') {
      return { x: parseInt(code.slice(1)), y: 0 };
    } else if (code[0] === 'L') {
      return { x: -1 * parseInt(code.slice(1)), y: 0 };
    }
}

// Find line intersections
module.exports.find = function find (lineAInstructions, lineBInstructions) {
  // Keep track of line segments (horizontal and vertical)
  const lines = { x: [], y: [] };
  // Track first line and store line segments
  let src = { x: 0, y: 0},
      distance = 0;
  for (let code of lineAInstructions) {
    const moved = decodeInstruction(code),
          dest = { x: src.x + moved.x, y: src.y + moved.y };
    if (src.x === dest.x) {
      // Register a vertical line segment (keep it oriented)
      if (!lines.x[dest.x]) { lines.x[dest.x] = []; }
      lines.x[dest.x].push({ distance, bounds: [src.y, dest.y] });
      distance += Math.abs(dest.y - src.y);
    } else if (src.y === dest.y) {
      // Register a horizontal line segment (keep it oriented)
      if (!lines.y[dest.y]) { lines.y[dest.y] = []; }
      lines.y[dest.y].push({ distance, bounds: [src.x, dest.x] });
      distance += Math.abs(dest.x - src.x);
    }
    src = dest;
  }
  // Keep track of line intersections
  let intersections = [];
  // Track 2nd line and search for intersections
  src =  { x: 0, y: 0};
  distance = 0;
  for (let code of lineBInstructions) {
    const moved = decodeInstruction(code),
          dest = { x: src.x + moved.x, y: src.y + moved.y };
    if (src.x === dest.x) {
      // Verify a vertical line segment for intersections
      for (let y in lines.y) {
        if ((y >= (src.y < dest.y ? src.y : dest.y)) && (y <= (src.y > dest.y ? src.y : dest.y))) {
          for (let segment of lines.y[y]) {
            if (src.x >= (segment.bounds[0] < segment.bounds[1] ? segment.bounds[0] : segment.bounds[1]) && src.x <= (segment.bounds[0] > segment.bounds[1] ? segment.bounds[0] : segment.bounds[1])) {
              const distances = [
                (distance + Math.abs(src.y - y)),
                (segment.distance + Math.abs(src.x - segment.bounds[0]))
              ];
              intersections.push({ x: src.x, y, distances });
            }
          }
        }
      }
      distance += Math.abs(dest.y - src.y);
    } else if (src.y === dest.y) {
      // Verify a horizontal line segment for intersections
      for (let x in lines.x) {
        if ((x >= (src.x < dest.x ? src.x : dest.x)) && (x <= (src.x > dest.x ? src.x : dest.x))) {
          for (let segment of lines.x[x]) {
            if (src.y >= (segment.bounds[0] < segment.bounds[1] ? segment.bounds[0] : segment.bounds[1]) && src.y <= (segment.bounds[0] > segment.bounds[1] ? segment.bounds[0] : segment.bounds[1])) {
              const distances = [
                (distance + Math.abs(src.x - x)),
                (segment.distance + Math.abs(src.y - segment.bounds[0]))
              ];
              intersections.push({ x, y: src.y, distances });
            }
          }
        }
      }
      distance += Math.abs(dest.x - src.x);
    }
    src = dest;
  }
  // Return found intersections
  return intersections;
}
