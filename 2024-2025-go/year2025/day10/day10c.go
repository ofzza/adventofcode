package year2025

import (
	solution "adventofcode/lib"
	"adventofcode/lib/matrix"
	"adventofcode/lib/numbers"
	"errors"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

// Day one definition
type Day10C struct{}

var Day = Day10C{}

// Year and day
func (day Day10C) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2025,
		Day:  10,
	}
}

// Executions
func (day Day10C) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
func (day Day10C) Run(index int, tag string, input any, verbose bool, log solution.Logger) (any, string, error) {
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

		// Initialize long-running cache
		var cache = make(map[int][][][]uint16);

		// Stabilize all machines in parallel
		var presses = 0
		for i, machine := range machines {
			// Prompt
			log.Log(fmt.Sprintf("  > Machine %d/%d:\n", i, len(machines)))
			// Find minimum joltage stabilizing sequence
			var p = _findButtonPressesC(machine.joltages, machine.buttonMap, cache, &log)
			if p < 0 {
				panic("Failed to find button presses permutation!")
			}
			// Prompt
			log.Log(fmt.Sprintf("  - Stabilized joltages #%d with %d button presses\n", i+1, p))
			// Count presses
			presses += p
		}

		// Return solution
		return presses, log.Dump(), nil
	}

	// Missing implementation
	return nil, log.Dump(), errors.New("missing implementation for required index")
}

func _findButtonPressesC(joltages []int, joltagesPerButtonsMap [][]int, cache map[int][][][]uint16, log *solution.Logger) int {
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

	// Echo matrix
	log.Log("\n")
	log.Log("- Initial matrix:\n")
	log.Log(m.ToString(2))

	// Perform Gaussian elimination
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
	nonPermutableButtons = []int{}
	for i := range joltagesPerButtonsMap {
		if !slices.Contains(permutableButtons, i) {
			nonPermutableButtons = append(nonPermutableButtons, i)
		}
	}
	
	// Log premutable buttons
	log.Log("\n")
	log.Log(fmt.Sprintf("- Permutable buttons: %v\n\n", permutableButtons))

	// Calculate max allowed presses
	var maxPresses = -1
	for _, button := range permutableButtons {
		var buttonJoltages = joltagesPerButtonsMap[button]
		var minJoltage = -1
		for _, j := range buttonJoltages {
			if minJoltage == -1 || joltages[j] < minJoltage {
				minJoltage = joltages[j]
			}
		}
		if maxPresses == -1 || maxPresses < minJoltage {
			maxPresses = minJoltage
		}
	}

	// If no permutable buttons, calculate valid solution
	if len(permutableButtons) == 0 {
		if done, foundSolution := _validateButtonPressesPermutationC(m, solution, nonPermutableButtons, nil); done {
			// Sub up button presses
			var sum = 0;
			for _, presses := range foundSolution {
				sum += presses
			}
			// Return button presses
			return sum
		} else {
			// Failed to validate solution
			return -1
		}
	}

	// Find matching button presses
	if done, solution := _findButtonPressesPermutationC(m, permutableButtons, nonPermutableButtons, maxPresses, solution, cache, nil); done {
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
	return -1;
}

func _findButtonPressesPermutationC(m matrix.Matrix, permutableButtons []int, nonPermutableButtons []int, maxPresses int, solution []int, cache map[int][][][]uint16, log *solution.Logger) (bool, []int) {
	var variables = m.Rank.Width - 1

	// Log new line
	if log != nil {
		log.Log("\n")
	}

	// Try assuming values for first unknown variable
	var bestPresses = -1
	var bestSolution = []int{}
	for limit:=0; limit<=maxPresses; limit++ {
		// Generate permutations
		var permutations = _generatePermutations(len(permutableButtons), limit, cache)
		for _, permutation := range permutations {

			// Compose an presumed solution
			var presumedSolutions = make([]int, len(solution))
			copy(presumedSolutions, solution)
			for i:=0; i<len(permutableButtons); i++ {
				presumedSolutions[permutableButtons[i]] = int(permutation[i])
			}
			// Log assumed value
			if log != nil {
				log.Log("\n")
				log.Log(fmt.Sprintf("... assuming solution limited to %d (%v): %v\n", limit, permutation, presumedSolutions))
			}

			// Compose matrix with presumed solution applied
			var cloned = m.Clone()
			for _, i :=  range permutableButtons {
				for equation:=0; equation<len(cloned.Values); equation++ {
					cloned.Values[equation][variables] = cloned.Values[equation][variables].Subtract(numbers.ToRationalNumber(presumedSolutions[i]).Multiply(m.Values[equation][i]))
					cloned.Values[equation][i] = numbers.ToRationalNumber(0)
				}
			}
			// Log presumed matrix
			if log != nil {
				log.Log("... assumed matrix:\n")
				log.Log(cloned.ToString(2))
			}

			// Validate presumed solution
			if done, foundSolution := _validateButtonPressesPermutationC(cloned, presumedSolutions, nonPermutableButtons, log); done {
				// Calculate presses
				var presses = 0;
				for _, i := range foundSolution { 
					presses += i
				}
				// Store best solution
				if bestPresses == -1 || bestPresses > presses {
					// Store best solution
					bestPresses = presses
					bestSolution = foundSolution
					// Log found BEST solution
					if log != nil {
						log.Log("\n")
						log.Log(fmt.Sprintf("  >>> FOUND SOLUTION WITH %d PRESSES: %v\n\n", bestPresses, foundSolution))
					}
				}
			}
		}
	}
	
	// Check if (best) solution found
	if len(bestSolution) == len(solution) {
		return true, bestSolution
	} else {
		return false, []int{}
	}
}

func _validateButtonPressesPermutationC(m matrix.Matrix, solution []int, nonPermutableButtons []int, log *solution.Logger) (bool, []int) {
	var variables = m.Rank.Width - 1
	var equations = m.Rank.Height	

	// Log new line
	if log != nil {
		log.Log("\n")
	}

	// Check if solved
	if !slices.Contains(solution, -1) {
		return true, solution
	}

	// Solve directly solvable equations
	for equation:=0; equation<equations; equation++ {
		var index = nonPermutableButtons[equation]
		if m.Values[equation][variables].ToFloat() / m.Values[equation][index].ToFloat() < 0 {
			// Log impossible solution
			if log != nil {
				log.Log(fmt.Sprintf("  >>> Found IMPOSSIBLE solution for a variable %d: %d\n", index, solution[index]))
			}
			// Return no solution
			return false, []int{}
		}
		if m.Values[equation][variables].ToFloat() / m.Values[equation][index].ToFloat() != float64(int(m.Values[equation][variables].ToFloat() / m.Values[equation][index].ToFloat())) {
			// Log impossible solution
			if log != nil {
				log.Log(fmt.Sprintf("  >>> Found IMPOSSIBLE solution for a variable %d: %d\n", index, solution[index]))
			}
			// Return no solution
			return false, []int{}
		}
		// Register solved equation and solution
		solution[index] = int(m.Values[equation][variables].ToFloat() / m.Values[equation][index].ToFloat())
		// Log found solutions
		if log != nil {
			log.Log(fmt.Sprintf("  >>> Found solution for a variable %d: %d\n", index, solution[index]))
		}
	}

	// No solution found
	return true, solution
}

func _generatePermutations(count int, target int, cache map[int][][][]uint16) [][]uint16 {
	// Initialize
	var initial = make([]uint16, count)
	var permutations = [][]uint16{initial}
	var total = 0

	// Check cache
	if cached, ok := cache[count]; ok {
		var index = len(cached)-1
		if index > target { index = target }
		permutations = append([][]uint16{}, cached[index]...)
		total = index
	} else {
		cache[count] = append(make([][][]uint16, 0), [][]uint16{initial})
	}

	for {
		if total == target {
			break
		}

		var nextPermutations = make([][]uint16, len(permutations)*count)
		for i, permutation := range permutations {
			for b := 0; b < count; b++ {
				var nextPermutation = append([]uint16{}, permutation...)
				nextPermutation[b]++
				nextPermutations[i*count+b] = nextPermutation
			}
		}

		// Deduplicate permutations
		nextPermutations = _deduplicatePermutations(nextPermutations)

		// Replace with next and continue ...
		permutations = nextPermutations
		total++

		// Cache generated permutations
		cache[count] = append(cache[count], nextPermutations)
	}


	// Return generated permutations
	return permutations
}

func _deduplicatePermutations(permutations [][]uint16) [][]uint16 {
	var dedup = make(map[string][]uint16)
	for _, n := range permutations {
		dedup[fmt.Sprintf("%v", n)] = n
	}
	permutations = make([][]uint16, len(dedup))
	var i = 0
	for _, n := range dedup {
		permutations[i] = n
		i++
	}
	return permutations
}
