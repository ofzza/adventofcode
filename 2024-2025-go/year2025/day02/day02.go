package year2025

import (
	solution "adventofcode/lib"
	"errors"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

// Day one definition
type Day02 struct{}

var Day = Day02{}

// Year and day
func (day Day02) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2025,
		Day:  2,
	}
}

// Executions
func (day Day02) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day02/input-test.txt"); return string(b) }(),
					Expect: 1227775554,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day02/input.txt"); return string(b) }(),
					Expect: 12599655151,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day02/input-test.txt"); return string(b) }(),
					Expect: 4174379265,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day02/input.txt"); return string(b) }(),
					Expect: 20942028255,
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day02) Run(index int, tag string, input any, verbose bool, log solution.Logger) (any, string, error) {
	// Initialize
	var value, ok = input.(string)
	if !ok {
		return nil, log.Dump(), errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var rangeStrs = strings.Split(strings.Trim(value, "\r\n "), ",")
	var ranges = make([]struct {
		From int
		To   int
	}, len(rangeStrs))
	for i, rangeStr := range rangeStrs {
		var rangeBoundsStr = strings.Split(rangeStr, "-")
		from, _ := strconv.Atoi(rangeBoundsStr[0])
		to, _ := strconv.Atoi(rangeBoundsStr[1])
		ranges[i] = struct {
			From int
			To   int
		}{
			From: from,
			To:   to,
		}
	}

	// Count invalid IDs
	var sum = 0

	// Part 1/2
	if index == 1 {

		// Process all ranges
		for _, r := range ranges {

			// Check if range contains no numbers of even length
			if len(fmt.Sprint(r.From)) == len(fmt.Sprint(r.To)) && len(fmt.Sprint(r.From))%2 == 1 {
				log.Log(fmt.Sprintf("- [%d, %d]: SKIPPING due to no even number length IDs in the range\n", r.From, r.To))
				continue
			}

			// Split range boundaries
			var fromStr = fmt.Sprint(r.From)
			from, _ := strconv.Atoi(fromStr[0:int(math.Floor(float64(len(fromStr))/2))])
			var toStr = fmt.Sprint(r.To)
			to, _ := strconv.Atoi(toStr[0:int(math.Ceil(float64(len(toStr))/2))])
			log.Log(fmt.Sprintf("- [%d, %d]: Processing as [%d, %d] ::: ", r.From, r.To, from, to))
			for i := from; i <= to; i++ {
				id, _ := strconv.Atoi(fmt.Sprintf("%d%d", i, i))
				if id >= r.From && id <= r.To {
					log.Log(fmt.Sprintf("%d, ", id))
					sum += id
				}
			}
			log.Log("\n")
		}

		// Return solution
		return sum, log.Dump(), nil
	} else

	// Part 2/2
	if index == 2 {

		// Process all ranges
		for _, r := range ranges {

			var fromStr = fmt.Sprint(r.From)
			var toStr = fmt.Sprint(r.To)
			log.Log(fmt.Sprintf("- [%d, %d]: ...\n", r.From, r.To))

			// Try splitting into different number of (repeating) parts
			var ids = make(map[int]bool)
			for n := 2; n <= len(toStr); n++ {

				// Split range boundaries
				from, err := strconv.Atoi(fromStr[0:int(math.Floor(float64(len(fromStr))/float64(n)))])
				if err != nil {
					from = 0
				}
				to, err := strconv.Atoi(toStr[0:int(math.Ceil(float64(len(toStr))/float64(n)))])
				if err != nil {
					to = 0
				}
				log.Log(fmt.Sprintf("  - Splitting into %d repetitions - processing as [%d, %d] ::: ", n, from, to))
				for i := from; i <= to; i++ {
					var idStr = ""
					for j := 0; j < n; j++ {
						idStr += fmt.Sprint(i)
					}
					id, _ := strconv.Atoi(idStr)
					if id >= r.From && id <= r.To {
						log.Log(fmt.Sprintf("%d, ", id))
						ids[id] = true
					}
				}
				log.Log("\n")
			}

			// Sum up all deduplicated IDs
			for id := range ids {
				sum += id
			}
		}

		// Return solution
		return sum, log.Dump(), nil
	}

	// Missing implementation
	return nil, log.Dump(), errors.New("missing implementation for required index")
}
