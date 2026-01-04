package year2025

import (
	solution "adventofcode/lib"
	matrix "adventofcode/lib/matrix"
	"errors"
	"os"
	"strings"
)

// Day one definition
type Day04 struct{}

var Day = Day04{}

// Year and day
func (day Day04) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2025,
		Day:  4,
	}
}

// Executions
func (day Day04) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day04/input-test.txt"); return string(b) }(),
					Expect: 13,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day04/input.txt"); return string(b) }(),
					Expect: 1569,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day04/input-test.txt"); return string(b) }(),
					Expect: 43,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day04/input.txt"); return string(b) }(),
					Expect: 9280,
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day04) Run(index int, tag string, input any, verbose bool, log solution.Logger) (any, string, error) {
	// Initialize
	var value, ok = input.(string)
	if !ok {
		return nil, log.Dump(), errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var valueRowsStr = strings.Split(strings.Trim(value, "\r\n "), "\n")
	var dimensions = []int{len(valueRowsStr[0]), len(valueRowsStr)}
	var fieldStr = strings.ReplaceAll(value, "\n", "")
	var field = make([]bool, dimensions[0]*dimensions[1])
	for i := 0; i < len(fieldStr); i++ {
		field[i] = fieldStr[i] == byte('@')
	}

	// Part 1/2
	if index == 1 {

		// Get candidate coordinates
		var candidates = findCandidateStacks(dimensions, field, func(i int, neighbors []int) bool {
			// Check if not empty
			if !field[i] {
				return false
			}
			// Count non empty neighbors
			var valid = 0
			for _, neighbor := range neighbors {
				if i != neighbor && field[neighbor] {
					valid++
				}
			}
			return valid < 4
		})

		// Return solution
		return len(candidates), log.Dump(), nil
	} else

	// Part 2/2
	if index == 2 {

		// Remove candidates as long as possible
		var count = 0
		for {
			// Get candidate coordinates
			var candidates = findCandidateStacks(dimensions, field, func(i int, neighbors []int) bool {
				// Check if not empty
				if !field[i] {
					return false
				}
				// Count non empty neighbors
				var valid = 0
				for _, neighbor := range neighbors {
					if i != neighbor && field[neighbor] {
						valid++
					}
				}
				return valid < 4
			})

			// Count candidates
			if len(candidates) == 0 {
				break
			}
			count += len(candidates)

			// "Remove" candidates
			for _, candidate := range candidates {
				field[candidate] = false
			}
		}

		// Return solution
		return count, log.Dump(), nil
	}

	// Missing implementation
	return nil, log.Dump(), errors.New("missing implementation for required index")
}

func findCandidateStacks(dimensions []int, field []bool, validationFn func(int, []int) bool) []int {
	var valid = make([]int, 0)
	var indexer = matrix.CreateIndexer(dimensions)

	for i := 0; i < len(field); i++ {
		var neighbors, _ = indexer.GetNeighboringIndicesForIndexWithValidation(i, true, true)
		if validationFn(i, neighbors) {
			valid = append(valid, i)
		}
	}

	return valid
}
