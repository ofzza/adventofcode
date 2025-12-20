package year2024

import (
	solution "adventofcode/lib"
	"adventofcode/lib/matrix"
	"errors"
	"fmt"
	"os"
	"strings"
)

// Day one definition
type Day20 struct{}

var Day = Day20{}

// Year and day
func (day Day20) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2024,
		Day:  20,
	}
}

// Executions
func (day Day20) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  []any{0, 2, func() string { var b, _ = os.ReadFile("./year2024/data/day20/input-test.txt"); return string(b) }()},
					Expect: 44,
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
					Input:  []any{100, 2, func() string { var b, _ = os.ReadFile("./year2024/data/day20/input.txt"); return string(b) }()},
					Expect: 1438,
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
					Input:  []any{50, 20, func() string { var b, _ = os.ReadFile("./year2024/data/day20/input-test.txt"); return string(b) }()},
					Expect: 285,
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
					Input:  []any{100, 20, func() string { var b, _ = os.ReadFile("./year2024/data/day20/input.txt"); return string(b) }()},
					Expect: 1026446,
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day20) Run(index int, tag string, input any, verbose bool, log solution.Logger) (any, string, error) {
	// Initialize
	var inputs, inputsOk = input.([]any)
	var shortcutLength, shortcutLengthOk = inputs[0].(int)
	var minSavings, minSavingsOk = inputs[1].(int)
	var value, valueOk = inputs[2].(string)
	if !inputsOk || !shortcutLengthOk || !minSavingsOk || !valueOk {
		return nil, log.Dump(), errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var lines = strings.Split(strings.Trim(value, " \n"), "\n")
	var width int = len(lines[0])
	var height int = len(lines)
	var start int = -1
	var end int = -1
	var labyrinth = make([]rune, 0, width*height)
	var indexer = matrix.CreateIndexer([]int{width, height})
	for y := 0; y < height; y++ {
		for x := 0; x < width; x++ {
			var index, _ = indexer.CoordinatesToIndex([]int{x, y})
			if lines[y][x] == 'S' {
				start = index
				labyrinth = append(labyrinth, rune('.'))
			} else if lines[y][x] == 'E' {
				end = index
				labyrinth = append(labyrinth, rune('.'))
			} else {
				labyrinth = append(labyrinth, rune(lines[y][x]))
			}
		}
	}

	// Part 1/2 && 2/2
	if index == 1 || index == 2 {

		// Echo labyrinth
		if verbose {
			log.Log("- Labyrinth:\n")
			log.Log(echoLabyrinth(labyrinth, indexer, -1, start, end, nil))
			log.Log("\n")
		}

		// Traverse labyrinth
		var finished, path = traverseLabyrinth(labyrinth, indexer, start, end, []int{}, make(map[int]int, 0), func(path []int) {})

		// Check if ifinished
		if !finished {
			return nil, log.Dump(), errors.New("failed finding path through the labyrinth")
		}

		// Echo best path without cheating
		if verbose {
			log.Log(fmt.Sprintf("- Labyrinth path without cheating (length=%v):\n", len(path)-1))
			log.Log(echoLabyrinth(labyrinth, indexer, -1, start, end, path))
			log.Log("\n")
		}

		// Search for shortcutsCount
		var _, shortcutsCount = findPathShortcuts(labyrinth, indexer, path, minSavings, shortcutLength, func(shortcut []int) {
			// Echo best path without cheating
			if verbose {
				log.Log(fmt.Sprint("- Shortcut found:\n"))
				log.Log(echoLabyrinth(labyrinth, indexer, shortcut[0], -1, -1, shortcut))
				log.Log("\n")
			}
		})

		// Echo number of shortcuts found
		log.Log(fmt.Sprintf("- Total shortcut found: %v\n", shortcutsCount))

		// Return solution
		return shortcutsCount, log.Dump(), nil
	}

	// Missing implementation
	return nil, log.Dump(), errors.New("missing implementation for required index")
}

func traverseLabyrinth(labyrinth []rune, indexer matrix.MatrixIndexer, current int, end int, path []int, history map[int]int, callback func(path []int)) (bool, []int) {

	// Check if position already visited with lower score
	var bestCurrentPositionScoreSoFar, bestCurrentPositionScoreSoFarOk = history[current]
	if bestCurrentPositionScoreSoFarOk && len(path) >= bestCurrentPositionScoreSoFar {
		return false, []int{}
	}

	// Add current position to path and history
	history[current] = len(path)
	var updatedPath = append(path, current)

	// Check if finished
	if current == end {
		// Execute callback
		callback(updatedPath)
		// Return end path
		return true, updatedPath
	}

	// Initialize best path info
	var bestFinishPath = make([]int, 0)

	// Try visiting all neighbors
	var neighbors, _ = indexer.GetNeighboringIndicesForIndex(current, false)
	for _, neighbor := range neighbors {
		// Check if neighbor is not a wall
		if neighbor != current && labyrinth[neighbor] == '.' {
			// Continue path
			var finished, finishedPath = traverseLabyrinth(labyrinth, indexer, neighbor, end, updatedPath, history, callback)
			// If best path, store
			if finished && (len(bestFinishPath) == 0 || len(finishedPath) < len(bestFinishPath)) {
				bestFinishPath = finishedPath
			}
		}
	}

	// If path found, return path
	return len(bestFinishPath) > 0, bestFinishPath
}

func findPathShortcuts(labyrinth []rune, indexer matrix.MatrixIndexer, path []int, maxShortcutLength int, minSavings int, callback func(shortcut []int)) (bool, int) {
	// Calculate distances from end for each point on the path
	var distances = make([]int, 0, indexer.GetLength())
	for i := 0; i < indexer.GetLength(); i++ {
		distances = append(distances, -1)
	}
	for i, index := range path {
		distances[index] = i
	}

	// Initialize validShortcutsMap
	var validShortcutsMap = make(map[int]int, 0)

	// (Re)Traverse path
	for _, current := range path {
		// Search for a shortcut of limited length through the wall
		var ok, shortcuts = findPathPointShortcutsViaManhattanDistance(labyrinth, indexer, []int{}, current, maxShortcutLength, make(map[int]int, 0))
		// Validate shortcuts
		if ok && len(shortcuts) > 0 {
			for _, shortcut := range shortcuts {
				// Check length between shortcut ends, without using the shortcut
				var normalLength = distances[shortcut[len(shortcut)-1]] - distances[shortcut[0]]
				var shortcutStartCoords, _ = indexer.IndexToCoordinates(shortcut[0])
				var shortcutEndCoords, _ = indexer.IndexToCoordinates(shortcut[len(shortcut)-1])
				var shortcutXDistance = shortcutStartCoords[0] - shortcutEndCoords[0]
				if shortcutXDistance < 0 {
					shortcutXDistance *= -1
				}
				var shortcutYDistance = shortcutStartCoords[1] - shortcutEndCoords[1]
				if shortcutYDistance < 0 {
					shortcutYDistance *= -1
				}
				var shortcutLength = shortcutXDistance + shortcutYDistance
				var savedLength = normalLength - shortcutLength
				if normalLength > 0 && shortcutLength > 0 && savedLength > 0 && savedLength >= minSavings {
					// Execute callback
					if callback != nil {
						callback(shortcut)
					}
					// Store shortcut
					var key = shortcut[0]*10000000 + shortcut[len(shortcut)-1]
					validShortcutsMap[key] += 1
				}
			}
		}
	}
	// Return found shortcuts
	var validShortcutsCount int = 0
	for range validShortcutsMap {
		validShortcutsCount++
	}
	return validShortcutsCount > 0, validShortcutsCount
}

func findPathPointShortcutsViaManhattanDistance(labyrinth []rune, indexer matrix.MatrixIndexer, shortcutPath []int, current int, maxShortcutLength int, history map[int]int) (bool, [][]int) {
	// Initialize list of shortcuts
	var shortcuts = make([][]int, 0, maxShortcutLength*maxShortcutLength)
	// Search all points within given Manhattan distance
	var currentCoords, _ = indexer.IndexToCoordinates(current)
	for y := currentCoords[1] - maxShortcutLength; y <= currentCoords[1]+maxShortcutLength; y++ {
		var yDistance = currentCoords[1] - y
		if yDistance < 0 {
			yDistance *= -1
		}
		for x := currentCoords[0] - (maxShortcutLength - yDistance); x <= currentCoords[0]+(maxShortcutLength-yDistance); x++ {
			if x != currentCoords[0] || y != currentCoords[1] {
				var endpoint, err = indexer.CoordinatesToIndex([]int{x, y})
				if err == nil && labyrinth[endpoint] == '.' {
					shortcuts = append(shortcuts, []int{current, endpoint})
				}
			}
		}
	}
	// Return found shortcuts
	return len(shortcuts) > 0, shortcuts
}

func echoLabyrinth(labyrinth []rune, indexer matrix.MatrixIndexer, current int, start int, end int, path []int) string {
	// Initialize output
	var output string = ""
	// Compose warehouse layout
	for y := 0; y < indexer.GetDimensions()[1]; y++ {
		for x := 0; x < indexer.GetDimensions()[0]; x++ {
			var index, _ = indexer.CoordinatesToIndex([]int{x, y})

			// Search for location on the path
			var isPath = false
			for _, s := range path {
				if index == s {
					isPath = true
					break
				}
			}

			// Check if current position
			if index == current {
				output += "@"
			} else
			// Check if path
			if isPath {
				output += "+"
			} else
			// Check if start
			if index == start {
				output += "S"
			} else
			// Check if end
			if index == end {
				output += "E"
			} else
			// Else, output labyrinth content
			{
				var i, _ = indexer.CoordinatesToIndex([]int{x, y})
				output += string(labyrinth[i])
			}
		}
		output += "\n"
	}
	// Return output
	return output
}
