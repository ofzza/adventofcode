package year2024

import (
	solution "adventofcode/lib"
	"errors"
	"fmt"
	"os"
	"strings"
)

// Day one definition
type Day11 struct{}

var Day = Day11{}

// Year and day
func (day Day11) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2025,
		Day:  11,
	}
}

// Executions
func (day Day11) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day11/input-test.txt"); return string(b) }(),
					Expect: 0,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day11/input.txt"); return string(b) }(),
					Expect: 0,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day11/input-test.txt"); return string(b) }(),
					Expect: 0,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day11/input.txt"); return string(b) }(),
					Expect: 0,
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day11) Run(index int, tag string, input any, verbose bool) (any, string, error) {
	// Initialize
	var output = ""
	var value, ok = input.(string)
	if !ok {
		return nil, output, errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	// TODO: ...

	// Part 1/2
	if index == 1 {

		// Mock implementation
		output += fmt.Sprintf("Not implemented! (Input: '%v')", strings.Trim(value, "\r\n"))

		// Return solution
		return 0, output, nil
	} else

	// Part 2/2
	if index == 2 {

		// Mock implementation
		output += fmt.Sprintf("Not implemented! (Input: '%v')", strings.Trim(value, "\r\n"))

		// Return solution
		return 0, output, nil
	}

	// Missing implementation
	return nil, output, errors.New("missing implementation for required index")
}
