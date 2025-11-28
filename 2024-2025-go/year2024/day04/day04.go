package year2024

import (
	solution "adventofcode/lib"
	"adventofcode/lib/matrix"
	"errors"
	"os"
	"strings"
)

// Day one definition
type Day04 struct{}

var Day = Day04{}

// Year and day
func (day Day04) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2024,
		Day:  4,
	}
}

// Executions
func (day Day04) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day04/input-test.txt"); return string(b) }(),
					Expect: 18,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day04/input.txt"); return string(b) }(),
					Expect: 2549,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day04/input-test.txt"); return string(b) }(),
					Expect: 9,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day04/input.txt"); return string(b) }(),
					Expect: 2003,
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day04) Run(index int, tag string, input any, verbose bool) (any, string, error) {
	// Initialize
	var output = ""
	var value, ok = input.(string)
	if !ok {
		return nil, output, errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var array = make([]rune, 0, len(value))
	for _, r := range value {
		if r != '\n' && r != '\r' {
			array = append(array, r)
		}
	}
	// Initialize matrix indexer
	var y = len(strings.Split(value, "\n")) - 1
	var x = len(value)/y - 1
	var indexer = matrix.CreateIndexer([]int{x, y})

	// Part 1/2
	if index == 1 {
		// Check each position
		var count int = 0
		for i := 0; i < len(array); i++ {
			var c, err = checkMatrixForWord(array, indexer, i, 0, []rune("XMAS"), 0)
			if err != nil {
				return nil, "", err
			}
			count += c
		}
		// Return solution
		return count, output, nil
	} else

	// Part 2/2
	if index == 2 {

		// Find X-MAX
		var count int = 0
	loop:
		for i := 0; i < len(array); i++ {
			// Check if 'A' character
			if array[i] != 'A' {
				continue
			}
			// Check if all neighbors fit
			var coords, _ = indexer.IndexToCoordinates((i))
			var topLeftCoords = []int{coords[0] - 1, coords[1] - 1}
			if !indexer.CheckIfValidCoordinates(topLeftCoords) {
				continue loop
			}
			var topRightCoords = []int{coords[0] + 1, coords[1] - 1}
			if !indexer.CheckIfValidCoordinates(topRightCoords) {
				continue loop
			}
			var bottomLeftCoords = []int{coords[0] - 1, coords[1] + 1}
			if !indexer.CheckIfValidCoordinates(bottomLeftCoords) {
				continue loop
			}
			var bottomRightCoords = []int{coords[0] + 1, coords[1] + 1}
			if !indexer.CheckIfValidCoordinates(bottomRightCoords) {
				continue loop
			}
			// Check if neighbors are M&S
			var topLeftIndex, _ = indexer.CoordinatesToIndex(topLeftCoords)
			var topRightIndex, _ = indexer.CoordinatesToIndex(topRightCoords)
			var bottomLeftIndex, _ = indexer.CoordinatesToIndex(bottomLeftCoords)
			var bottomRightIndex, _ = indexer.CoordinatesToIndex(bottomRightCoords)
			var firstDiagonalMatch = (array[topLeftIndex] == 'M' && array[bottomRightIndex] == 'S') || (array[topLeftIndex] == 'S' && array[bottomRightIndex] == 'M')
			var secondDiagonalMatch = (array[topRightIndex] == 'M' && array[bottomLeftIndex] == 'S') || (array[topRightIndex] == 'S' && array[bottomLeftIndex] == 'M')
			if firstDiagonalMatch && secondDiagonalMatch {
				count++
			}
		}

		// Return solution
		return count, output, nil
	}

	// Missing implementation
	return nil, output, errors.New("missing implementation for required index")

}

func checkMatrixForWord(array []rune, indexer matrix.MatrixIndexer, index int, diff int, word []rune, depth int) (int, error) {
	// Check if word has been fully checked
	if len(word) == 0 {
		return 1, nil
	}
	// Check if final match
	if len(word) == 1 && array[index] == word[0] {
		return 1, nil
	}
	// Check if current index matches first rune
	if !indexer.CheckIfValidIndex(index) || array[index] != word[0] {
		return 0, nil
	}

	// Initialize count
	var count int = 0

	// Count in all neighbors, or only those in preset direction if direction is set
	var neighbors, err = indexer.GetNeighboringIndicesForIndex(index, true)
	if err != nil {
		return 0, err
	}
	for _, neighbor := range neighbors {
		if index != neighbor && (diff == 0 || neighbor == index+diff) {
			var c, err = checkMatrixForWord(array, indexer, neighbor, neighbor-index, word[1:], depth+1)
			if err != nil {
				return 0, err
			}
			count += c
		}
	}
	// Return count of found matching directions
	return count, nil
}
