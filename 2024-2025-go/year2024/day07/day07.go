package year2024

import (
	solution "adventofcode/lib"
	"errors"
	"math/big"
	"os"
	"strconv"
	"strings"
	"sync"
)

// Day one definition
type Day07 struct{}

var Day = Day07{}

// Year and day
func (day Day07) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2024,
		Day:  7,
	}
}

// Executions
func (day Day07) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day07/input-test.txt"); return string(b) }(),
					Expect: "3749",
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day07/input.txt"); return string(b) }(),
					Expect: "5702958180383",
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day07/input-test.txt"); return string(b) }(),
					Expect: "11387",
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day07/input.txt"); return string(b) }(),
					Expect: "92612386119138",
				},
			)
		}
	}
	return executions
}

type Equation struct {
	result   *big.Int
	operands []*big.Int
}

// Implementation
func (day Day07) Run(index int, tag string, input any, verbose bool) (any, string, error) {
	// Initialize
	var output = ""
	var value, ok = input.(string)
	if !ok {
		return nil, output, errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var lines = strings.Split(strings.Trim(value, " \n"), "\n")
	var equations []Equation = []Equation{}
	for _, line := range lines {
		var parts = strings.Split(strings.Trim(line, " "), ":")
		var resultParsed, _ = strconv.Atoi(strings.Trim(parts[0], " "))
		var result = big.NewInt(int64(resultParsed))
		var operandStrings = strings.Split(strings.Trim(parts[1], " "), " ")
		var operands = make([]*big.Int, 0, len(operandStrings))
		for _, operandString := range operandStrings {
			var operand, _ = strconv.Atoi(strings.Trim(operandString, " "))
			operands = append(operands, big.NewInt(int64(operand)))
		}
		equations = append(equations, struct {
			result   *big.Int
			operands []*big.Int
		}{result: result, operands: operands})
	}

	// Part 1/2
	if index == 1 {

		// Check all equations
		var results = make(chan *big.Int, len(equations))
		var wg sync.WaitGroup
		var sum = big.NewInt(0)
		for _, equasion := range equations {
			wg.Add(1)
			go testOperatorsRutine(results, &wg, equasion, false)
		}
		wg.Wait()
		for i := 0; i < len(equations); i++ {
			var result = <-results
			if result != nil {
				sum.Add(sum, result)
			}
		}

		// Return solution
		return sum.String(), output, nil
	} else

	// Part 2/2
	if index == 2 {

		// Check all equations
		var results = make(chan *big.Int, len(equations))
		var wg sync.WaitGroup
		var sum = big.NewInt(0)
		for _, equasion := range equations {
			wg.Add(1)
			go testOperatorsRutine(results, &wg, equasion, true)
		}
		wg.Wait()
		for i := 0; i < len(equations); i++ {
			var result = <-results
			if result != nil {
				sum.Add(sum, result)
			}
		}

		// Return solution
		return sum.String(), output, nil
	}

	// Missing implementation
	return nil, output, errors.New("missing implementation for required index")
}

func testOperatorsRutine(channel chan *big.Int, wg *sync.WaitGroup, equation Equation, concatenate bool) *big.Int {
	// Ready to un-wait
	defer wg.Done()
	// Get result
	var result = testOperatorsRecursive(equation, concatenate)
	// Pipe and return result
	if channel != nil {
		channel <- result
	}
	return result
}

func testOperatorsRecursive(equation Equation, concatenate bool) *big.Int {
	// Check number of operands
	if len(equation.operands) < 1 {
		return nil
	}
	if len(equation.operands) == 1 {
		if equation.result.Cmp(equation.operands[0]) == 0 {
			return equation.result
		} else {
			return nil
		}
	}
	// Extract first 2 operands
	var a = equation.operands[0]
	var b = equation.operands[1]
	// Check if operands already too large
	if equation.result.Cmp(a) == -1 || equation.result.Cmp(b) == -1 {
		return nil
	}
	// Test out "+"" and "*"" operators
	if (testOperatorsRecursive(Equation{result: equation.result, operands: append([]*big.Int{new(big.Int).Add(a, b)}, equation.operands[2:]...)}, concatenate) != nil) {
		return equation.result
	}
	if (testOperatorsRecursive(Equation{result: equation.result, operands: append([]*big.Int{new(big.Int).Mul(a, b)}, equation.operands[2:]...)}, concatenate) != nil) {
		return equation.result
	}
	// Test out concatenation operator
	if concatenate {
		var concat, _ = new(big.Int).SetString(a.String()+b.String(), 10)
		if (testOperatorsRecursive(Equation{result: equation.result, operands: append([]*big.Int{concat}, equation.operands[2:]...)}, concatenate) != nil) {
			return equation.result
		}
	}
	// Return fail
	return nil
}
