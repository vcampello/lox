package scanner

func NewToken() *Token {
	return &Token{}
}

type Token struct {
	literal string
}
