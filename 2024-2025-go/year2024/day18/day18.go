package year2024

import (
	solution "adventofcode/lib"
	"adventofcode/lib/matrix"
	"errors"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

// Day one definition
type Day18 struct{}

var Day = Day18{}

// Year and day
func (day Day18) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2024,
		Day:  18,
	}
}

// Executions
func (day Day18) GetExecutions(index int, tag string) []solution.SolutionExecution {
	var executions = []solution.SolutionExecution{}
	// Part 1/2
	if index == 0 || index == 1 {
		// Test
		if tag == "" || tag == "test" {
			executions = append(
				executions,
				solution.SolutionExecution{
					Index:  1,
					Tag:    "test",
					Input:  []any{7, 7, 12, func() string { var b, _ = os.ReadFile("./year2024/data/day18/input-test.txt"); return string(b) }()},
					Expect: 22,
				},
			)
		}
		// Solution
		if tag == "" || tag == "solution" {
			executions = append(
				executions,
				solution.SolutionExecution{
					Index:  1,
					Tag:    "solution",
					Input:  []any{71, 71, 1024, func() string { var b, _ = os.ReadFile("./year2024/data/day18/input.txt"); return string(b) }()},
					Expect: 324,
				},
			)
		}
	}
	// Part 2/2
	if index == 0 || index == 2 {
		// Test
		if tag == "" || tag == "test" {
			executions = append(
				executions,
				solution.SolutionExecution{
					Index:  2,
					Tag:    "test",
					Input:  []any{7, 7, 0, func() string { var b, _ = os.ReadFile("./year2024/data/day18/input-test.txt"); return string(b) }()},
					Expect: "6,1",
				},
			)
		}
		// Solution
		if tag == "" || tag == "solution" {
			executions = append(
				executions,
				solution.SolutionExecution{
					Index:  2,
					Tag:    "solution",
					Input:  []any{71, 71, 0, func() string { var b, _ = os.ReadFile("./year2024/data/day18/input.txt"); return string(b) }()},
					Expect: "46,23",
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day18) Run(index int, tag string, input any, verbose bool, log solution.Logger) (any, string, error) {
	// Initialize
	var inputs = input.([]any)
	var width, okWidth = inputs[0].(int)
	var height, okHeight = inputs[1].(int)
	var duration, okDuration = inputs[2].(int)
	var value, valueOk = inputs[3].(string)
	if !okWidth || !valueOk || !okHeight || !okDuration {
		return nil, log.Dump(), errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var lines = strings.Split(strings.Trim(value, " \n"), "\n")
	var coords = make([][]int, 0, len(lines))
	for _, line := range lines {
		var coordsStrs = strings.Split(strings.Trim(line, " \n"), ",")
		var x, _ = strconv.Atoi(strings.Trim(coordsStrs[0], " \n"))
		var y, _ = strconv.Atoi(strings.Trim(coordsStrs[1], " \n"))
		coords = append(coords, []int{x, y})
	}

	// Initialize matrix indexer
	var indexer = matrix.CreateIndexer([]int{width, height})

	// Echo parsed inputs
	if verbose {
		log.Log(fmt.Sprintf("[%v x %v] field, after %vns of corruption on coordinates:\n", width, height, duration))
		log.Log(fmt.Sprintf("%v\n", coords))
	}

	// Part 1/2
	if index == 1 {

		// Get corrupter memory map after specified duration
		var memory = getMemoryMap(indexer, coords[:duration])

		// Echo memory map
		if verbose {
			log.Log("\n")
			log.Log(echoMemoryMap(indexer, memory, nil))
			log.Log("\n")
		}

		// Find shortest path traversing from start to finish
		var path = make([]int, 0, width*height)
		path = append(path, 0)
		var history = make([]int, 0, width*height)
		for i := 0; i < width*height; i++ {
			history = append(history, -1)
		}
		var endIndex, _ = indexer.CoordinatesToIndex([]int{width - 1, height - 1})
		var result []int = nil
		traverseMemoryMap(indexer, memory, endIndex, path, history, func(path []int) {
			// Check found path
			log.Log(fmt.Sprintf("- Found finishing path of length %v: %v\n", len(path), path))
			// Check if new best path
			if result == nil || len(path) < len(result) {
				result = append(make([]int, 0, len(path)), path...)
			}
		})

		// Echo memory map with result path
		if verbose {
			log.Log("\n")
			log.Log(echoMemoryMap(indexer, memory, result))
		}

		// Return solution
		return len(result) - 1, log.Dump(), nil
	} else

	// Part 2/2
	if index == 2 {

		// Find duration after which the path is blocked
		var results = make([]int, 0, len(coords))
		for i := 0; i < len(coords); i++ {
			results = append(results, -1)
		}
		var duration = len(coords) / 2
		var step = len(coords) / 2
		for {

			// Get corrupter memory map after specified duration
			var memory = getMemoryMap(indexer, coords[:duration])

			// Echo testing duration
			if verbose {
				log.Log(fmt.Sprintf("- Testing duration = %vns: ", duration))
			}

			// Test if path can be found
			var path = make([]int, 0, width*height)
			path = append(path, 0)
			var history = make([]int, 0, width*height)
			for i := 0; i < width*height; i++ {
				history = append(history, -1)
			}
			var endIndex, _ = indexer.CoordinatesToIndex([]int{width - 1, height - 1})
			var resultPath []int = nil
			var ok, _ = traverseMemoryMap(indexer, memory, endIndex, path, history, func(path []int) {
				// Check if new best path
				if resultPath == nil || len(path) < len(resultPath) {
					resultPath = append(make([]int, 0, len(path)), path...)
				}
			})

			// Echo if path blocked
			if verbose {
				if ok {
					log.Log(fmt.Sprintf(" Found path of length = %v\n", len(resultPath)))
				} else {
					log.Log(" No path found!\n")
				}
			}

			// Store result
			if ok {
				results[duration] = len(resultPath)
			} else {
				results[duration] = 0
			}

			// Check if found inflection point
			if ok && results[duration+1] == 0 {
				return fmt.Sprintf("%v,%v", coords[duration][0], coords[duration][1]), log.Dump(), nil
			} else if !ok && results[duration-1] > 0 {
				return fmt.Sprintf("%v,%v", coords[duration-1][0], coords[duration-1][1]), log.Dump(), nil
			}

			// Half duration
			step = step / 2
			if step < 1 {
				step = 1
			}
			if ok {
				duration += step
			} else {
				duration -= step
			}
		}
	}

	// Missing implementation
	return nil, log.Dump(), errors.New("missing implementation for required index")
}

func getMemoryMap(indexer matrix.MatrixIndexer, coords [][]int) []bool {
	// Initialize memory map array
	var dimensions = indexer.GetDimensions()
	var memory = make([]bool, 0, dimensions[0]*dimensions[1])
	for i := 0; i < dimensions[0]*dimensions[1]; i++ {
		memory = append(memory, true)
	}
	// Reply memory corruption
	for _, coord := range coords {
		var i, _ = indexer.CoordinatesToIndex(coord)
		memory[i] = false
	}
	// Return memory map
	return memory
}

func traverseMemoryMap(indexer matrix.MatrixIndexer, memory []bool, endIndex int, path []int, history []int, callback func(path []int)) (bool, [][]int) {
	// Check if path is longer than best path to this point so far
	var currentPathLength = len(path)
	var currentIndex = path[currentPathLength-1]
	if history[currentIndex] != -1 && currentPathLength >= history[currentIndex] {
		return false, [][]int{}
	} else {
		history[currentIndex] = currentPathLength
	}

	// Check if path has reached the end
	if currentIndex == endIndex {
		// Execute callback
		callback(path)
		// Return finishing path
		return true, [][]int{append(make([]int, 0, len(path)), path...)}
	}

	// Try "walking" in all neighboring directions
	var neighbors, _ = indexer.GetNeighboringIndicesForIndex(currentIndex, false)
	var paths = make([][]int, 0, len(neighbors))
	for _, neighborIndex := range neighbors {
		// Check if neighbor on corrupted coords
		if !memory[neighborIndex] {
			continue
		}
		// Recurse and search for rest of the path
		var ok, resultingPaths = traverseMemoryMap(indexer, memory, endIndex, append(path, neighborIndex), history, callback)
		if ok {
			paths = append(paths, resultingPaths...)
		}
	}

	// Return found paths
	if len(paths) == 0 {
		return false, [][]int{}
	} else {
		return true, paths
	}
}

func echoMemoryMap(indexer matrix.MatrixIndexer, memory []bool, path []int) string {
	// Initialize outout
	var output string = ""
	// Echo memory map
	for y := 0; y < indexer.GetDimensions()[1]; y++ {
		for x := 0; x < indexer.GetDimensions()[0]; x++ {
			// Get current coords' index
			var i, _ = indexer.CoordinatesToIndex([]int{x, y})
			// Check if index is on path
			if path != nil && slices.Contains(path, i) {
				output += "+"
			} else
			// Echo memory corruption state
			if memory[i] {
				output += "."
			} else {
				output += "#"
			}
		}
		output += "\n"
	}
	// Return output
	return output
}
