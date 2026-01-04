package year2025

import (
	solution "adventofcode/lib"
	"errors"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

// Day one definition
type Day01 struct{}

var Day = Day01{}

// Year and day
func (day Day01) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2025,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day01/input-test.txt"); return string(b) }(),
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day01/input.txt"); return string(b) }(),
					Expect: 989,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day01/input-test.txt"); return string(b) }(),
					Expect: 6,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day01/input.txt"); return string(b) }(),
					Expect: 5941,
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day01) Run(index int, tag string, input any, verbose bool, log solution.Logger) (any, string, error) {
	// Initialize
	var value, ok = input.(string)
	if !ok {
		return nil, log.Dump(), errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var instructions = make([]struct {
		direction byte
		distance  int
	}, 0)
	for _, instruction := range strings.Split(strings.Trim(value, "\r\n "), "\n") {
		var trimmed = strings.Trim(instruction, "\r\n ")
		var direction = trimmed[0]
		var distance, _ = strconv.Atoi(trimmed[1:])
		instructions = append(instructions, struct {
			direction byte
			distance  int
		}{direction: direction, distance: distance})
	}

	// Starting position
	var position int = 50

	// Part 1/2
	if index == 1 {

		// Count number of times dial is at zero
		var count = 0

		// Log state
		if verbose {
			log.Log(fmt.Sprintf("> Position: %d (%d)\n", position, count))
		}

		// Follow instructions
		for _, instruction := range instructions {
			switch instruction.direction {
			case 'R':
				position = (((instruction.distance/100)+1)*100 + position + instruction.distance) % 100
				log.Log(fmt.Sprintf("> Move R %d >>> Position: %d (%d)\n", instruction.distance, position, count))
			case 'L':
				position = (((instruction.distance/100)+1)*100 + position - instruction.distance) % 100
				log.Log(fmt.Sprintf("> Move L %d >>> Position: %d (%d)\n", instruction.distance, position, count))
			}
			if position == 0 {
				count++
			}
		}

		// Return solution
		return count, log.Dump(), nil
	} else

	// Part 2/2
	if index == 2 {

		// Count number of times dial is at zero
		var count = 0

		// Log state
		if verbose {
			log.Log(fmt.Sprintf("> Position: %d (%d)\n", position, count))
		}

		// Follow instructions
		for _, instruction := range instructions {
			switch instruction.direction {
			case 'R':
				if position < 0 && position+instruction.distance >= 0 {
					count++
				}
				position = position + instruction.distance
				count += position / 100
				position = position % 100
				log.Log(fmt.Sprintf("> Move R %d >>> Position: %d (%d)\n", instruction.distance, position, count))
			case 'L':
				if position > 0 && position-instruction.distance <= 0 {
					count++
				}
				position = position - instruction.distance
				count += int(math.Abs(float64(position))) / 100
				position = (100 + -1*((-1*position)%100)) % 100
				log.Log(fmt.Sprintf("> Move L %d >>> Position: %d (%d)\n", instruction.distance, position, count))
			}
		}

		// Return solution
		return count, log.Dump(), nil
	}

	// Missing implementation
	return nil, log.Dump(), errors.New("missing implementation for required index")
}
