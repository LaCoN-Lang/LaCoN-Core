use crate::shared::common::characters::*;

macro_rules! define_operators {
    ($($name:ident => $($ch:tt)|+),* $(,)?) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
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
