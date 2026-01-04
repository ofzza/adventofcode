package solution

import "fmt"

type Logger struct {
	Verbsose bool
	Debugging bool
	text string
}

func (log *Logger) Log(text string) {
	// If not verbose, ignore
	if !log.Verbsose { return }
	// If debugging, log directly
	if (log.Debugging) {
		fmt.Print(text)
	} else
	// Else store for later logging
	{
		log.text += text
	}
}

func (log *Logger) Dump() string {
	// If not verbose, ignore
	if !log.Verbsose { return "" }
	// If debugging, log line-breaks and dump no stored text
	if (log.Debugging) {
		fmt.Print("\n\n")
		return ""
	} else
	// Else return stored text
	{
		return log.text
	}
}
