use crate::shared::SourceCodeReadModes;

use super::KeywordKind;

impl KeywordKind {
	#[inline(always)]
	pub fn from_bytes(slice: &[u8]) -> Option<Self> {
		match slice {
			b"if" => Some(Self::If),
			b"else" => Some(Self::Else),
			b"elif" => Some(Self::Elif),
			b"match" => Some(Self::Match),
			b"case" => Some(Self::Case),
			b"default" => Some(Self::Default),
			b"switch" => Some(Self::Switch),
			b"for" => Some(Self::For),
			b"while" => Some(Self::While),
			b"until" => Some(Self::Until),
			b"spread" => Some(Self::Spread),
			b"generate" => Some(Self::Generate),
			b"combine" => Some(Self::Combine),
			b"enumerate" => Some(Self::Enumerate),
			b"filter" => Some(Self::Filter),
			b"flatten" => Some(Self::Flatten),
			b"repeat" => Some(Self::Repeat),
			b"transform" => Some(Self::Transform),
			b"transpose" => Some(Self::Transpose),
			b"loop" => Some(Self::Loop),
			b"break" => Some(Self::Break),
			b"continue" => Some(Self::Continue),
			b"return" => Some(Self::Return),
			b"yield" => Some(Self::Yield),
			b"exit" => Some(Self::Exit),
			b"cancel" => Some(Self::Cancel),
			b"defer" => Some(Self::Defer),

			// --- Обработка исключений ---
			b"try" => Some(Self::Try),
			b"catch" => Some(Self::Catch),
			b"finally" => Some(Self::Finally),
			b"throw" => Some(Self::Throw),

			// --- Асинхронность ---
			b"async" => Some(Self::Async),
			b"await" => Some(Self::Await),
			b"coroutine" => Some(Self::Coroutine),

			// --- Объявления и Структура ---
			b"class" => Some(Self::Class),
			b"interface" => Some(Self::Interface),
			b"enum" => Some(Self::Enum),
			b"cont" | b"container" => Some(Self::Container),
			b"func" | b"function" => Some(Self::Function),
			b"proc" | b"procedure" => Some(Self::Procedure),
			b"let" | b"var" | b"variable" => Some(Self::Variable),
			b"const" | b"constant" => Some(Self::Constant),
			b"entry" => Some(Self::Entry),
			b"struct" | b"structure" => Some(Self::Structure),
			b"import" => Some(Self::Import),
			b"export" => Some(Self::Export),
			b"from" => Some(Self::From),
			b"include" => Some(Self::Include),
			b"provide" => Some(Self::Provide),
			b"new" => Some(Self::New),
			b"use" => Some(Self::Use),
			b"schema" => Some(Self::Schema),

			b"sanction" => Some(Self::Sanction),
			b"be" => Some(Self::Be),
			b"only" => Some(Self::Only),
			b"context" => Some(Self::Context),
			b"condition" => Some(Self::Condition),
			b"action" => Some(Self::Action),
			b"capability" => Some(Self::Capability),
			b"may" => Some(Self::May),

			// --- Типовая система ---
			b"type" => Some(Self::Type),
			b"alias" => Some(Self::Alias),
			b"as" => Some(Self::As),
			b"is" => Some(Self::Is),
			b"extends" => Some(Self::Extends),
			b"implements" => Some(Self::Implements),
			b"in" => Some(Self::In),
			b"of" => Some(Self::Of),
			b"where" => Some(Self::Where),
			b"when" => Some(Self::When),
			b"contains" => Some(Self::Contains),
			b"with" => Some(Self::With),

			// --- Литералы-константы ---
			b"true" | b"false" | b"negate" => Some(Self::Boolean),
			b"auto" => Some(Self::AutoValue),
			b"nil" => Some(Self::NilValue),
			b"none" => Some(Self::NoneValue),
			b"undefined" => Some(Self::UndefinedValue),
			b"this" => Some(Self::This),
			b"self" => Some(Self::SelfScope),
			b"super" => Some(Self::Super),
			b"root" => Some(Self::Root),
			b"parent" => Some(Self::Parent),
			b"origin" => Some(Self::Origin),
			b"here" => Some(Self::Here),

			// --- Модификаторы доступа и ООП ---
			b"public" => Some(Self::Public),
			b"private" => Some(Self::Private),
			b"protected" => Some(Self::Protected),
			b"internal" => Some(Self::Internal),
			b"external" => Some(Self::External),
			b"global" => Some(Self::Global),
			b"local" => Some(Self::Local),
			b"static" => Some(Self::Static),
			b"virtual" => Some(Self::Virtual),
			b"abstract" => Some(Self::Abstract),
			b"override" => Some(Self::Override),
			b"final" => Some(Self::Final),

			// --- Метапрограммирование ---
			b"meta" => Some(Self::Meta),
			b"reflect" => Some(Self::Reflect),
			b"attribute" => Some(Self::Attribute),

			// --- Логические операторы (текстовые) ---
			b"and" => Some(Self::And),
			b"or" => Some(Self::Or),
			b"not" => Some(Self::Not),

			// --- Константы и Маркеры ---
			b"infinity" | b"Infinity" => Some(Self::NumberInfinity),
			b"delta" => Some(Self::Delta),
			b"xor" => Some(Self::Xor),
			b"bitwise" => Some(Self::Bitwise),
			b"section" => Some(Self::SectionMaker),

			b"Marker" => Some(Self::Marker),

			_ => None,
		}
	}

	pub fn in_allowed(&self, code_read_mode: &SourceCodeReadModes) -> bool {
		use SourceCodeReadModes::*;
		use KeywordKind::*;

		matches!(
			(code_read_mode, self),
			(
				DynamicData,
				Boolean
					| AutoValue
					| NilValue
					| NoneValue
					| UndefinedValue
					| Constant
					| Variable
					| Local
					| Attribute
					| This
					| SelfScope
					| Root
					| And
					| Or
					| Not
					| Xor
					| Delta
					| Bitwise
					| NumberInfinity
					| If
					| Else
					| Elif
					| As
					| Is
					| In
					| Of
					| Where
					| When
					| For
					| Loop
					| While
					| Until
					| Break
					| Continue
					| Yield
					| Switch
					| Case
					| Spread
					| Generate
					| Include
					| Provide
					| Use
					| Schema
					| SectionMaker
					| Marker
			) | (
				StaticData,
				Boolean | AutoValue | NilValue | NoneValue | UndefinedValue | Root | Xor | Delta | Bitwise | NumberInfinity | In | Of | Include | Provide | Use | Schema | SectionMaker | Marker
			)
		)
	}
}
