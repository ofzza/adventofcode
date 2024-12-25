package year2024

import (
	solution "adventofcode/lib"
	"errors"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

// Day one definition
type Day24 struct{}

var Day = Day24{}

// Year and day
func (day Day24) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2024,
		Day:  24,
	}
}

// Executions
func (day Day24) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  []any{'_', 0, func() string { var b, _ = os.ReadFile("./year2024/data/day24/input-test-01.txt"); return string(b) }()},
					Expect: 4,
				},
			)
			executions = append(
				executions,
				solution.SolutionExecution{
					Index:  1,
					Tag:    "test",
					Input:  []any{'_', 0, func() string { var b, _ = os.ReadFile("./year2024/data/day24/input-test-02.txt"); return string(b) }()},
					Expect: 2024,
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
					Input:  []any{'_', 0, func() string { var b, _ = os.ReadFile("./year2024/data/day24/input.txt"); return string(b) }()},
					Expect: 55114892239566,
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
					Input:  []any{'&', 2, func() string { var b, _ = os.ReadFile("./year2024/data/day24/input-test-03.txt"); return string(b) }()},
					Expect: "z00,z01,z02,z05",
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
					Input:  []any{'+', 4, func() string { var b, _ = os.ReadFile("./year2024/data/day24/input.txt"); return string(b) }()},
					Expect: nil,
					// "dkn,wvt,z09,z12,z16,z28,z32,z39" Not the right answer
					// "bbv,bch,bdd,bfw,bgj,bhn,bjj,sdn" Not the right answer
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day24) Run(index int, tag string, input any, verbose bool) (any, string, error) {
	// Initialize
	var output = ""
	var inputs, okInputs = input.([]any)
	var operation, okOperation = inputs[0].(rune)
	var switches, okSwitches = inputs[1].(int)
	var value, okValue = inputs[2].(string)
	if !okInputs || !okOperation || !okSwitches || !okValue {
		return nil, output, errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var sections = strings.Split(strings.Trim(value, " \n"), "\n\n")
	var wires = make(map[string]Wire, 0)

	// Parse initial states
	for _, line := range strings.Split(strings.Trim(sections[0], " \n"), "\n") {
		// Store wide definition
		var parsed = strings.Split(strings.Trim(line, " \n"), ":")
		var key = strings.Trim(parsed[0], " \n")
		var value = strings.Trim(parsed[1], " \n") == "1"
		wires[key] = Wire{key: key, resolved: true, value: value}
	}

	// Parse dependencies
	for _, line := range strings.Split(strings.Trim(sections[1], " \n"), "\n") {
		// Store wide definition
		var parsed = strings.Split(strings.Trim(line, " \n"), "->")
		var key = strings.Trim(parsed[1], " \n")
		parsed = strings.Split(strings.Trim(parsed[0], " \n"), " ")
		var inputA = strings.Trim(parsed[0], " \n")
		var inputB = strings.Trim(parsed[2], " \n")
		var operator = strings.Trim(parsed[1], " \n")
		wires[key] = Wire{key: key, resolved: false, inputs: []string{inputA, inputB}, operator: operator}
	}

	// Part 1/2
	if index == 1 {

		// Get 'z' wire numeric result
		var result, _, _, _ = resolveValue('z', wires)

		// Return solution
		return result, output, nil
	} else

	// Part 2/2
	if index == 2 {

		// Get x, y and t results
		var xInt, xBinary, _, _ = resolveValue('x', wires)
		var yInt, yBinary, _, _ = resolveValue('y', wires)
		var expectedInt int
		if operation == '&' {
			expectedInt = xInt & yInt
		} else if operation == '+' {
			expectedInt = xInt + yInt
		}
		var expectedBinary = strconv.FormatInt(int64(expectedInt), 2)
		var zInt, zBinary, _, _ = resolveValue('z', wires)

		// Output calculation
		var diff, diffPositions = diffBinary(expectedBinary, zBinary)
		if verbose {
			output += fmt.Sprintf("- X    : %16s = %v\n", xBinary, xInt)
			output += fmt.Sprintf("- Y    : %16s = %v\n", yBinary, yInt)
			output += fmt.Sprintf("- X + Y: %16s = %v \n", expectedBinary, expectedInt)
			output += fmt.Sprintf("- Z    : %16s = %v\n", zBinary, zInt)
			output += fmt.Sprintf("  diff=%v (%v)\n", diff, diffPositions)
		}

		// Fix wires
		if expectedInt != zInt {
			var ok, swapped = fixWires('z', expectedBinary, wires, int(switches))

			// Echo solution
			if verbose {
				output += fmt.Sprint("---\n\n")
				output += fmt.Sprintf("Found solution: %v\n", swapped)
			}

			// Validate solution
			var wiresValidation = cloneWires(wires)
			for _, swap := range swapped {
				var keyA = swap[0]
				var wireA = wires[keyA]
				var keyB = swap[1]
				var wireB = wires[keyB]
				wiresValidation[keyA] = wireB
				wiresValidation[keyB] = wireA
			}
			var _, zBinary, _, _ = resolveValue('z', wiresValidation)
			var diff, _ = diffBinary(zBinary, expectedBinary)
			if diff != 0 {
				return nil, output, errors.New("solution failed validation")
			}

			// Collect and sort swapped
			var solution = make([]string, 0, len(swapped)*2)
			for _, swap := range swapped {
				solution = append(solution, swap[0])
				solution = append(solution, swap[1])
			}
			slices.Sort(solution)

			if ok {
				return strings.Join(solution, ","), output, nil
			}
		}

		// Return solution
		return nil, output, nil
	}

	// Missing implementation
	return nil, output, errors.New("missing implementation for required index")
}

type Wire struct {
	// Resolved state
	key          string
	resolved     bool
	value        bool
	dependencies []string
	// Calculation definition
	operator string
	inputs   []string
	// Processing flags
	swapped bool
}

func fixWires(key rune, expectedBinaryValue string, wires map[string]Wire, maxDepth int) (bool, [][]string) {
	return fixWiresInternal(key, expectedBinaryValue, wires, 0, maxDepth)
}

func fixWiresInternal(key rune, expectedBinaryValue string, wires map[string]Wire, depth int, maxDepth int) (bool, [][]string) {
	// Get values to fix
	var _, zBinary, _, err = resolveValue('z', wires)
	if err != nil {
		return false, [][]string{}
	}
	// Get diff before fix
	var diff, diffPositions = diffBinary(zBinary, expectedBinaryValue)

	// Check if no difference
	if diff == 0 && depth == maxDepth {
		return true, [][]string{}
	} else if depth >= maxDepth {
		return false, [][]string{}
	}

	// For each diff position, check dependencies
	var diffWires = make(map[string]Wire, 0)
	for _, p := range diffPositions {
		var k = fmt.Sprintf("%c%02d", key, p)
		diffWires[k] = wires[k]
	}

	// Collect and deduplicate all dependencies of failing wires
	var diffDependenciesMap = make(map[string]bool, 0)
	for _, w := range diffWires {
		diffDependenciesMap[w.key] = true
		for _, dependencyKey := range w.dependencies {
			if len(wires[dependencyKey].inputs) > 0 {
				diffDependenciesMap[dependencyKey] = true
			}
		}
	}

	// If diff dependencies is empty (already matched), try finding any swap that doesn't undo the match
	if len(diffDependenciesMap) == 0 {
		diffDependenciesMap = make(map[string]bool, 0)
		for k, w := range wires {
			if len(w.inputs) > 0 && !w.swapped {
				diffDependenciesMap[k] = true
			}
		}
	}

	// Sort dependencies
	var diffDependencies = make([]string, 0, len(diffDependenciesMap))
	for k := range diffDependenciesMap {
		diffDependencies = append(diffDependencies, k)
	}

	// Sort dependencies
	slices.Sort(diffDependencies)

	// Try swapping dependencies
	for i, keyA := range diffDependencies {
		for _, keyB := range diffDependencies {
			if keyA >= keyB || wires[keyA].swapped || wires[keyB].swapped {
				continue
			}

			// Clone wires
			var wiresSwapped = cloneWires(wires)

			// Swap wires
			var swappedWireA = wiresSwapped[keyA]
			swappedWireA.swapped = true
			var swappedWireB = wiresSwapped[keyB]
			swappedWireB.swapped = true
			wiresSwapped[keyA] = swappedWireB
			wiresSwapped[keyB] = swappedWireA

			// Echo
			if depth < 3 {
				fmt.Printf("%*s- Swapped %v (%d / %d) ...\n", 2*depth, "", []string{keyA, keyB}, i, len(diffDependencies))
			}

			// Reevaluate with swapped wires
			var success, successKeys = fixWiresInternal(key, expectedBinaryValue, wiresSwapped, depth+1, maxDepth)
			if success {
				return true, append([][]string{{keyA, keyB}}, successKeys...)
			}

		}
	}

	// Return no fix found
	return false, [][]string{}
}

func diffBinary(a string, b string) (int, []int) {
	var count int = 0
	var positions = make([]int, 0, len(a))
	if len(a) > len(b) {
		a = a[0:len(b)]
	}
	if len(b) < len(a) {
		b = b[0:len(a)]
	}
	for i := 0; i < len(a); i++ {
		if a[i] != b[i] {
			count++
			positions = append(positions, i)
		}
	}
	return count, positions
}

func resolveValue(key rune, wires map[string]Wire) (int, string, []string, error) {
	// Find keys
	var keys = make([]string, 0, len(wires))
	for k, _ := range wires {
		if rune(k[0]) == key {
			keys = append(keys, k)
		}
	}
	slices.Sort(keys)
	slices.Reverse(keys)
	// Calculate values for keys
	var values = make([]string, 0, len(keys))
	var dedup = make(map[string]bool, 0)
	for _, key := range keys {
		var value, err, deps = resolveWire(key, wires, []string{})
		if err != nil {
			return 0, "", []string{}, err
		}
		for _, d := range deps {
			dedup[d] = true
		}
		var zValue string
		if value {
			zValue = "1"
		} else {
			zValue = "0"
		}
		values = append(values, zValue)
	}
	// Compose binary value
	var valueBinary = strings.Join(values, "")
	// Calculate integer value
	var valueInt, _ = strconv.ParseInt(valueBinary, 2, 64)
	// Return values
	var dependencies = make([]string, 0, len(dedup))
	for d := range dedup {
		dependencies = append(dependencies, d)
	}
	return int(valueInt), valueBinary, dependencies, nil
}

func resolveWire(key string, wires map[string]Wire, circular []string) (bool, error, []string) {
	// Check if circular dependency
	if slices.Contains(circular, key) {
		return false, errors.New("circular dependency detected"), []string{}
	}
	// Get wire
	var wire, _ = wires[key]
	// Check if wire already resolved
	if wire.resolved {
		return wire.value, nil, wire.dependencies
	} else
	// If wire not resolved, resolve from inputs
	{
		// Resolve input values
		var inputResultA, errA, dependenciesA = resolveWire(wire.inputs[0], wires, append(circular, []string{key}...))
		if errA != nil {
			return false, fmt.Errorf("failed resolving wire '%s': %s", wire.inputs[0], errA.Error()), []string{}
		}
		var inputResultB, errB, dependenciesB = resolveWire(wire.inputs[1], wires, append(circular, []string{key}...))
		if errA != nil {
			return false, fmt.Errorf("failed resolving wire '%s': %s", wire.inputs[1], errB.Error()), []string{}
		}
		var operator = wire.operator
		// Store dependencies
		var dedup = make(map[string]bool, 0)
		dedup[wire.inputs[0]] = true
		dedup[wire.inputs[1]] = true
		for _, d := range dependenciesA {
			dedup[d] = true
		}
		for _, d := range dependenciesB {
			dedup[d] = true
		}
		var dependencies = make([]string, 0, len(dedup))
		for d := range dedup {
			dependencies = append(dependencies, d)
		}
		// Calculate output
		if operator == "AND" {
			wire.resolved = true
			wire.value = inputResultA && inputResultB
			wire.dependencies = dependencies
			wires[key] = wire
		} else if operator == "OR" {
			wire.resolved = true
			wire.value = inputResultA || inputResultB
			wire.dependencies = dependencies
			wires[key] = wire
		} else if operator == "XOR" {
			wire.resolved = true
			wire.value = !((inputResultA && inputResultB) || (!inputResultA && !inputResultB))
			wire.dependencies = dependencies
			wires[key] = wire
		}
		// If resolved, return resolved value
		if wire.resolved {
			return wire.value, nil, wire.dependencies
		} else
		// If not resolved, return error
		{
			return false, fmt.Errorf("failed resolving wire '%s'", key), []string{}
		}
	}
}

func cloneWires(wires map[string]Wire) map[string]Wire {
	// Clone wires
	var wiresSwapped = make(map[string]Wire, 0)
	for k, w := range wires {
		if len(w.inputs) == 0 {
			wiresSwapped[k] = Wire{key: k, resolved: true, value: w.value, swapped: w.swapped}
		} else {
			wiresSwapped[k] = Wire{key: k, resolved: false, value: false, operator: w.operator, inputs: append(make([]string, 0, 2), w.inputs...), swapped: w.swapped}
		}
	}
	// Return cloned wires
	return wiresSwapped
}
