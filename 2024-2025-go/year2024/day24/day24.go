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
					Input:  []any{"_", 0, func() string { var b, _ = os.ReadFile("./year2024/data/day24/input-test-01.txt"); return string(b) }()},
					Expect: 4,
				},
			)
			executions = append(
				executions,
				solution.SolutionExecution{
					Index:  1,
					Tag:    "test",
					Input:  []any{"_", 0, func() string { var b, _ = os.ReadFile("./year2024/data/day24/input-test-02.txt"); return string(b) }()},
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
					Input:  []any{"_", 0, func() string { var b, _ = os.ReadFile("./year2024/data/day24/input.txt"); return string(b) }()},
					Expect: 55114892239566,
				},
			)
		}
	}
	// Part 2/2
	if index == 0 || index == 2 {
		// Solution
		if tag == "" || tag == "solution" {
			executions = append(
				executions,
				solution.SolutionExecution{
					Index:  2,
					Tag:    "solution",
					Input:  []any{"+", 4, func() string { var b, _ = os.ReadFile("./year2024/data/day24/input.txt"); return string(b) }()},
					Expect: nil,
					// "dkn,wvt,z09,z12,z16,z28,z32,z39" Not the right answer
					// "bbv,bch,bdd,bfw,bgj,bhn,bjj,sdn" Not the right answer
					// "bbv,bqf,crw,dhm,gfm,gnn,pqv,shg" Not the right answer
					// "bqf,cdj,gfm,mrb,qjd,z08,z16,z32" Not the right answer (verified!)
					// "cdj,dhm,gfm,mrb,qjd,z08,z16,z32"
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day24) Run(index int, tag string, input any, verbose bool, log solution.Logger) (any, string, error) {
	// Initialize
	var inputs, okInputs = input.([]any)
	var _, okOperation = inputs[0].(string)
	var _, okSwaps = inputs[1].(int)
	var value, okValue = inputs[2].(string)
	if !okInputs || !okOperation || !okSwaps || !okValue {
		return nil, log.Dump(), errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var sections = strings.Split(strings.Trim(value, " \n"), "\n\n")
	var wires = make(map[string]Wire, 0)
	var wiresVerification = make(map[string]Wire, 0)

	// Parse initial states
	for _, line := range strings.Split(strings.Trim(sections[0], " \n"), "\n") {
		// Store wide definition
		var parsed = strings.Split(strings.Trim(line, " \n"), ":")
		var key = strings.Trim(parsed[0], " \n")
		var value = strings.Trim(parsed[1], " \n") == "1"
		wires[key] = Wire{value: value}
		wiresVerification[key] = Wire{value: value}
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
		wires[key] = Wire{inputs: []string{inputA, inputB}, operator: operator}
		wiresVerification[key] = Wire{inputs: []string{inputA, inputB}, operator: operator}
	}

	// Part 1/2
	if index == 1 {

		// Get 'z' wire numeric result
		var result, _, _ = resolveValue(wires)

		// Return solution
		return result, log.Dump(), nil
	} else

	// Part 2/2
	if index == 2 {

		// Echo
		if verbose {
			log.Log("- Verifying digits:\n")
		}

		// Verify digits
		for i := 0; i <= 45; i++ {
			// Verify digit
			var verified = verifyHalfAdder(i, wires) && verifyDependencies(i, wires)
			// Echo
			if verbose {
				var key = fmt.Sprintf("z%02d", i)
				if verified {
					log.Log(fmt.Sprintf("  - Verifying digit #%02d: Verified\n", i))
					log.Log(fmt.Sprintf("    %v\n", unpackDefinition(key, wires)))
					log.Log("\n")
				} else {
					log.Log(fmt.Sprintf("  - Verifying digit #%02d: Failed!!!\n", i))
					log.Log(fmt.Sprintf("    %v\n", unpackDefinition(key, wires)))
					log.Log("\n")
				}
			}
		}

		// Echo
		if verbose {
			log.Log("\n\n")
			log.Log("- Fixing digits (STRICT):\n")
		}

		// Try fixing wires (STRICT)
		var ok, swaps = fixWires(wires, []string{}, []string{}, true, func(swaps []string, faults []int) {
			if verbose {
				log.Log(fmt.Sprintf("  %*s- Having swapped %v, remaining %v faults: %v\n", len(swaps), "", swaps, len(faults), faults))
			}
		})
		if ok {
			// Verify best solution
			for i := 0; i < len(swaps); i += 2 {
				var wireA = wiresVerification[swaps[i]]
				var wireB = wiresVerification[swaps[i+1]]
				wiresVerification[swaps[i]] = wireB
				wiresVerification[swaps[i+1]] = wireA
			}
			var faults = verifyAdder(wiresVerification)
			// Sort swapped wires
			slices.Sort(swaps)
			// Echo
			if verbose {
				log.Log(fmt.Sprintf("- Best solution found: %v\n", swaps))
				if len(faults) == 0 {
					log.Log("  ... verification: VERIFIED!\n")
				} else {
					log.Log("  ... verification: FAILED!!!\n")
				}
			}
			// Return sorted, swapped wires
			return strings.Join(swaps, ","), log.Dump(), nil
		} else

		// Try fixing in non strict mode
		{
			// Echo
			if verbose {
				log.Log("\n\n")
				log.Log("- Fixing digits (NOT STRICT):\n")
			}

			// Try fixing wires (NOT STRICT)
			var ok, swaps = fixWires(wires, []string{}, []string{}, false, func(swaps []string, faults []int) {
				if verbose {
					log.Log(fmt.Sprintf("  %*s- Having swapped %v, remaining %v faults: %v\n", len(swaps), "", swaps, len(faults), faults))
				}
			})
			if ok {
				// Verify best solution
				for i := 0; i < len(swaps); i += 2 {
					var wireA = wiresVerification[swaps[i]]
					var wireB = wiresVerification[swaps[i+1]]
					wiresVerification[swaps[i]] = wireB
					wiresVerification[swaps[i+1]] = wireA
				}
				var faults = verifyAdder(wiresVerification)
				// Sort swapped wires
				slices.Sort(swaps)
				// Echo
				if verbose {
					log.Log(fmt.Sprintf("- Best solution found: %v\n", swaps))
					if len(faults) == 0 {
						log.Log("  ... verification: VERIFIED!\n")
					} else {
						log.Log("  ... verification: FAILED!!!\n")
					}
				}
				// Return sorted, swapped wires
				return strings.Join(swaps, ","), log.Dump(), nil
			}
		}

		// Return solution
		return nil, log.Dump(), nil
	}

	// Missing implementation
	return nil, log.Dump(), errors.New("missing implementation for required index")
}

type Wire struct {
	value    bool
	operator string
	inputs   []string
}

func fixWires(wires map[string]Wire, wireKeys []string, swaps []string, strict bool, callback func(swaps []string, faults []int)) (bool, []string) {
	// Verify digits
	var faults = verifyAdder(wires)
	// Callback
	if callback != nil {
		callback(swaps, faults)
	}
	// If fixed, return fix
	if len(faults) == 0 {
		return true, swaps
	}
	// If too many swaps, stop
	if len(swaps) >= 8 {
		return false, []string{}
	}

	// Generate ordered wire keys if not passed
	if len(wireKeys) != len(wires) {
		wireKeys = make([]string, 0, len(wires))
		for key := range wires {
			wireKeys = append(wireKeys, key)
		}
		slices.Sort(wireKeys)
	}

	// Target a faulty wire and try only swapping its dependencies
	var faultyWiresDependencies = make([]string, 0)
	for _, fault := range faults {
		var faultyKey = fmt.Sprintf("z%02d", fault)
		var ok, faultyWireDependencies = unpackDependencies(faultyKey, wires, []string{})
		if !ok {
			return false, []string{}
		}
		faultyWiresDependencies = append(faultyWiresDependencies, faultyWireDependencies...)
	}
	// Deduplicate faulty wire dependencies
	var dedup = make(map[string]bool)
	var faultyWiresDependenciesDedupped = make([]string, 0, len(faultyWiresDependencies))
	for _, key := range faultyWiresDependencies {
		var _, ok = dedup[key]
		if !ok {
			faultyWiresDependenciesDedupped = append(faultyWiresDependenciesDedupped, key)
			dedup[key] = true
		}
	}
	slices.Sort(faultyWiresDependenciesDedupped)

	// Try swapping wires until fault index increases
	for _, keyA := range faultyWiresDependenciesDedupped {
		var wireA = wires[keyA]
		for _, keyB := range wireKeys {
			var wireB = wires[keyB]

			// Check if keys ok to process
			if keyA == keyB || len(wireA.inputs) == 0 || len(wireB.inputs) == 0 || slices.Contains(swaps, keyA) || slices.Contains(swaps, keyB) {
				continue
			}

			// Try swapping wires
			wires[keyA] = wireB
			wires[keyB] = wireA

			// Verify swapped wires' digits
			var faultsSwapped = verifyAdder(wires)
			if (strict && (len(faultsSwapped) == len(faults)-2 || len(faultsSwapped) == 0)) || (!strict && len(faultsSwapped) < len(faults)) {
				// Continue searching for next swap
				var nextOk, nextSwaps = fixWires(wires, wireKeys, append(swaps, []string{keyA, keyB}...), strict, callback)
				if nextOk {
					return true, nextSwaps
				}
			}

			// Un-swap wires
			wires[keyA] = wireA
			wires[keyB] = wireB

		}
	}

	// Failed finding a fix
	return false, []string{}
}

func verifyAdder(wires map[string]Wire) []int {
	// Verify first 44 digits
	var faults = make([]int, 0, 45)
	for i := 0; i <= 45; i++ {
		if !verifyHalfAdder(i, wires) || !verifyDependencies(i, wires) {
			faults = append(faults, i)
		}
	}
	return faults
}

func verifyDependencies(i int, wires map[string]Wire) bool {
	// Get dependencies
	var ok, dependencies = unpackDependencies(fmt.Sprintf("z%02d", i), wires, []string{})
	if !ok {
		return false
	}

	// Check if count of dependencies matches (TODO: Check count of each dep to be =1)
	// var xys int = 0
	// for _, d := range dependencies {
	// 	if d[0] == 'x' || d[0] == 'y' {
	// 		xys++
	// 	}
	// }
	// if xys != (i+1)*2 {
	// 	return false
	// }

	// Check if all lower indexed inputs are a dependency
	var limit = i
	if i == 45 {
		limit = 44
	}
	for j := 0; j <= limit; j++ {
		if !slices.Contains(dependencies, fmt.Sprintf("x%02d", j)) || !slices.Contains(dependencies, fmt.Sprintf("y%02d", j)) {
			return false
		}
	}

	// No conditions broken
	return true
}

func verifyHalfAdder(i int, wires map[string]Wire) bool {
	// Check 0+0
	if !verifyDigitExpression(i, wires, false, false, false, false) {
		return false
	}
	// Check 1+0
	if i != 45 && !verifyDigitExpression(i, wires, false, true, false, true) {
		return false
	}
	// Check 0+1
	if i != 45 && !verifyDigitExpression(i, wires, false, false, true, true) {
		return false
	}
	// Check 1+1
	if i != 45 && !verifyDigitExpression(i, wires, false, true, true, false) {
		return false
	}
	// Check 0+0 (+1)
	if i != 0 && !verifyDigitExpression(i, wires, true, false, false, true) {
		return false
	}
	// Check 1+0 (+1)
	if i != 0 && i != 45 && !verifyDigitExpression(i, wires, true, true, false, false) {
		return false
	}
	// Check 0+1 (+1)
	if i != 0 && i != 45 && !verifyDigitExpression(i, wires, true, false, true, false) {
		return false
	}
	// Check 1+1 (+1)
	if i != 0 && i != 45 && !verifyDigitExpression(i, wires, true, true, true, true) {
		return false
	}
	// No conditions broken
	return true
}

func verifyDigitExpression(i int, wires map[string]Wire, overflow bool, inputA bool, inputB bool, expected bool) bool {
	// Reset wires
	wires = resetWires(wires)
	// Set expression
	if i > 0 {
		var keyOX = fmt.Sprintf("x%02d", i-1)
		var wireOX = wires[keyOX]
		wireOX.value = overflow
		wires[keyOX] = wireOX
		var keyOY = fmt.Sprintf("y%02d", i-1)
		var wireOY = wires[keyOY]
		wireOY.value = overflow
		wires[keyOY] = wireOY
	}
	var keyX = fmt.Sprintf("x%02d", i)
	var wireX = wires[keyX]
	wireX.value = inputA
	wires[keyX] = wireX
	var keyY = fmt.Sprintf("y%02d", i)
	var wireY = wires[keyY]
	wireY.value = inputB
	wires[keyY] = wireY
	var keyZ = fmt.Sprintf("z%02d", i)
	var result, err = resolveWire(keyZ, wires, []string{})
	if err != nil {
		return false
	}
	if result != expected {
		return false
	}
	return true
}

func unpackDefinition(key string, wires map[string]Wire) string {
	if len(wires[key].inputs) == 0 {
		if wires[key].value {
			return fmt.Sprintf("%v = 1", key)
		} else {
			return fmt.Sprintf("%v = 0", key)
		}
	} else {
		var argA = fmt.Sprintf("(%v)", unpackDefinition(wires[key].inputs[0], wires))
		var argB = fmt.Sprintf("(%v)", unpackDefinition(wires[key].inputs[1], wires))
		return fmt.Sprintf("%v = %v %v %v", key, argA, wires[key].operator, argB)
	}
}

func unpackDependencies(key string, wires map[string]Wire, parents []string) (bool, []string) {
	// Check for circular dependency
	if slices.Contains(parents, key) {
		return false, []string{}
	}
	// Handle (pre)resolved wires
	if len(wires[key].inputs) == 0 {
		return true, []string{key}
	} else
	// Handle calculated wires
	{
		// Get dependencies
		var dependencies = make([]string, 0, len(wires))
		dependencies = append(dependencies, key)
		var okA, inputDependenciesA = unpackDependencies(wires[key].inputs[0], wires, append(parents, key))
		if !okA {
			return false, []string{}
		}
		dependencies = append(dependencies, inputDependenciesA...)
		var okB, inputDependenciesB = unpackDependencies(wires[key].inputs[1], wires, append(parents, key))
		if !okB {
			return false, []string{}
		}
		dependencies = append(dependencies, inputDependenciesB...)
		// Deduplicate dependencies (TODO: Keep count of each)
		var dedup = make(map[string]bool)
		var dependenciesDedupped = make([]string, 0, len(dependencies))
		for _, key := range dependencies {
			var _, ok = dedup[key]
			if !ok {
				dependenciesDedupped = append(dependenciesDedupped, key)
				dedup[key] = true
			}
		}
		// Sort dependencies
		slices.Sort(dependenciesDedupped)
		// Return dependencies
		return true, dependenciesDedupped
	}
}

func resetWires(wires map[string]Wire) map[string]Wire {
	for i := 0; true; i++ {
		// Reset X input
		var keyX = fmt.Sprintf("x%02d", i)
		var wireX, okX = wires[keyX]
		if okX {
			wireX.value = false
			wires[keyX] = wireX
		}
		// Reset Y input
		var keyY = fmt.Sprintf("y%02d", i)
		var wireY, okY = wires[keyY]
		if okY {
			wireY.value = false
			wires[keyY] = wireY
		}
		// Check if done
		if !okX && !okY {
			break
		}
	}
	return wires
}

func resolveValue(wires map[string]Wire) (int, string, error) {
	// Find keys
	var keys = make([]string, 0, len(wires))
	for k, _ := range wires {
		if rune(k[0]) == 'z' {
			keys = append(keys, k)
		}
	}
	slices.Sort(keys)
	slices.Reverse(keys)
	// Calculate values for keys
	var values = make([]string, 0, len(keys))
	for _, key := range keys {
		var value, err = resolveWire(key, wires, []string{})
		if err != nil {
			return 0, "", err
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
	return int(valueInt), valueBinary, nil
}

func resolveWire(key string, wires map[string]Wire, circular []string) (bool, error) {
	// Get wire
	var wire, _ = wires[key]
	// Check if wire already resolved
	if len(wire.inputs) == 0 {
		return wire.value, nil
	} else
	// If wire not resolved, resolve from inputs
	{
		// Check if circular dependency
		if slices.Contains(circular, key) {
			return false, errors.New("circular dependency detected")
		}
		// Resolve input values
		var inputResultA, errA = resolveWire(wire.inputs[0], wires, append(circular, []string{key}...))
		if errA != nil {
			return false, fmt.Errorf("failed resolving wire '%s': %s", wire.inputs[0], errA.Error())
		}
		var inputResultB, errB = resolveWire(wire.inputs[1], wires, append(circular, []string{key}...))
		if errA != nil {
			return false, fmt.Errorf("failed resolving wire '%s': %s", wire.inputs[1], errB.Error())
		}
		var operator = wire.operator

		// Calculate output
		var resolved bool = false
		var value bool = false
		if operator == "AND" {
			resolved = true
			value = inputResultA && inputResultB
		} else if operator == "OR" {
			resolved = true
			value = inputResultA || inputResultB
		} else if operator == "XOR" {
			resolved = true
			value = !((inputResultA && inputResultB) || (!inputResultA && !inputResultB))
		}

		// If resolved, return resolved value
		if resolved {
			return value, nil
		} else
		// If not resolved, return error
		{
			return false, fmt.Errorf("failed resolving wire '%s'", key)
		}
	}
}
