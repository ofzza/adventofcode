package year2025

import (
	solution "adventofcode/lib"
	"errors"
	"os"
	"sort"
	"strconv"
	"strings"
)

// Day one definition
type Day05 struct{}

var Day = Day05{}

// Year and day
func (day Day05) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2025,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day05/input-test.txt"); return string(b) }(),
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day05/input.txt"); return string(b) }(),
					Expect: 529,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day05/input-test.txt"); return string(b) }(),
					Expect: 14,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day05/input.txt"); return string(b) }(),
					Expect: 344260049617193,
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day05) Run(index int, tag string, input any, verbose bool, log solution.Logger) (any, string, error) {
	// Initialize
	var value, ok = input.(string)
	if !ok {
		return nil, log.Dump(), errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var valueSplit = strings.Split(strings.Trim((value), "\r\n "), "\n\n")
	var rangesStr = strings.Split(strings.Trim(valueSplit[0], "\r\n "), "\n")
	var ranges = make([][]int, len(rangesStr))
	for i, rangeStr := range rangesStr {
		var rangeStr = strings.Trim(rangeStr, "\r\n ")
		var rangeSplit = strings.Split(rangeStr, "-")
		rangeFrom, _ := strconv.Atoi(strings.Trim(rangeSplit[0], "\r\n "))
		rangeTo, _ := strconv.Atoi(strings.Trim(rangeSplit[1], "\r\n "))
		ranges[i] = []int{rangeFrom, rangeTo}
	}
	sort.Slice(ranges, func(i int, j int) bool {
		return ranges[i][0] < ranges[j][0] || (ranges[i][0] == ranges[j][0] && ranges[i][1] < ranges[j][1])
	})

	var idsStr = strings.Split(strings.Trim(valueSplit[1], "\r\n "), "\n")
	var ids = make([]int, len(idsStr))
	for i, idStr := range idsStr {
		var idStr = strings.Trim(idStr, "\r\n ")
		id, _ := strconv.Atoi(strings.Trim(idStr, "\r\n "))
		ids[i] = id
	}
	sort.Slice(ids, func(i int, j int) bool {
		return ids[i] < ids[j]
	})

	// Part 1/2
	if index == 1 {

		// Check if ID is in range
		var i = 0
		var count = 0
		for _, id := range ids {
			// Get range currently being checked
			var r = ranges[i]

			// Check if ID within or below the current range
			if id < r[0] {
				continue
			}
			if id >= r[0] && id <= r[1] {
				count++
				continue
			}

			// Find next range to check
			for j := i; j < len(ranges); j++ {
				if ranges[j][1] >= id {
					// Check if ID within next range
					if id >= ranges[j][0] && id <= ranges[j][1] {
						count++
					}
					// Found next range
					i = j
					// Stop searching next range
					break
				}
			}
		}

		// Return solution
		return count, log.Dump(), nil
	} else

	// Part 2/2
	if index == 2 {

		// Merge overlapping ranges
		var mergedRanges = make([][]int, 0)
		for _, r := range ranges {
			// Take first range
			if len(mergedRanges) == 0 {
				mergedRanges = append(mergedRanges, r)
				continue
			}

			// Get last merged range
			var m = mergedRanges[len(mergedRanges)-1]

			// Check if range is contained within the previous range
			if r[0] >= m[0] && r[1] <= m[1] {
				continue
			}
			// Check if range is overlapping with previous range
			if r[0] <= m[1] && r[1] > m[1] {
				m[1] = r[1]
				continue
			}

			// Add range as is
			mergedRanges = append(mergedRanges, r)
		}

		var count = 0
		// Count IDs in ranges
		for _, r := range mergedRanges {
			count += r[1] - r[0] + 1
		}

		// Return solution
		return count, log.Dump(), nil
	}

	// Missing implementation
	return nil, log.Dump(), errors.New("missing implementation for required index")
}
