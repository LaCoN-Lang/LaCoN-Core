#[derive(Debug, Clone)]
pub enum ErrorType {
	Unknown,
	Lexical(LexicalError),
	Syntax,
	Semantic,
	Runtime,
	Expected,
}

#[derive(Debug, Clone)]
pub enum LexicalError {
	InvalidCharacter(char),
	UnterminatedString,
	UnterminatedBlockComment,
	InvalidIndent,
}
