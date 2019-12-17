// DAY 13
// https://adventofcode.com/2019/day/13

// Import dependencies
const flags   = require('../../../lib').flags,
      readKey = require('../../../lib').readKey,
      puzzle  = require('../../../lib').puzzle,      
      arcade  = require('../../lib/arcade'),
      image   = require('../../lib/image');

// Set global inputs
const prog = require('fs').readFileSync(require('path').join(__dirname, './input.txt')).toString().trim().split(',').map(a => parseInt(a));

// Color palette to draw the screen with
const palette = {
  0: ' ',
  1: '░',
  2: '█',
  3: '=',
  4: 'O',
  5: '.'
};

// 1st puzzle of the day
function puzzle01 (...args) {
  // Initialize a game
  const game = arcade.play(args, [], { palette });
  // Run game to the end
  let screen;
  while (true) {
    let next = game.next();
    if (!next.done) {
      screen = next.value.screen;
    } else {
      break;
    }
  } 
  // Count number of blocks on screen
  let count = 0;
  for (let point of screen) {
    if (point.color === 2) { count++; }
  }
  // Return screen and count
  return { count, img: image.drawPointsAsImage(screen) };
}
module.exports.puzzle01 = () => {
  puzzle('2019', '13', '01', puzzle01, [
    prog, { expected: 412, transform: r => r.count, render: image.renderFieldFactory({ transform: r => r.img, palette }), example: false }
  ]);
};

// 2nd puzzle of the day
function puzzle02 (...args) {
  // Initialize a game
  let screen,
      score;
  const game = arcade.play(
    args,
    () => {
      // 
      if (flags.PROGRESS && flags.INTERACTIVE) {
        const key = readKey('Press "A" for left, "D" for right, "X" to quit and any other key to continue ...');
        if (key === 'x') {
          process.exit(0);
        } else if (key === 'a') {
          return [-1];
        } else if (key === 'd') {
          return [+1];
        } else {
          return [0];
        }
      } else {
        // Find ball on screen and follow with paddle
        const ball    = screen.find((p) => (p.color === 4)),
              paddle  = screen.find((p) => (p.color === 3));
        if (ball && paddle) {
          return [(paddle.coords.x === ball.coords.x ? 0 : (paddle.coords.x > ball.coords.x ? -1 : +1))];
        }
      }
    },
    {
      overlayFn: (screen, overlay, point) => {
        if (point.color === 4) {
          if (overlay.length > 32) { overlay.splice(0, overlay.length - 32); }
          for (let i in overlay) { overlay[i].color = 5; }            
          return [{ coords: point.coords, color: 4 }];
        } else {
          return [];
        }
      },
      palette
    }
  );
  // Run game to the end
  while (true) {
    let next = game.next();
    if (!next.done) {
      // Update screen
      screen = next.value.screen;
      score = next.value.score;
    } else {
      break;
    }
  } 
  // Return screen and count
  return { score, img: image.drawPointsAsImage(screen) };
}
module.exports.puzzle02 = () => {
  puzzle('2019', '13', '02', puzzle02, [
    [2, ...prog.slice(1)], { expected: 20940, transform: r => r.score, render: image.renderFieldFactory({ transform: r => r.img, palette }), example: false }
  ]);
};
