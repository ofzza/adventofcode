// (NEW)NEWTONIAN PHYSICS SIMULATION

// Import dependencies
const math = require('../math');

// Simulate a nbody (new)newtonian problem
module.exports.nbody = function * nbody (bodies, { coords = ['x', 'y', 'z'], energy = true } = {}) {
  // Initialize state
  const state = bodies.map((body) => {
    return coords.reduce((state, coord) => {
      state[coord] = body[coord];
      state[`v${coord}`] = 0;
      return state;
    }, {
      E:  null,
      pE: null,
      kE: null
    });
  });
  // Simulate
  while (true) {

    // Update velocity
    for (let a of state) {
      for (let b of state) {
        if (a !== b) {
          // Update coordinates
          for (let i of coords) {
            a[`v${i}`] += (a[i] !== b[i] ? (a[i] < b[i] ? +1 : -1) : 0);
          }
        }
      }
    }
    
    // Update position
    for (let a of state) {
      for (let i of coords) {
        a[i] += a[`v${i}`];
      }
    }

    // Update energy
    if (energy) {
      for (let a of state) {
        a.pE = coords.reduce((sum, coord) => (sum + Math.abs(a[coord])), 0);
        a.kE = coords.reduce((sum, coord) => (sum + Math.abs(a[`v${coord}`])), 0);
        a.E = a.pE * a.kE;
      }
    }
  
    // Yield updated state
    yield state;

  }
};

// Find cycles in nbody (new)newtonian problem
module.exports.nbodyFindPeriod = function nbodyFindPeriod (bodies, { coords = ['x', 'y', 'z'] } = {}) {

  // Initialize separate periods per coordinate axis
  const periods = {};
 
  // Calculate pariods per coordinate axis
  for (let coord of coords) {

    // Initialize history
    const history = {},
          simulation = module.exports.nbody(bodies, { coords: [ coord ], energy: false});
    
    // Run simulation and check for repeating states
    let state,
        cycles = 0;
    while (true) {

      // Get next simulates state
      state = simulation.next();

      // Check if fully repeated state
      const stateKey = state.value.map((a) => {
        return `${a[coord]},${a[`v${coord}`]}`;
      }).join('|');
      if (!history[stateKey]) {
        // Add state to history
        history[stateKey] = true;
        cycles++;
      } else {
        // Report found repeated state
        periods[coord] = cycles;
        break;
      }

    }
  }

  // Factor periods
  const factoredPeriods = Object.keys(periods).reduce((factoredPeriods, coord) => {
    factoredPeriods[coord] = math.primeFactor(periods[coord]);
    return factoredPeriods;
  }, {});

  // Remove excessive factors
  for (let i = 0; i < coords.length; i++) {
    for (let j = i + 1; j < coords.length; j++) {
      for (let p of factoredPeriods[coords[i]]) {
        const index = factoredPeriods[coords[j]].indexOf(p)
        if (index !== -1) { factoredPeriods[coords[j]].splice(index, 1); }
      }
    }  
  }

  // Calculate common multiple of periods' factors
  return Object.keys(factoredPeriods).reduce((common, coord) => {
    return factoredPeriods[coord].reduce((common, p) => {
      return common * p;
    }, common);
  }, 1);
};
