#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OperatorKind {
	Plus,
	Minus,
	Asterisk,
	Slash,
	Equal,
	Exclamation,
	Question,
	Percent,
	Ampersand,
	Pipe,
	Greater,
	Less,
	Dot,
	Colon,
	Tilde,
	Circumflex,

	PlusPlus,
	PlusEqual,
	MinusMinus,
	MinusEqual,
	PercentPercent,
	PercentEqual,
	CircumflexCircumflex,
	CircumflexEqual,
	QuestionQuestion,
	QuestionEqual,
	QuestionColon,
	QuestionDot,
	AsteriskAsterisk,
	AsteriskEqual,
	SlashSlash,
	SlashEqual,
	TildeEqual,
	AmpersandAmpersand,
	AmpersandEqual,
	LessMinus,
	LessEqual,
	LessLess,
	LessMinusLess,
	LessLessEqual,
	GreaterGreater,
	GreaterEqual,
	EqualGreater,
	GreaterGreaterEqual,
	DashGreater,
	DashGreaterGreater,
	GreaterDashGreater,
	GreaterGreaterDash,
	GreaterGreaterGreater,
	EqualEqualGreater,
	LessDashGreater,
	PipePipe,
	PipeEqual,
	LessPipe,
	PipeGreater,
	EqualEqual,
	NotEqual,
	DotDot,
	DotPlus,
	DotMinus,
	DotEqual,
	ColonColon,
	ColonEqual,
	NotColon,
	PipePipeEqual,

	AsteriskAsteriskEqual,
	SlashSlashEqual,
	AmpersandAmpersandEqual,
	LessLessDash,
	LessLessLess,
	LessEqualEqual,
	GreaterGreaterDashEqual,
	EqualEqualEqual,
	NotEqualEqual,
	DotDotDot,
	DotDotEqual,
	LessDotDot,
	DotDotLess,

	Multiplication,
	Ellipsis,
	Obelus,
	PlusMinus,
	Monus,
	DotPlusUni,
	NotEqualUni,
	AlmostEqual,
	IdenticalTo,
	StrictEqualUni,
	LessEqualUni,
	GreaterEqualUni,
	Xor,
	Ring,
	ElementOf,
	NotAnElementOf,
	ContainsAsMember,
	DoesNotContainsAsMember,
	FloorStart,
	FloorEnd,
	CeilStart,
	CeilEnd,
}

// define_operators! {
// 	definitions {
// 		// ASCII operators
// 		Plus             => '+',
// 		Minus            => '-' | MINUS_SIGN,
// 		Asterisk         => '*',
// 		Slash            => '/',
// 		Equal            => '=',
// 		Exclamation      => '!',
// 		Question         => '?',
// 		Percent          => '%',
// 		Ampersand        => '&',
// 		Pipe             => '|',
// 		Greater          => '>',
// 		Less             => '<',
// 		Dot              => '.',
// 		Colon            => ':',
// 		Tilde            => '~',
// 		Circumflex       => '^',

// 		// Unicode operators
// 		Multiplication           => MULTIPLICATION_SIGN,
// 		Obelus                   => DIVISION_SIGN,
// 		PlusMinus                => PLUS_MINUS_SIGN,
// 		Monus                    => DOT_MINUS_SIGN,
// 		DotPlus                  => DOT_PLUS_SIGN,
// 		NotEqual                 => NOT_EQUAL_SIGN,
// 		AlmostEqual              => ALMOST_EQUAL_SIGN,
// 		IdenticalTo              => IDENTICAL_TO_SIGN,
// 		StrictEqual              => STRICT_EQUAL_SIGN,
// 		LessEqual                => LESS_EQUAL_SIGN,
// 		GreaterEqual             => GREATER_EQUAL_SIGN,
// 		Xor                      => XOR_SIGN,
// 		Ring                     => RING_OPERATOR_SIGN, // Композиция функций, a(x) = x × 2; b(x) = x + 1; c = a ∘ b ≣ c = b(a(x)) || или без создания переменной → (a ∘ b)(x)
// 		ElementOf                => ELEMENT_OF_SIGN, // element ∈ array (element in array); char ∈ string; x ∈ x..x (in range)
// 		NotAnElementOf           => NOT_AN_ELEMENT_OF_SIGN,
// 		ContainsAsMember         => CONTAINS_AS_MEMBER_SIGN, // array ∋ element (array contains element); string ∋ char
// 		DoesNotContainsAsMember  => DOES_NOT_CONTAIN_AS_MEMBER_SIGN,

// 		// Unicode paired operators
// 		FloorStart       => FLOOR_START_SIGN,
// 		FloorEnd         => FLOOR_END_SIGN,
// 		CeilStart        => CEIL_START_SIGN,
// 		CeilEnd          => CEIL_END_SIGN,
// 	}
// 	allowed {
// 		"++", "+=",
// 		"--", "-=",
// 		"%%", "%=",
// 		"^^", "^=",
// 		"??", "?=", "?:", "?.",
// 		"**", "*=", "**=",
// 		"//", "/=", "//=",
// 		"~=",
// 		"&&", "&=", "&&=",
// 		"<-", "<=", "<<", "<<-", "<-<", "<<=", "<<<", "<==",
// 		"->", ">=", "=>", ">>", "->>", ">->", ">>-", ">>=", ">>>", "==>",
// 		"<->",
// 		"||", "|=", "||=", "<|", "|>",

// 		"==", "===",
// 		"!!", "!=", "!==", "!:",
// 		"..", "...", ".+", ".-", ".=", "..=", "<..", "..<",
// 		"::", ":=",
// 	}
// }

// .  \\ MemberAccess
// ..  \\ Range
// ... \\ Spread
// :  \\ TypeOrLabel
// :: \\ TypeOrLabel
// := \\ TypeOrLabel
// ?  \\ Conditional / Nullable
// +  \\ Add
// +- \\ ± Tolerance
// .+ \\ Декартово произведение (∔)
// -  \\ Subtract / Negate
// .- \\ Monus (∸), 10 .- 20 = 0
// -+ \\ Inverse Tolerance
// * \\ Multiply
// ** \\ Power
// /  \\ Divide
//  \\ IntegerDivide
// %  \\ Modulo
// %% \\

// ++ \\ Increment
// -- \\ Decrement
// =  \\ Assign
// += \\ AddAssign
// -= \\ SubAssign
// *= \\ MulAssign
// /= \\ DivAssign
// %= \\ ModAssign
// / / = \\ IntDivAssign
// .= \\ Append / ConcatAssign

// !   \\ LogicalNot
// !=  \\ NotEqual
// ==  \\ Equal
// === \\ StrictEqual (≣)
// >   \\ GreaterThan
// >>  \\ ShiftRight
// >>= \\ ShiftRightAssign
// >=  \\ GreaterOrEqual
// <   \\ LessThan
// <<  \\ ShiftLeft
// <<= \\ ShiftLeftAssign
// <=  \\ LessOrEqual
// ~=  \\ PatternMatch

// && \\ LogicalAnd
// ||  \\ LogicalOr
// ?? \\ NullishCoalescing

// &  \\ Ссылка и возможно ASCII для композиции функций
// |  \\
// ^  \\ Возведение в степень, обязательно без пробела — 2^3
// ~=  \\ RegExMatch → string ~= pattern
// &= \\
// |= \\
// ^= \\ Возведение в степень, a ^= b → a = a^b, a^^ → a = a^2

// -> \\ ThinArrow / Mapping
// => \\ Lambda / CaseArrow
// |> \\ PipeForward / ForwardApply
// <| \\ PipeBackward / BackwardApply

// ⌊  \\ FloorStart
// ⌋  \\ FloorEnd
// ⌈  \\ CeilStart
// ⌉  \\ CeilEnd
