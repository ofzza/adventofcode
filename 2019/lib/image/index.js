// IMAGE PROCESSING

// Default palette
module.exports.defaultPalette = { 0: '░', 1: '█', 2: ' ' };

// Extract layers from image data
module.exports.processLayers = function processLayers (width, height, data) {
  const layers = [];
  for (let i=0; i<data.length; i++) {
    // Initialize a layer
    const layerIndex = Math.floor(i / (width * height)),
          layer = layers[layerIndex] || (layers[layerIndex] = {
            data:   [],
            digits: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
          });
    // Copy layer data
    layer.data.push(data[i]);
    // Count digits per layer
    layer.digits[data[i]] = (layer.digits[data[i]] || 0) + 1;
  }
  return layers;
}

// Draw points as image
module.exports.drawPointsAsImage = function drawPointsAsImage (points, { transparentColor = 2 } = {}) {
  // Copy points
  points = JSON.parse(JSON.stringify(points));
  // Normalize trail
  let dx = { min: null, max: null }, 
      dy = { min: null, max: null };
  for (const point of points) {
    if (dx.min === null || point.coords.x < dx.min) { dx.min = point.coords.x; }
    if (dx.max === null || point.coords.x > dx.max) { dx.max = point.coords.x; }
    if (dy.min === null || point.coords.y < dy.min) { dy.min = point.coords.y; }
    if (dy.max === null || point.coords.y > dy.max) { dy.max = point.coords.y; }
  }
  for (const point of points) {
    if (dx.min !== 0) { point.coords.x += (-1 * dx.min); }
    if (dy.min !== 0) { point.coords.y += (-1 * dy.min); }
  }
  // Initialize image
  const image = [...Array(dy.max - dy.min + 1)].map(() => {
    return [...Array(dx.max - dx.min + 1)].map(n => transparentColor);
  });
  // Draw trail onto the image
  for (const point of points) {
    image[point.coords.y][point.coords.x] = point.color;
  }
  // Return image
  return image;
}

// Renders 1D array as image
module.exports.renderLinearFactory = function renderLinearFactory ({ width, height, transform = null, palette } = {}) {
  return function * render (image) {
    // Check if image needs to be extracted from result
    if (transform !== null) { image = transform(image); }
    // Render image
    for (let y = 0; y < height; y++) {
      for (let x = 0; x < width; x++) {
        yield drawPixel(image[y * width + x], palette);
      }
      yield drawPixel();
    }
  };
};

// Renders 2D array as image
module.exports.renderFieldFactory = function renderFieldFactory ({ transform = null, palette } = {}) {
  return function * render (image) {
    // Check if image needs to be extracted from result
    if (transform !== null) { image = transform(image); }
    // Render image
    for (let y = 0; y < image.length; y++) {
      for (let x = 0; x < image[0].length; x++) {
        yield drawPixel(image[y][x], palette);
      }
      yield drawPixel();
    }
  };
};

// Draw a pixes as ASCII
function drawPixel (pixel = null, palette = module.exports.defaultPalette) {
  if (pixel !== null) {
    return palette[pixel] || pixel;
  } else  {
    return '\n';
  }
}
