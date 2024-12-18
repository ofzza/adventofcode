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
type Day02 struct{}

var Day = Day02{}

// Year and day
func (day Day02) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2024,
		Day:  2,
	}
}

// Executions
func (day Day02) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day02/input-test.txt"); return string(b) }(),
					Expect: 2,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day02/input.txt"); return string(b) }(),
					Expect: 218,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day02/input-test.txt"); return string(b) }(),
					Expect: 4,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day02/input.txt"); return string(b) }(),
					Expect: 290,
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day02) Run(index int, tag string, input any, verbose bool) (any, string, error) {
	// Initialize
	var output = ""
	var value, ok = input.(string)
	if !ok {
		return nil, output, errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var lines = strings.Split(strings.Trim(value, " \n"), "\n")
	var reports = [][]int{}
	for _, line := range lines {
		var items = strings.Split(strings.Trim(line, " "), " ")
		var report = make([]int, 0, len(items))
		for _, item := range items {
			var n, _ = strconv.Atoi(strings.Trim(item, " "))
			report = append(report, n)
		}
		reports = append(reports, report)
	}

	// Part 1/2
	if index == 1 {

		// Count "safe" reports
		var safe = 0
		for reportIndex, report := range reports {
			// Check if safe
			var unsafeIndices, unsafeOutput = findUnsafe(report, verbose)
			// Handle unsafe report
			if unsafeIndices != nil {
				if verbose {
					output += fmt.Sprintf("- Unsafe report #%v %v: %v\n", reportIndex, report, unsafeOutput)
				}
			} else
			// Count as safe
			{
				safe++
			}
		}

		// Return solution
		return safe, output, nil
	} else

	// Part 2/2
	if index == 2 {

		// Count "safe" reports
		var safe = 0
	report:
		for reportIndex, report := range reports {
			var unsafeIndices, unsafeOutput = findUnsafe(report, verbose)
			// Handle unsafe report
			if unsafeIndices != nil {
				if verbose {
					output += fmt.Sprintf("- Unsafe report #%v %v: %v\n", reportIndex, report, unsafeOutput)
				}
				// Try removing potentially unsafe levels
				for _, unsafeIndex := range unsafeIndices {
					var removed = []int{}
					removed = append(removed, report[:unsafeIndex]...)
					removed = append(removed, report[unsafeIndex+1:]...)
					var unsafeIndices, unsafeOutput = findUnsafe(removed, verbose)
					if unsafeIndices != nil {
						if verbose {
							output += fmt.Sprintf("  - Unsafe after retry having dropped #%v %v: %v\n", unsafeIndex, removed, unsafeOutput)
						}
					} else
					// Count as safe after retry
					{
						safe++
						if verbose {
							output += fmt.Sprintf("  ... SAFE after retry having dropped #%v %v!\n", unsafeIndex, removed)
						}
						continue report
					}
				}
			} else
			// Count as safe
			{
				safe++
			}
		}

		// Return solution
		return safe, output, nil
	}

	// Missing implementation
	return nil, output, errors.New("missing implementation for required index")
}

func findUnsafe(report []int, verbose bool) ([]int, string) {
	var output = ""
	var ascs = [][]int{}
	var descs = [][]int{}
	for i := 1; i < len(report); i++ {
		// Check asc/desc
		if report[i-1] == report[i] {
			if verbose {
				output += fmt.Sprintf("Level: [%v %v]", report[i-1], report[i])
			}
			return []int{i - 1, i}, output
		}
		var asc = report[i-1] < report[i]
		// Check neighboring diff
		if !asc && (report[i-1]-report[i] < 1 || report[i-1]-report[i] > 3) {
			if verbose {
				output += fmt.Sprintf("Diff <1 || >3: %v - %v = %v", report[i-1], report[i], (report[i-1] - report[i]))
			}
			return []int{i - 1, i}, output
		}
		if asc && (report[i]-report[i-1] < 1 || report[i]-report[i-1] > 3) {
			if verbose {
				output += fmt.Sprintf("Diff <1 || >3: %v - %v = %v", report[i], report[i-1], (report[i] - report[i-1]))
			}
			return []int{i - 1, i}, output
		}
		// Count asc/desc
		if asc {
			ascs = append(ascs, []int{i - 1, i})
		} else {
			descs = append(descs, []int{i - 1, i})
		}
		if len(ascs) > 0 && len(descs) > 0 {
			if verbose {
				output += fmt.Sprintf("Inflection: [%v %v]", report[i-1], report[i])
			}
			return append([]int{}, ascs[len(ascs)-1][0], ascs[len(ascs)-1][1], descs[len(descs)-1][0], descs[len(descs)-1][1]), output
		}
	}
	return nil, output
}
