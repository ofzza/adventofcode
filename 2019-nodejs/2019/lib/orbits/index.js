// ORBIT CALCULATOR

// Recursively increases graph node and it's children's depth
function increaseChildrenDepth (child, depth) {
  child.depth += depth;
  for (let c of child.children) { increaseChildrenDepth(c, depth); }
}

// Parses orbits and arranges them into a graph
module.exports.orbitsIntoGraph = function orbitsIntoGraph (...args) {
  const orbits = {};
  for (let orbit of args) {
    const parsed    = orbit.split(')'),
          parentKey = parsed[0],
          parent    = orbits[parentKey] || (orbits[parentKey] = { key: parentKey, parent: null, depth: 0, children: [] }),
          childKey  = parsed[1];
    let child
    if (!orbits[childKey]) {
      child = (orbits[childKey] = { key: childKey, parent: parent, depth: (parent.depth + 1), children: [] });
    } else {
      child = orbits[childKey];
      child.parent = parent;
      increaseChildrenDepth(child, parent.depth + 1);
    }
    parent.children.push(child);
  }
  return orbits;
}
