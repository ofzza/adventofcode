package year2025

import (
	solution "adventofcode/lib"
	"errors"
	"fmt"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
)

// Day one definition
type Day08 struct{}

var Day = Day08{}

// Year and day
func (day Day08) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2025,
		Day:  8,
	}
}

// Executions
func (day Day08) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  Input{Iterations: 10, Input: func() string { var b, _ = os.ReadFile("./year2025/data/day08/input-test.txt"); return string(b) }()},
					Expect: 40,
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
					Input:  Input{Iterations: 1000, Input: func() string { var b, _ = os.ReadFile("./year2025/data/day08/input.txt"); return string(b) }()},
					Expect: 105952,
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
					Input:  Input{Iterations: 0, Input: func() string { var b, _ = os.ReadFile("./year2025/data/day08/input-test.txt"); return string(b) }()},
					Expect: 25272,
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
					Input:  Input{Iterations: 0, Input: func() string { var b, _ = os.ReadFile("./year2025/data/day08/input.txt"); return string(b) }()},
					Expect: 975931446,
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day08) Run(index int, tag string, input any, verbose bool, log solution.Logger) (any, string, error) {
	// Initialize
	var value, ok = input.(Input)
	if !ok {
		return nil, log.Dump(), errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var coordsStr = strings.Split(strings.Trim(value.Input, "\r\n "), "\n")
	var junctions = make([]Junction, len(coordsStr))
	var circuitMap = make(map[int]Circuit)
	for i, coordStr := range coordsStr {
		var xyzStr = strings.Split(strings.Trim(coordStr, "\r\n "), ",")
		x, _ := strconv.Atoi(strings.Trim(xyzStr[0], "\r\n "))
		y, _ := strconv.Atoi(strings.Trim(xyzStr[1], "\r\n "))
		z, _ := strconv.Atoi(strings.Trim(xyzStr[2], "\r\n "))
		var circuit = Circuit{index: i, junctions: []int{i}}
		circuitMap[i] = circuit
		var junction = Junction{index: i, x: x, y: y, z: z, circuit: i, distances: make(map[int]Distance)}
		junctions[i] = junction
	}

	// Calculate all distances
	var distances = make([]Distance, int(float64(len(junctions)*len(junctions)-len(junctions))/2))
	var next = 0
	for i := 0; i < len(junctions)-1; i++ {
		var a = junctions[i]
		for j := i + 1; j < len(junctions); j++ {
			var b = junctions[j]
			if j > i {
				var distance = Distance{
					a:        i,
					b:        j,
					distance: math.Sqrt(float64(a.x-b.x)*float64(a.x-b.x) + float64(a.y-b.y)*float64(a.y-b.y) + float64(a.z-b.z)*float64(a.z-b.z)),
				}
				junctions[i].distances[j] = distance
				junctions[j].distances[i] = distance
				distances[next] = distance
				next++
			}
		}
	}

	// Sort distances
	sort.Slice(distances, func(i int, j int) bool {
		return distances[i].distance < distances[j].distance
	})

	// Define how circuits are joined
	var joinJunctions = func(a int, b int) {
		// Get junctions to join
		var junctionA = junctions[a]
		var junctionB = junctions[b]

		// Echo joining junctions
		log.Log(fmt.Sprintf("> Joining junctions %d [%d, %d, %d] and %d [%d, %d, %d]\n", junctionA.index, junctionA.x, junctionA.y, junctionA.z, junctionB.index, junctionB.x, junctionB.y, junctionB.z))

		// Join junctions/circuits
		for _, index := range circuitMap[junctionB.circuit].junctions {
			junctions[index].circuit = junctionA.circuit
		}
		var circuit = circuitMap[junctionA.circuit]
		circuit.junctions = append(circuit.junctions, circuitMap[junctionB.circuit].junctions...)
		circuitMap[junctionA.circuit] = circuit
		delete(circuitMap, junctionB.circuit)
	}

	// Part 1/2
	if index == 1 {

		// Join closest N junctions
		for i := 0; i < value.Iterations; i++ {
			// Check if already on same circuit
			if junctions[distances[i].a].circuit == junctions[distances[i].b].circuit {
				continue
			}

			// Join junctions/circuits
			joinJunctions(distances[i].a, distances[i].b)
		}

		// Find 3 largest circuits
		var circuits = make([]Circuit, len(circuitMap))
		var next = 0
		for _, circuit := range circuitMap {
			circuits[next] = circuit
			next++
		}
		sort.Slice(circuits, func(i int, j int) bool {
			return len(circuits[i].junctions) > len(circuits[j].junctions)
		})

		// Return solution
		return len(circuits[0].junctions) * len(circuits[1].junctions) * len(circuits[2].junctions), log.Dump(), nil
	} else

	// Part 2/2
	if index == 2 {

		// Join closest N junctions
		for i := 0; i < len(distances); i++ {
			// Check if already on same circuit
			if junctions[distances[i].a].circuit == junctions[distances[i].b].circuit {
				continue
			}

			// Join junctions/circuits
			joinJunctions(distances[i].a, distances[i].b)

			// Check if single circuit
			if len(circuitMap) == 1 {
				// Return solution
				return junctions[distances[i].a].x * junctions[distances[i].b].x, log.Dump(), nil
			}
		}
	}

	// Missing implementation
	return nil, log.Dump(), errors.New("missing implementation for required index")
}

type Input struct {
	Iterations int
	Input      string
}

type Junction struct {
	index     int
	x         int
	y         int
	z         int
	circuit   int
	distances map[int]Distance
}

type Circuit struct {
	index     int
	junctions []int
}

type Distance struct {
	a        int
	b        int
	distance float64
}
