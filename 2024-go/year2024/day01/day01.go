package year2024

import (
	solution "adventofcode/lib"
	"errors"
	"os"
	"sort"
	"strconv"
	"strings"
)

// Day one definition
type Day01 struct{}

var Day = Day01{}

// Year and day
func (day Day01) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2024,
		Day:  1,
	}
}

// Executions
func (day Day01) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day01/input-test.txt"); return string(b) }(),
					Expect: 11,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day01/input.txt"); return string(b) }(),
					Expect: 1970720,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day01/input-test.txt"); return string(b) }(),
					Expect: 31,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day01/input.txt"); return string(b) }(),
					Expect: 17191599,
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day01) Run(index int, tag string, input any, verbose bool) (any, string, error) {
	// Initialize
	var output = ""
	var value, ok = input.(string)
	if !ok {
		return nil, output, errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var lines = strings.Split(strings.Trim(value, " \n"), "\n")
	var listA []int = make([]int, 0, len(lines))
	var listB []int = make([]int, 0, len(lines))
	for _, line := range lines {
		var items = strings.Split(line, "   ")
		if len(items) == 2 {
			var a, _ = strconv.Atoi(strings.Trim(items[0], " "))
			listA = append(listA, a)
			var b, _ = strconv.Atoi(strings.Trim(items[1], " "))
			listB = append(listB, b)
		}
	}

	// Part 1/2
	if index == 1 {

		// Sort inputs
		sort.Ints(listA)
		sort.Ints(listB)

		// Calculate distances
		var distances int = 0
		for i := 0; i < len(listA); i++ {
			if listB[i] > listA[i] {
				distances += listB[i] - listA[i]
			} else {
				distances += listA[i] - listB[i]
			}
		}

		// Return solution
		return distances, output, nil
	} else

	// Part 2/2
	if index == 2 {

		// Group
		var countB map[int]int = map[int]int{}
		for _, val := range listB {
			countB[val] += 1
		}

		// Calculate similarity
		var similarity int = 0
		for _, val := range listA {
			similarity += val * countB[val]
		}

		// Return solution
		return similarity, output, nil
	}

	// Missing implementation
	return nil, output, errors.New("missing implementation for required index")
}
