package year2025

import (
	solution "adventofcode/lib"
	"errors"
	"fmt"
	"os"
	"strconv"
	"strings"
)

// Day one definition
type Day03 struct{}

var Day = Day03{}

// Year and day
func (day Day03) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2025,
		Day:  3,
	}
}

// Executions
func (day Day03) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day03/input-test.txt"); return string(b) }(),
					Expect: 357,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day03/input.txt"); return string(b) }(),
					Expect: 16858,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day03/input-test.txt"); return string(b) }(),
					Expect: 3121910778619,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day03/input.txt"); return string(b) }(),
					Expect: 167549941654721,
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day03) Run(index int, tag string, input any, verbose bool, log solution.Logger) (any, string, error) {
	// Initialize
	var value, ok = input.(string)
	if !ok {
		return nil, log.Dump(), errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var banksStr = strings.Split(strings.Trim(value, "\r\n "), "\n")
	var banks = make([][]int, len(banksStr))
	for i, bankStr := range banksStr {
		bankStr := strings.Trim(bankStr, "\r\n ")
		var bank = make([]int, len(bankStr))
		for j := 0; j < len(bankStr); j++ {
			var batteryChar = bankStr[j]
			bank[j] = int(batteryChar) - 48
		}
		banks[i] = bank
	}

	// Part 1/2
	if index == 1 {

		// Init
		var joltageSum = 0

		// Process all banks
		for _, bank := range banks {
			var joltage, err, _output = maxJoltage(bank, 2, "")
			log.Log(_output)
			if err != nil {
				panic(err)
			}
			joltageSum += joltage
		}

		// Return solution
		return joltageSum, log.Dump(), nil
	} else

	// Part 2/2
	if index == 2 {

		// Init
		var joltageSum = 0

		// Process all banks
		for _, bank := range banks {
			var joltage, err, _output = maxJoltage(bank, 12, "")
			log.Log(_output)
			if err != nil {
				panic(err)
			}
			joltageSum += joltage
		}

		// Return solution
		return joltageSum, log.Dump(), nil
	}

	// Missing implementation
	return nil, log.Dump(), errors.New("missing implementation for required index")
}

func maxJoltage(bank []int, count int, output string) (int, error, string) {
	if count > len(bank) {
		return -1, fmt.Errorf("can't compose %d part joltage within a %d sized battery bank!", count, len(bank)), output
	}

	// Initialize max indexes array
	var max = make([]int, count)
	for i := 0; i < count; i++ {
		max[i] = -1
	}

	// Find highest joltage for each battery bank
	for i := 0; i < count; i++ {
		var start = 0
		if i > 0 {
			start = max[i-1] + 1
		}
		var end = len(bank) - count + i + 1
		for j := start; j < end; j++ {
			if max[i] == -1 || bank[j] > bank[max[i]] {
				max[i] = j
			}
		}
	}

	// Compose joltage string
	var joltageStr = ""
	for _, m := range max {
		joltageStr += fmt.Sprintf("%d", bank[m])
	}

	// Echo
	output += fmt.Sprintf("- Bank %v - joltage: %s\n", bank, joltageStr)

	// Calculate joltage
	var joltage, err = strconv.Atoi(joltageStr)
	if err != nil {
		return -1, err, output
	}

	return joltage, nil, output
}
