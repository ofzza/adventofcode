package year2024

import (
	solution "adventofcode/lib"
	"errors"
	"fmt"
	"math"
	"os"
	"slices"
	"strconv"
	"strings"
)

// Day one definition
type Day10 struct{}

var Day = Day10{}

// Year and day
func (day Day10) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2025,
		Day:  10,
	}
}

// Executions
func (day Day10) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Expect: 0,
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day10) Run(index int, tag string, input any, verbose bool) (any, string, error) {
	// Initialize
	var output = ""
	var value, ok = input.(string)
	if !ok {
		return nil, output, errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var linesStr = strings.Split(strings.Trim(value, "\r\n "), "\n")
	var machines = make([]Machine, len(linesStr))
	for i, lineStr := range linesStr {
		var lineParts = strings.Split(strings.Trim(lineStr, "\r\n "), " ")

		// Echo machine
		output += fmt.Sprintf("> Machine #%d:\n", i)

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
		output += fmt.Sprintf(fmt.Sprintf("  - LEDs: %%s -> %%0%db\n", size), ledsStr, leds)

		// Echo Buttons
		output += "  - Buttons: "
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
			output += fmt.Sprintf(fmt.Sprintf("%%v -> %%0%db, ", size), button, buttonBitMask)
		}
		// Echo Buttons
		output += "\n"

		// Parse joltages
		var joltagesPart = lineParts[len(lineParts)-1]
		var joltagesStr = strings.Split(strings.Trim(joltagesPart[1:len(joltagesPart)-1], "\r\n "), ",")
		var joltages = make([]int, len(joltagesStr))
		for i, joltageStr := range joltagesStr {
			var joltage, _ = strconv.Atoi(strings.Trim(joltageStr, "\r\n "))
			joltages[i] = joltage
		}
		// Echo Joltages
		output += fmt.Sprintf("  - Joltages: %v\n", joltages)

		// Store parsed machine spec
		machines[i] = Machine{size: uint(size), leds: leds, buttonMap: buttons, buttonsBitMap: buttonBitMasks, joltages: joltages}
	}

	// Part 1/2
	if index == 1 {

		// Prompt
		output += "\n"
		output += "> Initializing:\n"

		// Initialize all machines
		var presses = 0
		for i, machine := range machines {
			// Initialize machine
			var p = findInitializationSequence(machine.leds, 0b00000000, machine.buttonsBitMap, 0, 0)
			// Prompt
			output += fmt.Sprintf("  - Initialized #%d with %d button presses\n", i+1, p)
			// Store initialization presses count
			presses += p
		}

		// Return solution
		return presses, output, nil
	} else

	// Part 2/2
	if index == 2 {

		// Prompt
		fmt.Printf("\n")
		fmt.Printf("> Initializing:\n")
		output += "\n"
		output += "> Initializing:\n"

		// Stabilize all machines
		var presses = 0
		for i, machine := range machines {
			// Prompt
			fmt.Printf("  > Machine %d/%d:\n", i+1, len(machines))
			// output += fmt.Sprintf("  - Stabilized joltages #%d with %d button presses\n", i+1, p)
			// Find minimum joltage stabilizing sequence
			var p = findJoltageSequence(machine.joltages, machine.buttonMap)
			// Prompt
			fmt.Printf("    = Stabilized joltages #%d with %d button presses\n", i+1, p)
			// output += fmt.Sprintf("  - Stabilized joltages #%d with %d button presses\n", i+1, p)
			// Store initialization presses count
			presses += p
		}

		// Return solution
		return presses, output, nil
	}

	// Missing implementation
	return nil, output, errors.New("missing implementation for required index")
}

type Machine struct {
	size          uint
	leds          uint
	buttonMap     [][]int
	buttonsBitMap []uint
	joltages      []int
}

func findInitializationSequence(target uint, state uint, buttonBitMaps []uint, i int, presses int) int {
	// Try pressing or skipping the button
	var stateAfterButtonSkip = state
	var stateAfterButtonPress = state ^ buttonBitMaps[i]
	// Check resulting state(s) against target
	if stateAfterButtonSkip == target {
		return presses
	}
	if stateAfterButtonPress == target {
		return presses + 1
	}

	// Check if more buttons remain
	if i+1 >= len(buttonBitMaps) {
		return len(buttonBitMaps) + 1
	}

	// Proceed to try pressing more buttons
	var minAfterButtonSkip = findInitializationSequence(target, stateAfterButtonSkip, buttonBitMaps, i+1, presses)
	var minAfterButtonPress = findInitializationSequence(target, stateAfterButtonPress, buttonBitMaps, i+1, presses+1)
	if minAfterButtonSkip < minAfterButtonPress {
		return minAfterButtonSkip
	} else {
		return minAfterButtonPress
	}
}

func findJoltageSequence(joltages []int, joltagesPerButtonsMap [][]int) int {

	// Organize buttons by joltages
	var buttonsPerJoltageMap = make([][]int, len(joltages))
	for i, button := range joltagesPerButtonsMap {
		for _, joltage := range button {
			buttonsPerJoltageMap[joltage] = append(buttonsPerJoltageMap[joltage], i)
		}
	}

	// Calculate per button maximum presses, from allowed joltages
	var buttonMaxima = make([]uint16, len(joltagesPerButtonsMap))
	for j, buttons := range buttonsPerJoltageMap {
		var joltage = joltages[j]
		for _, b := range buttons {
			if buttonMaxima[b] == 0 || joltage < int(buttonMaxima[b]) {
				buttonMaxima[b] = uint16(joltage)
			}
		}
	}

	// Try to find ordering of joltages, where each next only has a single button not already pressed!?
	var orderedJoltageIndexes, orderedJoltageUnusedButtonsCounts, _ = orderJoltages(buttonsPerJoltageMap, []int{}, []int{})
	fmt.Printf("    ... determined order: %v\n", orderedJoltageIndexes)
	fmt.Printf("    ... buttons/degrees of freedom per joltage: %v\n", orderedJoltageUnusedButtonsCounts)

	// For each joltage, generate possible button presses
	var buttonsReadoutPermutations = [][]uint16{make([]uint16, len(joltagesPerButtonsMap))}
	var previouslyUsedButtons = make([]int, 0)
	for i, joltageIndex := range orderedJoltageIndexes {
		var cache = make(map[int][][]uint16)
		var targetJoltage = joltages[joltageIndex]
		var nextButtonsReadoutPermutations = [][]uint16{}

		// Get connected buttons, not already set by the state
		var allConnectedButtons = buttonsPerJoltageMap[joltageIndex]
		var unpressedConnectedButtons = getOnlyUnpressedButtons(allConnectedButtons, previouslyUsedButtons)
		// if len(unpressedConnectedButtons) == 0 {
		// 	continue
		// }

		// Prompt generation starting state
		fmt.Printf("    - Joltage %d/%d (#%d = %d): Generating off of %d states with %d unpressed buttons ...", i+1, len(joltages), joltageIndex+1, targetJoltage, len(buttonsReadoutPermutations), len(unpressedConnectedButtons))

		// Generate permutations
		for _, buttonReadoutPermutation := range buttonsReadoutPermutations {
			// Generate possible clicking states, compatible with current joltage
			var new = generateButtonPermutations(buttonReadoutPermutation, allConnectedButtons, unpressedConnectedButtons, buttonMaxima, targetJoltage, cache)
			nextButtonsReadoutPermutations = append(nextButtonsReadoutPermutations, new...)
		}
		buttonsReadoutPermutations = nextButtonsReadoutPermutations

		// Set buttons as used
		previouslyUsedButtons = append(previouslyUsedButtons, allConnectedButtons...)

		// Prompt generated permutations
		fmt.Printf(" found %d compatible states\n", len(buttonsReadoutPermutations))
	}

	// Find minimum button presses in all valid permutations
	var minButtonPresses = -1
	var minButtonPressesPermutation []uint16 = []uint16{}
	for _, buttonsReadoutPermutation := range buttonsReadoutPermutations {
		var presses = 0
		for _, s := range buttonsReadoutPermutation {
			presses += int(s)
		}
		if minButtonPresses == -1 || presses < minButtonPresses {
			minButtonPresses = presses
			minButtonPressesPermutation = buttonsReadoutPermutation
		}
	}

	// Verify button presses
	var reconstructedJoltages = make([]uint16, len(joltages))
	for i, presses := range minButtonPressesPermutation {
		for _, j := range joltagesPerButtonsMap[i] {
			reconstructedJoltages[j] += presses
		}
	}
	for i := 0; i < len(joltages); i++ {
		if joltages[i] != int(reconstructedJoltages[i]) {
			panic(fmt.Errorf("failed reconstructing joltages: %v / %v", joltages, reconstructedJoltages))
		}
	}

	// Return min presses
	return minButtonPresses
}

func orderJoltages(buttonsPerJoltageMap [][]int, orderedJoltageIndexes []int, orderedJoltageUnusedButtonsCounts []int) ([]int, []int, float64) {
	// Check if ordering done
	if len(orderedJoltageIndexes) == len(buttonsPerJoltageMap) {
		return orderedJoltageIndexes, orderedJoltageUnusedButtonsCounts, 1
	}

	// Collect all already "used" buttons
	var usedButtons = []int{}
	for _, i := range orderedJoltageIndexes {
		usedButtons = append(usedButtons, buttonsPerJoltageMap[i]...)
	}

	// Try orderings starting with any/all joltages (not already used)
	var bestScore = float64(-1)
	var bestJoltageOrderedIndexes = []int{}
	var bestJoltageOrderedUnusedButtonsCounts = []int{}
	for i, joltageButtons := range buttonsPerJoltageMap {
		var unusedJoltageButtons = []int{}
		for _, b := range joltageButtons {
			if !slices.Contains(usedButtons, b) {
				unusedJoltageButtons = append(unusedJoltageButtons, b)
			}
		}
		if !slices.Contains(orderedJoltageIndexes, i) {
			var ordering, buttons, score = orderJoltages(buttonsPerJoltageMap, append(orderedJoltageIndexes, i), append(orderedJoltageUnusedButtonsCounts, len(unusedJoltageButtons)))
			var calcScore = calculateOrderingScore(len(unusedJoltageButtons), score)
			if bestScore == -1 || calcScore < bestScore {
				bestScore = calcScore
				bestJoltageOrderedIndexes = ordering
				bestJoltageOrderedUnusedButtonsCounts = buttons
			}
		}
	}

	// Return best ordering
	return bestJoltageOrderedIndexes, bestJoltageOrderedUnusedButtonsCounts, bestScore
}

func calculateOrderingScore(unusedButtonsCount int, score float64) float64 {
	var normalizedUnusedButtonsCount = unusedButtonsCount
	if normalizedUnusedButtonsCount == 0 {
		normalizedUnusedButtonsCount = 5
	}
	return math.Pow(float64(normalizedUnusedButtonsCount), 3) + score
}

func generateButtonPermutations(buttonReadoutPermutation []uint16, allConnectedButtons []int, unpressedConnectedButtons []int, buttonMaxima []uint16, targetJoltage int, cache map[int][][]uint16) [][]uint16 {
	// Get current joltage
	var joltage = getReadoutJoltage(buttonReadoutPermutation, allConnectedButtons)
	// If joltage overshot, return no acceptable permutations
	if joltage > targetJoltage {
		return [][]uint16{}
	}

	// Generate permutations for unpressed buttons
	var unpressedButtonPermutations = generateRawButtonPermutations(len(unpressedConnectedButtons), targetJoltage-joltage, cache)

	// Merge existing permutation with unpressed buttons permutations
	var buttonReadoutPermutations = make([][]uint16, len(unpressedButtonPermutations))
	var buttonReadoutPermutationsIndex = 0
	for i := range unpressedButtonPermutations {
		// Merge permutation
		var mergedPermutation = append([]uint16{}, buttonReadoutPermutation...)
		for j := range unpressedConnectedButtons {
			mergedPermutation[unpressedConnectedButtons[j]] = unpressedButtonPermutations[i][j]
		}
		// Verify if any single button overshoot its allowed maximum
		var valid = true
		for i := range mergedPermutation {
			if mergedPermutation[i] > buttonMaxima[i] {
				valid = false
				break
			}
		}
		// Store merged permutation
		if valid {
			buttonReadoutPermutations[buttonReadoutPermutationsIndex] = mergedPermutation
			buttonReadoutPermutationsIndex++
		}
	}

	// Return permutations
	return buttonReadoutPermutations[0:buttonReadoutPermutationsIndex]
}

func generateRawButtonPermutations(count int, target int, cache map[int][][]uint16) [][]uint16 {
	// Check cache
	var cacheKey = 1000000*target + count
	if cacheValue, ok := cache[cacheKey]; ok {
		return cacheValue
	}

	// Initialize
	var initial = make([]uint16, count)
	var permutations = [][]uint16{initial}
	var total = 0

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
		nextPermutations = deduplicatePermutations(nextPermutations)

		// Replace with next and continue ...
		permutations = nextPermutations
		total++
	}

	// Cache generated permutations
	cache[cacheKey] = permutations

	// Return generated permutations
	return permutations
}

func deduplicatePermutations(permutations [][]uint16) [][]uint16 {
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

func getReadoutJoltage(buttonReadoutPermutation []uint16, readoutButtons []int) int {
	var joltage = 0
	for _, b := range readoutButtons {
		joltage += int(buttonReadoutPermutation[b])
	}
	return joltage
}

func getOnlyUnpressedButtons(allConnectedButtons []int, previouslyUsedButtons []int) []int {
	var filtered = make([]int, len(allConnectedButtons))
	var i = 0
	for _, b := range allConnectedButtons {
		if !slices.Contains(previouslyUsedButtons, b) {
			filtered[i] = b
			i++
		}
	}

	return filtered[0:i]
}
