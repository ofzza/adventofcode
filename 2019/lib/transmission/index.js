// TRANSMISSION PROCESSING

// Import dependencies
const flags       = require('../../../lib').flags,
      logProgress = require('../../../lib').logProgress;

// Flawed Frequency Transmission cleanup
module.exports.fft = function * fft (input, { repeatInput = 1, startIndex, endIndex } = {}) {

  // Hard-coded pattern
  const pattern = [0, 1, 0, -1];

  // Initialize start-index, if not initialized
  startIndex = (startIndex !== undefined ? startIndex : 0);

  // Repeat input
  const repeatedInput = Array(input.length * repeatInput);
  for (let i = 0; i < repeatInput; i++) {
    repeatedInput.splice((i * input.length), input.length, ...input);
  }
  input = repeatedInput;

  // Repeat for as long as generator is in use
  for (let n = 0; true; n++) {
    let timeLog = Date.now(),
        timeA = 0,
        timeB = 0;
    // Process entire input
    for (let i = startIndex; i < input.length; i++) {
      let outputValueA = 0,
          outputValueB = 0;

      if ((i % 1000 === 0) && ((Date.now() - timeLog) > 10000)) {
        timeLog = Date.now();
        console.log(`Phase ${ n + 1 } progress: ${ ~~(10000 * (i - startIndex) / (input.length - startIndex)) / 100 }%`.green);
      }

      // Calculate by pattern
      const timestampA = Date.now();
      for (let p = 0; p < pattern.length; p++) {
        const pf = pattern[p];
        if (pf !== 0) {
          const jstep   = (pattern.length * (i + 1))
                joffset = ~~(startIndex / jstep) * jstep;
          for (let j = joffset + (p * (i + 1)) - 1; j < input.length; j += jstep) {
            if (j >= 0) {
              const limitLoc  = i + 1,
                    limitGlob = input.length - j;
                    limit     = (limitLoc < limitGlob ? limitLoc : limitGlob); 
              for (let k = 0; k < limit; k++) { outputValueA += input[j + k]; }
              outputValueA *= pf;
            }
          }
        }
      }
      timeA += Date.now() - timestampA;

      // Calculate by input
      // const timestampB = Date.now();
      // for (let j = (startIndex !== undefined && startIndex > i ? startIndex : i); j < input.length; j++) {
      //   const pf = pattern[~~(1 + (j - i) / (i + 1)) % pattern.length];
      //   if (pf === 0) {
      //     // Skip until next pattern
      //     j += i; continue;
      //   } else {
      //     // Calculate
      //     outputValueB += ((input[j] * pf));
      //   }
      // }
      // timeB += Date.now() - timestampB;

      // if (outputValueA !== outputValueB) { throw new Error("Different values depending on calculating method!!!"); }
      input[i] = Math.abs(outputValueA || outputValueB) % 10;
    }

    // If logging progress, log phase
    if (flags.PROGRESS) {
      logProgress(`${ (n + 1).toString().padStart(3, ' ') }. phase => ${ input.slice(startIndex, endIndex).join('') }`
                + ` (in ${ timeA }|${ timeB } ms)`);
    }

    // Yield updated input
    yield input;

  }
};
