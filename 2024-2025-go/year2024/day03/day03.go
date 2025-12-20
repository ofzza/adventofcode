package year2024

import (
	solution "adventofcode/lib"
	"errors"
	"os"
	"regexp"
	"strconv"
	"strings"
)

// Day one definition
type Day03 struct{}

var Day = Day03{}

// Year and day
func (day Day03) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2024,
		Day:  3,
	}
}

// Executions
func (day Day03) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day03/input-test-01.txt"); return string(b) }(),
					Expect: 161,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day03/input.txt"); return string(b) }(),
					Expect: 175615763,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day03/input-test-02.txt"); return string(b) }(),
					Expect: 48,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day03/input.txt"); return string(b) }(),
					Expect: 74361272,
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day03) Run(index int, tag string, input any, verbose bool, log solution.Logger) (any, string, error) {
	// Initialize
	var value, ok = input.(string)
	if !ok {
		return nil, log.Dump(), errors.New("failed casting execution to correct Input/Output types")
	}

	// Part 1/2
	if index == 1 {

		// Parse inputs
		var r = regexp.MustCompile(`mul\((\d{1,3}),(\d{1,3})\)`)
		var instructions = r.FindAllStringSubmatch(strings.Trim(value, " "), -1)

		// Run multiplication instructions
		var result int = 0
		for _, instruction := range instructions {
			var a, _ = strconv.Atoi(instruction[1])
			var b, _ = strconv.Atoi(instruction[2])
			result += a * b
		}

		// Return solution
		return result, log.Dump(), nil
	} else

	// Part 2/2
	if index == 2 {

		// Parse inputs
		var r = regexp.MustCompile(`(mul\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don't\(\))`)
		var instructions = r.FindAllStringSubmatch(strings.Trim(value, " "), -1)

		// Run conditional and multiplication instructions
		var do = true
		var result int = 0
		for _, instruction := range instructions {
			// Handle "don't()"" instruction
			if len(instruction[0]) >= 5 && instruction[0][:5] == "don't" {
				do = false
			} else
			// Handle "do()" instruction
			if len(instruction[0]) >= 2 && instruction[0][:2] == "do" {
				do = true
			} else
			// Handle "mul()" instruction
			if len(instruction[0]) >= 3 && instruction[0][:3] == "mul" && do {
				var a, _ = strconv.Atoi(instruction[2])
				var b, _ = strconv.Atoi(instruction[3])
				result += a * b
			}
		}

		// Return solution
		return result, log.Dump(), nil
	}

	// Missing implementation
	return nil, log.Dump(), errors.New("missing implementation for required index")
}
