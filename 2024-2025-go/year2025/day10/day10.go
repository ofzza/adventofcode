package year2024

import (
	solution "adventofcode/lib"
	"errors"
	"fmt"
	"os"
	"slices"
	"sort"
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

type State struct {
	set   bool
	value int
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

func findJoltageSequence(joltages []int, buttonsMap [][]int) int {
	// Organize joltages by buttons
	var joltageMap = make([][]int, len(joltages))
	for i, button := range buttonsMap {
		for _, joltage := range button {
			joltageMap[joltage] = append(joltageMap[joltage], i)
		}
	}

	// Order joltages by size
	var joltageIndexes = make([]int, len(joltages))
	for i, _ := range joltages {
		joltageIndexes[i] = i
	}
	sort.Slice(joltageIndexes, func(i int, j int) bool {
		if len(joltageMap[joltageIndexes[i]]) != len(joltageMap[joltageIndexes[j]]) {
			return len(joltageMap[joltageIndexes[i]]) < len(joltageMap[joltageIndexes[j]])
		} else {
			return joltages[joltageIndexes[i]] < joltages[joltageIndexes[j]]
		}
	})

	// For each joltage, generate possible button presses
	var states = [][]State{make([]State, len(buttonsMap))}
	for i, j := range joltageIndexes {
		var joltage = joltages[j]
		var next = [][]State{}

		// Prompt compatible states
		fmt.Printf("    - Joltage %d/%d (#%d): Generating off of %d states ...", i+1, len(joltages), j+1, len(states))

		for _, state := range states {

			// Get connected buttons, not already set by the state
			var joltageButtons = joltageMap[j]
			var freeButtons = filterOutButtons(joltageButtons, state)
			// Generate possible clicking states, compatible with current joltage
			if len(freeButtons) > 0 {
				var new = generateClickPermutations(state, joltageButtons, freeButtons, joltage)
				next = append(next, new...)
			} else {
				next = append(next, state)
			}
		}
		states = next

		// Prompt compatible states
		fmt.Printf(" found %d compatible states\n", len(states))
	}

	// Find minimum state
	var min = -1
	for _, state := range states {
		var count = 0
		for _, s := range state {
			count += s.value
		}
		if min == -1 || count < min {
			min = count
		}
	}
	return min
}

func generateClickPermutations(state []State, allButtons []int, freeButtons []int, limit int) [][]State {
	var count = countClickInState(state, allButtons)
	for _, b := range freeButtons {
		state[b].set = true
	}
	var states = [][]State{state}
	for {
		if count >= limit {
			break
		}

		var next = make([][]State, len(states)*len(freeButtons))
		for i, state := range states {
			for j, b := range freeButtons {
				var n = append([]State{}, state...)
				n[b].value++
				next[i*len(freeButtons)+j] = n
			}
		}

		var dedup = make(map[string][]State)
		for _, n := range next {
			dedup[fmt.Sprintf("%v", n)] = n
		}
		next = make([][]State, len(dedup))
		var i = 0
		for _, n := range dedup {
			next[i] = n
			i++
		}

		states = next
		count++
	}

	return states
}

func countClickInState(state []State, buttons []int) int {
	var count = 0
	for _, b := range buttons {
		if state[b].set {
			count += state[b].value
		}
	}
	return count
}

func filterOutButtons(buttons []int, state []State) []int {
	var remove = make([]int, len(state))
	var i = 0
	for j, s := range state {
		if s.set {
			remove[i] = j
			i++
		}
	}
	if i == 0 {
		return buttons
	}
	remove = remove[0:i]

	var filtered = make([]int, len(buttons))
	i = 0
	for _, b := range buttons {
		if !slices.Contains(remove, b) {
			filtered[i] = b
			i++
		}
	}

	return filtered[0:i]
}
