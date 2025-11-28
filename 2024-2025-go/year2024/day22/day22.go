package year2024

import (
	solution "adventofcode/lib"
	"errors"
	"os"
	"strconv"
	"strings"
)

// Day one definition
type Day22 struct{}

var Day = Day22{}

// Year and day
func (day Day22) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2024,
		Day:  22,
	}
}

// Executions
func (day Day22) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day22/input-test-01.txt"); return string(b) }(),
					Expect: 37327623,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day22/input.txt"); return string(b) }(),
					Expect: 12979353889,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day22/input-test-02.txt"); return string(b) }(),
					Expect: 23,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day22/input.txt"); return string(b) }(),
					Expect: 1449,
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day22) Run(index int, tag string, input any, verbose bool) (any, string, error) {
	// Initialize
	var output = ""
	var value, ok = input.(string)
	if !ok {
		return nil, output, errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var lines = strings.Split(strings.Trim(value, " \n"), "\n")
	var seeds = make([]int, 0, len(lines))
	for _, line := range lines {
		var seed, _ = strconv.ParseInt(strings.Trim(line, " \n"), 0, 64)
		seeds = append(seeds, int(seed))
	}

	// Part 1/2
	if index == 1 {

		// Calculate sum of 2000th pseudorandom numbers of all buyers
		var sum int = 0
		for _, seed := range seeds {
			// Initialize pseudorandom geenrator
			var pseudo = PseudoRandomGenerator{seed: seed, value: seed}
			// Calculate 2000th
			for i := 0; i < 2000; i++ {
				pseudo.next()
			}
			// Sum up 2000th value
			sum += pseudo.value
		}

		// Return solution
		return sum, output, nil
	} else

	// Part 2/2
	if index == 2 {

		// Generate prices for each buyer
		var allSequencePricesSums = make(map[int]int, 0)
		for _, seed := range seeds {
			var prices = predictMarket(seed, 2000)
			// Analyze prices and store most profitable first sequences of 4 diffs
			var buyerSequencePrices = make(map[int]int, 0)
			for i := 4; i < len(prices); i++ {
				// Calculate unique key for the sequence
				var key = generateUniqueIntSequenceKey(prices[i-3].diff, prices[i-2].diff, prices[i-1].diff, prices[i].diff)
				var price = prices[i].price
				// Store as best price for the sequence (if new sequence only)
				var _, ok = buyerSequencePrices[key]
				if !ok {
					buyerSequencePrices[key] = price
				}
			}
			// Add all sequence prices to all other buyers
			for key, price := range buyerSequencePrices {
				allSequencePricesSums[key] += price
			}
		}

		// Find best price
		var max int = -1
		for _, price := range allSequencePricesSums {
			if price > max {
				max = price
			}
		}

		// Return solution
		return max, output, nil
	}

	// Missing implementation
	return nil, output, errors.New("missing implementation for required index")
}

type Step struct {
	value int
	price int
	diff  int
}

func predictMarket(seed int, n int) []Step {
	// Initialize
	var pseudo = PseudoRandomGenerator{seed: seed, value: seed}
	var steps []Step = append(make([]Step, 0, n+1), Step{value: seed})
	//  Calculate first N pseudorandom numbers
	for i := 0; i < n; i++ {
		var step = Step{
			value: pseudo.next(),
		}
		step.price = step.value % 10
		if i > 0 {
			step.diff = step.price - steps[len(steps)-1].price
		} else {
			step.diff = step.price - (seed % 10)
		}
		steps = append(steps, step)
	}
	// Return next pseudorandom value
	return steps
}
func generateUniqueIntSequenceKey(a int, b int, c int, d int) int {
	return 1e9*a + 1e6*b + 1e3*c + 1e0*d
}

type PseudoRandomGenerator struct {
	seed  int
	value int
}

func (pseudo *PseudoRandomGenerator) next() int {
	// Calculate next pseudorandom number
	var next = pseudo.value
	// ... multiplying the secret number by 64, mix, prune
	next = ((next << 6) ^ next) & 0b00000000111111111111111111111111 // next = ((next * 64) ^ next) % 16777216
	// ... dividing the secret number by 32, min, prune
	next = ((next >> 5) ^ next) & 0b00000000111111111111111111111111 // next = ((next / 32) ^ next) % 16777216
	// ... multiplying the secret number by 2048, mix, prune
	next = ((next << 11) ^ next) & 0b00000000111111111111111111111111 // next = ((next * 2048) ^ next) % 16777216
	pseudo.value = next
	// Return next pseudorandom value
	return pseudo.value
}
