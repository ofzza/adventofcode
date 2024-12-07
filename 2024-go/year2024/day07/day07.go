package day02

import (
	solution "adventofcode/lib"
	"errors"
	"math/big"
	"os"
	"strconv"
	"strings"
)

// Day one definition
type Day07 struct {}
var Day = Day07 {}

// Year and day
func (day Day07) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo {
		Year: 2024,
		Day: 	7,
	}
}

// Executions
func (day Day07) GetExecutions(index int, tag string) []solution.SolutionExecution {
	var executions = []solution.SolutionExecution {};
	// Part 1/2
	if index == 0 || index == 1 {
		// Test
		if tag == "" || tag == "test" {
			executions = append(
				executions,
				solution.SolutionExecution{
					Index: 	1,
					Tag: 		"test",
					Input: 	func () string { var b, _ = os.ReadFile("./year2024/data/day07/input-test.txt"); return string(b) }(),
					Expect:	"3749",
				},
			)
		}
		// Solution
		if tag == "" || tag == "solution" {
			executions = append(
				executions,
				solution.SolutionExecution{
					Index: 	1,
					Tag: 		"solution",
					Input: 	func () string { var b, _ = os.ReadFile("./year2024/data/day07/input.txt"); return string(b) }(),
					Expect:	"5702958180383",
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
					Index: 	2,
					Tag: 		"test",
					Input: 	func () string { var b, _ = os.ReadFile("./year2024/data/day07/input-test.txt"); return string(b) }(),
					Expect:	"11387",
				},
			)
		}
		// Solution
		if tag == "" || tag == "solution" {
			executions = append(
				executions,
				solution.SolutionExecution{
					Index: 	2,
					Tag: 		"solution",
					Input: 	func () string { var b, _ = os.ReadFile("./year2024/data/day07/input.txt"); return string(b) }(),
					Expect:	"92612386119138",
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day07) Run (index int, input any, verbose bool) (any, string, error) {
	// Initialize
	var output = ""
	var value, ok = input.(string)
	if !ok { return nil, output, errors.New("failed casting execution to correct Input/Output types") }

	// Parse inputs
	var lines = strings.Split(strings.Trim(value, " "), "\n")
	var equations []struct{ result *big.Int; operands []*big.Int } = []struct{ result *big.Int; operands []*big.Int }{ }
	for _, line := range lines {
		if len(line) > 0 {
			var parts = strings.Split(strings.Trim(line, " "), ":")
			var resultParsed, _ = strconv.Atoi(strings.Trim(parts[0], " "))
			var result = big.NewInt(int64(resultParsed))
			var operandStrings = strings.Split(strings.Trim(parts[1], " "), " ")
			var operands = make([]*big.Int, 0, len(operandStrings))
			for _, operandString := range operandStrings {
				var operand, _ = strconv.Atoi(strings.Trim(operandString, " "))
				operands = append(operands, big.NewInt(int64(operand)))
			}
			equations = append(equations, struct{ result *big.Int; operands []*big.Int }{ result: result, operands: operands })
		}
	}

	// Part 1/2
	if index == 1 {

		// Check all equasions
		var sum = big.NewInt(0)
		for _, equasion := range equations {
			if testOperators(equasion.result, equasion.operands, false) { sum.Add(sum, equasion.result) }
		}

		// Return count
		return sum.String(), output, nil
	} else

	// Part 2/2
	if index == 2 {

		// Check all equasions
		var sum = big.NewInt(0)
		for _, equasion := range equations {
			if testOperators(equasion.result, equasion.operands, true) { sum.Add(sum, equasion.result) }
		}

		// Return count
		return sum.String(), output, nil
	}

	// Missing implementation
	return nil, output, errors.New("missing implementation for required index")

}

func testOperators (result *big.Int, operands []*big.Int, concatenate bool) bool {
	// Check number of operands
	if len(operands) < 1 { return false }
	if len(operands) == 1 { return result.Cmp(operands[0]) == 0 }
	// Extract first 2 operands
	var a = operands[0]
	var b = operands[1]
	var remaining = operands[2:]
	// Test out "+"" and "*"" operators
	if (testOperators(result, append([]*big.Int { new(big.Int).Add(a, b) }, remaining...), concatenate)) { return true }
	if (testOperators(result, append([]*big.Int { new(big.Int).Mul(a, b) }, remaining...), concatenate)) { return true }
	// Test out concatenation operator
	if (concatenate) {
		var concat, _ = new(big.Int).SetString(a.String() + b.String(), 10);
		if (testOperators(result, append([]*big.Int { concat }, remaining...), concatenate)) { return true }
	}

	return false
}
