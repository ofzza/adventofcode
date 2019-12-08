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

module.exports.renderFactory = function (width, height) {
  return function * render (image) {
    for (let y = 0; y < height; y++) {
      for (let x = 0; x < width; x++) {
        const pixel = image[y * width + x];
        if (pixel === 2) {
          yield ' ';
        } else if (pixel === 1) {
          yield '░';
        } else if (pixel === 0){
          yield '█';
        }
      }
      yield '\n';
    }
  };
};
