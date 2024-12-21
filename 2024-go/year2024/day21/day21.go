package year2024

import (
	solution "adventofcode/lib"
	"errors"
	"fmt"
	"os"
	"strconv"
	"strings"
)

// Day one definition
type Day21 struct{}

var Day = Day21{}

// Year and day
func (day Day21) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2024,
		Day:  21,
	}
}

// Executions
func (day Day21) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day21/input-test.txt"); return string(b) }(),
					Expect: 126384,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day21/input.txt"); return string(b) }(),
					Expect: 177814,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day21/input.txt"); return string(b) }(),
					Expect: nil, // 177814too low
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day21) Run(index int, tag string, input any, verbose bool) (any, string, error) {
	// Initialize
	var output = ""
	var value, ok = input.(string)
	if !ok {
		return nil, output, errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var codes = strings.Split(strings.Trim(value, " \n"), "\n")

	// Set number of directional terminals
	var directionalCount int
	// Part 1/2
	if index == 1 {
		directionalCount = 2
	} else
	// Part 2/2
	if index == 2 {
		directionalCount = 25
	}

	// Find shortest robot command sequences for each code
	var score int = 0
	for _, code := range codes {
		// Echo starting work on code
		if verbose {
			fmt.Printf("- Processing input code '%v' ...", code)
			output += fmt.Sprintf("- Processing input code '%v' ...", code)
		}

		// Initialize keymaps
		var keymaps = []KeyMap{numericKeymap}
		for i := 0; i < directionalCount; i++ {
			keymaps = append(keymaps, directionalKeymap)
		}
		var sequences, _ = generateChainedControllersCommands([]string{code}, keymaps, 0, make(map[string][]string, 0))

		// Find shortest sequence
		var minSequence string = ""
		for _, sequence := range sequences {
			if len(minSequence) == 0 || len(sequence) < len(minSequence) {
				minSequence = sequence
			}
		}

		// Calculate sequence score
		var sequenceScore, _ = strconv.ParseInt(code[:len(code)-1], 10, 64)
		score += int(sequenceScore) * len(minSequence)

		// Echo completed code
		if verbose {
			fmt.Printf(" found %v solutions, minimal of which has length=%v\n", len(sequences), len(minSequence))
			output += fmt.Sprintf(" found %v solutions, minimal of which has length=%v\n", len(sequences), len(minSequence))
		}
	}

	// Return solution
	return score, output, nil
}

func generateChainedControllersCommands(codes []string, keymaps []KeyMap, depth int, cache map[string][]string) ([]string, error) {
	// Initialize all sequences
	var allSequences = make([]string, 0)
	var shortestSequenceYet int = -1
	// Find all sequences
	for _, code := range codes {
		// Generate caching key
		var cacheKey = generateCacheKey("", fmt.Sprintf("%v#%v", keymaps[0].id, depth), code)
		// Check cache for solutions
		var cachedSequences, ok = cache[cacheKey]
		if ok {
			allSequences = append(allSequences, cachedSequences...)
			continue
		}
		// Generate sequences on 1st keymap
		var sequences, err = generateControllerCommands(code, keymaps[0])
		if err != nil {
			return []string{}, err
		}
		// If final keymap, return sequences as final
		if len(keymaps) == 1 {
			allSequences = append(allSequences, sequences...)
			cache[cacheKey] = sequences
		} else
		// If more keymaps available, forward sequences
		{
			// Extract only shortest sequences
			for _, sequence := range sequences {
				if shortestSequenceYet == -1 || len(sequence) < shortestSequenceYet {
					shortestSequenceYet = len(sequence)
				}
			}
			var shortestSequences = make([]string, 0, len(sequences))
			for _, sequence := range sequences {
				if len(sequence) <= shortestSequenceYet {
					shortestSequences = append(shortestSequences, sequence)
				}
			}
			// Forward sequences
			var sequences, err = generateChainedControllersCommands(shortestSequences, keymaps[1:], depth+1, cache)
			if err != nil {
				return []string{}, err
			}
			allSequences = append(allSequences, sequences...)
			cache[cacheKey] = sequences
		}
	}
	// Return all sequences
	return allSequences, nil
}

func generateControllerCommands(code string, keymap KeyMap) ([]string, error) {
	// Find starting position
	var current, ok = keymap.keys['A']
	if !ok {
		return []string{}, errors.New("failed finding 'A' key")
	}
	// Find all partial paths between requested keys
	var sequences = append(make([]string, 0), "")
	for _, key := range code {
		// Initialize partial sequences
		var partials = make([]string, 0, len(code)*2)

		// Find all partials from key to key
		var target, ok = keymap.keys[key]
		if !ok {
			return []string{}, fmt.Errorf("failed finding '%v' key", key)
		}
		// If NULL path, just hit current key
		if current[0] == target[0] && current[1] == target[1] {
			// Add hitting 'A' to all sequences
			partials = append(partials, "")
		} else
		// If not NULL path, process path options
		{
			// Try different midpoints
			var midpoints [][]int
			if current[0] == target[0] || current[1] == target[1] {
				midpoints = [][]int{{target[0], current[0]}}
			} else {
				midpoints = [][]int{{target[0], current[1]}, {current[0], target[1]}}
			}
			for _, midpoint := range midpoints {
				// Check if path midpoint is not forbidden
				var midpointValid = true
				for _, forbidden := range keymap.forbidden {
					if midpoint[0] == forbidden[0] && midpoint[1] == forbidden[1] {
						midpointValid = false
						break
					}
				}
				// If path is valid, generate commands
				if midpointValid {
					// Initialize partial sequence
					var partial = ""
					// Generate partial sequence (part #1)
					var paths [][][]int
					if current[0] == target[0] || current[1] == target[1] {
						paths = [][][]int{{current, target}}
					} else {
						paths = [][][]int{{current, midpoint}, {midpoint, target}}
					}
					for _, path := range paths {
						// Check if NULL path
						if path[0][0] == path[1][0] && path[0][1] == path[1][1] {
							continue
						}
						// If horizontal move right
						if path[0][0] < path[1][0] {
							partial += strings.Replace(fmt.Sprintf("%*s", path[1][0]-path[0][0], ""), " ", ">", -1)
						} else
						// If horizontal move left
						if path[0][0] > path[1][0] {
							partial += strings.Replace(fmt.Sprintf("%*s", path[0][0]-path[1][0], ""), " ", "<", -1)
						} else
						// If vertical move down
						if path[0][1] < path[1][1] {
							partial += strings.Replace(fmt.Sprintf("%*s", path[1][1]-path[0][1], ""), " ", "v", -1)
						} else
						// If vertical move up
						if path[0][1] > path[1][1] {
							partial += strings.Replace(fmt.Sprintf("%*s", path[0][1]-path[1][1], ""), " ", "^", -1)
						}
					}
					// Store partial sequence
					partials = append(partials, partial)
				}
			}
		}

		// Generate all permutations of existing sequences and partials
		var updatedSequences = make([]string, 0, len(partials)*len(sequences))
		for _, sequence := range sequences {
			for _, partial := range partials {
				updatedSequences = append(updatedSequences, sequence+partial+"A")
			}
		}
		sequences = updatedSequences

		// Set key as current position
		current = target
	}
	// Return all sequences
	return sequences, nil
}

func generateCacheKey(domain string, keymapId string, data string) string {
	return fmt.Sprintf("%v:%v:%v", domain, keymapId, data)
}
