// IMAGE PROCESSING

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

// Renders 1D array as image
module.exports.renderLinearFactory = function renderLinearFactory ({ width, height, extract = null } = {}) {
  return function * render (image) {
    // Check if image needs to be extracted from result
    if (extract !== null) { image = extract(image); }
    // Render image
    for (let y = 0; y < height; y++) {
      for (let x = 0; x < width; x++) {
        yield drawPixel(image[y * width + x]);
      }
      yield drawPixel(null);
    }
  };
};

// Renders 2D array as image
module.exports.renderFieldFactory = function renderFieldFactory ({ extract = null } = {}) {
  return function * render (image) {
    // Check if image needs to be extracted from result
    if (extract !== null) { image = extract(image); }
    // Render image
    for (let y = 0; y < image.length; y++) {
      for (let x = 0; x < image[0].length; x++) {
        yield drawPixel(image[y][x]);
      }
      yield drawPixel(null);
    }
  };
};

// Draw a pixes as ASCII
function drawPixel (pixel) {
  if (pixel === 2) {
    return ' ';
  } else if (pixel === 1) {
    return '█';
  } else if (pixel === 0){
    return '░';
  } else if (pixel === null) {
    return '\n';
  }
}
