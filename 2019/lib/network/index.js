// COMPUTER NETWORK

// Import dependencies
const turing  = require('../turing');

// Creates a network connected computer box
module.exports.spawnNetworkedBox = function spawnNetworkedBox (prog, address) {
  const inputs = [ address ],
        buffer = [],
        computer = turing.run(
          [...prog],
          () => {
            const input = (inputs.length ? inputs.splice(0, 1)[0] : -1);
            idleCount = (input === -1 ? (idleCount + 1) : 0);
            return (lastInput = input);
          },
          { fastForwardUntilOutput: false }
        );
  let lastInput,
      lastOutput,
      idleCount = 0;
  return {
    address,
    inputs,
    computer,
    tick: () => {
      address;
      const output = computer.next();
      if (!output.done && output.value !== undefined) {
        buffer.push(lastOutput = output.value);
        if (buffer.length === 3) {
          return buffer.splice(0, 3);
        }
      }
    },
    get lastInput  () { return lastInput; },
    get lastOutput () { return lastOutput; },
    get idleCount  () { return idleCount; }
  };
};

// Bootstraps a computer network
module.exports.bootstrap = function * bootstrap (boxes) {

  // Organize by address
  const addresses = {};
  for (let box of boxes) { addresses[box.address] = box; }

  // Keep running the network
  let natPacket;
  while (true) {

    // Tick network
    for (let box of boxes) {
      // Run next tick      
      const packet = box.tick();
      // If packet ...
      if (packet) {
        // Yield packet
        yield [box.address, ...packet];
        // Route packet
        if (addresses[packet[0]]) {
          // Route to computer
          addresses[packet[0]].inputs.push(packet[1], packet[2]);
        } else if (packet[0] === 255) {
          // Route keep-alive to NAT
          natPacket = [box.address, 0, ...packet.slice(1)];
        } else {
          throw new Error('Routing error!');
        }
      }
    }

    // Check if network idle
    if(!boxes.find(box => (box.inputs.length || (box.idleCount <= 10)))) {
      // Yield NAT keep-alive packet
      yield [255, 0, ...natPacket.slice(2)];
      // Route NAT keep-alive packet to address 0
      addresses[natPacket[1]].inputs.push(natPacket[2], natPacket[3]);
    }

  }

};
