use super::super::keyword::KeywordKind;
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

	// ─────────────────────────────────────────────
	// Маркер
	// ─────────────────────────────────────────────
	Marker,

	// ─────────────────────────────────────────────
	// Структурные символы / разделители
	// ─────────────────────────────────────────────
	LeftParen,          // (  \\ GroupStart
	RightParen,         // )  \\ GroupEnd
	LeftBrace,          // {  \\ BlockStart
	RightBrace,         // }  \\ BlockEnd
	LeftBracket,        // [  \\ IndexStart
	RightBracket,       // ]  \\ IndexEnd
	Comma,              // ,  \\ Separator
	Dot,                // .  \\ MemberAccess
	DotDot,             // ..  \\ Range
	DotDotDot,          // ... \\ Destructuring
	Semicolon,          // ;  \\ StatementEnd
	Colon,              // :  \\ TypeOrLabel
	ColonColon,         // :: \\ TypeOrLabel
	ColonEqual,         // := \\ TypeOrLabel
	Backslash,          // \  \\ Escape or Difference
	BackslashBackslash, // \\ \\
	Question,           // ?  \\ Conditional / Nullable
	DollarLeftBrace,    // ${ \\ InterpolationStart

	FloorStart, // ⌊  \\ FloorStart
	FloorEnd,   // ⌋  \\ FloorEnd
	CeilStart,  // ⌈  \\ CeilStart
	CeilEnd,    // ⌉  \\ CeilEnd

	Underscore, // _  \\ Wildcard

	// ─────────────────────────────────────────────
	// Арифметические и математические операторы
	// ─────────────────────────────────────────────
	Plus,             // +  \\ Add
	PlusMinus,        // +- \\ ± Tolerance
	DotPlus,          // .+ \\ Декартово произведение (∔)
	Minus,            // -  \\ Subtract / Negate
	DotMinus,         // .- \\ Monus (∸), 10 .- 20 = 0
	MinusPlus,        // -+ \\ Inverse Tolerance
	Asterisk,         // * \\ Multiply
	AsteriskAsterisk, // ** \\ Power
	Slash,            // /  \\ Divide
	SlashSlash,       //  \\ IntegerDivide
	Percent,          // %  \\ Modulo
	PercentPercent,   // %% \\

	Delta, // Δ \\ Delta

	// ─────────────────────────────────────────────
	// Инкременты и присваивания
	// ─────────────────────────────────────────────
	PlusPlus,        // ++ \\ Increment
	MinusMinus,      // -- \\ Decrement
	Equal,           // =  \\ Assign
	PlusEqual,       // += \\ AddAssign
	MinusEqual,      // -= \\ SubAssign
	AsteriskEqual,   // *= \\ MulAssign
	SlashEqual,      // /= \\ DivAssign
	PercentEqual,    // %= \\ ModAssign
	SlashSlashEqual, // //= \\ IntDivAssign
	DotEqual,        // .= \\ Append / ConcatAssign

	// ─────────────────────────────────────────────
	// Сравнение и равенство
	// ─────────────────────────────────────────────
	Bang,                // !   \\ LogicalNot
	BangEqual,           // !=  \\ NotEqual
	EqualEqual,          // ==  \\ Equal
	EqualEqualEqual,     // === \\ StrictEqual (≣)
	Greater,             // >   \\ GreaterThan
	GreaterGreater,      // >>  \\ ShiftRight
	GreaterGreaterEqual, // >>= \\ ShiftRightAssign
	GreaterEqual,        // >=  \\ GreaterOrEqual
	Less,                // <   \\ LessThan
	LessLess,            // <<  \\ ShiftLeft
	LessLessEqual,       // <<= \\ ShiftLeftAssign
	LessEqual,           // <=  \\ LessOrEqual
	TildeEqual,          // ~=  \\ PatternMatch

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
	// Логические операторы
	// ─────────────────────────────────────────────
	AmpersandAmpersand, // && \\ LogicalAnd
	PipePipe,           // ||  \\ LogicalOr
	QuestionQuestion,   // ?? \\ NullishCoalescing

	// ─────────────────────────────────────────────
	// Битовые операторы
	// ─────────────────────────────────────────────
	Ampersand, // &  \\ BitwiseAnd
	Pipe,      // |  \\ BitwiseOr
	Caret,     // ^  \\ BitwiseXor
	Tilde,     // ~  \\ BitwiseNot
	AndEqual,  // &= \\ BitwiseAndAssign
	OrEqual,   // |= \\ BitwiseOrAssign
	XorEqual,  // ^= \\ BitwiseXorAssign

	// ─────────────────────────────────────────────
	// Pipe / функциональный поток
	// ─────────────────────────────────────────────
	Arrow,        // -> \\ ThinArrow / Mapping
	FatArrow,     // => \\ Lambda / CaseArrow
	PipeForward,  // |> \\ PipeForward / ForwardApply
	PipeBackward, // <| \\ PipeBackward / BackwardApply

	// ─────────────────────────────────────────────
	// Литералы и идентификаторы
	// ─────────────────────────────────────────────
	Identifier,           // name \\ Identifier
	Keyword(KeywordKind), // let \\ Keyword
	Number,               // 123  \\ NumericLiteral
	NumberInfinity,       // inf  \\ NumericLiteral
	String,               // " "  \\ StringLiteral
	SingleQuotedString,   // ' '  \\ StringLiteral
	GraveQuotedString,    // ` `  \\ StringLiteral
	MultilineString,      // """ \\ MultilineStringLiteral
	Placeholder,          // _    \\ Placeholder / PartialApply

	// ─────────────────────────────────────────────
	// Комментарии
	// ─────────────────────────────────────────────
	LineComment,  // //   \\ LineComment
	BlockComment, // /* */\\ BlockComment
	DocComment,   // ///  \\ DocumentationComment

	At,          // @ \\ AttributePrefix
	AtEqual,     // @= \\ AttributeAssign
	Hash,        // # \\ Directive / Macro
	HashEqual,   // #= \\ MacroAssign
	Dollar,      // $ \\ SpecialIdentifier
	DollarEqual, // $= \\ SpecialAssign

	Expression, // \\ Выражение: Some / Some * Soma + Some...

	Unit(UnitKind),
}
