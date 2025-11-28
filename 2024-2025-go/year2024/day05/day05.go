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
type Day05 struct{}

var Day = Day05{}

// Year and day
func (day Day05) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2024,
		Day:  5,
	}
}

// Executions
func (day Day05) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day05/input-test.txt"); return string(b) }(),
					Expect: 143,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day05/input.txt"); return string(b) }(),
					Expect: 6949,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day05/input-test.txt"); return string(b) }(),
					Expect: 123,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day05/input.txt"); return string(b) }(),
					Expect: 4145,
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day05) Run(index int, tag string, input any, verbose bool) (any, string, error) {
	// Initialize
	var output = ""
	var value, ok = input.(string)
	if !ok {
		return nil, output, errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var sections = strings.Split(strings.Trim(value, " \n"), "\n\n")
	var rulesLines = strings.Split(strings.Trim(sections[0], " "), "\n")
	var rules [][]int = make([][]int, 0, len(rulesLines))
	for _, line := range rulesLines {
		var items = strings.Split(strings.Trim(line, " "), "|")
		var a, _ = strconv.Atoi(items[0])
		var b, _ = strconv.Atoi(items[1])
		rules = append(rules, []int{a, b})
	}
	var manualsLines = strings.Split(strings.Trim(sections[1], " "), "\n")
	var manuals [][]int = make([][]int, 0, len(manualsLines))
	for _, line := range manualsLines {
		var pages = strings.Split(strings.Trim(line, " "), ",")
		var ps []int = make([]int, 0, len(pages))
		for _, page := range pages {
			var p, _ = strconv.Atoi(page)
			ps = append(ps, p)
		}
		manuals = append(manuals, ps)
	}

	// Part 1/2
	if index == 1 {

		// Process all manuals
		var sum int = 0
		for _, pages := range manuals {
			// Check against all rules
			var valid, indexA, indexB = checkPages(pages, rules)
			if valid {
				var middleValue = pages[len(pages)/2]
				sum += middleValue
				if verbose {
					output += fmt.Sprintf("- %v: Valid -> += %v\n", pages, middleValue)
				}
			} else if verbose {
				output += fmt.Sprintf("- %v: Invalid -> %v > %v\n", pages, pages[indexA], pages[indexB])
			}
		}

		// Return solution
		return sum, output, nil
	} else

	// Part 2/2
	if index == 2 {

		// Process all manuals
		var sum int = 0
		for _, pages := range manuals {
			// Check against all rules
			var valid, indexA, indexB = checkPages(pages, rules)
			if valid {
				if verbose {
					output += fmt.Sprintf("- %v: Valid\n", pages)
				}
			} else {
				if verbose {
					output += fmt.Sprintf("- %v: Invalid -> %v > %v\n", pages, pages[indexA], pages[indexB])
				}
				var fixed = pages
				for {
					fixed = attemptFix(fixed, indexA, indexB)
					if verbose {
						output += fmt.Sprintf("  - Attempting fix: %v", fixed)
					}
					valid, indexA, indexB = checkPages(fixed, rules)
					if valid {
						var middleValue = fixed[len(fixed)/2]
						sum += middleValue
						if verbose {
							output += fmt.Sprintf(" ... succedeed -> %v!\n", middleValue)
						}
						break
					} else if verbose {
						output += fmt.Sprintf(" ... failed! -> %v > %v\n", fixed[indexA], fixed[indexB])
					}
				}
			}
		}

		// Return solution
		return sum, output, nil
	}

	// Missing implementation
	return nil, output, errors.New("missing implementation for required index")
}

func mapPageIndexes(pages []int) map[int]int {
	var mapped = make(map[int]int)
	for i, page := range pages {
		mapped[page] = i
	}
	return mapped
}

func checkPages(pages []int, rules [][]int) (bool, int, int) {
	// Map pages' indexes
	var mapped = mapPageIndexes(pages)
	// Check against all rules
	for _, rule := range rules {
		var indexA, okA = mapped[rule[0]]
		var indexB, okB = mapped[rule[1]]
		if okA && okB && indexA > indexB {
			return false, indexA, indexB
		}
	}
	return true, 0, 0
}

func attemptFix(pages []int, indexA int, indexB int) []int {
	var fixed = make([]int, 0, len(pages))
	fixed = append(fixed, pages[:indexB]...)
	fixed = append(fixed, pages[indexA])
	fixed = append(fixed, pages[indexB:indexA]...)
	fixed = append(fixed, pages[indexA+1:]...)
	return fixed
}
