package main

import (
	// Import built in packages
	"flag"
	"fmt"
	"time"

	// Import solutions
	Days "adventofcode/year2024"
)

// Arguments
var (
	pYear      *int    = flag.Int("year", 0, "Will run only puzzles marked as with specified year")
	pDay       *int    = flag.Int("day", 0, "Will run only puzzles marked as with specified day")
	pIndex     *int    = flag.Int("index", 0, "Will run only puzzles marked with specified index")
	pTag       *string = flag.String("tag", "", "Will run only puzzles marked with specified tag")
	pSingle    *int    = flag.Int("single", -1, "Will run only single puzzle input with specified index")
	pVerbose   *bool   = flag.Bool("verbose", false, "If execution output should be verbose")
	pObfuscate *bool   = flag.Bool("obfuscate", false, "If execution output should be obfuscated")
	pSummary   *bool   = flag.Bool("summary", false, "If execution summary should be output")
)

// Main entry point
func main() {

	// Parse arguments
	flag.Parse()

	// Initialize summary
	var tags = map[string]bool{}
	var successByTag = map[string]int{"*": 0}
	var failByTag = map[string]int{"*": 0}
	var unknownByTag = map[string]int{"*": 0}
	var timeByTag = map[string]int64{"*": 0}
	for _, day := range Days.Days {
		for _, execution := range day.GetExecutions(0, "") {
			tags[execution.Tag] = true
			successByTag[execution.Tag] = 0
			failByTag[execution.Tag] = 0
			unknownByTag[execution.Tag] = 0
			timeByTag[execution.Tag] = 0
		}
	}

	// Process all available solutions
	for _, day := range Days.Days {
		// Check solution's year/day
		var info = day.GetInfo()
		if (*pYear != 0 && info.Year != *pYear) || (*pDay != 0 && info.Day != *pDay) {
			continue
		}
		// Execute solution
		var executions = day.GetExecutions(*pIndex, *pTag)
		for i, execution := range executions {

			// Check if running single
			if *pSingle != -1 && *pSingle != i+1 {
				continue
			}

			// Initialize stopwatch
			var startTime = time.Now()

			// Get execution result
			var result, output, err = day.Run(execution.Index, execution.Tag, execution.Input, *pVerbose)
			var duration = time.Since(startTime)
			if err != nil {
				fmt.Printf("  ERROR %v\n", err.Error())
				return
			}

			// Update summary timing
			timeByTag[execution.Tag] += duration.Microseconds()
			timeByTag["*"] += duration.Microseconds()

			// Output execution result
			fmt.Printf("➡️ Year %v, Day %v, Index %v, Tag \"%v\" (%v/%v):\n", info.Year, info.Day, execution.Index, execution.Tag, i+1, len(executions))
			if *pVerbose && output != "" {
				fmt.Print("\033[34m")
				fmt.Printf("\n%v\n", output)
				fmt.Print("\033[0m")
			}

			// Check and output execution result
			if execution.Expect != nil && execution.Expect == result {
				// Update summary
				successByTag[execution.Tag] += 1
				successByTag["*"] += 1
				// Output result
				var resultOutput string
				if !*pObfuscate {
					resultOutput = fmt.Sprintf("%v == %v", result, execution.Expect)
				} else {
					resultOutput = "###### == ######"
				}
				if len(resultOutput) > 32 {
					resultOutput = resultOutput[:32]
				}
				fmt.Printf("   ✅ %v (In %vμs)\n", resultOutput, duration.Microseconds())
			} else if execution.Expect != nil && execution.Expect != result {
				// Update summary
				failByTag[execution.Tag] += 1
				failByTag["*"] += 1
				// Output result
				var resultOutput string
				if !*pObfuscate {
					resultOutput = fmt.Sprintf("%v != %v", result, execution.Expect)
				} else {
					resultOutput = "###### != ######"
				}
				if len(resultOutput) > 32 {
					resultOutput = resultOutput[:32]
				}
				fmt.Printf("   ❌ %v (In %vμs)\n", resultOutput, duration.Microseconds())
			} else {
				// Update summary
				unknownByTag[execution.Tag] += 1
				unknownByTag["*"] += 1
				// Output result
				var resultOutput string
				if !*pObfuscate {
					resultOutput = fmt.Sprintf("%v", result)
				} else {
					resultOutput = "######"
				}
				if len(resultOutput) > 32 {
					resultOutput = resultOutput[:32]
				}
				fmt.Printf("   ❔ %v (In %vμs)\n", resultOutput, duration.Microseconds())
			}

		}
	}

	// Print out summary
	if *pSummary {
		fmt.Printf("\n")
		fmt.Printf("--- SUMMARY ---\n")
		if successByTag["*"] > 0 {
			fmt.Printf("\n")
			fmt.Printf("- ✅ Successful executions: %v/%v\n", successByTag["*"], (successByTag["*"] + failByTag["*"] + unknownByTag["*"]))
			for tag, _ := range tags {
				fmt.Printf("   - %v: %v/%v\n", tag, successByTag[tag], (successByTag[tag] + failByTag[tag] + unknownByTag[tag]))
			}
		}
		if unknownByTag["*"] > 0 {
			fmt.Printf("\n")
			fmt.Printf("- ❔ Unknown executions: %v/%v\n", unknownByTag["*"], (successByTag["*"] + failByTag["*"] + unknownByTag["*"]))
			for tag, _ := range tags {
				fmt.Printf("   - %v: %v/%v\n", tag, unknownByTag[tag], (successByTag[tag] + failByTag[tag] + unknownByTag[tag]))
			}
		}
		if failByTag["*"] > 0 {
			fmt.Printf("\n")
			fmt.Printf("- ❌ Failed executions: %v/%v\n", failByTag["*"], (successByTag["*"] + failByTag["*"] + unknownByTag["*"]))
			for tag, _ := range tags {
				fmt.Printf("   - %v: %v/%v\n", tag, failByTag[tag], (successByTag[tag] + failByTag[tag] + unknownByTag[tag]))
			}
		}
		fmt.Printf("\n")
		fmt.Printf("- 🕐 Execution time: %vμs\n", timeByTag["*"])
		for tag, _ := range tags {
			fmt.Printf("   - %v: Execution time: %vμs\n", tag, timeByTag[tag])
		}
	}

}
