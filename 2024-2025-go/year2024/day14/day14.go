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
type Day14 struct{}

var Day = Day14{}

// Year and day
func (day Day14) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2024,
		Day:  14,
	}
}

// Executions
func (day Day14) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  []any{11, 7, 100, func() string { var b, _ = os.ReadFile("./year2024/data/day14/input-test.txt"); return string(b) }()},
					Expect: 12,
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
					Input:  []any{101, 103, 100, func() string { var b, _ = os.ReadFile("./year2024/data/day14/input.txt"); return string(b) }()},
					Expect: 229868730,
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
					Input:  []any{101, 103, 100, func() string { var b, _ = os.ReadFile("./year2024/data/day14/input.txt"); return string(b) }()},
					Expect: 7861,
				},
			)
		}
	}
	return executions
}

type Robot struct {
	position []int
	velocity []int
}

// Implementation
func (day Day14) Run(index int, tag string, input any, verbose bool) (any, string, error) {
	// Initialize
	var output = ""
	var inputs = input.([]any)
	var width, okWidth = inputs[0].(int)
	var height, okHeight = inputs[1].(int)
	var duration, okDuration = inputs[2].(int)
	var value, okValue = inputs[3].(string)
	if !okWidth || !okHeight || !okDuration || !okValue {
		return nil, output, errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var lines = strings.Split(strings.Trim(value, " \n"), "\n")
	var robots = make([]Robot, 0, len(lines))
	for _, line := range lines {
		// Parse our position and velocity
		var variableStrs = strings.Split(strings.Trim(line, " "), " ")
		var positionStrs = strings.Split(strings.Trim(variableStrs[0], " p="), ",")
		var positionCoordsX, _ = strconv.Atoi(strings.Trim(positionStrs[0], " "))
		var positionCoordsY, _ = strconv.Atoi(strings.Trim(positionStrs[1], " "))
		var velocityStrs = strings.Split(strings.Trim(variableStrs[1], " v="), ",")
		var velocityCoordsX, _ = strconv.Atoi(strings.Trim(velocityStrs[0], " "))
		var velocityCoordsY, _ = strconv.Atoi(strings.Trim(velocityStrs[1], " "))
		// Initialize and store robot
		var robot = Robot{
			position: []int{positionCoordsX, positionCoordsY},
			velocity: []int{velocityCoordsX, velocityCoordsY},
		}
		robots = append(robots, robot)
	}

	// Part 1/2
	if index == 1 {

		// Echo initial robots' state
		if verbose {
			output += fmt.Sprintf("Calculating position in %vs for robots in space [%v, %v] with initial state:\n", duration, width, height)
			for _, robot := range robots {
				output += fmt.Sprintf("  - Robot: position=%v velocity=%v\n", robot.position, robot.velocity)
			}
		}

		// Calculate robot positions after requested duration
		var updated = update(robots, width, height, duration)

		// Echo robots updated state after requested duration
		if verbose {
			output += "\n"
			output += fmt.Sprintf("After %vs ...\n", duration)
		}

		// Calculate safety factor
		var midX = int(width / 2)
		var midY = int(height / 2)
		var quadrants = make([]int, 0, 4)
		for i := 0; i < 4; i++ {
			quadrants = append(quadrants, 0)
		}
		for _, robot := range updated {
			// Echo
			if verbose {
				output += fmt.Sprintf("  - Robot: position=%v velocity=%v", robot.position, robot.velocity)
			}
			// Add to correct quadrant
			if robot.position[0] < midX && robot.position[1] < midY {
				quadrants[0]++
				if verbose {
					output += fmt.Sprintln(" (Q1)")
				}
			} else if robot.position[0] > midX && robot.position[1] < midY {
				quadrants[1]++
				if verbose {
					output += fmt.Sprintln(" (Q2)")
				}
			} else if robot.position[0] < midX && robot.position[1] > midY {
				quadrants[2]++
				if verbose {
					output += fmt.Sprintln(" (Q3)")
				}
			} else if robot.position[0] > midX && robot.position[1] > midY {
				quadrants[3]++
				if verbose {
					output += fmt.Sprintln(" (Q4)")
				}
			} else if verbose {
				output += fmt.Sprintln(" (--)")
			}
		}
		var safetyScore = quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]

		// Echo robots updated state after requested duration
		if verbose {
			output += fmt.Sprintf("... robots safety score is %v!\n", safetyScore)
			output += fmt.Sprintf("  - Quadrant #1 (<%v, <%v): %v\n", midX, midY, quadrants[0])
			output += fmt.Sprintf("  - Quadrant #2 (>%v, <%v): %v\n", midX, midY, quadrants[1])
			output += fmt.Sprintf("  - Quadrant #3 (<%v, >%v): %v\n", midX, midY, quadrants[2])
			output += fmt.Sprintf("  - Quadrant #4 (>%v, >%v): %v\n", midX, midY, quadrants[3])
		}

		// Return solution
		return safetyScore, output, nil
	} else

	// Part 2/2
	if index == 2 {

		// Keep animating robots and displaying their position
		for i := 0; true; i++ {
			// Update robots position
			var updated = update(robots, width, height, i)
			// Try to detect a christmas tree
			var count int = 0
			for _, r := range updated {
				if r.position[0] > 10 && r.position[0] < width-10 && r.position[1] > 10 && r.position[1] < height-10 {
					count++
				}
			}
			var detected = float64(count)/float64(len(updated)) > 0.8
			// Display robots' position
			if detected && verbose {
				output += fmt.Sprintf("Robots' positions after %vs:\n", i)
				for y := 0; y < height; y++ {
					for x := 0; x < width; x++ {
						// Count up robots at this position
						var count int = 0
						for _, r := range updated {
							if y == r.position[1] && x == r.position[0] {
								count++
							}
						}
						// Output number of robots at this position
						if count == 0 {
							output += fmt.Sprintf("%c", '.')
						} else {
							output += fmt.Sprintf("%c", '#')
						}
					}
					output += fmt.Sprintln()
				}
				output += fmt.Sprintln()

				// Return result
				return i, output, nil
			}
		}

		// Return solution
		return nil, output, nil
	}

	// Missing implementation
	return nil, output, errors.New("missing implementation for required index")
}

func update(robots []Robot, width int, height int, duration int) []Robot {
	// Initialize updated robots' state
	var updated = make([]Robot, 0, len(robots))
	// Calculate updated state
	for _, r := range robots {
		// Calculate updated position
		var updatedX = (r.position[0] + r.velocity[0]*duration) % width
		if updatedX < 0 {
			updatedX = width + updatedX
		}
		var updatedY = (r.position[1] + r.velocity[1]*duration) % height
		if updatedY < 0 {
			updatedY = height + updatedY
		}
		// Add an updated robot state
		var robot Robot = Robot{
			position: []int{updatedX, updatedY},
			velocity: []int{r.velocity[0], r.velocity[1]},
		}
		updated = append(updated, robot)
	}
	// Return updated state
	return updated
}
