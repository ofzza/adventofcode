// DECK OF SPACE CARDS
// --------------------------------------------------------
// Card positions change with each shuffle:
// - DEAL INTO NEW STACK     --> -1 x N - 1  | stack.length
// - CUT X                   --> N - X       | stack.length
// - DEAL WITH INCREMENT X   --> X x N       | stack.length

// Import dependencies
const modnorm = require('../math').modNormalize,
      modmult = require('../math').modMultiply,
      moddiv  = require('../math').modDivide;
      modpow  = require('../math').modPower;

// Process shuffle instructions into a function giving the end position of each card after shuffling
module.exports.processShuffleInstructions = function processShuffleInstructions (instructions, n) {

  // Calculate shuffle parameters:
  // x' = a * x + b | deck.length
  let a = 1,
      b = 0;
  // Process instructions
  for (let instruction of instructions) {
    // Process instructions
    if (instruction.substr(0, 19) === 'deal into new stack') {
      a = modnorm((-1 * a), n);
      b = modnorm((-1 * b) - 1, n);
    } else if (instruction.substr(0, 3) === 'cut') {
      const x = parseInt(instruction.substr(3).trim());
      b = modnorm(b - x, n);
    } else if (instruction.substr(0, 19) === 'deal with increment') {
      const x = parseInt(instruction.substr(19).trim());
      a = modnorm(modmult(x, a, n), n);
      b = modnorm(modmult(x, b, n), n);
    }
  }

  // Calculate repeating shuffle parameters
  // x' = A * x + B | deck.length
  function calculateRepeatingParams (reps) {
    // Calculate A
    let A;
    A = modpow(a, reps, n);
    // Calculate B and c
    let B = modnorm(
              modmult(
                b,
                moddiv(
                  modpow(a, reps, n ) - 1,
                  (a - 1),
                  n
                ),
                n
              ),
              n
            );
    // Return params
    return { A, B };
  }

  // Card shuffle function
  function shuffleAnalytically (x, reps = 1) {
    // Get parameters
    const { A, B } = calculateRepeatingParams(reps);
    // Calculate shuffle
    return modnorm(modmult(A, x, n) + B, n);
  };

  // Iterative card shuffle function
  function shuffleIteratively (x, reps = 1) {
    for (let i = 0; i < reps; i++) {
      x = (a * x + b) % n;
      x = (x >= 0 ? x : (x % n) + n)
    }
    return x;
  };

  // Card unshuffle function
  function unshuffleAnalytically (x, reps = 1) {
    // Get parameters
    const { A, B } = calculateRepeatingParams(reps);
    // Calculate unshuffle
    return modnorm(moddiv(modnorm(x - B, n), A, n), n);
  };

  // Iterative card unshuffle function
  function unshuffleIteratively (x, reps = 1) {
    reps: for (let i = 0; i < reps; i++) {
      for (let c = 0; c < a; c++) {
        const y = (x + (c * n) - b) / a;
        if (y === Math.trunc(y)) {
          x = (y >= 0 ? y : (y % n) + n) % n;
          continue reps;
        }
      }
      throw new Error('Failed unshuffling!');
    }
    return x;
  }

  // Return shuffle/unshuffle functions
  return {
    shuffleAnalytically,
    unshuffleAnalytically,
    shuffleIteratively,
    unshuffleIteratively
  };

}
