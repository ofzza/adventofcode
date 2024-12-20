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
type Day13 struct{}

var Day = Day13{}

// Year and day
func (day Day13) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2024,
		Day:  13,
	}
}

// Executions
func (day Day13) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day13/input-test.txt"); return string(b) }(),
					Expect: 480,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day13/input.txt"); return string(b) }(),
					Expect: 25629,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day13/input.txt"); return string(b) }(),
					Expect: 107487112929999,
				},
			)
		}
	}
	return executions
}

type Machine struct {
	a     []int
	b     []int
	prize []int
}

// Implementation
func (day Day13) Run(index int, tag string, input any, verbose bool) (any, string, error) {
	// Initialize
	var output = ""
	var value, ok = input.(string)
	if !ok {
		return nil, output, errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var machinesStr = strings.Split(strings.Trim(value, " \n"), "\n\n")
	var machines = make([]Machine, 0, len(machinesStr))
	for _, machineStr := range machinesStr {
		// Parse machine info
		var lineStrs = strings.Split(strings.Trim(machineStr, " \n"), "\n")
		var aValueStrs = strings.Split(strings.Trim(strings.Split(strings.Trim(lineStrs[0], " \n"), ":")[1], " \n"), ", ")
		var aX, _ = strconv.Atoi(strings.Trim(aValueStrs[0], " X+"))
		var aY, _ = strconv.Atoi(strings.Trim(aValueStrs[1], " Y+"))
		var bValueStrs = strings.Split(strings.Trim(strings.Split(strings.Trim(lineStrs[1], " \n"), ":")[1], " \n"), ", ")
		var bX, _ = strconv.Atoi(strings.Trim(bValueStrs[0], " X+"))
		var bY, _ = strconv.Atoi(strings.Trim(bValueStrs[1], " Y+"))
		var prizeValueStrs = strings.Split(strings.Trim(strings.Split(strings.Trim(lineStrs[2], " \n"), ":")[1], " \n"), ", ")
		var prizeX, _ = strconv.Atoi(strings.Trim(prizeValueStrs[0], " X="))
		var prizeY, _ = strconv.Atoi(strings.Trim(prizeValueStrs[1], " Y="))
		// Initialize a machine
		var machine = Machine{
			a:     []int{aX, aY},
			b:     []int{bX, bY},
			prize: []int{prizeX, prizeY},
		}
		machines = append(machines, machine)
	}

	// Part 1/2
	if index == 1 {

		// Calculate presses
		var cost int = 0
		for i, machine := range machines {
			// Calculate presses
			var solutions = calculatePresses(machine)
			if len(solutions) == 0 {
				if verbose {
					output += fmt.Sprintf("- Machine %v: Can't be solved!\n", i+1)
				}
				continue
			}
			// Find lowest price
			if verbose {
				output += fmt.Sprintf("- Machine %v: A=%v, B=%v, Prize=%v\n", i+1, machine.a, machine.b, machine.prize)
			}
			var lowest int = -1
			for _, solution := range solutions {
				var price = 3*solution[0] + solution[1]
				if lowest == -1 || price < lowest {
					lowest = price
				}
				if verbose {
					output += fmt.Sprintf("  - Ax%v + Bx%v = %v\n", solution[0], solution[1], price)
				}
			}
			// Sum up price
			cost += lowest
			if verbose {
				output += fmt.Sprintf("  - Solution price = %v\n", lowest)
			}
		}

		// Return solution
		return cost, output, nil
	} else

	// Part 2/2
	if index == 2 {

		// Update prize coordinates
		for _, m := range machines {
			m.prize[0] += 10000000000000
			m.prize[1] += 10000000000000
		}

		// Calculate presses
		var cost int = 0
		for i, machine := range machines {
			// Calculate presses
			var solutions = calculatePresses(machine)
			if len(solutions) == 0 {
				if verbose {
					output += fmt.Sprintf("- Machine %v: Can't be solved!\n", i+1)
				}
				continue
			}
			// Find lowest price
			if verbose {
				output += fmt.Sprintf("- Machine %v: A=%v, B=%v, Prize=%v\n", i+1, machine.a, machine.b, machine.prize)
			}
			var lowest int = -1
			for _, solution := range solutions {
				var price = 3*solution[0] + solution[1]
				if lowest == -1 || price < lowest {
					lowest = price
				}
				if verbose {
					output += fmt.Sprintf("  - Ax%v + Bx%v = %v\n", solution[0], solution[1], price)
				}
			}
			// Sum up price
			cost += lowest
			if verbose {
				output += fmt.Sprintf("  - Solution price = %v\n", lowest)
			}
		}

		// Return solution
		return cost, output, nil
	}

	// Missing implementation
	return nil, output, errors.New("missing implementation for required index")
}

func testPasses(m Machine) [][]int {
	// Initialize
	var solutions = make([][]int, 0)

	// Try different values
	for i := 0; i <= 100; i++ {
		var a = i
		var b1 = (float64(m.prize[0]) - float64(a)*float64(m.a[0])) / float64(m.b[0])
		var b2 = (float64(m.prize[1]) - float64(a)*float64(m.a[1])) / float64(m.b[1])
		if b1 == b2 && b1 == math.Floor(b1) {
			solutions = append(solutions, []int{int(a), int(b1)})
		}
	}

	// Return solutions
	return solutions
}

func calculatePresses(m Machine) [][]int {
	// Initialize
	var solutions = make([][]int, 0)

	// Calculate as y = kx + b
	var k1 = float64(-1) * float64(m.a[0]) / float64(m.b[0])
	var l1 = float64(m.prize[0]) / float64(m.b[0])
	var k2 = float64(-1) * float64(m.a[1]) / float64(m.b[1])
	var l2 = float64(m.prize[1]) / float64(m.b[1])

	// Check equations
	if k1 == k2 && l1 != l2 {
		return [][]int{}
	}
	if k1 == k2 && l1 == l2 {
		// This would allow for multiple solutions, but it never happens!
		fmt.Println("WARNING: Identical equations - this would allow for solutions based on only a single button!!!")
		return [][]int{}
	}
	if m.b[0] == 0 || m.b[1] == 0 {
		// Static value equation - this would allow for solutions based on only a single button, but this never happens!
		fmt.Println("WARNING: Static value equation - this would allow for solutions based on only a single button!!!")
		return [][]int{}
	}

	// Calculate A and B
	var a = (l2 - l1) / (k1 - k2)
	var b1 = k1*a + l1
	var b2 = k2*a + l2

	// Fix float rounding issues
	var precision float64 = 1e2
	a = math.Round(precision*a) / precision
	b1 = math.Round(precision*b1) / precision
	b2 = math.Round(precision*b2) / precision

	// Check if both equations return same B with calculated A
	if b1 != b2 {
		fmt.Println("WARNING: Both equations should return same B with calculated A!!!")
		fmt.Printf("  B = %v A + %v\n", a, b1)
		fmt.Printf("  B = %v A + %v\n", a, b2)
		return [][]int{}
	}

	if a == math.Floor(a) && b1 == math.Floor(b1) {
		solutions = append(solutions, []int{int(a), int(b1)})
	}

	// Return solutions
	return solutions
}
