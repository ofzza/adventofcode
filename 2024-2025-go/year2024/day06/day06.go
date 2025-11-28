package year2024

import (
	solution "adventofcode/lib"
	"adventofcode/lib/matrix"
	pathing "adventofcode/lib/pathing"
	"errors"
	"fmt"
	"maps"
	"os"
	"slices"
	"strings"
)

// Day one definition
type Day06 struct{}

var Day = Day06{}

// Year and day
func (day Day06) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2024,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day06/input-test.txt"); return string(b) }(),
					Expect: 41,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day06/input.txt"); return string(b) }(),
					Expect: 5409,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day06/input-test.txt"); return string(b) }(),
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day06/input.txt"); return string(b) }(),
					Expect: 2022,
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day06) Run(index int, tag string, input any, verbose bool) (any, string, error) {
	// Initialize
	var output = ""
	var value, ok = input.(string)
	if !ok {
		return nil, output, errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse and initialize inputs
	var lines = strings.Split(strings.Trim(value, " \n"), "\n")
	var obstacles = map[int]bool{}
	var obstaclesXY = map[int][]int{}
	var obstaclesYX = map[int][]int{}
	var guard = struct {
		x         int
		y         int
		direction byte
	}{direction: pathing.DirectionTop}
	var boundsHorizontal = []int{0, 0}
	var boundsVertical = []int{0, 0}

	// Process inputs
	var indexer = matrix.CreateIndexer([]int{len(lines[0]), len(lines)})
	var traversed = make(map[int]bool)
	for y, line := range lines {
		boundsVertical[1] = y
		for x, r := range line {
			boundsHorizontal[1] = x
			// Set obstacle
			if r == '#' {
				// Store by coordinate
				var _, okXY = obstaclesXY[x]
				if !okXY {
					obstaclesXY[x] = []int{}
				}
				obstaclesXY[x] = append(obstaclesXY[x], y)
				var _, okYX = obstaclesYX[y]
				if !okYX {
					obstaclesYX[y] = []int{}
				}
				obstaclesYX[y] = append(obstaclesYX[y], x)
				// Store by index
				var index, _ = indexer.CoordinatesToIndex([]int{x, y})
				obstacles[index] = true
			} else
			// Set starting position
			if r == '^' {
				guard.x = x
				guard.y = y
			}
		}
	}

	// Part 1/2
	if index == 1 {

		// Trace guard's path
		if verbose {
			output += fmt.Sprintf("Bounds: X[%v, %v] Y[%v, %v]\n", boundsHorizontal[0], boundsHorizontal[1], boundsVertical[0], boundsVertical[1])
		}
		tracePath(guard, boundsHorizontal, boundsVertical, obstaclesXY, obstaclesYX, func(direction byte, length int, a []int, b []int) bool {
			// Output section
			var label string
			if direction == pathing.DirectionRight {
				label = "RIGHT"
			}
			if direction == pathing.DirectionLeft {
				label = "LEFT "
			}
			if direction == pathing.DirectionTop {
				label = "UP   "
			}
			if direction == pathing.DirectionBottom {
				label = "DOWN "
			}
			if verbose {
				output += fmt.Sprintf("- Moving %v ([%v, %v] -> [%v, %v]) -> %v: ", label, a[0], a[1], b[0], b[1], length)
			}
			// Note all traveled positions
			var section = make([][]int, 0, length)
			if direction == pathing.DirectionRight {
				for x := a[0]; x <= b[0]; x++ {
					section = append(section, []int{x, a[1]})
				}
			}
			if direction == pathing.DirectionLeft {
				for x := a[0]; x >= b[0]; x-- {
					section = append(section, []int{x, a[1]})
				}
			}
			if direction == pathing.DirectionTop {
				for y := a[1]; y >= b[1]; y-- {
					section = append(section, []int{a[0], y})
				}
			}
			if direction == pathing.DirectionBottom {
				for y := a[1]; y <= b[1]; y++ {
					section = append(section, []int{a[0], y})
				}
			}
			for _, position := range section {
				var index, err = indexer.CoordinatesToIndex(position)
				if err == nil {
					traversed[index] = true
					if verbose {
						output += fmt.Sprintf("%v, ", position)
					}
				}
			}
			if verbose {
				output += fmt.Sprintln()
			}
			return true
		})

		// Return solution
		return len(slices.Collect(maps.Keys(traversed))), output, nil
	} else

	// Part 2/2 (working 11.5sec)
	if index == 2 {

		// Try setting additional obstacle on each position
		var count = 0
		for y := boundsVertical[0]; y <= boundsVertical[1]; y++ {
			for x := boundsHorizontal[0]; x <= boundsHorizontal[1]; x++ {
				// Prompt
				if verbose {
					output += fmt.Sprintf("- Additional obstacle: [%v, %v] -> Starting from [%v, %v]\n", x, y, guard.x, guard.y)
				}
				// fmt.Printf("- Additional obstacle: [%v, %v] -> Starting from [%v, %v]\n", x, y, guard.x, guard.y)
				// Create additional obstacle
				var updatedObstaclesXY = make(map[int][]int)
				for key, value := range obstaclesXY {
					updatedObstaclesXY[key] = append([]int{}, value...)
				}
				var updatedObstaclesYX = make(map[int][]int)
				for key, value := range obstaclesYX {
					updatedObstaclesYX[key] = append([]int{}, value...)
				}
				var updatedObstacleCoords = []int{x, y}
				// Check if obstacle can be placed: Within bounds
				if !indexer.CheckIfValidCoordinates(updatedObstacleCoords) {
					if verbose {
						output += "  ~~~> SKIP: Not within bounds!\n"
					}
					continue
				}
				// Check if obstacle can be placed: Not on start position
				if updatedObstacleCoords[0] == guard.x && updatedObstacleCoords[1] == guard.y {
					if verbose {
						output += "  ~~~> SKIP: Starting position!\n"
					}
					continue
				}
				// Check if obstacle can be placed: Not on top of existing obstacle
				var updateObstacleIndex, _ = indexer.CoordinatesToIndex(updatedObstacleCoords)
				var _, okObstacle = obstacles[updateObstacleIndex]
				if okObstacle {
					if verbose {
						output += "  ~~~> SKIP: Already obstacle!\n"
					}
					continue
				}
				// Place additional obstacle
				updatedObstaclesXY[updatedObstacleCoords[0]] = append(updatedObstaclesXY[updatedObstacleCoords[0]], updatedObstacleCoords[1])
				updatedObstaclesYX[updatedObstacleCoords[1]] = append(updatedObstaclesYX[updatedObstacleCoords[1]], updatedObstacleCoords[0])

				// // Prompt map
				// if verbose { output += "\n" }
				// // fmt.Println()
				// for y:=boundsVertical[0]; y<=boundsVertical[1]; y++ {
				// 	if verbose { output += "  " }
				// 	// fmt.Print("    ")
				// 	for x:=boundsHorizontal[0]; x<=boundsHorizontal[1]; x++ {
				// 		var foundY = false
				// 		for _, dy := range updatedObstaclesXY[x] { if dy == y { foundY = true; break} }
				// 		var foundX = false
				// 		for _, dx := range updatedObstaclesYX[y] { if dx == x { foundX = true; break } }
				// 		if verbose { if foundX { output += "#" } else if x == guard.x && y == guard.y { output += "^"} else { output += "." } }
				// 		if foundX && foundY { fmt.Sprintf("#") } else if x == guard.x && y == guard.y { fmt.Sprintf("^")} else { fmt.Sprintf(".") }
				// 	}
				// 	if verbose { output += "\n" }
				// 	// fmt.Println()
				// }
				// if verbose { output += "\n" }
				// // fmt.Println()

				// Test if looping
				var traversed = make(map[int]byte)
				var loopDetectionTraceFinished = tracePath(guard, boundsHorizontal, boundsVertical, updatedObstaclesXY, updatedObstaclesYX, func(direction byte, length int, a []int, b []int) bool {
					// Output section
					var label string
					if direction == pathing.DirectionRight {
						label = "RIGHT"
					}
					if direction == pathing.DirectionLeft {
						label = "LEFT "
					}
					if direction == pathing.DirectionTop {
						label = "UP   "
					}
					if direction == pathing.DirectionBottom {
						label = "DOWN "
					}
					if verbose {
						output += fmt.Sprintf("  - Checking %v ([%v, %v] -> [%v, %v]) -> %v: ", label, a[0], a[1], b[0], b[1], length)
					}
					// Check all traveled positions for looping
					if length < 0 {
						fmt.Printf("WARNING: negative length=%v\n", length)
					}
					var section = make([][]int, 0, length)
					if direction == pathing.DirectionRight {
						for x := a[0]; x <= b[0]; x++ {
							section = append(section, []int{x, a[1]})
						}
					}
					if direction == pathing.DirectionLeft {
						for x := a[0]; x >= b[0]; x-- {
							section = append(section, []int{x, a[1]})
						}
					}
					if direction == pathing.DirectionTop {
						for y := a[1]; y >= b[1]; y-- {
							section = append(section, []int{a[0], y})
						}
					}
					if direction == pathing.DirectionBottom {
						for y := a[1]; y <= b[1]; y++ {
							section = append(section, []int{a[0], y})
						}
					}
					for _, position := range section {

						// Check position for looping and store position
						var index, err = indexer.CoordinatesToIndex(position)
						if err == nil {
							// Get previously noted position direction(s)
							var p, ok = traversed[index]
							// Log position
							if verbose {
								output += fmt.Sprintf("%v(%v), ", position, p)
							}
							// Set/Update position direction(s)
							if !ok {
								traversed[index] = direction
							} else {
								traversed[index] = p | direction
							}
							// Check if loop detected
							if ok && (p&direction) != 0 {
								if verbose {
									output += " ~~~> LOOP DETECTED!\n"
								}
								return false
							}
						}
					}
					if verbose {
						output += "\n"
					}
					return true
				})
				if !loopDetectionTraceFinished {
					count++
					if verbose {
						output += "    ~~~> LOOP DETECTED!\n"
					}
				} else {
					if verbose {
						output += "    ~~~> no loop detected\n"
					}
				}
			}
		}

		// Return solution
		return count, output, nil
	} else

	// Part 2/2 (optimized 1.5sec, but broken)
	if index == -1 {

		// Trace guard's path
		var count int = 0
		if verbose {
			output += fmt.Sprintf("Bounds: X[%v, %v] Y[%v, %v]\n", boundsHorizontal[0], boundsHorizontal[1], boundsVertical[0], boundsVertical[1])
		}
		tracePath(guard, boundsHorizontal, boundsVertical, obstaclesXY, obstaclesYX, func(direction byte, length int, a []int, b []int) bool {
			// Output section
			var label string
			if direction == pathing.DirectionRight {
				label = "RIGHT"
			}
			if direction == pathing.DirectionLeft {
				label = "LEFT "
			}
			if direction == pathing.DirectionTop {
				label = "UP   "
			}
			if direction == pathing.DirectionBottom {
				label = "DOWN "
			}
			if verbose {
				output += fmt.Sprintf("- Moving %v ([%v, %v] -> [%v, %v]) -> %v:\n", label, a[0], a[1], b[0], b[1], length)
			}
			// Note all traveled positions
			var section = make([][]int, 0, length)
			if direction == pathing.DirectionRight {
				for x := a[0]; x <= b[0]; x++ {
					section = append(section, []int{x, a[1]})
				}
			}
			if direction == pathing.DirectionLeft {
				for x := a[0]; x >= b[0]; x-- {
					section = append(section, []int{x, a[1]})
				}
			}
			if direction == pathing.DirectionTop {
				for y := a[1]; y >= b[1]; y-- {
					section = append(section, []int{a[0], y})
				}
			}
			if direction == pathing.DirectionBottom {
				for y := a[1]; y <= b[1]; y++ {
					section = append(section, []int{a[0], y})
				}
			}
			for _, position := range section {

				// Copy obstacles (TODO: Optimization - don't copy, instead just pass additional obstacle)
				var updatedObstaclesXY = make(map[int][]int)
				for key, value := range obstaclesXY {
					updatedObstaclesXY[key] = append([]int{}, value...)
				}
				var updatedObstaclesYX = make(map[int][]int)
				for key, value := range obstaclesYX {
					updatedObstaclesYX[key] = append([]int{}, value...)
				}
				// Position additional obstacle
				var updatedObstacleCoords []int
				if direction == pathing.DirectionRight {
					updatedObstacleCoords = []int{position[0] + 1, position[1]}
				}
				if direction == pathing.DirectionLeft {
					updatedObstacleCoords = []int{position[0] - 1, position[1]}
				}
				if direction == pathing.DirectionTop {
					updatedObstacleCoords = []int{position[0], position[1] - 1}
				}
				if direction == pathing.DirectionBottom {
					updatedObstacleCoords = []int{position[0], position[1] + 1}
				}
				// Check if obstacle can be placed: Within bounds
				if !indexer.CheckIfValidCoordinates(updatedObstacleCoords) {
					continue
				}
				// Check if obstacle can be placed: Not on start position
				if updatedObstacleCoords[0] == guard.x && updatedObstacleCoords[1] == guard.y {
					continue
				}
				// Check if obstacle can be placed: Not on top of existing obstacle
				var updateObstacleIndex, _ = indexer.CoordinatesToIndex(updatedObstacleCoords)
				var _, okObstacle = obstacles[updateObstacleIndex]
				if okObstacle {
					continue
				}
				// Check if obstacle can be placed: Not on top of previously traversed path
				var _, okTraversed = traversed[updateObstacleIndex]
				if okTraversed {
					continue
				}
				// Place additional obstacle
				updatedObstaclesXY[updatedObstacleCoords[0]] = append(updatedObstaclesXY[updatedObstacleCoords[0]], updatedObstacleCoords[1])
				updatedObstaclesYX[updatedObstacleCoords[1]] = append(updatedObstaclesYX[updatedObstacleCoords[1]], updatedObstacleCoords[0])

				// Check for looping
				var loopingGuard = struct {
					x         int
					y         int
					direction byte
				}{x: position[0], y: position[1], direction: direction}
				var loopingPositions = make(map[int]byte)
				var loopDetectionTraceFinished = tracePath(loopingGuard, boundsHorizontal, boundsVertical, updatedObstaclesXY, updatedObstaclesYX, func(direction byte, length int, a []int, b []int) bool {
					// Output section
					var label string
					if direction == pathing.DirectionRight {
						label = "RIGHT"
					}
					if direction == pathing.DirectionLeft {
						label = "LEFT "
					}
					if direction == pathing.DirectionTop {
						label = "UP   "
					}
					if direction == pathing.DirectionBottom {
						label = "DOWN "
					}
					if verbose {
						output += fmt.Sprintf("  - Checking %v ([%v, %v] -> [%v, %v]) -> %v: ", label, a[0], a[1], b[0], b[1], length)
					}
					// Check all traveled positions for looping
					if length < 0 {
						fmt.Printf("WARNING: negative length=%v\n", length)
					}
					var section = make([][]int, 0, length)
					if direction == pathing.DirectionRight {
						for x := a[0]; x <= b[0]; x++ {
							section = append(section, []int{x, a[1]})
						}
					}
					if direction == pathing.DirectionLeft {
						for x := a[0]; x >= b[0]; x-- {
							section = append(section, []int{x, a[1]})
						}
					}
					if direction == pathing.DirectionTop {
						for y := a[1]; y >= b[1]; y-- {
							section = append(section, []int{a[0], y})
						}
					}
					if direction == pathing.DirectionBottom {
						for y := a[1]; y <= b[1]; y++ {
							section = append(section, []int{a[0], y})
						}
					}
					for _, position := range section {

						// Check position for looping and store position
						var index, err = indexer.CoordinatesToIndex(position)
						if err == nil {
							// Get previously noted position direction(s)
							var p, ok = loopingPositions[index]
							// Log position
							if verbose {
								output += fmt.Sprintf("%v(%v), ", position, p)
							}
							// Set/Update position direction(s)
							if !ok {
								loopingPositions[index] = direction
							} else {
								loopingPositions[index] = p | direction
							}
							// Check if loop detected
							if ok && (p&direction) != 0 {
								if verbose {
									output += " ~~~> LOOP DETECTED!\n"
								}
								return false
							}
						}

					}
					if verbose {
						output += " ~~~> No loop detected!\n"
					}
					return true
				})

				// Store position to path
				var index, err = indexer.CoordinatesToIndex(position)
				if err == nil {
					traversed[index] = true
				}

				// Count loop
				if !loopDetectionTraceFinished {
					count++
				}

			}
			if verbose {
				output += fmt.Sprintln()
			}
			return true
		})

		// Return solution
		return count, output, nil
	}

	// Missing implementation
	return nil, output, errors.New("missing implementation for required index")
}

func tracePath(
	guard struct {
		x         int
		y         int
		direction byte
	},
	boundsHorizontal []int,
	boundsVertical []int,
	obstaclesXY map[int][]int,
	obstaclesYX map[int][]int,
	pathSectionCallback func(direction byte, length int, a []int, b []int) bool,
) bool {
	// Trace guard's route
	for {
		// Horizontal
		if guard.direction&pathing.DirectionHorizontal != 0 {
			var inlinedObstacles = obstaclesYX[guard.y]
			// Right
			if guard.direction == pathing.DirectionRight {
				var nextX int = -1
				for _, x := range inlinedObstacles {
					if x > guard.x && (nextX == -1 || x < nextX) {
						nextX = x
					}
				}
				if nextX != -1 {
					nextX -= 1
				}
				if nextX != -1 {
					var length = nextX - guard.x
					if !pathSectionCallback(guard.direction, length, []int{guard.x, guard.y}, []int{nextX, guard.y}) {
						return false
					}
					guard.x = nextX
					guard.direction = pathing.DirectionBottom
				} else {
					var length = boundsHorizontal[1] - guard.x
					if !pathSectionCallback(guard.direction, length, []int{guard.x, guard.y}, []int{boundsHorizontal[1], guard.y}) {
						return false
					}
					break
				}
			} else
			// Left
			if guard.direction == pathing.DirectionLeft {
				var nextX int = -1
				for _, x := range inlinedObstacles {
					if x < guard.x && (nextX == -1 || x > nextX) {
						nextX = x
					}
				}
				if nextX != -1 {
					nextX += 1
				}
				if nextX != -1 {
					var length = guard.x - nextX
					if !pathSectionCallback(guard.direction, length, []int{guard.x, guard.y}, []int{nextX, guard.y}) {
						return false
					}
					guard.x = nextX
					guard.direction = pathing.DirectionTop
				} else {
					var length = guard.x - boundsHorizontal[0]
					if !pathSectionCallback(guard.direction, length, []int{guard.x, guard.y}, []int{boundsHorizontal[0], guard.y}) {
						return false
					}
					break
				}
			}
		} else
		// Vertical
		if guard.direction&pathing.DirectionVertical != 0 {
			var inlinedObstacles = obstaclesXY[guard.x]
			// Top
			if guard.direction == pathing.DirectionTop {
				var nextY int = -1
				for _, y := range inlinedObstacles {
					if y < guard.y && (nextY == -1 || y > nextY) {
						nextY = y
					}
				}
				if nextY != -1 {
					nextY += 1
				}
				if nextY != -1 {
					var length = guard.y - nextY
					if !pathSectionCallback(guard.direction, length, []int{guard.x, guard.y}, []int{guard.x, nextY}) {
						return false
					}
					guard.y = nextY
					guard.direction = pathing.DirectionRight
				} else {
					var length = guard.y - boundsVertical[0]
					if !pathSectionCallback(guard.direction, length, []int{guard.x, guard.y}, []int{guard.x, boundsVertical[0]}) {
						return false
					}
					break
				}
			} else
			// Bottom
			if guard.direction == pathing.DirectionBottom {
				var nextY int = -1
				for _, y := range inlinedObstacles {
					if y > guard.y && (nextY == -1 || y < nextY) {
						nextY = y
					}
				}
				if nextY != -1 {
					nextY -= 1
				}
				if nextY != -1 {
					var length = nextY - guard.y
					if !pathSectionCallback(guard.direction, length, []int{guard.x, guard.y}, []int{guard.x, nextY}) {
						return false
					}
					guard.y = nextY
					guard.direction = pathing.DirectionLeft
				} else {
					var length = boundsVertical[1] - guard.y
					if !pathSectionCallback(guard.direction, length, []int{guard.x, guard.y}, []int{guard.x, boundsVertical[1]}) {
						return false
					}
					break
				}
			}
		}
	}
	// Return successfully traced path
	return true
}
