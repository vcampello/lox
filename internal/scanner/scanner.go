package scanner

import "fmt"

func NewScanner() *Scanner {
	return &Scanner{}
}

type Scanner struct {
}

func (s Scanner) ScanTokens(source string) []Token {
	fmt.Println(source)
	tokens := make([]Token, 0)
	return tokens

}
