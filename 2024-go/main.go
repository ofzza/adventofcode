package main

import (
	// Import built in packages
	"flag"
	"fmt"

	// Import custom packages

	// Import solutions
	Days "adventofcode/year2024"
)

// Arguments
var (
	pYear 			*int				= flag.Int(		"year", 				0, 			"Will run only puzzles marked as with specified year")
	pDay   			*int				= flag.Int(		"day", 					0, 			"Will run only puzzles marked as with specified day")
	pIndex 			*int				= flag.Int(		"index", 				0, 			"Will run only puzzles marked with specified index")
	pTag 				*string			= flag.String("tag", 					"", 		"Will run only puzzles marked with specified tag")
	pExpect 		*string			= flag.String("expect", 			"", 		"Expected result of the puzzle")
	pVerbose 		*bool				= flag.Bool(	"verbose", 			false, 	"If execution output should be verbose")
	pObfuscate 	*bool				= flag.Bool(	"obfuscate", 		false, 	"If execution output should be obfuscated")
	pSummary 		*bool				= flag.Bool(	"summary", 			false, 	"If execution summary should be output")
)


// Main entry point
func main () {

	// Process runtime arguments
	{
		// Parse arguments
		flag.Parse()	
		// Output arguments
		fmt.Println("Running with args: ")
		fmt.Println("  - year:        ", *pYear)
		fmt.Println("  - day:         ", *pDay)
		fmt.Println("  - index:       ", *pIndex)
		fmt.Println("  - tag:         ", *pTag)
		fmt.Println("  - expect:      ", *pExpect)
		fmt.Println("  - verbose:     ", *pVerbose)
		fmt.Println("  - obfuscate:   ", *pObfuscate)
		fmt.Println("  - summary:     ", *pSummary)
    fmt.Println()
	} 
  
	// Initialize summary
	// TODO: ...

  // Ready parameter input


	// Process all available solutions
  for _, day := range Days.Days {
    // Check solution's year/day
    var info = day.GetInfo()
    if (*pYear != 0 && info.Year != *pYear) || (*pDay != 0 && info.Day != *pDay) { continue }
    // Execute solution
    for _, execution := range day.GetExecutions(*pIndex, *pTag) {


      // Get execution result
      var result, err = day.Run(execution.Index, execution.Input);
      if err != nil {
        fmt.Printf("> %v/%v Part #%v (%v): ERROR %v\n", info.Year, info.Day, execution.Index, execution.Tag, err.Error())
        return
      }

      // Check and output execution result
      if result == execution.Expect {
        fmt.Printf("> %v/%v Part #%v (%v): %v (CORRECT)\n", info.Year, info.Day, execution.Index, execution.Tag, result)
      } else {
        fmt.Printf("> %v/%v Part #%v (%v): %v (WRONG!!!)\n", info.Year, info.Day, execution.Index, execution.Tag, result)
      }

    }
  }

	// Print out summary
	// TODO: ...

}
