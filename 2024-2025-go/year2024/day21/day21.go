package year2024

import (
	solution "adventofcode/lib"
	"errors"
	"fmt"
	"os"
	"strconv"
	"strings"
)

// Day one definition
type Day21 struct{}

var Day = Day21{}

// Year and day
func (day Day21) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2024,
		Day:  21,
	}
}

// Executions
func (day Day21) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day21/input-test.txt"); return string(b) }(),
					Expect: 126384,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day21/input.txt"); return string(b) }(),
					Expect: 177814,
				},
			)
		}
	}
	// Part 2/2
	if index == 0 || index == 2 {
		// Solution
		if tag == "" || tag == "solution" {
			executions = append(
				executions,
				solution.SolutionExecution{
					Index:  2,
					Tag:    "solution",
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day21/input.txt"); return string(b) }(),
					Expect: 220493992841852,
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day21) Run(index int, tag string, input any, verbose bool) (any, string, error) {
	// Initialize
	var output = ""
	var value, ok = input.(string)
	if !ok {
		return nil, output, errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var codes = strings.Split(strings.Trim(value, " \n"), "\n")

	// Set number of directional terminals
	var directionalCount int
	// Part 1/2
	if index == 1 {
		directionalCount = 2
	} else
	// Part 2/2
	if index == 2 {
		directionalCount = 25
	}

	// Find shortest robot command sequences for each code
	var score int = 0
	for _, code := range codes {
		// Echo starting work on code
		if verbose {
			output += fmt.Sprintf("- Processing input code '%v' ...", code)
		}

		// Initialize keymaps
		var keymaps = []KeyMap{numericKeymap}
		for i := 0; i < directionalCount; i++ {
			keymaps = append(keymaps, directionalKeymap)
		}

		// Try piping commands
		var minSequenceLength = pipeCommands(code, keymaps, make(map[int]int, 0))

		// Calculate sequence score
		var sequenceValue, _ = strconv.ParseInt(code[:len(code)-1], 10, 64)
		var sequenceScore = int(sequenceValue) * minSequenceLength
		score += sequenceScore

		// Echo completed code
		if verbose {
			output += fmt.Sprintf(" ... found solutions, minimal of which has length=%v, score=%v\n", minSequenceLength, sequenceScore)
		}
	}

	// Return solution
	return score, output, nil
}

func pipeCommands(code string, keymaps []KeyMap, cache map[int]int) int {
	// Initialize
	var robots = make([]Robot, 0, len(keymaps))
	for _, keymap := range keymaps {
		robots = append(robots, Robot{key: 'A', keymap: keymap})
	}
	// Pass commands to first robot and pipe to last
	return pipeCommandsInternal(code, robots, 0, cache)
}
func pipeCommandsInternal(code string, robots []Robot, depth int, cache map[int]int) int {
	// Update current robot
	var robotsUpdated = append(make([]Robot, 0, len(robots)), robots...)
	// Process each of the code keys with first robot
	var minCodeSequenceLength int = 0
	for _, key := range code {
		// Generate sequences from next robot
		var minKeySequenceLength = pipeKeyInternal(key, robotsUpdated, depth, cache)
		// Append to previously generated sequences
		minCodeSequenceLength += minKeySequenceLength
		// Next key for current robot
		robotsUpdated[0].key = key
	}
	// Return generated sequences
	return minCodeSequenceLength
}
func pipeKeyInternal(nextKey rune, robots []Robot, depth int, cache map[int]int) int {
	// Check cache
	var cacheKey = generateCacheKey(depth, robots[0].key, nextKey)
	var cachedSequences, cacheOk = cache[cacheKey]
	if cacheOk {
		return cachedSequences
	}
	// Pipe key to next robot
	var paths, _ = robots[0].pathToNextKey(nextKey)
	// If no next robots, return paths
	if len(robots) == 1 {
		// Get shortest path
		var length = shortest(paths)
		// Write to cache
		cache[cacheKey] = length
		// Return final paths
		return length
	} else
	// If robots remaining ...
	{
		// For each path, pipe to next robot
		var minSequenceLength int = -1
		for _, path := range paths {
			var pathSequenceLength = pipeCommandsInternal(path, robots[1:], depth+1, cache)
			if minSequenceLength == -1 || pathSequenceLength < minSequenceLength {
				minSequenceLength = pathSequenceLength
			}
		}
		// Write to cache
		cache[cacheKey] = minSequenceLength
		// Return collected sequences
		return minSequenceLength
	}
}

func shortest[T []any | string](sequences []T) int {
	var length int = -1
	for _, sequence := range sequences {
		if length == -1 || len(sequence) < length {
			length = len(sequence)
		}
	}
	return length
}

func generateCacheKey(depth int, currentKey rune, nextKey rune) int {
	return 1e9*depth + 1e6*int([]byte(string(currentKey))[0]) + 1e3*int([]byte(string(nextKey))[0])
}
