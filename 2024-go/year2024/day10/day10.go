package year2024

import (
	solution "adventofcode/lib"
	"adventofcode/lib/matrix"
	"errors"
	"fmt"
	"os"
	"strconv"
	"strings"
)

// Day one definition
type Day10 struct{}

var Day = Day10{}

// Year and day
func (day Day10) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2024,
		Day:  10,
	}
}

// Executions
func (day Day10) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day10/input-test.txt"); return string(b) }(),
					Expect: 36,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day10/input.txt"); return string(b) }(),
					Expect: 744,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day10/input-test.txt"); return string(b) }(),
					Expect: 81,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day10/input.txt"); return string(b) }(),
					Expect: 1651,
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day10) Run(index int, tag string, input any, verbose bool) (any, string, error) {
	// Initialize
	var output = ""
	var value, ok = input.(string)
	if !ok {
		return nil, output, errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var lines = strings.Split(strings.Trim(value, " \n"), "\n")
	var dimensions = []int{len(lines[0]), len(lines)}
	var indexer = matrix.CreateIndexer(dimensions)

	// Compose map and find all starting positions
	var topologicalMap = make([]int, 0, len(lines)*len(lines[0]))
	var trailheads = make([]int, 0, len(lines)*len(lines[0]))
	for y, line := range lines {
		for x, r := range line {
			// Add to map
			var height, _ = strconv.Atoi(fmt.Sprintf("%c", r))
			topologicalMap = append(topologicalMap, height)
			// Check if trailhead
			if r == '0' {
				var index, _ = indexer.CoordinatesToIndex([]int{x, y})
				trailheads = append(trailheads, index)
			}
		}
	}

	// Search for all trails and reachable summits from all trailheads
	var trailsCount int = 0
	var summitsCount int = 0
	for _, trailhead := range trailheads {
		// Initialize summits deduplication
		var sumits = make(map[int]bool)
		// Prompt
		if verbose {
			var coords, _ = indexer.IndexToCoordinates(trailhead)
			output += fmt.Sprintf("- Testing trailhead %v:", coords)
		}
		// Test trailhead
		var c, o = climb(topologicalMap, indexer, trailhead, func(sumit int) {
			sumits[sumit] = true
		})
		output += o
		trailsCount += c
		// Count summits
		var s int = 0
		for range sumits {
			s++
		}
		summitsCount += s
		// Prompt
		if verbose {
			output += fmt.Sprintf(" Trails=%v, Summits=%v\n", c, s)
		}
	}

	// Part 1/2
	if index == 1 {

		// Return solution
		return summitsCount, output, nil
	} else

	// Part 2/2
	if index == 2 {

		// Return solution
		return trailsCount, output, nil
	}

	// Missing implementation
	return nil, output, errors.New("missing implementation for required index")
}

func climb(topologicalMap []int, indexer matrix.MatrixIndexer, index int, callback func(sumit int)) (int, string) {
	// Initialize output
	var output string = ""
	// Look for next steps
	var count int = 0
	var neighbors, _ = indexer.GetNeighboringIndicesForIndex(index, false)
	for _, neighbor := range neighbors {
		// Check if reached the summit
		if topologicalMap[index] == 8 && topologicalMap[neighbor] == 9 {
			// Count sumit
			count++
			// Execute callback
			if callback != nil {
				callback(neighbor)
			}
		} else
		// Check if higher ground, allowed to continue the path
		if topologicalMap[neighbor] == topologicalMap[index]+1 {
			var c, o = climb(topologicalMap, indexer, neighbor, callback)
			count += c
			output += o
		}
	}
	// Return found trails
	return count, output
}
