package solution

// Solution interface
type ISolution interface {
	GetInfo() SolutionInfo
	GetExecutions(index int, tag string) []SolutionExecution
	Run(index int, tag string, input any, verbose bool) (any, string, error)
}

// Solution info
type SolutionInfo struct {
	Year int
	Day  int
}

// Solution execution definition
type SolutionExecution struct {
	// Execution index
	Index int
	// Execution tag
	Tag string
	// Execution inputS
	Input any
	// Execution expected result
	Expect any
}
