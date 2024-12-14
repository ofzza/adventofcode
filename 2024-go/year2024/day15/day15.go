package year2024

import (
	solution "adventofcode/lib"
	"errors"
	"os"
	"strings"
)

// Day one definition
type Day15 struct{}

var Day = Day15{}

// Year and day
func (day Day15) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2024,
		Day:  15,
	}
}

// Executions
func (day Day15) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day15/input-test.txt"); return string(b) }(),
					Expect: nil,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day15/input.txt"); return string(b) }(),
					Expect: nil,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day15/input-test.txt"); return string(b) }(),
					Expect: nil,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day15/input.txt"); return string(b) }(),
					Expect: nil,
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day15) Run(index int, input any, verbose bool) (any, string, error) {
	// Initialize
	var output = ""
	var value, ok = input.(string)
	if !ok {
		return nil, output, errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var lines = strings.Split(strings.Trim(value, " \n"), "\n")

	// Part 1/2
	if index == 1 {

		// Return count
		return lines, output, nil
	} else

	// Part 2/2
	if index == 2 {

		// Return count
		return lines, output, nil
	}

	// Missing implementation
	return nil, output, errors.New("missing implementation for required index")
}
