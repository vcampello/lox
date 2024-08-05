package interpreter

import (
	"bufio"
	"fmt"
	"os"
	"strings"

	"github.com/vcampello/lox/internal/errors"
	"github.com/vcampello/lox/internal/scanner"
)

func NewInterpreter() *Interpreter {
	return &Interpreter{hadError: false}
}

type Interpreter struct {
	hadError bool
}

func (i *Interpreter) RunPrompt() {
	prompt := "> "
	reader := bufio.NewReader(os.Stdin)

	for {
		fmt.Print(prompt)

		if line, err := reader.ReadString('\n'); err != nil {
			// Exit
			return
		} else if line != "\n" {
			i.Run(line)
		}

		i.hadError = false
	}
}
func (i Interpreter) RunFile(path string) {
	bytes, err := os.ReadFile(path)

	if err != nil {
		fmt.Println("Error opening script", path)
		os.Exit(errors.EX_IOERR)
	}
	source := string(bytes)
	i.Run(source)

	// Terminate
	if i.hadError {
		os.Exit(errors.EX_DATAERR)
	}
}

func (i Interpreter) Run(source string) error {
	source = strings.TrimSpace(source)
	scanner := scanner.NewScanner()
	for tok := range scanner.ScanTokens(source) {
		fmt.Println("token: ", tok)
	}
	return nil
}
func (i Interpreter) error(line int, message string) {
	i.report(line, "", message)
}

func (i *Interpreter) report(line int, where string, message string) {
	fmt.Printf("[line %d] Error %s: %s", line, where, message)
	i.hadError = true
}
