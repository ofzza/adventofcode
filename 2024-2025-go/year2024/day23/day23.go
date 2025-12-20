package year2024

import (
	solution "adventofcode/lib"
	"errors"
	"fmt"
	"os"
	"slices"
	"strings"
)

// Day one definition
type Day23 struct{}

var Day = Day23{}

// Year and day
func (day Day23) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2024,
		Day:  23,
	}
}

// Executions
func (day Day23) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day23/input-test.txt"); return string(b) }(),
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day23/input.txt"); return string(b) }(),
					Expect: 1348,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day23/input-test.txt"); return string(b) }(),
					Expect: "co,de,ka,ta",
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day23/input.txt"); return string(b) }(),
					Expect: "am,bv,ea,gh,is,iy,ml,nj,nl,no,om,tj,yv",
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day23) Run(index int, tag string, input any, verbose bool, log solution.Logger) (any, string, error) {
	// Initialize
	var value, ok = input.(string)
	if !ok {
		return nil, log.Dump(), errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var lines = strings.Split(strings.Trim(value, " \n"), "\n")
	var connections = make([][]string, 0, len(lines))
	for _, line := range lines {
		var nodesStrs = strings.Split(strings.Trim(line, " \n"), "-")
		connections = append(connections, []string{nodesStrs[0], nodesStrs[1]})
	}

	// Part 1/2
	if index == 1 {

		// Generate interconnected groups of size 3
		var groups = findInterconnectedGroupsBySize(connections, 3, nil, nil, nil)

		// Filter groups with a node starting with "t"
		var filtered = make([][]string, 0, len(groups))
		for _, group := range groups {
			for _, node := range group {
				if node[0] == 't' {
					filtered = append(filtered, group)
					break
				}
			}
		}

		// Echo groups
		if verbose {
			for _, group := range filtered {
				log.Log(fmt.Sprintf("- Group identified: %v\n", group))
			}
		}

		// Return solution
		return len(filtered), log.Dump(), nil
	} else

	// Part 2/2
	if index == 2 {

		// Try generating group of max size
		var groups [][]string
		findInterconnectedGroupsBySize(connections, len(connections), func(size int, groupsOfSize [][]string) {
			if len(groupsOfSize) > 0 {
				groups = groupsOfSize
			}
		}, nil, nil)

		// Check if found single group
		if len(groups) != 1 {
			return nil, log.Dump(), errors.New("couldn't find a single, large, interconnected group")
		}

		// Echo groups
		if verbose {
			log.Log(fmt.Sprintf("- Group identified: %v\n", strings.Join(groups[0], ",")))
		}

		// Return solution
		return strings.Join(groups[0], ","), log.Dump(), nil
	}

	// Missing implementation
	return nil, log.Dump(), errors.New("missing implementation for required index")
}

func findInterconnectedGroupsBySize(connections [][]string, size int, callback func(size int, groups [][]string), nodesMap map[string]bool, connectionsMap map[string]bool) [][]string {
	if nodesMap == nil || connectionsMap == nil {
		// Generate helper maps connection map
		nodesMap = make(map[string]bool, 0)
		connectionsMap = make(map[string]bool, 0)
		for _, connection := range connections {
			// Write nodes to nodes map
			nodesMap[connection[0]] = true
			nodesMap[connection[1]] = true
			// Write connection to the connections map
			connectionsMap[generateConnectionKey(connection[0], connection[1])] = true
			connectionsMap[generateConnectionKey(connection[1], connection[0])] = true
		}
	}

	// If group of size 1, just return all nodes
	if size == 1 {
		var groups = make([][]string, 0, len(nodesMap))
		for node, _ := range nodesMap {
			groups = append(groups, []string{node})
		}
		var deduplicated = deduplicateGroups(groups)
		if callback != nil {
			callback(size, deduplicated)
		}
		return deduplicated
	} else
	// Generate and expand groups of smaller size
	{
		// Generate candidateGroups of smaller size
		var candidates = findInterconnectedGroupsBySize(connections, size-1, callback, nodesMap, connectionsMap)
		// Try expanding each group with each node
		var groups = make([][]string, 0, len(candidates)*len(nodesMap))
		for _, candidate := range candidates {
			for node, _ := range nodesMap {
				// Check if node already added
				if slices.Contains(candidate, node) {
					continue
				}
				// Check if all members of the candidate group are connected to the node
				var connected = true
				for _, candidateNode := range candidate {
					var _, ok = connectionsMap[generateConnectionKey(candidateNode, node)]
					if !ok {
						connected = false
						break
					}
				}
				// If connected, expand candidate group
				if connected {
					var group = append(append(make([]string, 0, len(candidate)+1), candidate...), node)
					groups = append(groups, group)
				}
			}
		}
		// Return expanded groups
		var deduplicated = deduplicateGroups(groups)
		if callback != nil {
			callback(size, deduplicated)
		}
		return deduplicated
	}
}

func deduplicateGroups(groups [][]string) [][]string {
	// Initialize
	var deduplicated = make(map[string][]string, 0)
	// Deduplicate
	for _, group := range groups {
		// Sort group
		var sorted = append(make([]string, 0, len(group)), group...)
		slices.Sort(sorted)
		// Convert to a unique key
		var key = strings.Join(sorted, "-")
		// Store by unique key
		deduplicated[key] = sorted
	}
	// Extract deduplicated groups
	var result = make([][]string, 0, len(deduplicated))
	for _, group := range deduplicated {
		result = append(result, group)
	}
	// Return deduplicated groups
	return result
}

func generateConnectionKey(a string, b string) string {
	return fmt.Sprintf("%s-%s", a, b)
}
