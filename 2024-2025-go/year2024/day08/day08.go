package year2024

import (
	solution "adventofcode/lib"
	"adventofcode/lib/matrix"
	"errors"
	"fmt"
	"os"
	"strings"
)

// Day one definition
type Day08 struct{}

var Day = Day08{}

// Year and day
func (day Day08) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2024,
		Day:  8,
	}
}

// Executions
func (day Day08) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day08/input-test.txt"); return string(b) }(),
					Expect: 14,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day08/input.txt"); return string(b) }(),
					Expect: 323,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day08/input-test.txt"); return string(b) }(),
					Expect: 34,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day08/input.txt"); return string(b) }(),
					Expect: 1077,
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day08) Run(index int, tag string, input any, verbose bool, log solution.Logger) (any, string, error) {
	// Initialize
	var value, ok = input.(string)
	if !ok {
		return nil, log.Dump(), errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var lines = strings.Split(strings.Trim(value, " \n"), "\n")
	var array = make([]rune, 0, len(value))
	for _, l := range lines {
		for _, r := range l {
			array = append(array, r)
		}
	}
	var dimensions = []int{len(lines[0]), len(lines)}
	var indexer = matrix.CreateIndexer(dimensions)

	// Organize freqAntennas by frequency
	var freqAntennas = make(map[rune][][]int)
	for y := 0; y < dimensions[1]; y++ {
		for x := 0; x < dimensions[0]; x++ {
			var coords = []int{x, y}
			var index, _ = indexer.CoordinatesToIndex(coords)
			var frequency = array[index]
			if frequency != '.' {
				var _, ok = freqAntennas[frequency]
				if !ok {
					freqAntennas[frequency] = [][]int{}
				}
				freqAntennas[frequency] = append(freqAntennas[frequency], coords)
			}
		}
	}

	// Part 1/2
	if index == 1 {

		// For each frequency, process each antenna pair
		var count int = 0
		var anodes = make(map[int]bool)
		iterateAntennaPairs(freqAntennas, func(frequency rune, coordsA []int, coordsB []int) {
			// Echo antenna pair
			if verbose {
				log.Log(fmt.Sprintf("  - %v -> %v: ", coordsA, coordsB))
			}
			// Get antinodes
			var anodesArray = make([][]int, 0, indexer.GetDimensions()[0])
			anodesArray = append(anodesArray, findAntinodes(indexer, coordsA, coordsB, 1)...)
			anodesArray = append(anodesArray, findAntinodes(indexer, coordsB, coordsA, 1)...)
			// Echo antinodes
			if verbose {
				log.Log(fmt.Sprintf("%v\n", anodesArray))
			}
			// Deduplicate antinodes
			for _, anode := range anodesArray {
				// Store anode by index
				var index, _ = indexer.CoordinatesToIndex(anode)
				var _, okIndex = anodes[index]
				if !okIndex {
					anodes[index] = true
					count++
				}
			}
		})

		// Return solution
		return count, log.Dump(), nil
	} else

	// Part 2/2
	if index == 2 {

		// For each frequency, process each antenna pair
		var count int = 0
		var anodes = make(map[int]bool)
		iterateAntennaPairs(freqAntennas, func(frequency rune, coordsA []int, coordsB []int) {
			// Echo antenna pair
			if verbose {
				log.Log(fmt.Sprintf("  - %v -> %v: ", coordsA, coordsB))
			}
			// Get antinodes
			var anodesArray = make([][]int, 0, indexer.GetDimensions()[0])
			anodesArray = append(anodesArray, findAntinodes(indexer, coordsA, coordsB, 0)...)
			anodesArray = append(anodesArray, findAntinodes(indexer, coordsB, coordsA, 0)...)
			anodesArray = append(anodesArray, coordsA)
			anodesArray = append(anodesArray, coordsB)
			// Echo antinodes
			if verbose {
				log.Log(fmt.Sprintf("%v\n", anodesArray))
			}
			// Deduplicate antinodes
			for _, anode := range anodesArray {
				// Store anode by index
				var index, _ = indexer.CoordinatesToIndex(anode)
				var _, okIndex = anodes[index]
				if !okIndex {
					anodes[index] = true
					count++
				}
			}
		})

		// Return solution
		return count, log.Dump(), nil
	}

	// Missing implementation
	return nil, log.Dump(), errors.New("missing implementation for required index")
}

// Iterates over all antenna pairs
func iterateAntennaPairs(freqAntennas map[rune][][]int, callback func(frequency rune, coordsA []int, coordsB []int)) {
	// Iterate over all frequencies and antenna pairs
	for frequency, singleFreqAntennas := range freqAntennas {
		// Process antenna pairs
		for i := 0; i < len(singleFreqAntennas); i++ {
			for j := 0; j < i; j++ {
				// Skip antenna self interaction
				if i == j {
					continue
				}
				// Get antenna coordinates
				var coordsA = singleFreqAntennas[i]
				var coordsB = singleFreqAntennas[j]
				// Run antenna pair callback
				callback(frequency, coordsA, coordsB)
			}
		}
	}
}

// Finds antinodes of 2 provided antennas
func findAntinodes(indexer matrix.MatrixIndexer, coordsA []int, coordsB []int, limit int) [][]int {
	// Initialize
	var anodes = make([][]int, 0, indexer.GetDimensions()[0])
	// Calculate diff
	var diff = []int{coordsA[0] - coordsB[0], coordsA[1] - coordsB[1]}
	var node = coordsA
	for {
		// Check if limit rached
		if limit > 0 && len(anodes) >= limit {
			break
		}
		// Calculate antinode
		var anode = []int{node[0] + diff[0], node[1] + diff[1]}
		// Check if out-of-bounds
		if !indexer.CheckIfValidCoordinates(anode) {
			break
		}
		// Store antinode
		anodes = append(anodes, anode)
		node = anode
	}
	// Return found antinodes
	return anodes
}
