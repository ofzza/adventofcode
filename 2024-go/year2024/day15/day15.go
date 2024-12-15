package year2024

import (
	solution "adventofcode/lib"
	"adventofcode/lib/matrix"
	"adventofcode/lib/pathing"
	"errors"
	"fmt"
	"os"
	"strings"
)

// Day one definition
type Day15 struct{}

var Day = Day15{}

// Year and day
func (day Day15) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2024,
		Day:  15,
	}
}

// Executions
func (day Day15) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day15/input-test-01.txt"); return string(b) }(),
					Expect: 2028,
				},
			)
			executions = append(
				executions,
				solution.SolutionExecution{
					Index:  1,
					Tag:    "test",
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day15/input-test-03.txt"); return string(b) }(),
					Expect: 10092,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day15/input.txt"); return string(b) }(),
					Expect: 1438161,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day15/input-test-02.txt"); return string(b) }(),
					Expect: 105,
				},
			)
			executions = append(
				executions,
				solution.SolutionExecution{
					Index:  2,
					Tag:    "test",
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day15/input-test-03.txt"); return string(b) }(),
					Expect: 9021,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day15/input.txt"); return string(b) }(),
					Expect: nil,
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day15) Run(index int, input any, verbose bool) (any, string, error) {
	// Initialize
	var output = ""
	var value, ok = input.(string)
	if !ok {
		return nil, output, errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var sections = strings.Split(strings.Trim(value, " \n"), "\n\n")

	// Parse warehouse
	var warehouseLines = strings.Split(strings.Trim(sections[0], " \n"), "\n")

	// Parse directions
	var directionLines = strings.Split(strings.Trim(sections[1], " \n"), "\n")
	var directions []byte = make([]byte, 0, len(directionLines)*len(directionLines[0]))
	for _, line := range directionLines {
		for _, r := range line {
			if r == '<' {
				directions = append(directions, pathing.DirectionLeft)
			} else if r == '>' {
				directions = append(directions, pathing.DirectionRight)
			} else if r == '^' {
				directions = append(directions, pathing.DirectionTop)
			} else if r == 'v' {
				directions = append(directions, pathing.DirectionBottom)
			}
		}
	}

	// Part 1/2
	if index == 1 {

		var width int = len(warehouseLines[0])
		var height int = len(warehouseLines)
		var robot = make([]int, 0, 2)
		var warehouse = make([]rune, 0, width*height)
		var indexer = matrix.CreateIndexer([]int{width, height})
		for y := 0; y < height; y++ {
			for x := 0; x < width; x++ {
				if warehouseLines[y][x] == '@' {
					robot = []int{x, y}
					warehouse = append(warehouse, rune('.'))
				} else {
					warehouse = append(warehouse, rune(warehouseLines[y][x]))
				}
			}
		}

		// Echo warehouse
		if verbose {
			output += fmt.Sprintf("- #%v:\n", 0)
			output += fmt.Sprintf("%v\n", echoWarehouse(warehouse, indexer, robot))
		}

		// Apply all directions
		for i, direction := range directions {
			// Update warehouse state
			robot, warehouse = updateWarehouse(robot, warehouse, indexer, direction)
			// Echo warehouse
			if verbose {
				var directionStr string
				if direction == pathing.DirectionLeft {
					directionStr = "Left"
				}
				if direction == pathing.DirectionRight {
					directionStr = "Right"
				}
				if direction == pathing.DirectionTop {
					directionStr = "Top"
				}
				if direction == pathing.DirectionBottom {
					directionStr = "Bottom"
				}
				output += fmt.Sprintf("- #%v (%v):\n", i+1, directionStr)
				output += fmt.Sprintf("%v\n", echoWarehouse(warehouse, indexer, robot))
			}
		}

		// Return count
		return calculateResult(warehouse, indexer), output, nil
	} else

	// Part 2/2
	if index == 2 {

		var width int = 2 * len(warehouseLines[0])
		var height int = len(warehouseLines)
		var robot = make([]int, 0, 2)
		var warehouse = make([]rune, 0, 2*width*height)
		var indexer = matrix.CreateIndexer([]int{width, height})
		for y := 0; y < height; y++ {
			for x := 0; x < len(warehouseLines[0]); x++ {
				if warehouseLines[y][x] == '@' {
					robot = []int{x * 2, y}
					warehouse = append(warehouse, rune('.'))
					warehouse = append(warehouse, rune('.'))
				} else if warehouseLines[y][x] == '.' {
					warehouse = append(warehouse, rune('.'))
					warehouse = append(warehouse, rune('.'))
				} else if warehouseLines[y][x] == '#' {
					warehouse = append(warehouse, rune('#'))
					warehouse = append(warehouse, rune('#'))
				} else if warehouseLines[y][x] == 'O' {
					warehouse = append(warehouse, rune('['))
					warehouse = append(warehouse, rune(']'))
				}
			}
		}

		// Echo warehouse
		if verbose {
			output += fmt.Sprintf("- #%v:\n", 0)
			output += fmt.Sprintf("%v\n", echoWarehouse(warehouse, indexer, robot))
		}

		// Apply all directions
		for i, direction := range directions {
			// Update warehouse state
			robot, warehouse = updateWarehouse(robot, warehouse, indexer, direction)
			// Echo warehouse
			if verbose {
				var directionStr string
				if direction == pathing.DirectionLeft {
					directionStr = "Left"
				}
				if direction == pathing.DirectionRight {
					directionStr = "Right"
				}
				if direction == pathing.DirectionTop {
					directionStr = "Top"
				}
				if direction == pathing.DirectionBottom {
					directionStr = "Bottom"
				}
				output += fmt.Sprintf("- #%v (%v):\n", i+1, directionStr)
				output += fmt.Sprintf("%v\n", echoWarehouse(warehouse, indexer, robot))
			}
		}

		// Return count
		return calculateResult(warehouse, indexer), output, nil
	}

	// Missing implementation
	return nil, output, errors.New("missing implementation for required index")
}

func updateWarehouse(robot []int, warehouse []rune, indexer matrix.MatrixIndexer, direction byte) ([]int, []rune) {
	// Initialize robot and warehouse updated position
	var updatedRobot = make([]int, 0, 2)
	var updatedWarehouse = make([]rune, 0, len(warehouse))
	var move = []int{0, 0}
	if direction == pathing.DirectionLeft {
		move = []int{-1, 0}
	}
	if direction == pathing.DirectionRight {
		move = []int{1, 0}
	}
	if direction == pathing.DirectionTop {
		move = []int{0, -1}
	}
	if direction == pathing.DirectionBottom {
		move = []int{0, 1}
	}
	// Check if move is possible
	var moveIndex, _ = indexer.CoordinatesToIndex([]int{robot[0] + move[0], robot[1] + move[1]})
	// If moving into empty space, move is allowed
	if warehouse[moveIndex] == '.' {
		updatedRobot = []int{robot[0] + move[0], robot[1] + move[1]}
		updatedWarehouse = warehouse
	} else
	// If moving into wall, move is not allowed
	if warehouse[moveIndex] == '#' {
		updatedRobot = robot
		updatedWarehouse = warehouse
	} else
	// If moving into a box or a double sided box, check if box(es) can be pushed
	if warehouse[moveIndex] == 'O' || warehouse[moveIndex] == '[' || warehouse[moveIndex] == ']' {
		var count, warehouseWithMovedBoxes = pushBoxes(warehouse, indexer, []int{robot[0] + move[0], robot[1] + move[1]}, move)
		// If boxes moved
		if count > 0 {
			updatedRobot = []int{robot[0] + move[0], robot[1] + move[1]}
			updatedWarehouse = warehouseWithMovedBoxes
		} else
		// If boxes can't move
		{
			updatedRobot = robot
			updatedWarehouse = warehouse
		}
	}

	// Return update robot and warehouse
	return updatedRobot, updatedWarehouse
}

func pushBoxes(warehouse []rune, indexer matrix.MatrixIndexer, box []int, move []int) (int, []rune) {
	// Check if next box over can move
	var boxIndex, _ = indexer.CoordinatesToIndex([]int{box[0], box[1]})
	var moveIndex, _ = indexer.CoordinatesToIndex([]int{box[0] + move[0], box[1] + move[1]})
	// If moving single box into empty space, or any box horizontally move is allowed
	if warehouse[moveIndex] == '.' && (warehouse[boxIndex] == 'O' || move[0] != 0) {
		warehouse[moveIndex] = warehouse[boxIndex]
		warehouse[boxIndex] = '.'
		return 1, warehouse
	} else
	// If moving double sided box vertically into empty space, check if other side of box can be pushed
	if warehouse[moveIndex] == '.' && move[1] != 0 && (warehouse[boxIndex] == '[' || warehouse[boxIndex] == ']') {
		fmt.Printf("%v\n", echoWarehouse(warehouse, indexer, []int{999, 999}))

		// Copy warehouse and move this side
		var warehouseWithMovedBoxes = append(make([]rune, 0, len(warehouse)), warehouse...)
		warehouseWithMovedBoxes[moveIndex] = warehouseWithMovedBoxes[boxIndex]
		warehouseWithMovedBoxes[boxIndex] = '.'
		fmt.Println("Moved main side into empty ...")
		fmt.Printf("%v\n", echoWarehouse(warehouseWithMovedBoxes, indexer, []int{999, 999}))
		// Check if other side can be pushed
		var sideOffset int
		if warehouse[boxIndex] == '[' {
			sideOffset = 1
		} else if warehouse[boxIndex] == ']' {
			sideOffset = -1
		}
		var count int
		// Find other box side
		var boxIndex, _ = indexer.CoordinatesToIndex([]int{box[0] + sideOffset, box[1]})
		var moveIndex, _ = indexer.CoordinatesToIndex([]int{box[0] + move[0] + sideOffset, box[1] + move[1]})
		// Check if other side needs to be checked
		if warehouseWithMovedBoxes[boxIndex] == '[' || warehouseWithMovedBoxes[boxIndex] == ']' {
			// Check if other box side can be moved
			count, warehouseWithMovedBoxes = pushBoxes(warehouse, indexer, []int{box[0] + sideOffset, box[1]}, move)
			if count > 0 {
				// Move other box part
				warehouseWithMovedBoxes[boxIndex] = warehouseWithMovedBoxes[moveIndex]
				warehouseWithMovedBoxes[moveIndex] = '.'
				fmt.Println("Moved other side into empty ...")
				fmt.Printf("%v\n", echoWarehouse(warehouseWithMovedBoxes, indexer, []int{999, 999}))
				return 1, warehouseWithMovedBoxes
			} else {
				return 0, warehouse
			}
		} else {
			return 1, warehouseWithMovedBoxes
		}
	} else
	// If moving into wall, move is not allowed
	if warehouse[moveIndex] == '#' {
		return 0, warehouse
	} else
	// If moving into a box or a double sided box horizontally, check if box(es) can be pushed
	if warehouse[moveIndex] == 'O' || (move[0] != 0 && (warehouse[moveIndex] == '[' || warehouse[moveIndex] == ']')) {
		// Check if next box over can move
		var count, warehouseWithMovedBoxes = pushBoxes(warehouse, indexer, []int{box[0] + move[0], box[1] + move[1]}, move)
		if count > 0 {
			warehouseWithMovedBoxes[moveIndex] = warehouseWithMovedBoxes[boxIndex]
			warehouseWithMovedBoxes[boxIndex] = '.'
			return count + 1, warehouseWithMovedBoxes
		} else {
			return 0, warehouse
		}
	} else
	// If moving into a double sized box in a vertical direction, check if box(es) can be pushed
	if warehouse[moveIndex] == '[' || warehouse[moveIndex] == ']' {
		fmt.Printf("%v\n", echoWarehouse(warehouse, indexer, []int{999, 999}))

		// Initialize
		var sideOffset int
		if warehouse[boxIndex] == '[' {
			sideOffset = 1
		} else if warehouse[boxIndex] == ']' {
			sideOffset = -1
		}
		var count int
		var warehouseWithMovedBoxes []rune
		// Check if direct box part can be moved
		count, warehouseWithMovedBoxes = pushBoxes(warehouse, indexer, []int{box[0] + move[0], box[1] + move[1]}, move)
		if count > 0 {
			// Move box part
			warehouseWithMovedBoxes[moveIndex] = warehouseWithMovedBoxes[boxIndex]
			warehouseWithMovedBoxes[boxIndex] = '.'
			fmt.Println("Moved main side into another box ...")
			fmt.Printf("%v\n", echoWarehouse(warehouseWithMovedBoxes, indexer, []int{999, 999}))
			// Find other box side
			var boxIndex, _ = indexer.CoordinatesToIndex([]int{box[0] + sideOffset, box[1]})
			var moveIndex, _ = indexer.CoordinatesToIndex([]int{box[0] + move[0] + sideOffset, box[1] + move[1]})
			// Check if other box side can be moved
			count, warehouseWithMovedBoxes = pushBoxes(warehouseWithMovedBoxes, indexer, []int{box[0] + move[0] + sideOffset, box[1] + move[1]}, move)
			if count > 0 {
				// Move other box part
				warehouseWithMovedBoxes[boxIndex] = warehouseWithMovedBoxes[moveIndex]
				warehouseWithMovedBoxes[moveIndex] = '.'
				fmt.Println("Moved other side  another box ...")
				fmt.Printf("%v\n", echoWarehouse(warehouseWithMovedBoxes, indexer, []int{999, 999}))
				return count, warehouseWithMovedBoxes
			} else {
				return 0, warehouse
			}
		} else {
			return 0, warehouse
		}
	}
	// This will never happen
	return 0, warehouse
}

func calculateResult(warehouse []rune, indexer matrix.MatrixIndexer) int {
	// Initialize result
	var sum int = 0
	// Calculate sum of GPS
	for i, r := range warehouse {
		if r == 'O' || r == '[' {
			var coords, _ = indexer.IndexToCoordinates(i)
			sum += 100*coords[1] + coords[0]
		}
	}
	// Return result
	return sum
}

func echoWarehouse(warehouse []rune, indexer matrix.MatrixIndexer, robot []int) string {
	// Initialize output
	var output string = ""
	// Compose warehouse layout
	for y := 0; y < indexer.GetDimensions()[1]; y++ {
		for x := 0; x < indexer.GetDimensions()[0]; x++ {
			// Check if robot
			if x == robot[0] && y == robot[1] {
				output += "@"
			} else
			// Else, output warehouse content
			{
				var i, _ = indexer.CoordinatesToIndex([]int{x, y})
				output += string(warehouse[i])
			}
		}
		output += "\n"
	}
	// Return output
	return output
}
