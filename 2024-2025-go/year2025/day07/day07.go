package year2025

import (
	solution "adventofcode/lib"
	"errors"
	"fmt"
	"os"
	"strings"
)

// Day one definition
type Day07 struct{}

var Day = Day07{}

// Year and day
func (day Day07) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2025,
		Day:  7,
	}
}

// Executions
func (day Day07) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day07/input-test.txt"); return string(b) }(),
					Expect: 21,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day07/input.txt"); return string(b) }(),
					Expect: 1587,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day07/input-test.txt"); return string(b) }(),
					Expect: 40,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day07/input.txt"); return string(b) }(),
					Expect: 5748679033029,
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day07) Run(index int, tag string, input any, verbose bool, log solution.Logger) (any, string, error) {
	// Initialize
	var value, ok = input.(string)
	if !ok {
		return nil, log.Dump(), errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var lines = strings.Split(strings.Trim(value, "\r\n "), "\n")
	var start = strings.Index(lines[0], "S")

	// Part 1/2
	if index == 1 {

		// Initialize tachyon beams
		var beams = make([]bool, len(lines[0]))
		beams[start] = true

		// Echo initial line
		if verbose {
			var beamsInt = make([]int, len(beams))
			for i, beam := range beams {
				if beam {
					beamsInt[i] = 1
				}
			}
			log.Log(fmt.Sprintf("%s\n", echoLine(lines[0], beamsInt)))
		}

		// Simulate tachyon beams
		var splits = 0
		for y := 1; y < len(lines); y++ {
			// Initialize next line
			var next = make([]bool, len(beams))

			// Look for beams hitting splitters
			for x, beam := range beams {
				if beam {
					if lines[y][x:x+1] != "^" {
						next[x] = true
					} else {
						splits++
						if x > 0 {
							next[x-1] = true
						}
						if x < len(next)-1 {
							next[x+1] = true
						}
					}
				}
			}

			// Set next line
			beams = next

			// Echo next line
			if verbose {
				var beamsInt = make([]int, len(beams))
				for i, beam := range beams {
					if beam {
						beamsInt[i] = 1
					}
				}
				log.Log(fmt.Sprintf("%s\n", echoLine(lines[y], beamsInt)))
			}
		}

		// Return solution
		return splits, log.Dump(), nil
	} else

	// Part 2/2
	if index == 2 {

		// Initialize tachyon beams
		var beams = make([]int, len(lines[0]))
		beams[start] = 1

		// Echo initial line
		if verbose {
			log.Log(fmt.Sprintf("%s\n", echoLine(lines[0], beams)))
		}

		// Simulate tachyon beams
		var splits = 0
		for y := 1; y < len(lines); y++ {
			// Initialize next line
			var next = make([]int, len(beams))

			// Look for beams hitting splitters
			for x, beam := range beams {
				if beam > 0 {
					if lines[y][x:x+1] != "^" {
						next[x] = next[x] + beam
					} else {
						splits++
						if x > 0 {
							next[x-1] = next[x-1] + beam
						}
						if x < len(next)-1 {
							next[x+1] = next[x+1] + beam
						}
					}
				}
			}

			// Set next line
			beams = next

			// Echo next line
			if verbose {
				log.Log(fmt.Sprintf("%s\n", echoLine(lines[y], beams)))
			}
		}

		// Sum up all the worlds
		var sum = 0
		for x := 0; x < len(beams); x++ {
			sum += beams[x]
		}

		// Return solution
		return sum, log.Dump(), nil
	}

	// Missing implementation
	return nil, log.Dump(), errors.New("missing implementation for required index")
}

func echoLine(line string, beams []int) string {
	var output = ""
	for i := 0; i < len(beams); i++ {
		if beams[i] == 1 {
			output += "|"
		} else if beams[i] > 1 {
			output += fmt.Sprint(beams[i] % 10)
		} else if i < len(line) {
			output += line[i : i+1]
		}
	}
	return output
}
