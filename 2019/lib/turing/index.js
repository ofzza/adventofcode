// TURING AUTOMATA

// Set globals
const instructions = [
  null,
  { opcode: 1, params: 3, func: 'add' },
  { opcode: 2, params: 3, func: 'multiply' },
  { opcode: 3, params: 1, func: 'input' },
  { opcode: 4, params: 1, func: 'output' },
  { opcode: 5, params: 2, func: 'jump-true' },
  { opcode: 6, params: 2, func: 'jump-false' },
  { opcode: 7, params: 3, func: 'less-than' },
  { opcode: 8, params: 3, func: 'equals' },
  { opcode: 9, params: 1, func: 'set-rel-base' }
];

// Computer as output generator implementation
module.exports.run = function * run (prog, inputs = [], actions = []) {
  // Set relative-base pointer value
  let relbase = 0;
  // Run program
  loopProg: for (let i=0; prog[i] !== 99 && prog[i] !== undefined;) {
    // Parse opcode
    const instruction = instructions[prog[i] % 100],
          modes = Math.round(prog[i] / 100).toString().padStart(instruction.params, '0').split('').reverse().map((n) => { return parseInt(n); });
    // Run extensions
    for (let action of actions) {
      const args = getArguments({ prog, i, instruction, modes, relbase }),
            output = action({ prog, instruction, args, inputs });
      if (output !== undefined) {
        // Check for output value
        if (output.output !== undefined) {
          yield output.output;
        }
        // Check for jump
        if (output.jump !== undefined) {
          i = output.jump;
          continue loopProg;
        }
        // Check for relative base update
        if (output.relbase !== undefined) {
          relbase += output.relbase;
        }
      }
    }
    // Move to next opcode
    i += 1 + instruction.params;
  }
};

// Turing actions
module.exports.actions = {
  // Add
  turingAdd: function turingAdd ({ prog, instruction, args } = {}) {
    if (instruction.func === 'add') {
      prog[args[2].index] = args[0].value + args[1].value;
    }
  },
  // Multiply
  turingMultiply: function turingMultiply ({ prog, instruction, args } = {}) {
    if (instruction.func === 'multiply') {
      prog[args[2].index] = args[0].value * args[1].value;
    }
  },
  // Read from input
  turingInput: function turingInput ({ prog, instruction, args, inputs } = {}) {
    if (instruction.func === 'input') {
      if (typeof inputs === 'object' && inputs.length !== undefined) {
        prog[args[0].index] = inputs.splice(0, 1)[0];
        return;
      } else if (typeof inputs === 'function') {
        prog[args[0].index] = inputs();
        return;
      }
      throw new Error('Trying to read a non-existent input!');
    }
  },
  // Write to output
  turingOutput: function turingOutput ({ instruction, args } = {}) {
    if (instruction.func === 'output') {
      return { output: args[0].value };
    }
  },
  // Jump if true
  turingJumpTrue: function turingJumpTrue ({ instruction, args } = {}) {
    if (instruction.func === 'jump-true') {
      if (args[0].value) {
        return { jump: args[1].value };
      }
    }
  },
  // Jump if false
  turingJumpFalse: function turingJumpFalse ({ instruction, args } = {}) {
    if (instruction.func === 'jump-false') {
      if (!args[0].value) {
        return { jump: args[1].value };
      }
    }
  },
  // Check if less-than
  turingLessThan: function turingLessThan ({ prog, instruction, args } = {}) {
    if (instruction.func === 'less-than') {
      prog[args[2].index] = (args[0].value < args[1].value ? 1 : 0);
    }
  },
  // Check if equals
  turingEquals: function turingEquals ({ prog, instruction, args } = {}) {
    if (instruction.func === 'equals') {
      prog[args[2].index] = (args[0].value === args[1].value ? 1 : 0);
    }
  },
  // Update relative base pointer
  turingSetRelativeBase: function turingEquals ({ instruction, args } = {}) {
    if (instruction.func === 'set-rel-base') {
      return { relbase: args[0].value };
    }
  }
};

// Get argument values
function getArguments ({ prog, i, instruction, modes, relbase } = {}) {
  return [...Array(instruction.params)].map((u, j) => {
    if (modes[j] === 0) {
      // Read in absolute mode
      return { index: (prog[i + j + 1] || 0), value: (prog[prog[i + j + 1] || 0] || 0) };
    } else if (modes[j] === 1) {
      // Read in parameter mode
      return { index: (i + j + 1), value: (prog[i + j + 1] || 0) };
    } else if (modes[j] === 2) {
      // Read in relative mode
      return { index: (relbase + (prog[i + j + 1] || 0)), value: (prog[relbase + (prog[i + j + 1] || 0)] || 0) };
    }
  });
}
