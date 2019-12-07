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
  { opcode: 8, params: 3, func: 'equals' }
];

// Computer as output generator implementation
module.exports.run = function * run (prog, inputs = [], actions = []) {
  const output = []
  let inputCounter = 0;
  loopProg: for (let i=0; prog[i] !== 99 && prog[i] !== undefined;) {
    // Parse opcode
    const instruction = instructions[prog[i] % 100],
          modes = Math.round(prog[i] / 100).toString().padStart(instruction.params, '0').split('').reverse().map((n) => { return parseInt(n); });
    // Run extensions
    for (let action of actions) {
      let output = action(prog, i, instruction, modes, inputs);
      // Check for output value
      if (output !== undefined && output.output !== undefined) {
        yield output.output;
      }
      // Check for jump
      if (output !== undefined && output.jump) {
        i = output.jump;
        continue loopProg;
      }
    }
    // Move to next opcode
    i += 1 + instruction.params;
  }
};

// Turing actions
module.exports.actions = {
  turingAdd: function turingAdd (prog, i, instruction, modes, inputs) {
    if (instruction.func === 'add') {
      prog[!modes[2] ? prog[i+3] : (i+3)] = (!modes[0] ? prog[prog[i+1]] : prog[i+1]) + (!modes[1] ? prog[prog[i+2]] : prog[i+2]);
    }
  },
  turingMultiply: function turingMultiply (prog, i, instruction, modes, inputs) {
    if (instruction.func === 'multiply') {
      prog[!modes[2] ? prog[i+3] : (i+3)] = (!modes[0] ? prog[prog[i+1]] : prog[i+1]) * (!modes[1] ? prog[prog[i+2]] : prog[i+2]);
    }
  },
  turingInput: function turingInput (prog, i, instruction, modes, inputs) {
    if (instruction.func === 'input') {
      prog[!modes[0] ? prog[i+1] : (i+1)] = inputs.splice(0, 1)[0];
    }
  },
  turingOutput: function turingOutput (prog, i, instruction, modes, inputs) {
    if (instruction.func === 'output') {
      return { output: prog[!modes[0] ? prog[i+1] : (i+1)] };
    }
  },
  turingJumpTrue: function turingJumpTrue (prog, i, instruction, modes, inputs) {
    if (instruction.func === 'jump-true') {
      if (!modes[0] ? prog[prog[i+1]] : prog[i+1]) {
        return { jump: !modes[1] ? prog[prog[i+2]] : prog[i+2] };
      }
    }
  },
  turingJumpFalse: function turingJumpFalse (prog, i, instruction, modes, inputs) {
    if (instruction.func === 'jump-false') {
      if (!(!modes[0] ? prog[prog[i+1]] : prog[i+1])) {
        return { jump: !modes[1] ? prog[prog[i+2]] : prog[i+2] };
      }
    }
  },
  turingLessThan: function turingLessThan (prog, i, instruction, modes, inputs) {
    if (instruction.func === 'less-than') {
      prog[!modes[2] ? prog[i+3] : (i+3)] = ((!modes[0] ? prog[prog[i+1]] : prog[i+1]) < (!modes[1] ? prog[prog[i+2]] : prog[i+2]) ? 1 : 0);
    }
  },
  turingEquals: function turingEquals (prog, i, instruction, modes, inputs) {
    if (instruction.func === 'equals') {
      prog[!modes[2] ? prog[i+3] : (i+3)] = ((!modes[0] ? prog[prog[i+1]] : prog[i+1]) === (!modes[1] ? prog[prog[i+2]] : prog[i+2]) ? 1 : 0);
    }
  }
};
