package year2024

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
type Day17 struct{}

var Day = Day17{}

// Year and day
func (day Day17) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2024,
		Day:  17,
	}
}

// Executions
func (day Day17) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day17/input-test-01.txt"); return string(b) }(),
					Expect: "4,6,3,5,6,3,5,2,1,0",
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day17/input.txt"); return string(b) }(),
					Expect: "7,1,3,4,1,2,6,7,1",
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day17/input-test-02.txt"); return string(b) }(),
					Expect: 117440,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day17/input.txt"); return string(b) }(),
					Expect: 109019476330651,
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day17) Run(index int, tag string, input any, verbose bool, log solution.Logger) (any, string, error) {
	// Initialize
	var output = ""
	var value, ok = input.(string)
	if !ok {
		return nil, log.Dump(), errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var sections = strings.Split(strings.Trim(value, " \n"), "\n\n")
	// Parse registers' initial values
	var registersStrs = strings.Split(strings.Trim(sections[0], " \n"), "\n")
	var registers = make([]int, 0, len(registersStrs))
	for _, registerStr := range registersStrs {
		var value, _ = strconv.Atoi(strings.Trim(strings.Split(strings.Trim(registerStr, " \n"), ":")[1], " \n"))
		registers = append(registers, value)
	}
	// Parse program code
	var codeStrs = strings.Split(strings.Trim(strings.Split(strings.Trim(sections[1], " \n"), ":")[1], " \n"), ",")
	var code = make([]int, 0, len(codeStrs))
	for _, codeStr := range codeStrs {
		var value, _ = strconv.Atoi(codeStr)
		code = append(code, value)
	}

	// Initialize a Chronospatial Computer instance
	var computer = CreateChronospatialComputer(code, registers)

	// Part 1/2
	if index == 1 {

		// Run computer
		computer.Run(func(ipBefore int, ipAfter int, opCode int, operand int, regsBefore []int, regsAfter []int, outputValues []int) bool {
			// Echo execution
			if verbose {
				log.Log(fmt.Sprintf("- %v. %v %v | %v -> %v: %v. %v\n", ipBefore, opCode, operand, regsBefore, regsAfter, ipAfter, outputValues))
			}
			return true
		})

		// Process output
		var outputStrs = make([]string, 0, len(output))
		for _, i := range computer.output {
			outputStrs = append(outputStrs, strconv.Itoa(i))
		}
		var outputStr = strings.Join(outputStrs, ",")

		// Return solution
		return outputStr, log.Dump(), nil
	} else

	// Part 2/2 (solution)
	if index == 2 {

		// Find register A value
		var ok, a = findRegisterValue(code, registers, computer)

		// Return solution
		if ok {
			return a, log.Dump(), nil
		} else {
			return nil, log.Dump(), errors.New("solution not found")
		}

	}

	// Missing implementation
	return nil, log.Dump(), errors.New("missing implementation for required index")
}

func findRegisterValue(code []int, regs []int, computer ChronospatialComputer) (bool, int) {
	// Detect the step of output length increase
	var initialOutputLength int = -1
	var continuations = make([]string, 0)
	for i := 0; true; i++ {
		// (Re)Initialize and run computer
		computer.reset([]int{i, regs[1], regs[2]})
		computer.output = make([]int, 0)
		computer.Run(func(ipBefore int, ipAfter int, opCode int, operand int, regsBefore []int, regsAfter []int, outputValues []int) bool {
			return true
		})
		if initialOutputLength != -1 && len(computer.output) > initialOutputLength {
			// Generate continuations
			for j := 0; j < i; j++ {
				var binary = strconv.FormatInt(int64(j), 2)
				continuations = append(continuations, fmt.Sprintf("%0"+strconv.FormatInt(int64(math.Ceil(math.Log2(float64(i-1)))), 10)+"s", binary))
			}
			break
		} else if initialOutputLength == -1 {
			initialOutputLength = len(computer.output)
		}
	}
	// Find register value
	return findRegisterValueInternal("", continuations, code, regs, computer)
}
func findRegisterValueInternal(prefix string, continuations []string, code []int, regs []int, computer ChronospatialComputer) (bool, int) {

	// Test out options for next digit
	for index, continuation := range continuations {
		// Skip first "0" continuation when no prefix
		if len(prefix) == 0 && index == 0 {
			continue
		}
		// Compose candidate value
		var candidateString = prefix + continuation
		var candidateValue, _ = strconv.ParseInt(candidateString, 2, 64)
		// (Re)Initialize and run computer
		computer.reset([]int{int(candidateValue), regs[1], regs[2]})
		computer.output = make([]int, 0)
		computer.Run(func(ipBefore int, ipAfter int, opCode int, operand int, regsBefore []int, regsAfter []int, outputValues []int) bool {
			return true
		})
		// Check if output matches
		var matches = true
		for i := 0; i < len(computer.output); i++ {
			if computer.output[len(computer.output)-i-1] != code[len(code)-i-1] {
				matches = false
				break
			}
		}
		if matches {
			// Check if fully solution
			if len(computer.output) == len(code) {
				return true, int(candidateValue)
			}
			// Accept candidate as new prefix and continue searching for matches
			var ok, solution = findRegisterValueInternal(candidateString, continuations, code, regs, computer)
			if ok {
				return true, solution
			}
		}
	}

	// No match found, return no solution found
	return false, 0

}

// Chronospatial Computer factory
func CreateChronospatialComputer(code []int, regs []int) ChronospatialComputer {
	// Initialize computer
	var comp = ChronospatialComputer{
		code: code,
	}
	// Reset computer
	comp.reset(regs)
	// Return computer
	return comp
}

// Chronospatial Computer implementation
type ChronospatialComputer struct {
	ip     int
	regs   []int
	code   []int
	output []int
}

func (comp *ChronospatialComputer) reset(regs []int) {
	comp.ip = 0
	comp.regs = regs
	comp.output = make([]int, 0)
}

func (comp *ChronospatialComputer) Run(callback func(ipBefore int, ipAfter int, pCode int, operand int, regsBefore []int, regsAfter []int, output []int) bool) []int {
	// Clear output
	comp.output = make([]int, 0)
	// Run computer
	for {
		// If instruction pointer out of bounds, end execution
		if comp.ip < 0 || comp.ip > len(comp.code)-2 {
			break
		}
		// Get opcode and operand
		var opCode = comp.code[comp.ip]
		var operand = comp.code[comp.ip+1]
		// Store registers before execution
		var ipBefore = comp.ip
		var regsBefore = append(make([]int, 0, len(comp.regs)), comp.regs...)
		// Execute instruction
		var err = comp.executeInstruction(opCode, operand)
		// Execute callback
		if callback != nil {
			if !callback(ipBefore, comp.ip, opCode, operand, regsBefore, comp.regs, comp.output) {
				break
			}
		}
		// Check if error
		if err != nil {
			return comp.output
		}
	}
	// Return output value
	return comp.output
}

func (comp *ChronospatialComputer) executeInstruction(opCode int, operand int) error {
	// The adv instruction (opcode 0)
	if opCode == 0 {
		// Perform instruction
		var numerator = comp.regs[0]
		var combo, err = comp.getComboRegisterValue(operand)
		if err != nil {
			return errors.New("unsupported combo operand")
		}
		// var denominator = math.Exp2(float64(combo))
		// var result = int(math.Trunc(float64(numerator) / denominator))
		// comp.regs[0] = result
		comp.regs[0] = numerator >> combo
		// Increase instruction pointer
		comp.ip += 2
	} else
	// The bxl instruction (opcode 1)
	if opCode == 1 {
		// Perform instruction
		var first = comp.regs[1]
		var second = operand
		var result = first ^ second
		comp.regs[1] = result
		// Increase instruction pointer
		comp.ip += 2
	} else
	// The bst instruction (opcode 2)
	if opCode == 2 {
		// Perform instruction
		var combo, err = comp.getComboRegisterValue(operand)
		if err != nil {
			return errors.New("unsupported combo operand")
		}
		// var result = combo % 8
		var result = combo & 7
		comp.regs[1] = result
		// Increase instruction pointer
		comp.ip += 2
	} else
	// The jnz instruction (opcode 3)
	if opCode == 3 {
		// Perform instruction
		var first = comp.regs[0]
		if first == 0 {
			// Increase instruction pointer
			comp.ip += 2
		} else {
			// Jump
			var second = operand
			comp.ip = second
		}
	} else
	// The bxc instruction (opcode 4)
	if opCode == 4 {
		// Perform instruction
		var first = comp.regs[1]
		var second = comp.regs[2]
		var result = first ^ second
		comp.regs[1] = result
		// Increase instruction pointer
		comp.ip += 2
	} else
	// The out instruction (opcode 5)
	if opCode == 5 {
		// Perform instruction
		var combo, err = comp.getComboRegisterValue(operand)
		if err != nil {
			return errors.New("unsupported combo operand")
		}
		var first = combo
		// var result = first % 8
		var result = first & 7
		comp.output = append(comp.output, result)
		// Increase instruction pointer
		comp.ip += 2
	} else
	// The bdv instruction (opcode 6)
	if opCode == 6 {
		// Perform instruction
		var numerator = comp.regs[0]
		var combo, err = comp.getComboRegisterValue(operand)
		if err != nil {
			return errors.New("unsupported combo operand")
		}
		// var denominator = math.Exp2(float64(combo))
		// var result = int(math.Trunc(float64(numerator) / denominator))
		// comp.regs[1] = result
		comp.regs[1] = numerator >> combo
		// Increase instruction pointer
		comp.ip += 2
	}
	// The cdv instruction (opcode 7)
	if opCode == 7 {
		// Perform instruction
		var numerator = comp.regs[0]
		var combo, err = comp.getComboRegisterValue(operand)
		if err != nil {
			return errors.New("unsupported combo operand")
		}
		// var denominator = math.Exp2(float64(combo))
		// var result = int(math.Trunc(float64(numerator) / denominator))
		// comp.regs[2] = result
		comp.regs[2] = numerator >> combo
		// Increase instruction pointer
		comp.ip += 2
	}
	// Return no error
	return nil
}

func (comp ChronospatialComputer) getComboRegisterValue(operand int) (int, error) {
	// Combo operands 0 through 3 represent literal values 0 through 3
	if operand <= 3 {
		return operand, nil
	} else
	// Combo operand 4-6 represents the value of register A-C
	if operand >= 4 && operand <= 6 {
		return comp.regs[operand-4], nil
	}
	// Combo operand 7 is reserved and will not appear in valid programs
	{
		return -1, errors.New("unsupported combo operand value")
	}
}
