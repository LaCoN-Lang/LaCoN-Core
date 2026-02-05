use super::super::{KeywordKind, OperatorKind, SyntaxKind};
use crate::shared::unit::UnitKind;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
	// ─────────────────────────────────────────────
	// Служебные
	// ─────────────────────────────────────────────
	Illegal,
	Invalid, // \\ InvalidToken
	Error,   // \\ LexicalError
	Unknown, // \\ UnknownToken
	SOF,     // \\ StartOfFile
	EOF,     // \\ EndOfFile

	// ─────────────────────────────────────────────
	// Layout / whitespace-sensitive синтаксис
	// ─────────────────────────────────────────────
	Newline,        // \n \\ LineBreak
	CarriageReturn, // \r \\ CarriageReturn
	Indent,         // →  \\ IndentIncrease
	Dedent,         // ←  \\ IndentDecrease
	SectionMaker,   // §  \\ Scope

	FloorStart, // ⌊  \\ FloorStart
	FloorEnd,   // ⌋  \\ FloorEnd
	CeilStart,  // ⌈  \\ CeilStart
	CeilEnd,    // ⌉  \\ CeilEnd

	Underscore, // _  \\ Wildcard

	//
	LeftArrow,
	RightArrow,
	UpArrow,
	DownArrow,
	LeftDoubleArrow,
	RightDoubleArrow,
	UpDoubleArrow,
	DownDoubleArrow,

	// ─────────────────────────────────────────────
	// Литералы и идентификаторы
	// ─────────────────────────────────────────────
	Identifier,             // name \\ Identifier
	Keyword(KeywordKind),   // let \\ Keyword
	Operator(OperatorKind), // + \\ Operator
	Syntax(SyntaxKind),     // ( \\ Syntax
	Literal(String),        // \\ Literal
	Number,                 // 123  \\ NumericLiteral
	String,                 // " "  \\ StringLiteral
	Placeholder,            // _    \\ Placeholder / PartialApply

	// ─────────────────────────────────────────────
	// Комментарии
	// ─────────────────────────────────────────────
	LineComment,  // //   \\ LineComment
	BlockComment, // /* */\\ BlockComment
	DocComment,   // ///  \\ DocumentationComment

	Expression, // \\ Выражение: Some / Some * Soma + Some...

	Unit(UnitKind),
}
