mod enums;
mod impls;
mod kind;
mod structs;

pub use enums::*;
pub use kind::*;
pub use structs::*;

use crate::shared::position::Position;

#[cfg(test)]
mod errtest {
	use super::*;

	#[test]
	fn errtestfn() {
		let mut store = ErrorStorage::new();
		let s_start = Position::new(1, 59, 153);
		let s_end = Position::new(7, 59, 160);
		let s_kind = ErrorKind::Syntax;
		let s_subkind = SyntaxError::Expected { expected: "Function", found: "Variable" };
		let s_err = Error::span(s_kind(s_subkind), s_start, s_end);
		store.add(s_err, ErrorFlag::Critical);

		let l_start = Position::new(8, 59, 153);
		let l_kind = ErrorKind::Lexical;
		let l_subkind = LexicalError::InvalidCharacter('\\');
		let l_err = Error::at(l_kind(l_subkind), l_start);
		store.add(l_err, ErrorFlag::Critical);

		for err in store.all() {
			ErrorReporter::Console.report(&err.error);
		}

		println!("\n\n------------------\n\n");

		let lexical_errors = store.filter_by_kind(|k| matches!(k, ErrorKind::Lexical(_)));
		for err in lexical_errors {
			ErrorReporter::Console.report(&err.error);
		}
	}
}
