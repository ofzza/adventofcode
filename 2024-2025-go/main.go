package main

import (
	// Import built in packages
	"flag"
	"fmt"
	"math"
	"strings"
	"time"

	// Import solutions
	solution "adventofcode/lib"
	Days2024 "adventofcode/year2024"
	Days2025 "adventofcode/year2025"
)

// Arguments
var (
	pYear      *int    = flag.Int("year", 0, "Will run only puzzles marked as with specified year")
	pDay       *int    = flag.Int("day", 0, "Will run only puzzles marked as with specified day")
	pIndex     *int    = flag.Int("index", 0, "Will run only puzzles marked with specified index")
	pTag       *string = flag.String("tag", "", "Will run only puzzles marked with specified tag")
	pSingle    *int    = flag.Int("single", -1, "Will run only single puzzle input with specified index")
	pVerbose   *bool   = flag.Bool("verbose", false, "If execution output should be verbose")
	pDebugging *bool   = flag.Bool("debugging", false, "If execution output should be immediate instead of post execution, making execution potentially slower, but more appropriate for debugging purposes")
	pObfuscate *bool   = flag.Bool("obfuscate", false, "If execution output should be obfuscated")
	pSummary   *bool   = flag.Bool("summary", false, "If execution summary should be output")
	pGraph   	 *bool   = flag.Bool("graph", false, "If execution summary graph should be output")
)

// Main entry point
func main() {

	// Parse arguments
	flag.Parse()

	// Aggregate years
	var Days = []solution.ISolution{}
	Days = append(Days, Days2024.Days...)
	Days = append(Days, Days2025.Days...)

	// Initialize summary
	var tags = map[string]bool{}
	var successByTag = map[string]int{"*": 0}
	var failByTag = map[string]int{"*": 0}
	var unknownByTag = map[string]int{"*": 0}
	var timeByExecution = []struct{label string; time int64}{}
	var timeByTag = map[string]int64{"*": 0}
	for _, day := range Days {
		for _, execution := range day.GetExecutions(0, "") {
			tags[execution.Tag] = true
			successByTag[execution.Tag] = 0
			failByTag[execution.Tag] = 0
			unknownByTag[execution.Tag] = 0
			timeByTag[execution.Tag] = 0
		}
	}

	// Process all available solutions
	for _, day := range Days {
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

			// Initialize logger
			var log = solution.Logger{
				Verbsose: *pVerbose,
				Debugging: *pDebugging,
			}

			// Initialize stopwatch
			var startTime = time.Now()

			// Get execution result
			var result, output, err = day.Run(execution.Index, execution.Tag, execution.Input, *pVerbose, log)
			var duration = time.Since(startTime)
			if err != nil {
				fmt.Printf("  ERROR %v\n", err.Error())
				return
			}

			// Update summary timing
			var label = fmt.Sprintf("%04d-%02d.%02d %s", info.Year, info.Day, execution.Index, execution.Tag)
			var time = duration.Microseconds()
			timeByExecution = append(timeByExecution, struct{label string; time int64}{ label, time })
			timeByTag[execution.Tag] += duration.Microseconds()
			timeByTag["*"] += duration.Microseconds()

			// Output execution result
			fmt.Printf("➡️  Year %v, Day %v, Index %v, Tag \"%v\" (%v/%v):\n", info.Year, info.Day, execution.Index, execution.Tag, i+1, len(executions))
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
				if len(resultOutput) > 36 {
					resultOutput = resultOutput[:36] + " ..."
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
				if len(resultOutput) > 36 {
					resultOutput = resultOutput[:32] + " ..."
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
				if len(resultOutput) > 36 {
					resultOutput = resultOutput[:36] + " ..."
				}
				fmt.Printf("   ❔ %v (In %vμs)\n", resultOutput, duration.Microseconds())
			}

		}
	}

	// Print out summary grapg
	if *pGraph {
		// Calculate proportions
		var maxExecutionTime float64 = 0;
		for _, exec := range timeByExecution {
			if math.Log2(float64(exec.time)) > maxExecutionTime {
				maxExecutionTime = math.Log2(float64(exec.time));
			}
		}
		var terminalWidth = 64
		var labelWidth = 20
		var timePerSection float64 = maxExecutionTime / float64(terminalWidth - labelWidth)

		// Output execution time graph
		fmt.Printf("\n")
		fmt.Printf("--- EXECUTION TIME (Log2) ---\n")
		fmt.Printf("\n")
		for _, exec := range timeByExecution {
			var paddedLabelFmt = fmt.Sprintf("%s-%ds", "%", labelWidth)
			var paddedLabel = fmt.Sprintf(paddedLabelFmt, exec.label)
			var timeLineFmt = fmt.Sprintf(fmt.Sprintf("%s%ds", "%", int(math.Log2(float64(exec.time)) / timePerSection)), "");
			var timeLine = strings.Replace(timeLineFmt, " ", "#", -1)
			fmt.Printf("- %s [🕐 %9d μs]: %s\n",paddedLabel,  exec.time, timeLine)
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
