package scanner

import "fmt"

type TokenType int8

const (
	UNKNOWN TokenType = iota
	LEFT_PAREN
	RIGHT_PAREN
	LEFT_BRACE
	RIGHT_BRACE
	COMMA
	DOT
	MINUS
	PLUS
	SEMICOLON
	SLASH
	STAR
	BANG
	BANG_EQUAL
	EQUAL
	EQUAL_EQUAL
	GREATER
	GREATER_EQUAL
	LESS
	LESS_EQUAL
	IDENTIFIER
	STRING
	NUMBER
	AND
	CLASS
	ELSE
	FALSE
	FUN
	FOR
	IF
	NIL
	OR
	PRINT
	RETURN
	SUPER
	THIS
	TRUE
	VAR
	WHILE
	EOF
)

func NewToken(Literal string, Line int, Lexeme string, Type TokenType) *Token {
	return &Token{Literal: Literal, Line: Line, Lexeme: Lexeme, Type: Type}
}

type Token struct {
	Literal string
	Line    int
	Lexeme  string
	Type    TokenType
}

func (t Token) String() string {
	// return type + " " + lexeme + " " + literal;
	return fmt.Sprintf("%d %s %s", t.Type, t.Lexeme, t.Literal)
}
