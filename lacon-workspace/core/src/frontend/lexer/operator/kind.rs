use crate::shared::common::characters::*;

macro_rules! define_operators {
		($($name:ident => $($ch:tt)|+),* $(,)?) => {
				#[derive(Clone, PartialEq, Eq)]
				pub enum OperatorKind {
						$($name(Option<Box<OperatorKind>>)),*
				}

				impl OperatorKind {
						pub fn from_char(c: char) -> Option<fn(Option<Box<OperatorKind>>) -> Self> {
								match c {
										$($($ch)|+ => Some(OperatorKind::$name),)*
										_ => None,
								}
						}

						pub fn is_op_char(c: char) -> bool {
								Self::from_char(c).is_some()
						}

						pub fn from_str(s: &str) -> Option<Self> {
								let mut chars = s.chars().peekable();
								Self::build_recursive(&mut chars)
						}

						fn build_recursive(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Option<Self> {
								let c = chars.next()?;
								let constructor = Self::from_char(c)?;

								let next_op = if let Some(&next_c) = chars.peek() {
										if Self::is_op_char(next_c) {
												Self::build_recursive(chars).map(Box::new)
										} else {
												None
										}
								} else {
										None
								};

								Some(constructor(next_op))
						}
				}
				impl std::fmt::Display for OperatorKind {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        OperatorKind::$name(next) => {
                            write!(f, "{}", stringify!($name))?;
                            if let Some(next_op) = next {
                                write!(f, " {}", next_op)?;
                            }
                            Ok(())
                        }
                    ),*
                }
            }
        }

        impl std::fmt::Debug for OperatorKind {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self)
						}
				}
		};
}

define_operators! {
		// ASCII operators
		Plus           => '+',
		Minus          => '-' | MINUS_SIGN,
		Asterisk       => '*',
		Slash          => '/',
		Equal          => '=',
		Exclamation    => '!',
		Question       => '?',
		Percent        => '%',
		Ampersand      => '&',
		Pipe           => '|',
		Greater        => '>',
		Less           => '<',
		Dot            => '.',
		Colon          => ':',
		Tilde          => '~',
		Circumflex      => '^',

		// Unicode operators
		Multiplication => MULTIPLICATION_SIGN,
		Obelos         => DIVISION_SIGN,
		PlusMinus      => PLUS_MINUS_SIGN,
		DotMinus       => DOT_MINUS_SIGN,
		DotPlus        => DOT_PLUS_SIGN,
		NotEqual       => NOT_EQUAL_SIGN,
		AlmostEqual    => ALMOST_EQUAL_SIGN,
		IdenticalTo    => IDENTICAL_TO_SIGN,
		StrictEqual    => STRICT_EQUAL_SIGN,
		LessEqual      => LESS_EQUAL_SIGN,
		GreaterEqual   => GREATER_EQUAL_SIGN,
}

// .  \\ MemberAccess
// ..  \\ Range
// ... \\ Destructuring
// ;  \\ StatementEnd
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

// &  \\ BitwiseAnd
// |  \\ BitwiseOr
// ^  \\ BitwiseXor
// ~  \\ BitwiseNot
// &= \\ BitwiseAndAssign
// |= \\ BitwiseOrAssign
// ^= \\ BitwiseXorAssign

// -> \\ ThinArrow / Mapping
// => \\ Lambda / CaseArrow
// |> \\ PipeForward / ForwardApply
// <| \\ PipeBackward / BackwardApply
