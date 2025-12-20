package year2024

import (
	solution "adventofcode/lib"
	"errors"
	"fmt"
	"os"
	"strings"
)

// Day one definition
type Day25 struct{}

var Day = Day25{}

// Year and day
func (day Day25) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2024,
		Day:  25,
	}
}

// Executions
func (day Day25) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day25/input-test.txt"); return string(b) }(),
					Expect: 3,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day25/input.txt"); return string(b) }(),
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day25/input-test.txt"); return string(b) }(),
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day25/input.txt"); return string(b) }(),
					Expect: nil,
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day25) Run(index int, tag string, input any, verbose bool, log solution.Logger) (any, string, error) {
	// Initialize
	var value, ok = input.(string)
	if !ok {
		return nil, log.Dump(), errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var sections = strings.Split(strings.Trim(value, " \n"), "\n\n")
	var locks = make([][]int, 0, len(sections))
	var keys = make([][]int, 0, len(sections))
	for _, section := range sections {
		var section = strings.Split(strings.Trim(section, " \n"), "\n")
		// Process lock
		if section[0] == "#####" {
			var lock = make([]int, 0, len(section[0]))
			for x := 0; x < len(section[0]); x++ {
				var count int = 0
				for y := 0; y < len(section); y++ {
					if section[y][x] == '#' {
						count++
					}
				}
				lock = append(lock, count-1)
			}
			locks = append(locks, lock)
		} else
		// Process key
		if section[len(section)-1] == "#####" {
			var key = make([]int, 0, len(section[0]))
			for x := 0; x < len(section[0]); x++ {
				var count int = 0
				for y := len(section) - 1; y >= 0; y-- {
					if section[y][x] == '#' {
						count++
					}
				}
				key = append(key, count-1)
			}
			keys = append(keys, key)
		}
	}

	// Echo locks and keys
	if verbose {
		log.Log(fmt.Sprintf("- Locks: %v\n", locks))
		log.Log(fmt.Sprintf("- Keys : %v\n", keys))
	}

	// Part 1/2
	if index == 1 {

		// Match keys to locks
		var count int = 0
		for _, key := range keys {
			for _, lock := range locks {
				if len(key) == len(lock) {
					var match = true
					for i := 0; i < len(key); i++ {
						if key[i]+lock[i] > 5 {
							match = false
							break
						}
					}
					if match {
						count++
					}
				}
			}
		}

		// Return solution
		return count, log.Dump(), nil
	} else

	// Part 2/2
	if index == 2 {

		// Return solution
		return nil, log.Dump(), nil
	}

	// Missing implementation
	return nil, log.Dump(), errors.New("missing implementation for required index")
}
