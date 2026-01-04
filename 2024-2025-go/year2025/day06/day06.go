package year2025

import (
	solution "adventofcode/lib"
	"errors"
	"os"
	"regexp"
	"strconv"
	"strings"
)

// Day one definition
type Day06 struct{}

var Day = Day06{}

// Year and day
func (day Day06) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2025,
		Day:  6,
	}
}

// Executions
func (day Day06) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day06/input-test.txt"); return string(b) }(),
					Expect: 4277556,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day06/input.txt"); return string(b) }(),
					Expect: 4693159084994,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day06/input-test.txt"); return string(b) }(),
					Expect: 3263827,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day06/input.txt"); return string(b) }(),
					Expect: 11643736116335,
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day06) Run(index int, tag string, input any, verbose bool, log solution.Logger) (any, string, error) {
	// Initialize
	var value, ok = input.(string)
	if !ok {
		return nil, log.Dump(), errors.New("failed casting execution to correct Input/Output types")
	}

	// Part 1/2
	if index == 1 {

		// Clean up inputs
		value = regexp.MustCompile(` {2,}`).ReplaceAllString(value, " ")
		value = regexp.MustCompile(`\n {1,}`).ReplaceAllString(value, "\n")
		value = regexp.MustCompile(` {1,}\n`).ReplaceAllString(value, "\n")

		// Parse inputs
		var linesStr = strings.Split(strings.Trim(value, "\r\n "), "\n")
		var lines = make([][]string, len(linesStr))
		for i := 0; i < len(linesStr); i++ {
			var line = strings.Split(strings.Trim(linesStr[i], "\r\n "), " ")
			lines[i] = line
		}

		var assignments = make([]Assignemt, len(lines[0]))
		for i := 0; i < len(lines[0]); i++ {
			var assigment = Assignemt{
				args:     make([]int, len(lines)-1),
				operator: lines[len(lines)-1][i],
			}
			for j := 0; j < len(lines)-1; j++ {
				n, _ := strconv.Atoi(lines[j][i])
				assigment.args[j] = n
			}
			assignments[i] = assigment
		}

		// Calculate homework
		var sum = 0
		for _, assignment := range assignments {
			switch assignment.operator {
			case "+":
				var result = 0
				for _, arg := range assignment.args {
					result += arg
				}
				sum += result
			case "*":
				var result = 1
				for _, arg := range assignment.args {
					result *= arg
				}
				sum += result
			}
		}

		// Return solution
		return sum, log.Dump(), nil
	} else

	// Part 2/2
	if index == 2 {

		// Parse inputs (using cephalopod notation)
		var linesStr = strings.Split(strings.Trim(value, "\r\n "), "\n")
		var assignments = make([]Assignemt, 0)
		var assignment = Assignemt{
			args: make([]int, 0),
		}
		for i := len(linesStr[0]) - 1; i >= 0; i-- {
			// Read rightmost digits
			var digits = make([]string, len(linesStr)-1)
			for j := 0; j < len(linesStr)-1; j++ {
				if len(linesStr[j]) < i+1 {
					digits[j] = " "
				} else {
					digits[j] = linesStr[j][i : i+1]
				}
			}
			var digitsStr = strings.Join(digits, "")
			arg, _ := strconv.Atoi(strings.Trim(digitsStr, " "))
			assignment.args = append(assignment.args, arg)

			// Look for operator
			if len(linesStr[len(linesStr)-1]) >= i+1 && (linesStr[len(linesStr)-1][i:i+1] == "+" || linesStr[len(linesStr)-1][i:i+1] == "*") {
				assignment.operator = linesStr[len(linesStr)-1][i : i+1]
				assignments = append(assignments, assignment)
				assignment = Assignemt{
					args: make([]int, 0),
				}
				i--
			}
		}

		// Calculate homework
		var sum = 0
		for _, assignment := range assignments {
			switch assignment.operator {
			case "+":
				var result = 0
				for _, arg := range assignment.args {
					result += arg
				}
				sum += result
			case "*":
				var result = 1
				for _, arg := range assignment.args {
					result *= arg
				}
				sum += result
			}
		}

		// Return solution
		return sum, log.Dump(), nil
	}

	// Missing implementation
	return nil, log.Dump(), errors.New("missing implementation for required index")
}

type Assignemt struct {
	args     []int
	operator string
}
