package year2024

import (
	solution "adventofcode/lib"
	"adventofcode/lib/matrix"
	"adventofcode/lib/pathing"
	"errors"
	"fmt"
	"os"
	"slices"
	"strings"
)

// Day one definition
type Day12 struct{}

var Day = Day12{}

// Year and day
func (day Day12) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2024,
		Day:  12,
	}
}

// Executions
func (day Day12) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day12/input-test-01.txt"); return string(b) }(),
					Expect: 140,
				},
			)
			executions = append(
				executions,
				solution.SolutionExecution{
					Index:  1,
					Tag:    "test",
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day12/input-test-02.txt"); return string(b) }(),
					Expect: 772,
				},
			)
		}
		if tag == "" || tag == "test" {
			executions = append(
				executions,
				solution.SolutionExecution{
					Index:  1,
					Tag:    "test",
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day12/input-test-03.txt"); return string(b) }(),
					Expect: 1930,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day12/input.txt"); return string(b) }(),
					Expect: 1431316,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day12/input-test-01.txt"); return string(b) }(),
					Expect: 80,
				},
			)
			executions = append(
				executions,
				solution.SolutionExecution{
					Index:  2,
					Tag:    "test",
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day12/input-test-04.txt"); return string(b) }(),
					Expect: 236,
				},
			)
		}
		if tag == "" || tag == "test" {
			executions = append(
				executions,
				solution.SolutionExecution{
					Index:  2,
					Tag:    "test",
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day12/input-test-05.txt"); return string(b) }(),
					Expect: 368,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day12/input.txt"); return string(b) }(),
					Expect: 821428,
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day12) Run(index int, tag string, input any, verbose bool, log solution.Logger) (any, string, error) {
	// Initialize
	var value, ok = input.(string)
	if !ok {
		return nil, log.Dump(), errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var lines = strings.Split(strings.Trim(value, " \n"), "\n")

	// Initialize
	var indexer = matrix.CreateIndexer([]int{len(lines[0]), len(lines)})
	var field = make([]rune, 0, len(value))
	for _, line := range lines {
		for _, r := range line {
			field = append(field, r)
		}
	}

	// Part 1/2
	if index == 1 {

		// Parse all regions
		var regions = parseRegions(indexer, field, func(region Region) {
			if verbose {
				log.Log(fmt.Sprintf("- Region %v: name=%c area=%v perimeter=%v sides=%v\n", region.id, region.name, len(region.plots), len(region.perimeter), region.sides))
			}
		})

		// Calculate price
		var price int = 0
		for _, region := range regions {
			price += len(region.plots) * len(region.perimeter)
		}

		// Return solution
		return price, log.Dump(), nil
	} else

	// Part 2/2
	if index == 2 {

		// Parse all regions
		var regions = parseRegions(indexer, field, func(region Region) {
			if verbose {
				log.Log(fmt.Sprintf("- Region %v: name=%c area=%v perimeter=%v sides=%v\n", region.id, region.name, len(region.plots), len(region.perimeter), region.sides))
			}
		})

		// Calculate price
		var price int = 0
		for _, region := range regions {
			price += len(region.plots) * region.sides
		}

		// Return solution
		return price, log.Dump(), nil
	}

	// Missing implementation
	return nil, log.Dump(), errors.New("missing implementation for required index")
}

type Region struct {
	id        int
	name      rune
	plots     []int
	perimeter []Perimeter
	sides     int
}

type Perimeter struct {
	index     int
	x         int
	y         int
	direction byte
}

func parseRegions(indexer matrix.MatrixIndexer, field []rune, callback func(region Region)) []Region {
	// Initialize regions
	var regions = make([]Region, 0, len(field))
	var traversed = make([]bool, 0, len(field))
	for i := 0; i < len(field); i++ {
		traversed = append(traversed, false)
	}

	// Traverse all plots and find continuous regions
	for i, r := range field {
		if traversed[i] {
			continue
		}
		// Initialize new region
		var region = Region{
			id:    i,
			name:  r,
			plots: make([]int, 0),
		}
		// Search for entirety of the continuous region
		var plots, updatedTraversed, perimeter = detectContinuousRegion(indexer, field, i, traversed)
		traversed = updatedTraversed
		region.plots = plots
		region.perimeter = perimeter
		// Calculate sides
		region.sides = 0
		for _, direction := range []byte{pathing.DirectionLeft, pathing.DirectionRight} {
			// Group by X
			var perimeterByX = make(map[int][]int)
			for _, p := range perimeter {
				if p.direction == direction {
					var _, ok = perimeterByX[p.x]
					if !ok {
						perimeterByX[p.x] = make([]int, 0)
					}
					perimeterByX[p.x] = append(perimeterByX[p.x], p.y)
				}
			}
			// Look for continuous sections of Ys
			for _, ys := range perimeterByX {
				slices.Sort(ys)
				region.sides++
				for i := 1; i < len(ys); i++ {
					if ys[i] != ys[i-1]+1 {
						region.sides++
					}
				}
			}
		}
		for _, direction := range []byte{pathing.DirectionTop, pathing.DirectionBottom} {
			// Group by X
			var perimeterByY = make(map[int][]int)
			for _, p := range perimeter {
				if p.direction == direction {
					var _, ok = perimeterByY[p.y]
					if !ok {
						perimeterByY[p.y] = make([]int, 0)
					}
					perimeterByY[p.y] = append(perimeterByY[p.y], p.x)
				}
			}
			// Look for continuous sections of Ys
			for _, xs := range perimeterByY {
				slices.Sort(xs)
				region.sides++
				for i := 1; i < len(xs); i++ {
					if xs[i] != xs[i-1]+1 {
						region.sides++
					}
				}
			}
		}
		// Store region
		regions = append(regions, region)
		// Run callback
		if callback != nil {
			callback(region)
		}
	}

	// Return regions
	return regions
}

func detectContinuousRegion(indexer matrix.MatrixIndexer, field []rune, i int, traversed []bool) ([]int, []bool, []Perimeter) {
	// Initialize
	var name rune = field[i]
	var totalArea int = 0
	var perimeter = make([]Perimeter, 0, len(field))
	var plots = make([]int, 0, len(field))
	// Mark self as plot and traversed
	plots = append(plots, i)
	traversed[i] = true
	// Process self-perimeter
	totalArea += 1
	var perimeterNeighborCoords, _ = indexer.GetNeighboringCoordinatesForIndexWithValidation(i, false, false)
	for _, neighbor := range perimeterNeighborCoords {
		// If edge of field, add perimeter
		var index, err = indexer.CoordinatesToIndex(neighbor)
		var coord, _ = indexer.IndexToCoordinates(i)
		var diff = []int{neighbor[0] - coord[0], neighbor[1] - coord[1]}
		var direction byte
		if diff[0] == -1 {
			direction = pathing.DirectionLeft
		}
		if diff[0] == +1 {
			direction = pathing.DirectionRight
		}
		if diff[1] == -1 {
			direction = pathing.DirectionTop
		}
		if diff[1] == +1 {
			direction = pathing.DirectionBottom
		}
		if err != nil {
			perimeter = append(perimeter, Perimeter{index: i, x: coord[0], y: coord[1], direction: direction})
		} else if field[index] != name {
			perimeter = append(perimeter, Perimeter{index: i, x: coord[0], y: coord[1], direction: direction})
		}
	}
	// Traverse neighbors
	var regionNeighbors, _ = indexer.GetNeighboringIndicesForIndex(i, false)
	for _, n := range regionNeighbors {
		if field[n] == name && !traversed[n] {
			var _plots, _traversed, _perimeter = detectContinuousRegion(indexer, field, n, traversed)
			traversed = _traversed
			plots = append(plots, _plots...)
			perimeter = append(perimeter, _perimeter...)
		}
	}
	// Return updates, area and perimeter
	return plots, traversed, perimeter
}
