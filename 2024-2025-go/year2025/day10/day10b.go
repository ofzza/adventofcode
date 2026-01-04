package year2025

import (
	solution "adventofcode/lib"
	"adventofcode/lib/matrix"
	"adventofcode/lib/numbers"
	"errors"
	"fmt"
	"math"
	"os"
	"slices"
	"strconv"
	"strings"
)

// Day one definition
type Day10B struct{}

var DayB = Day10B{}

// Year and day
func (day Day10B) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2025,
		Day:  10,
	}
}

// Executions
func (day Day10B) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day10/input-test.txt"); return string(b) }(),
					Expect: 7,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day10/input.txt"); return string(b) }(),
					Expect: 488,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day10/input-test.txt"); return string(b) }(),
					Expect: 33,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day10/input.txt"); return string(b) }(),
					Expect: 18771,
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day10B) Run(index int, tag string, input any, verbose bool, log solution.Logger) (any, string, error) {
	// Initialize
	var value, ok = input.(string)
	if !ok {
		return nil, log.Dump(), errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var linesStr = strings.Split(strings.Trim(value, "\r\n "), "\n")
	var machines = make([]Machine, len(linesStr))
	for i, lineStr := range linesStr {
		var lineParts = strings.Split(strings.Trim(lineStr, "\r\n "), " ")

		// Echo machine
		log.Log(fmt.Sprintf("> Machine #%d:\n", i))

		// Parse LEDs
		var ledsPart = lineParts[0]
		var ledsStr = ledsPart[1 : len(ledsPart)-1]
		var size = len(ledsStr)
		var leds uint = 0b00000000
		for i, ledStr := range ledsStr {
			if ledStr == '#' {
				leds |= (1 << (size - i - 1))
			}
		}
		// Echo LEDs
		log.Log(fmt.Sprintf(fmt.Sprintf("  - LEDs: %%s -> %%0%db\n", size), ledsStr, leds))

		// Echo Buttons
		log.Log("  - Buttons: ")
		// Parse button wiring
		var buttonsParts = lineParts[1 : len(lineParts)-1]
		var buttons = make([][]int, len(buttonsParts))
		var buttonBitMasks = make([]uint, len(buttonsParts))
		for j, buttonsPart := range buttonsParts {
			var buttonsPartStr = buttonsPart[1 : len(buttonsPart)-1]
			var buttonStr = strings.Split(strings.Trim(buttonsPartStr, "\r\n "), ",")
			var button = make([]int, len(buttonStr))
			var buttonBitMask uint = 0b00000000
			for k, wiringStr := range buttonStr {
				wiring, _ := strconv.Atoi(strings.Trim(wiringStr, "\r\n "))
				button[k] = wiring
				buttonBitMask |= (1 << (size - wiring - 1))
			}
			buttons[j] = button
			buttonBitMasks[j] = buttonBitMask
			// Echo Button
			log.Log(fmt.Sprintf(fmt.Sprintf("%%v -> %%0%db, ", size), button, buttonBitMask))
		}
		// Echo Buttons
		log.Log("\n")

		// Parse joltages
		var joltagesPart = lineParts[len(lineParts)-1]
		var joltagesStr = strings.Split(strings.Trim(joltagesPart[1:len(joltagesPart)-1], "\r\n "), ",")
		var joltages = make([]int, len(joltagesStr))
		for i, joltageStr := range joltagesStr {
			var joltage, _ = strconv.Atoi(strings.Trim(joltageStr, "\r\n "))
			joltages[i] = joltage
		}
		// Echo Joltages
		log.Log(fmt.Sprintf("  - Joltages: %v\n", joltages))

		// Store parsed machine spec
		machines[i] = Machine{size: uint(size), leds: leds, buttonMap: buttons, buttonsBitMap: buttonBitMasks, joltages: joltages}
	}

	// Part 1/2
	if index == 1 {

		// Prompt
		log.Log("\n")
		log.Log("> Initializing:\n")

		// Initialize all machines
		var presses = 0
		for i, machine := range machines {
			// Initialize machine
			var p = findInitializationSequence(machine.leds, 0b00000000, machine.buttonsBitMap, 0, 0)
			// Prompt
			log.Log(fmt.Sprintf("  - Initialized #%d with %d button presses\n", i, p))
			// Store initialization presses count
			presses += p
		}

		// Return solution
		return presses, log.Dump(), nil
	} else

	// Part 2/2
	if index == 2 {

		// Prompt
		log.Log("\n")
		log.Log("> Initializing:\n")

		// Stabilize all machines
		var presses = 0
		for i, machine := range machines {
			// Prompt
			log.Log(fmt.Sprintf("  > Machine %d/%d:\n", i, len(machines)))
			// Find minimum joltage stabilizing sequence
			var p = findButtonPressesB(machine.joltages, machine.buttonMap, &log)
			// Prompt
			log.Log(fmt.Sprintf("  - Stabilized joltages #%d with %d button presses\n", i, p))
			// Store initialization presses count
			presses += p
		}

		// Return solution
		return presses, log.Dump(), nil
	}

	// Missing implementation
	return nil, log.Dump(), errors.New("missing implementation for required index")
}

func findButtonPressesB(joltages []int, joltagesPerButtonsMap [][]int, log *solution.Logger) int {
	// Form initial matrix
	var values = [][]numbers.RationalNumber{}
	for _, joltage := range joltages {
		var variables = make([]numbers.RationalNumber, len(joltagesPerButtonsMap))
		for j := range variables {
			variables[j] = numbers.ToRationalNumber(0)
		}
		values = append(values, append(variables, numbers.ToRationalNumber(joltage)))
	}
	for button, joltages := range joltagesPerButtonsMap {
		for _, joltage := range joltages {
			var r = values[joltage][button]
			values[joltage][button] = r.Add(numbers.ToRationalNumber(1))
		}
	}
	var m = matrix.Matrix{ Rank: matrix.MatrixRank{ Width: len(joltagesPerButtonsMap) + 1, Height: len(joltages) }, Values: values }
	
	// Form initial solution
	var solution = make([]int, len(joltagesPerButtonsMap))
	for i:=0; i<len(joltagesPerButtonsMap); i++ {
		solution[i] = -1
	}

	// Calculate maximum possible presses per button
	var pressesLimit = make([]int, len(joltagesPerButtonsMap))
	for i := range pressesLimit {
		for j := range joltagesPerButtonsMap[i] {
			if pressesLimit[i] == 0 || joltages[joltagesPerButtonsMap[i][j]] < pressesLimit[i] {
				pressesLimit[i] = joltages[joltagesPerButtonsMap[i][j]]
			}
		}
	}

	// Echo matrix
	log.Log("\n")
	log.Log("- Initial matrix:\n")
	log.Log(m.ToString(2))

	// Perform Gaussian elimination step
	var status = matrix.MatrixGaussianEliminationStatus{}
	for {
		if m.PerformGaussianEliminationStep(&status); status.Done {
			break
		}
	}

	// Log reduced matrix
	log.Log("\n")
	log.Log("- Reduced matrix:\n")
	log.Log(m.ToString(2))

	// Find which button values need to be permuted
	var nonPermutableButtons = []int{}
	for _, equation := range m.Values {
		for i:=0; i<m.Rank.Width-1; i++ {
			if equation[i].ToFloat() == 1 {
				nonPermutableButtons = append(nonPermutableButtons, i)
				break;
			} else if equation[i].ToFloat() != 0 {
				break;
			}
		}
	}
	var permutableButtons = []int{}
	for i := range joltagesPerButtonsMap {
		if !slices.Contains(nonPermutableButtons, i) {
			permutableButtons = append(permutableButtons, i)
		}
	}
	
	// Log reduced matrix
	log.Log("\n")
	log.Log(fmt.Sprintf("- Permutable buttons: %v\n\n", permutableButtons))

	// Find matching button presses
	if done, solution := _findButtonPressesB(m, pressesLimit, -1, permutableButtons, solution, nil); done {
		// Sub up button presses
		var sum = 0;
		for _, presses := range solution {
			sum += presses
		}

		// Log reduced matrix
		log.Log("\n")
		log.Log(fmt.Sprintf("- FOUND SOLUTION: %d %v\n\n", sum, solution))
		
		return sum;
	}

	// If no solution found, return 0
	return 0;
}

func _findButtonPressesB(m matrix.Matrix, pressesLimit []int, pressesGlobalLimit int, permutableButtons []int, solution []int, log *solution.Logger) (bool, []int) {
	var variables = m.Rank.Width - 1
	var equations = m.Rank.Height	

	// Log new line
	if log != nil {
		log.Log("\n")
	}

	// Validate current solution isn+t over the limit
	var solutionPresses = 0
	for _, v := range solution {
		if v != -1 {
			solutionPresses += v
		}
	}
	if pressesGlobalLimit != -1 && solutionPresses >= pressesGlobalLimit {
		return false, []int{}
	}

	// Check if solved
	var solved = true
	for _, i := range solution {
		if i == -1 {
			solved = false;
			break;
		}
	}
	if solved {
		return true, solution
	}

	// If no equations left -> not solvable
	if m.Rank.Height == 0 {
		return false, []int{}
	}
	// If last variable and different sign than result -> not solvable
	if m.Rank.Width == 2 && m.Values[0][0].ToFloat() / math.Abs(m.Values[0][0].ToFloat()) != m.Values[0][1].ToFloat() / math.Abs(m.Values[0][1].ToFloat()) {
		return false, []int{}
	}

	// Check if any equations are directly solvedEquationIndexes with a single value
	var solvedEquationIndexes = []int{}
	var solvedVariableIndexes = []int{}
	for equation:=0; equation<equations; equation++ {
		var count = 0
		var index = 0;
		for variable:=0; variable<variables; variable++ {
			if m.Values[equation][variable].ToFloat() != 0 {
				count++
				index = variable
			}
		}
		if count == 1 {
			if slices.Contains(solvedVariableIndexes, index) && solution[index] != int(m.Values[equation][variables].ToFloat() / m.Values[equation][index].ToFloat()) {
				// Log impossible solution
				if log != nil {
					log.Log(fmt.Sprintf(">>> Found IMPOSSIBLE solution for a variable %d: %d\n", index, solution[index]))
				}
				// Return no solution
				return false, []int{}
			}
			if m.Values[equation][variables].ToFloat() / m.Values[equation][index].ToFloat() < 0 {
				// Log impossible solution
				if log != nil {
					log.Log(fmt.Sprintf(">>> Found IMPOSSIBLE solution for a variable %d: %d\n", index, solution[index]))
				}
				// Return no solution
				return false, []int{}
			}
			if m.Values[equation][variables].ToFloat() / m.Values[equation][index].ToFloat() != float64(int(m.Values[equation][variables].ToFloat() / m.Values[equation][index].ToFloat())) {
				// Log impossible solution
				if log != nil {
					log.Log(fmt.Sprintf(">>> Found IMPOSSIBLE solution for a variable %d: %d\n", index, solution[index]))
				}
				// Return no solution
				return false, []int{}
			}
			// Register solved equation and solution
			solvedEquationIndexes = append(solvedEquationIndexes, equation)
			solvedVariableIndexes = append(solvedVariableIndexes, index)
			solution[index] = int(m.Values[equation][variables].ToFloat() / m.Values[equation][index].ToFloat())
			// Log found solutions
			if log != nil {
				log.Log(fmt.Sprintf(">>> Found solution for a variable %d: %d\n", index, solution[index]))
			}
		}
	}
	// Clear solved variable from all equations
	if len(solvedVariableIndexes) > 0 {
		for _, i	:= range solvedVariableIndexes {
			for equation:=0; equation<len(m.Values); equation++ {
				m.Values[equation][variables] = m.Values[equation][variables].Subtract(numbers.ToRationalNumber(solution[i]).Multiply(m.Values[equation][i]))
				m.Values[equation][i] = numbers.ToRationalNumber(0)
			}
		}
		// Log reduced matrix
		if log != nil {
			log.Log("'... replaced solved variables:\n")
			log.Log(m.ToString(2))
		}
	}
	// Remove solved equations
	if len(solvedEquationIndexes) > 0 {
		// Remove solved equations
		m.RemoveRows(solvedEquationIndexes)
		// Log reduced matrix
		if log != nil {
			log.Log("... removed solved equations:\n")
			log.Log(m.ToString(2))
		}
	}

	// If solution updated, try solving another variable
	if len(solvedVariableIndexes) > 0 {
		if done, foundSolution := _findButtonPressesB(m.Clone(), pressesLimit, pressesGlobalLimit, permutableButtons, append([]int{}, solution...), log); done {
			return true, foundSolution
		} else {
			return false, []int{}
		}
	}

	// Count up presses in the current solution
	solutionPresses = 0
	for _, v := range solution {
		if v != -1 {
			solutionPresses += v
		}
	}

	// Try assuming values for first unknown variable
	var bestPresses = pressesGlobalLimit
	var bestSolution = []int{}
	for _, i := range permutableButtons {
		if solution[i] == -1 && (bestPresses == -1 || solutionPresses <= bestPresses) {
			for assumed:=0; assumed<=pressesLimit[i] && (bestPresses == -1 || (solutionPresses <= bestPresses && assumed<=(bestPresses-solutionPresses))); assumed++ {
				// Compose proposed solution
				var assumedSolution = append([]int{}, solution...)
				assumedSolution[i] = assumed
				// Log assumed value
				if log != nil {
					log.Log(fmt.Sprintf("... assuming value for a variable #%d: %d %v\n", i, assumed, assumedSolution))
				}
				// Compose matrix with assumed solution applied
				var cloned = m.Clone()
				for equation:=0; equation<len(cloned.Values); equation++ {
					cloned.Values[equation][variables] = cloned.Values[equation][variables].Subtract(numbers.ToRationalNumber(assumedSolution[i]).Multiply(m.Values[equation][i]))
					cloned.Values[equation][i] = numbers.ToRationalNumber(0)
				}
				// Log assumed matrix
				if log != nil {
					log.Log("... assumed matrix:\n")
					log.Log(cloned.ToString(2))
				}
				// Try solving remaining equations
				if done, foundSolution := _findButtonPressesB(cloned, pressesLimit, bestPresses, permutableButtons, append([]int{}, assumedSolution...), log); done {
					// Calculate presses
					var presses = 0;
					for _, i := range foundSolution { 
						presses += i
					}
					if bestPresses == -1 || presses < bestPresses {
						bestPresses = presses
						bestSolution = foundSolution
						// Log found BEST solution
						if log != nil {
							log.Log("\n")
							log.Log(fmt.Sprintf(">>> FOUND SOLUTION WITH %d PRESSES: %v\n", presses, foundSolution))
						}
					}
				}
			}
		}
	}
	
	// No solution found
	if bestPresses != -1 && len(bestSolution) == len(solution) {
		return true, bestSolution
	} else {
		return false, []int{}
	}
}

