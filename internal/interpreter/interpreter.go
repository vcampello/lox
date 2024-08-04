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
			return
		} else if line != "\n" {
			i.Run(line)
		}
	}
}
func (i *Interpreter) RunFile(path string) {
	bytes, err := os.ReadFile(path)

	if err != nil {
		fmt.Println("Error opening script", path)
		os.Exit(errors.EX_IOERR)
	}
	source := string(bytes)
	i.Run(source)

}
func (*Interpreter) Run(source string) error {
	source = strings.TrimSpace(source)
	scanner := scanner.NewScanner()
	for tok := range scanner.ScanTokens(source) {
		fmt.Println("token: ", tok)
	}
	return nil
}
