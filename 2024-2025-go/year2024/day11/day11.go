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
type Day11 struct{}

var Day = Day11{}

// Year and day
func (day Day11) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2024,
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
					Input:  []any{6, func() string { var b, _ = os.ReadFile("./year2024/data/day11/input-test.txt"); return string(b) }()},
					Expect: 22,
				},
			)
		}
		if tag == "" || tag == "test" {
			executions = append(
				executions,
				solution.SolutionExecution{
					Index:  1,
					Tag:    "test",
					Input:  []any{25, func() string { var b, _ = os.ReadFile("./year2024/data/day11/input-test.txt"); return string(b) }()},
					Expect: 55312,
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
					Input:  []any{25, func() string { var b, _ = os.ReadFile("./year2024/data/day11/input.txt"); return string(b) }()},
					Expect: 211306,
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
					Input:  []any{75, func() string { var b, _ = os.ReadFile("./year2024/data/day11/input.txt"); return string(b) }()},
					Expect: 250783680217283,
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day11) Run(index int, tag string, input any, verbose bool, log solution.Logger) (any, string, error) {
	// Initialize
	var value, okValue = input.([]any)
	if !okValue {
		return nil, log.Dump(), errors.New("failed casting execution to correct Input/Output types")
	}
	var iterationCount, okIterationCount = value[0].(int)
	var values, okValues = value[1].(string)
	if !okIterationCount || !okValues {
		return nil, log.Dump(), errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var lineStr = strings.Split(strings.Trim(values, " \n"), " ")
	var stones = make(map[int]int, 0)

	// Initialize list
	for _, str := range lineStr {
		var num, _ = strconv.Atoi(str)
		stones[num] = 1
	}

	// Part 1/2 & 2/2
	if index == 1 || index == 2 {

		// Echo list
		if verbose {
			var distinct, total int = 0, 0
			for _, n := range stones {
				distinct++
				total += n
			}
			if verbose {
				log.Log(fmt.Sprintf("- %v. Stones count: %v (distinct=%v) \n", 0, total, distinct))
			}
		}

		// Run N steps
		for i := 0; i < iterationCount; i++ {
			// Preallocate enough space in the array
			var updated = make(map[int]int, 0)
			// Update list to next step
			for value, count := range stones {
				var strValue = strconv.Itoa(value)

				// Rule #1
				if value == 0 {
					updated[1] += count
				} else

				// Rule #2
				if len(strValue)%2 == 0 {
					// Update old node
					var newValueA, _ = strconv.Atoi(strValue[:len(strValue)/2])
					updated[newValueA] += count
					// Create new node
					var newValueB, _ = strconv.Atoi(strValue[len(strValue)/2:])
					updated[newValueB] += count
				} else

				// Rule #3
				{
					updated[value*2024] += count
				}
			}

			// Replace with update step
			stones = updated

			// Echo updated list
			if verbose {
				var distinct, total int = 0, 0
				for _, n := range stones {
					distinct++
					total += n
				}
				if verbose {
					log.Log(fmt.Sprintf("- %v. Stones count: %v (distinct=%v) \n", i+1, total, distinct))
				}
			}
		}

		// Count result
		var total int = 0
		for _, n := range stones {
			total += n
		}

		// Return solution
		return total, log.Dump(), nil
	}

	// Missing implementation
	return nil, log.Dump(), errors.New("missing implementation for required index")
}
