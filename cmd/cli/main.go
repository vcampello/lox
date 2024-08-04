package main

import (
	"fmt"
	"os"

	"github.com/vcampello/lox/internal/errors"
	"github.com/vcampello/lox/internal/interpreter"
)

func main() {
	args := os.Args[1:]

	i := interpreter.NewInterpreter()
	switch len(args) {
	case 0:
		i.RunPrompt()
		break
	case 1:
		i.RunFile(args[0])
		break
	default:
		fmt.Println("Usage: golox [script]")
		os.Exit(errors.EX_USAGE)
	}
}
