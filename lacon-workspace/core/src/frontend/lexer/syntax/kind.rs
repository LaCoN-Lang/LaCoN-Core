#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxKind {
	LeftParenthesis,  // (
	RightParenthesis, // )
	LeftBracket,      // [
	RightBracket,     // ]
	LeftBrace,        // {
	RightBrace,       // }

	SingleQuote, // '
	DoubleQuote, // "
	GraveAccent, // `

	Semicolon, // ;
	Comma,     // ,
}
