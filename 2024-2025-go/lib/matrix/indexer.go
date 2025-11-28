package matrix

import (
	"errors"
	"slices"
)

// Creates an instance of MatrixIndexer with specified dimensionality
func CreateIndexer(dimensions []int) MatrixIndexer {
	return CreateIndexerWithInfinitePlain(dimensions, false)
}

// Creates an instance of MatrixIndexer with specified dimensionality and infinite-plain mode explicitly set
func CreateIndexerWithInfinitePlain(dimensions []int, infinitePlain bool) MatrixIndexer {
	// Compose struct
	var indexer = MatrixIndexer{
		dimensions:    dimensions,
		infinitePlain: infinitePlain,
	}
	// Initialize indexer
	indexer.initialize()
	// Return indexer
	return indexer
}

// Matrix Indexer struct
type MatrixIndexer struct {
	// Total length of all cells in the matrix
	length int
	// Dimensions of a N-dimensional matrix
	dimensions []int
	// If infinite plain mode is used. When enabled, all coordinates are mapped back to the indexed area as if the edges of the area were connected on a torus.
	infinitePlain bool

	// Precalculated offsets in linear array per dimension
	dimensionOffsets []int

	// Holds coordinates of all neighbors
	relativeNeighborCoordsWithDiagonals [][]int
	// Holds coordinates of all non-diagonal neighbors
	relativeNeighborCoordsWithoutDiagonals [][]int
}

// Initialize matrix indexer based on specified dimensions
func (indexer *MatrixIndexer) initialize() {
	// Calculate and set dimension offsets and length
	indexer.dimensionOffsets = make([]int, 0, len(indexer.dimensions))
	indexer.dimensionOffsets = append(indexer.dimensionOffsets, 1)
	indexer.length = indexer.dimensions[0]
	var offset int = 1
	for i := 1; i < len(indexer.dimensions); i++ {
		indexer.length *= indexer.dimensions[i]
		offset *= indexer.dimensions[i-1]
		indexer.dimensionOffsets = append(indexer.dimensionOffsets, offset)
	}
	// Calculate relative neighboring coordinates
	var coords = make([][]int, 0, 2*len(indexer.dimensions)*len(indexer.dimensions))
	coords = append(coords, make([]int, 0, 2*len(indexer.dimensions)))
	for i := 0; i < len(indexer.dimensions); i++ {
		// Extend all coordinates with an extra dimension
		for i := range coords {
			coords[i] = append(coords[i], 0)
		}
		// For each coordinate, add neighbor in recent dimension
		var next = make([][]int, 0, len(indexer.dimensions))
		for _, coord := range coords {
			next = append(next, coord)
			var c1p = make([]int, 0, len(coord))
			var c1m = make([]int, 0, len(coord))
			for _, x := range coord {
				c1p = append(c1p, x)
				c1m = append(c1m, x)
			}
			c1p[len(c1p)-1] = 1
			next = append(next, c1p)
			c1m[len(c1p)-1] = -1
			next = append(next, c1m)
		}
		coords = next
	}
	for _, coord := range coords {
		indexer.relativeNeighborCoordsWithDiagonals = append(indexer.relativeNeighborCoordsWithDiagonals, coord)
		if slices.Contains(coord, 0) {
			indexer.relativeNeighborCoordsWithoutDiagonals = append(indexer.relativeNeighborCoordsWithoutDiagonals, coord)
		}
	}
}

// Gets set length
func (indexer *MatrixIndexer) GetLength() int {
	return indexer.length
}

// Gets set dimensions
func (indexer *MatrixIndexer) GetDimensions() []int {
	return indexer.dimensions
}

// Converts a linear index into a N-dimensional coordinate
func (indexer *MatrixIndexer) IndexToCoordinates(index int) ([]int, error) {
	if !indexer.CheckIfValidIndex(index) {
		return nil, errors.New("invalid index provided")
	}
	var normalizedIndex int
	if index >= 0 {
		normalizedIndex = index % indexer.length
	} else {
		normalizedIndex = ((-1 * ((-1 * index) % indexer.length)) + indexer.length) % indexer.length
	}
	var coords = make([]int, len(indexer.dimensions), len(indexer.dimensions))
	var remainder = normalizedIndex
	for i := (len(indexer.dimensions) - 1); i >= 0; i-- {
		var coord = remainder / indexer.dimensionOffsets[i]
		remainder = remainder % indexer.dimensionOffsets[i]
		coords[i] = coord
	}
	return coords, nil
}

// Converts a N-dimensional coordinate into a linear index
func (indexer *MatrixIndexer) CoordinatesToIndex(coords []int) (int, error) {
	if !indexer.CheckIfValidCoordinates(coords) {
		return -1, errors.New("invalid coordinates provided")
	}
	var normalizedCoords = make([]int, 0, len(coords))
	for i := 0; i < len(coords); i++ {
		if coords[i] >= 0 {
			normalizedCoords = append(normalizedCoords, (coords[i] % indexer.dimensions[i]))
		} else {
			normalizedCoords = append(normalizedCoords, (((-1 * ((-1 * coords[i]) % indexer.dimensions[i])) + indexer.dimensions[i]) % indexer.dimensions[i]))
		}
	}
	var index int = 0
	for i := 0; i < len(indexer.dimensions); i++ {
		index += indexer.dimensionOffsets[i] * normalizedCoords[i]
	}
	return index, nil
}

// Returns indices of all neighboring cells
func (indexer *MatrixIndexer) GetNeighboringIndicesForIndex(index int, diagonal bool) ([]int, error) {
	return indexer.GetNeighboringIndicesForIndexWithValidation(index, diagonal, true)
}
func (indexer *MatrixIndexer) GetNeighboringIndicesForIndexWithValidation(index int, diagonal bool, validate bool) ([]int, error) {
	var coords, err = indexer.IndexToCoordinates(index)
	if err != nil {
		return nil, err
	}
	return indexer.GetNeighboringIndicesForCoordsWithValidation(coords, diagonal, validate)
}

// Returns coordinates of all neighboring cells
func (indexer *MatrixIndexer) GetNeighboringCoordinatesForIndex(index int, diagonal bool) ([][]int, error) {
	return indexer.GetNeighboringCoordinatesForIndexWithValidation(index, diagonal, true)
}

// Returns coordinates of all neighboring cells
func (indexer *MatrixIndexer) GetNeighboringCoordinatesForIndexWithValidation(index int, diagonal bool, validate bool) ([][]int, error) {
	var coords, err = indexer.IndexToCoordinates(index)
	if err != nil {
		return nil, err
	}
	return indexer.GetNeighboringCoordinatesForCoordsWithValidation(coords, diagonal, validate)
}

// Returns indices of all neighboring cells
func (indexer *MatrixIndexer) GetNeighboringIndicesForCoords(coords []int, diagonal bool) ([]int, error) {
	return indexer.GetNeighboringIndicesForCoordsWithValidation(coords, diagonal, true)
}

// Returns indices of all neighboring cells
func (indexer *MatrixIndexer) GetNeighboringIndicesForCoordsWithValidation(coords []int, diagonal bool, validate bool) ([]int, error) {
	var neighborCoords, err = indexer.GetNeighboringCoordinatesForCoordsWithValidation(coords, diagonal, validate)
	if err != nil {
		return nil, err
	}
	var neighborIndices = make([]int, 0, len(neighborCoords))
	for _, neighborCoord := range neighborCoords {
		var index, err = indexer.CoordinatesToIndex(neighborCoord)
		if err != nil {
			return nil, err
		}
		neighborIndices = append(neighborIndices, index)
	}
	return neighborIndices, nil
}

// Returns coordinates of all neighboring cells
func (indexer *MatrixIndexer) GetNeighboringCoordinatesForCoords(coords []int, diagonal bool) ([][]int, error) {
	return indexer.GetNeighboringCoordinatesForCoordsWithValidation(coords, diagonal, true)
}

// Returns coordinates of all neighboring cells
func (indexer *MatrixIndexer) GetNeighboringCoordinatesForCoordsWithValidation(coords []int, diagonal bool, validate bool) ([][]int, error) {
	if validate && !indexer.CheckIfValidCoordinates(coords) {
		return nil, errors.New("invalid coordinates provided")
	}
	var neighbors = make([][]int, 0, len(indexer.relativeNeighborCoordsWithDiagonals))
	var relatives [][]int
	if diagonal {
		relatives = indexer.relativeNeighborCoordsWithDiagonals
	} else {
		relatives = indexer.relativeNeighborCoordsWithoutDiagonals
	}
	for _, relative := range relatives {
		var valid = true
		var neighbor = make([]int, 0, len(indexer.dimensions))
		for i := 0; i < len(indexer.dimensions); i++ {
			var x = coords[i] + relative[i]
			neighbor = append(neighbor, x)
			if validate && !indexer.infinitePlain && !((x >= 0) && (x < indexer.dimensions[i])) {
				valid = false
				break
			}
		}
		if valid {
			neighbors = append(neighbors, neighbor)
		}
	}
	return neighbors, nil
}

// Checks if index is a valid index inside the scope of the matrix
func (indexer *MatrixIndexer) CheckIfValidIndex(index int) bool {
	if indexer.infinitePlain {
		return true
	}
	return index >= 0 && index < indexer.length
}

// Checks if coordinates are valid and inside the scope of the matrix
func (indexer *MatrixIndexer) CheckIfValidCoordinates(coords []int) bool {
	if indexer.infinitePlain {
		return true
	}
	if len(coords) != len(indexer.dimensions) {
		return false
	}
	for i := 0; i < len(coords); i++ {
		if coords[i] < 0 || coords[i] >= indexer.dimensions[i] {
			return false
		}
	}
	return true
}
