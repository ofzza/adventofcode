package year2024

import (
	solution "adventofcode/lib"
	"errors"
	"fmt"
	"os"
	"strings"
)

// Day one definition
type Day19 struct{}

var Day = Day19{}

// Year and day
func (day Day19) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2024,
		Day:  19,
	}
}

// Executions
func (day Day19) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day19/input-test.txt"); return string(b) }(),
					Expect: 6,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day19/input.txt"); return string(b) }(),
					Expect: 371,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day19/input-test.txt"); return string(b) }(),
					Expect: 16,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day19/input.txt"); return string(b) }(),
					Expect: 650354687260341,
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day19) Run(index int, tag string, input any, verbose bool, log solution.Logger) (any, string, error) {
	// Initialize
	var value, ok = input.(string)
	if !ok {
		return nil, log.Dump(), errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var sections = strings.Split(strings.Trim(value, " \n"), "\n\n")
	// Parse towels
	var towelStrs = strings.Split(strings.Trim(sections[0], " \n"), ",")
	var towels = make([]string, 0, len(towelStrs))
	for _, towelStr := range towelStrs {
		towels = append(towels, strings.Trim(towelStr, " \n"))
	}
	// Parse designs
	var designStrs = strings.Split(strings.Trim(sections[1], " \n"), "\n")
	var designs = make([]string, 0, len(designStrs))
	for _, designStr := range designStrs {
		designs = append(designs, strings.Trim(designStr, " \n"))
	}

	// Part 1/2
	if index == 1 {

		// Find all ways to compose each of the designs
		var total int = 0
		for i, design := range designs {

			// Echo
			if verbose {
				log.Log(fmt.Sprintf("- Checking %v/%v designs ... ", i+1, len(designs)))
			}

			// Find permutations
			var count = findPermutations(towels, design)
			if count > 0 {
				// Count as composed
				total++
				// Echo
				if verbose {
					log.Log(fmt.Sprintf(" Found %v permutations!\n", count))
				}
			} else {
				// Echo
				if verbose {
					log.Log(fmt.Sprintf(" No permutations found!\n"))
				}
			}

		}

		// Return solution
		return total, log.Dump(), nil
	} else

	// Part 2/2
	if index == 2 {

		// Find all ways to compose each of the designs
		var total int = 0
		for i, design := range designs {

			// Echo
			if verbose {
				log.Log(fmt.Sprintf("- Checking %v/%v designs ... ", i+1, len(designs)))
			}

			// Find permutations
			var count = findPermutations(towels, design)
			if count > 0 {
				// Count as composed
				total += count
				// Echo
				if verbose {
					log.Log(fmt.Sprintf(" Found %v permutations!\n", count))
				}
			} else {
				// Echo
				if verbose {
					log.Log(fmt.Sprintf(" No permutations found!\n"))
				}
			}

		}

		// Return solution
		return total, log.Dump(), nil
	}

	// Missing implementation
	return nil, log.Dump(), errors.New("missing implementation for required index")
}

func findPermutations(towels []string, design string) int {
	var cache = make(map[string]int)
	var _, count = findPermutationsInternal(towels, design, cache)
	return count
}

func findPermutationsInternal(towels []string, design string, cache map[string]int) (bool, int) {

	// If design is empty, return success
	if len(design) == 0 {
		return true, 1
	}

	// Check which towels work in the design and count permutations
	var success = false
	var permutations int = 0
	for _, towel := range towels {
		// If design starts with
		if len(design) >= len(towel) && design[:len(towel)] == towel {
			// Get remaining design
			var remainder = design[len(towel):]
			// Check cache
			var cachedCount, cachedOk = cache[remainder]
			if cachedOk {
				success = true
				permutations += cachedCount
				continue
			}
			// Find permutations
			var calculatedOk, calculatedCount = findPermutationsInternal(towels, remainder, cache)
			if calculatedOk {
				// Count up permutations
				success = true
				permutations += calculatedCount
				// Cache answer
				cache[remainder] = calculatedCount
			}
		}
	}

	// Return number of permutations
	return success, permutations
}
