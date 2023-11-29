// INVESTIGATOR DROID

// Import dependencies
const flags       = require('../../../../lib').flags,
      logProgress = require('../../../../lib').logProgress,
      readText    = require('../../../../lib').readText,
      turing      = require('../../turing'),
      image       = require('../../image');

// Control Investigator Droid
// ...
// Routines:
// - To explore all rooms and pick up all objects, run "explore"
// - To move in a direction, run "go [north|south|east|west]"
// - To revisit a previously explored room, run "goto [room name]"
// - To pick up an item, run "take [item name]"
// - To drop an item, run "drop [item name]"
// - To list items in your inventory, run "inventory"
// - To end execution run "exit"
module.exports.control = function control (prog, routines = [], { dontExploreRooms = [], dontPickUpItems = [] } = {}) {
  
  // Initialize droid
  const commands = [],
        droid = module.exports.run(prog, commands);

  // Initialize shared routine outputs
  let map       = {},
      inventory = {};

  // Get Initial prompt
  let prompt = droid.next().value;
  // Just run routines on droid ...
  while (true) {

    // Check if already executing a routine
    if (!commands.length) {

      // Get next routine
      let routine = (routines instanceof Function ? routines() : routines.splice(0, 1)[0]);
      // Log or prompt routine
      if (routine) {            
        // Log next routine
        if (flags.PROGRESS) { logProgress(`Routine: ${ routine }`); }
      } else {
        // Prompt for routine
        if (flags.PROGRESS && flags.INTERACTIVE) { 
          routine = readText('Routine: ');
        }
      }

      // Execute routine
      if (routine === 'explore') {

        // Explore ship
        map = {};
        prompt = exploreRoutine(droid, commands, prompt, { map, inventory, dontExploreRooms, dontPickUpItems });

        // Log output
        if (flags.PROGRESS) {
          logMap(map, inventory, { logRooms: true, logInventory: true });
        }

      } else if (routine === 'cheat pressure floor') {

        // Go in a direction
        const code = tryPassingPressureGateRoutine(droid, commands, inventory);

        // Return code
        return parseInt(code);

      } else if (routine && routine.substr(0, 3) === 'go ') {

        // Go in a direction
        prompt = goDirectionRoutine(routine.substr(3).trim(), droid, commands, map);

        // Log output
        if (flags.PROGRESS) {
          logMap(map, inventory, { logRooms: true, logInventory: true });
        }

      } else if (routine && routine.substr(0, 5) === 'goto ') {

        // Go to a room
        prompt = gotoRoomRoutine(routine.substr(5).trim(), droid, commands, map);

        // Log output
        if (flags.PROGRESS) {
          logMap(map, inventory, { logRooms: true, logInventory: true });
        }

      } else if (routine && routine.substr(0, 5) === 'take ') {

        // Go in a direction
        prompt = takeItemRoutine(routine.substr(5).trim(), droid, commands, inventory);

        // Log output
        if (flags.PROGRESS) {
          logMap(map, inventory, { logRooms: true, logInventory: true });
        }

      } else if (routine && routine.substr(0, 5) === 'drop ') {

        // Go in a direction
        prompt = dropItemRoutine(routine.substr(5).trim(), droid, commands, inventory);

        // Log output
        if (flags.PROGRESS) {
          logMap(map, inventory, { logRooms: true, logInventory: true });
        }

      } else if (routine === 'inventory') {

        // List inventory
        prompt = listItemsRoutine(droid, commands);

        // Log output
        if (flags.PROGRESS) {
          logMap(map, inventory, { logRooms: true, logInventory: true });
        }

      } else if (routine === 'exit' || routine === 'quit') {        
        // Finish execution
        break;
      }

    }

  }
  
};

// Run Investigator Droid
// ...
// Commands:
// - Movement via: "north", "south", "east", or "west".
// - To take an item the droid sees in the environment, use the command "take <name of item>"
// - To drop an item the droid is carrying, use the command "drop <name of item>".
// - To get a list of all of the items the droid is currently carrying, use the command "inv".
module.exports.run = function * run (prog, commands = []) {

  // Initialize command into input translation
  const inputs = []
  let inputTranslationFn = () => {
    // Check if queued inputs empty
    if (!inputs.length) {
      // Get next command
      let command = (commands instanceof Function ? commands() : commands.splice(0, 1)[0]);
      // Log or prompt command
      if (command) {            
        // Log next command
        if (flags.PROGRESS) { logProgress(`Command: ${ command }`); }
      } else {
        // Prompt for command
        if (flags.PROGRESS && flags.INTERACTIVE) { 
          command = readText('Command: ');
        }
      }
      // Translate into input
      inputs.push(...`${command}\n`.split('').map(c => c.charCodeAt(0)));
    }
    // Return next queued input
    return inputs.splice(0, 1)[0];
  };

  // Initialize robot
  const robot   = turing.run(prog, inputTranslationFn);

  // Run robot loop
  let outputLine    = '',
      outputPrompt  = [],
      roomPrompt,
      result;
  while  (!(result = robot.next()).done) {

    if (String.fromCharCode(result.value) !== '\n') {
      // Compose output line
      outputLine += String.fromCharCode(result.value);
    } else {
      // If output "Command?", get next command
      if (outputLine !== 'Command?') {
        // Compose room prompt
        outputPrompt.push(outputLine);
        outputLine = '';
      } else {

        // Print output
        if (flags.PROGRESS) { logProgress(outputPrompt.join('\n')); }

        // Process room prompt
        if (outputPrompt.find(p => p.match(/^==.*==$/))) {
          roomPrompt = {
            room:     outputPrompt.find(p => p.match(/^==.*==$/)).replace(/==/g, '').trim(),
            move: {
              north:  !!outputPrompt.find(p => p.match(/^- north$/)),
              south:  !!outputPrompt.find(p => p.match(/^- south$/)),
              east:   !!outputPrompt.find(p => p.match(/^- east$/)),
              west:   !!outputPrompt.find(p => p.match(/^- west$/))
            },
            items:    outputPrompt.filter(p => p.match(/^- (?!.*(north|south|east|west)).*$/)).map(i => i.substr(2)),
            output:   outputPrompt
          };
        } else {
          roomPrompt = null;
        }

        // Yield room prompt
        yield roomPrompt;

        // Reset prompt
        outputLine = '';
        outputPrompt = [];

      }

    }

  }

  // Once done, output and yield last prompt
  if (flags.PROGRESS) {
    logProgress(outputPrompt.join('\n'));
  }
  yield { ...roomPrompt, output: outputPrompt };

};

// Runs exploration routine finding all the rooms and picking up all objects
function exploreRoutine (droid, commands, roomPrompt, { map = {}, inventory = {}, dontExploreRooms = [], dontPickUpItems = []} = {}) {

  if (!roomPrompt && droid.next().done) {
    logProgress('DROID DONE/DIED!');
    process.exit(1);
  }

  // Check if room already visited
  if (map[roomPrompt.room]) {
    return roomPrompt;
  }

  // Add current prompt into the map
  map[roomPrompt.room] = {
    color: 0,
    prompt: roomPrompt
  };

  // Log current map state
  logMap(map, inventory, { logRooms: false, logInventory: false });

  // Pick up all items in the room
  for (let item of roomPrompt.items) {
    if (dontPickUpItems.indexOf(item) === -1) {
      // Pick up item
      commands.push(`take ${item}`);
      droid.next();
      inventory[item] = true;
    }
  }

  // Check if room is allowed to be explored
  let latestRoomPrompt;
  if (dontExploreRooms.indexOf(roomPrompt.room) === -1) {

    // Move into all other available rooms
    if (roomPrompt.move.north) {
      // Move north, continue exploring, then come back
      commands.push('north');
      trackMovementRelativeToRooms(map, 'north');
      exploreRoutine(droid, commands, (latestRoomPrompt = droid.next().value), { map, inventory, dontExploreRooms, dontPickUpItems });
      commands.push('south');
      trackMovementRelativeToRooms(map, 'south');
      latestRoomPrompt = droid.next().value;
      // Log current map state
      logMap(map, inventory, { logRooms: false, logInventory: false });
    }
    if (roomPrompt.move.south) {
      // Move south, continue exploring, then come back
      commands.push('south');
      trackMovementRelativeToRooms(map, 'south');
      exploreRoutine(droid, commands, (latestRoomPrompt = droid.next().value), { map, inventory, dontExploreRooms, dontPickUpItems });
      commands.push('north');
      trackMovementRelativeToRooms(map, 'north');
      latestRoomPrompt = droid.next().value;
      // Log current map state
      logMap(map, inventory, { logRooms: false, logInventory: false });
    }
    if (roomPrompt.move.east) {
      // Move east, continue exploring, then come back
      commands.push('east');
      trackMovementRelativeToRooms(map, 'east');
      exploreRoutine(droid, commands, (latestRoomPrompt = droid.next().value), { map, inventory, dontExploreRooms, dontPickUpItems });
      commands.push('west');
      trackMovementRelativeToRooms(map, 'west');
      latestRoomPrompt = droid.next().value;
      // Log current map state
      logMap(map, inventory, { logRooms: false, logInventory: false });
    }
    if (roomPrompt.move.west) {
      // Move west, continue exploring, then come back
      commands.push('west');
      trackMovementRelativeToRooms(map, 'west');
      exploreRoutine(droid, commands, (latestRoomPrompt = droid.next().value), { map, inventory, dontExploreRooms, dontPickUpItems });
      commands.push('east');
      trackMovementRelativeToRooms(map, 'east');
      latestRoomPrompt = droid.next().value;
      // Log current map state
      logMap(map, inventory, { logRooms: false, logInventory: false });
    }

  }

  // Return composed map
  return (latestRoomPrompt || roomPrompt);

}

// Try passing the pressure plate with all combinations of items available
function tryPassingPressureGateRoutine (droid, commands, inventory) {

  // Generate all combinations of items
  function combine (items) {
    if (items.length > 1) {
      let combs = combine(items.slice(1));
      return [
        ...combs,
        ...combs.map(comb => [items[0], ...comb])
      ];
    } else {
      return [[], [items[0]]];
    }
  }  
  const items = Object.keys(inventory).filter(key => inventory[key]),
        combinations = combine(items);

  // Try all combinations
  for (let combination of combinations) {
    // Drop everything
    for (let item of Object.keys(inventory).filter(key => inventory[key])) {
      dropItemRoutine(item, droid, commands, inventory);
    }
    // Pick up combination
    for (let item of combination) {
      takeItemRoutine(item, droid, commands, inventory);
    }
    // Log inventory
    if (flags.PROGRESS) {
      if (inventory && Object.keys(inventory).length) {
        logProgress('Inventory:');
        for (let item of Object.keys(inventory).filter(key => inventory[key])) {
          logProgress(`- ${ item }`);
        }
      }
    }
    // Step north
    commands.push('north');
    let prompt = droid.next().value;
    // Check prompt
    const fail = !prompt.output.find((line) => {
      return (line.indexOf('Analysis complete! You may proceed.') !== -1);
    });
    if (!fail) {
      return prompt.output.find(line => {
        return (line.indexOf('You should be able to get in by typing') !== -1);
      })
        .replace(/"/g, '')
        .replace('Oh, hello! ', '')
        .replace('You should be able to get in by typing ', '')
        .replace(' on the keypad at the main airlock.', '')
        .trim();
    }
  }
}

// Moves in a direction
function goDirectionRoutine (direction, droid, commands, map) {

  // Move droid
  commands.push(direction);
  trackMovementRelativeToRooms(map, direction);
  return droid.next().value;

}

// Moves to a room
function gotoRoomRoutine (room, droid, commands, map) {

  // Find room and path to it
  const path = Object.values(map).find(r => r.prompt.room === room).prompt.path.map(m => {
    return { 'north': 'south', 'south': 'north', 'east': 'west', 'west': 'east' }[m];
  }).reverse();

  // Input commands and run droid to the room
  let prompt;
  for (let i = 0; i < path.length; i++) {
    // Move droid
    commands.push(path[i]);
    trackMovementRelativeToRooms(map, path[i]);
    prompt = droid.next().value;
  }

  // Return latest prompt
  return prompt;

}

// Take an item
function takeItemRoutine (item, droid, commands, inventory) {

  // Take an item
  commands.push(`take ${ item }`);
  inventory[item] = true;
  return droid.next().value;

}

// Drop an item
function dropItemRoutine (item, droid, commands, inventory) {

  // Drop an item
  commands.push(`drop ${ item }`);
  inventory[item] = false;
  return droid.next().value;

}

// List items
function listItemsRoutine (droid, commands) {

  // Drop an item
  commands.push(`inv`);
  return droid.next().value;

}

// Adds a move to the path of all the rooms previously visited
function trackMovementRelativeToRooms (map, move) {
  for (let room of Object.values(map)) {
    // Initialize path
    if (!room.prompt.path) { room.prompt.path = []; }
    // Add move to path
    room.prompt.path.push(move);
    // Prune path
    if (room.prompt.path.length >= 2) {
      for (let i = (room.prompt.path.length - 2); i >= 0; i--) {
        if ((room.prompt.path[i] === 'north'  && room.prompt.path[i + 1] === 'south')
         || (room.prompt.path[i] === 'south'  && room.prompt.path[i + 1] === 'north')
         || (room.prompt.path[i] === 'east'   && room.prompt.path[i + 1] === 'west')
         || (room.prompt.path[i] === 'west'   && room.prompt.path[i + 1] === 'east')) {
          room.prompt.path.splice(i, 2);
          i--;
        }
      }
    }
  }
}

// Outputs map state
function logMap (map, inventory = null, { logRooms = true, logInventory = true } = {}) {
  // // Log map
  // const img = image.drawPointsAsImage(
  //         [
  //           ...Object.values(map),
  //           { coords: { x: 0, y: 0 }, color: 'o' },
  //           ...(coords ? [{ coords, color: '*' }] : [])
  //         ],
  //         { transparentColor: 1 }
  //       );
  // // Log map as image
  // logProgress('MAP:');
  // logProgress(`${ [...image.renderFieldFactory()(img)].join('').trim() }`);
  // if (coords) {
  //   logProgress(`> * ${map[`${ coords.x }x${ coords.y }`].prompt.room}`);
  // }
  // logProgress();
  // Log rooms
  if (logRooms) {
    logProgress('Rooms:');
    for (let room of Object.values(map)) {
      logProgress(`- ${ room.prompt.room } (${ room.prompt.path.map(m => m[0]).join('') })`);
    }
    logProgress();
  }
  // Log inventory
  if (logInventory && inventory && Object.keys(inventory).length) {
    logProgress('Inventory:');
    for (let item of Object.keys(inventory).filter(key => inventory[key])) {
      logProgress(`- ${ item }`);
    }
    logProgress();
  }
}
