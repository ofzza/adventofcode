package year2025

import (
	solution "adventofcode/lib"
	"errors"
	"math"
	"os"
	"strconv"
	"strings"
)

// Day one definition
type Day09 struct{}

var Day = Day09{}

// Year and day
func (day Day09) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2025,
		Day:  9,
	}
}

// Executions
func (day Day09) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day09/input-test.txt"); return string(b) }(),
					Expect: 50,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day09/input.txt"); return string(b) }(),
					Expect: 4735222687,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day09/input-test.txt"); return string(b) }(),
					Expect: 24,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day09/input.txt"); return string(b) }(),
					Expect: 1569262188,
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day09) Run(index int, tag string, input any, verbose bool, log solution.Logger) (any, string, error) {
	// Initialize
	var value, ok = input.(string)
	if !ok {
		return nil, log.Dump(), errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var linesStr = strings.Split(strings.Trim(value, "\r\n "), "\n")
	var coords = make([]Coord, len(linesStr))
	for i, lineStr := range linesStr {
		var coordsStr = strings.Split(strings.Trim(lineStr, "\r\n "), ",")
		var x, _ = strconv.Atoi(strings.Trim(coordsStr[0], "\r\n "))
		var y, _ = strconv.Atoi(strings.Trim(coordsStr[1], "\r\n "))
		coords[i] = Coord{x, y}
	}

	// if !validateNoSuperfluousLines(coords) {
	// 	return 0, log.Dump(), errors.New("detected superfluous lines - need to implemented preprocessing step")
	// }
	// if !validateNoOverlappingLines(coords) {
	// 	return 0, log.Dump(), errors.New("detected overlapping lines - ouch")
	// }

	// Part 1/2
	if index == 1 {

		// Find largest rectangle
		var max = 0
		for i := 0; i < len(coords)-1; i++ {
			for j := i + 1; j < len(coords); j++ {
				var area = int(math.Abs(float64((coords[i].x - coords[j].x + 1) * (coords[i].y - coords[j].y + 1))))
				if area > max {
					max = area
				}
			}
		}

		// Return solution
		return max, log.Dump(), nil
	} else

	// Part 2/2
	if index == 2 {

		// Find largest internal rectangle
		var max = 0
		for i := 0; i < len(coords)-1; i++ {
			for j := i + 1; j < len(coords); j++ {
				var area = int((math.Abs(float64(coords[i].x-coords[j].x)) + 1) * (math.Abs(float64(coords[i].y-coords[j].y)) + 1))
				if area > max && isValidArea(coords, i, j) {
					max = area
				}
			}
		}
		// Return solution
		return max, log.Dump(), nil
	}

	// Missing implementation
	return nil, log.Dump(), errors.New("missing implementation for required index")
}

type Coord struct {
	x int
	y int
}

func validateNoSuperfluousLines(coords []Coord) bool {
	for i := 0; i < len(coords); i++ {
		if coords[i].x == coords[(i+1)%len(coords)].x && coords[i].x == coords[(i+2)%len(coords)].x {
			return false
		}
		if coords[i].y == coords[(i+1)%len(coords)].y && coords[i].y == coords[(i+2)%len(coords)].y {
			return false
		}
	}
	return true
}

func validateNoOverlappingLines(coords []Coord) bool {
	for i := 0; i < len(coords); i++ {
		var a1 = coords[i]
		var a2 = coords[(i+1)%len(coords)]
		for j := 0; j < len(coords); j++ {
			if i != j {
				var b1 = coords[j]
				var b2 = coords[(j+1)%len(coords)]
				if a1.x == a2.x && b1.x == b2.x && a1.x == b1.x && ((b1.y > a1.y && b1.y < a2.y) || (b1.y > a2.y && b1.y < a1.y) || (b2.y > a1.y && b2.y < a2.y) || (b2.y > a2.y && b2.y < a1.y)) {
					return false
				}
				if a1.y == a2.y && b1.y == b2.y && a1.y == b1.y && ((b1.x > a1.x && b1.x < a2.x) || (b1.x > a2.x && b1.x < a1.x) || (b2.x > a1.x && b2.x < a2.x) || (b2.x > a2.x && b2.x < a1.x)) {
					return false
				}
			}
		}
	}
	return true
}

func isValidArea(coords []Coord, i int, j int) bool {
	// Get coords
	var a = coords[i]
	var b = coords[j]

	// Find area
	var minX = a.x
	if b.x < minX {
		minX = b.x
	}
	var maxX = a.x
	if b.x > maxX {
		maxX = b.x
	}
	var minY = a.y
	if b.y < minY {
		minY = b.y
	}
	var maxY = a.y
	if b.y > maxY {
		maxY = b.y
	}

	// Skip "1D" areas
	if minX == maxX || minY == maxY {
		return false
	}

	// Validate area
	for _, coord := range coords {
		// Check if coord inside area
		if coord.x > minX && coord.x < maxX && coord.y > minY && coord.y < maxY {
			return false
		}
	}
	for n, coord := range coords {
		// Check if coord forms a line passing through the area
		var next = coords[(n+1)%len(coords)]
		if coord.y == next.y && coord.y > minY && coord.y < maxY {
			if (coord.x <= minX && next.x >= maxX) || (next.x <= minX && coord.x >= maxX) {
				return false
			}
		}
		if coord.x == next.x && coord.x > minX && coord.x < maxX {
			if (coord.y <= minY && next.y >= maxY) || (next.y <= minY && coord.y >= maxY) {
				return false
			}
		}
	}

	// Check if area is inside the closed curve (defined by coords)
	// var count = 0
	// var centerX = float64(minX) + 0.5
	// var centerY = float64(minY) + 0.5
	// for n := 0; n < len(coords); n++ {
	// 	var coord = coords[n]
	// 	var next = coords[(n+1)%len(coords)]
	// 	if (coord.x == next.x) && (float64(coord.x) < centerX) && ((float64(coord.y) < centerY && float64(next.y) > centerY) || (float64(next.y) < centerY && float64(coord.y) > centerY)) {
	// 		count++
	// 	}
	// }
	// if count%2 == 0 {
	// 	return false
	// }

	// Default to valid
	return true
}
