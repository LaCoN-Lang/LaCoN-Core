mod impls;
mod kind;
mod structs;

pub use kind::*;
// pub use structs::*;

// use phf::phf_map;

// static KEYWORDS: phf::Map<&'static [u8], KeywordKind> = phf_map! {
// 	// --- Управление потоком (Control Flow) ---
// 	b"if" => KeywordKind::If,
// 	b"else" => KeywordKind::Else,
// 	b"elif" => KeywordKind::Elif,
// 	b"match" => KeywordKind::Match,
// 	b"case" => KeywordKind::Case,
// 	b"default" => KeywordKind::Default,
// 	b"switch" => KeywordKind::Switch,
// 	b"for" => KeywordKind::For,
// 	b"while" => KeywordKind::While,
// 	b"until" => KeywordKind::Until,
// 	b"spread" => KeywordKind::Spread,
// 	b"generate" => KeywordKind::Generate,
// 	b"combine" => KeywordKind::Combine,
// 	b"enumerate" => KeywordKind::Enumerate,
// 	b"filter" => KeywordKind::Filter,
// 	b"flatten" => KeywordKind::Flatten,
// 	b"repeat" => KeywordKind::Repeat,
// 	b"transform" => KeywordKind::Transform,
// 	b"transpose" => KeywordKind::Transpose,
// 	b"loop" => KeywordKind::Loop,
// 	b"break" => KeywordKind::Break,
// 	b"continue" => KeywordKind::Continue,
// 	b"return" => KeywordKind::Return,
// 	b"yield" => KeywordKind::Yield,
// 	b"exit" => KeywordKind::Exit,
// 	b"cancel" => KeywordKind::Cancel,
// 	b"defer" => KeywordKind::Defer,

// 	// --- Обработка исключений ---
// 	b"try" => KeywordKind::Try,
// 	b"catch" => KeywordKind::Catch,
// 	b"finally" => KeywordKind::Finally,
// 	b"throw" => KeywordKind::Throw,

// 	// --- Асинхронность ---
// 	b"async" => KeywordKind::Async,
// 	b"await" => KeywordKind::Await,
// 	b"coroutine" => KeywordKind::Coroutine,

// 	// --- Объявления и Структура ---
// 	b"class" => KeywordKind::Class,
// 	b"interface" => KeywordKind::Interface,
// 	b"enum" => KeywordKind::Enum,
// 	b"cont" | b"container" => KeywordKind::Container,
// 	b"func" | b"function" => KeywordKind::Function,
// 	b"proc" | b"procedure" => KeywordKind::Procedure,
// 	b"let" | b"var" | b"variable" => KeywordKind::Variable,
// 	b"const" | b"constant" => KeywordKind::Constant,
// 	b"entry" => KeywordKind::Entry,
// 	b"struct" | b"structure" => KeywordKind::Structure,
// 	b"import" => KeywordKind::Import,
// 	b"export" => KeywordKind::Export,
// 	b"from" => KeywordKind::From,
// 	b"include" => KeywordKind::Include,
// 	b"provide" => KeywordKind::Provide,
// 	b"new" => KeywordKind::New,
// 	b"use" => KeywordKind::Use,
// 	b"schema" =>  KeywordKind::Schema,

// 	b"sanction" => KeywordKind::Sanction,
// 	b"be" => KeywordKind::Be,
// 	b"only" => KeywordKind::Only,
// 	b"context" => KeywordKind::Context,
// 	b"condition" => KeywordKind::Condition,
// 	b"action" => KeywordKind::Action,
// 	b"capability" => KeywordKind::Capability,
// 	b"may" => KeywordKind::May,

// 	// --- Типовая система ---
// 	b"type" => KeywordKind::Type,
// 	b"alias" => KeywordKind::Alias,
// 	b"as" => KeywordKind::As,
// 	b"is" => KeywordKind::Is,
// 	b"extends" => KeywordKind::Extends,
// 	b"implements" => KeywordKind::Implements,
// 	b"in" => KeywordKind::In,
// 	b"of" => KeywordKind::Of,
// 	b"where" => KeywordKind::Where,
// 	b"when" => KeywordKind::When,
// 	b"contains" => KeywordKind::Contains,
// 	b"with" => KeywordKind::With,

// 	// --- Литералы-константы ---
// 	b"true" | b"false" | b"negate" => KeywordKind::Boolean,
// 	b"auto" => KeywordKind::AutoValue,
// 	b"nil" => KeywordKind::NilValue,
// 	b"none" => KeywordKind::NoneValue,
// 	b"undefined" => KeywordKind::UndefinedValue,
// 	b"this" => KeywordKind::This,
// 	b"self" => KeywordKind::SelfScope,
// 	b"super" => KeywordKind::Super,
// 	b"root" => KeywordKind::Root,
// 	b"parent" => KeywordKind::Parent,
// 	b"origin" => KeywordKind::Origin,
// 	b"here" => KeywordKind::Here,

// 	// --- Модификаторы доступа и ООП ---
// 	b"public" => KeywordKind::Public,
// 	b"private" => KeywordKind::Private,
// 	b"protected" => KeywordKind::Protected,
// 	b"internal" => KeywordKind::Internal,
// 	b"external" => KeywordKind::External,
// 	b"global" => KeywordKind::Global,
// 	b"local" => KeywordKind::Local,
// 	b"static" => KeywordKind::Static,
// 	b"virtual" => KeywordKind::Virtual,
// 	b"abstract" => KeywordKind::Abstract,
// 	b"override" => KeywordKind::Override,
// 	b"final" => KeywordKind::Final,

// 	// --- Метапрограммирование ---
// 	b"meta" => KeywordKind::Meta,
// 	b"reflect" => KeywordKind::Reflect,
// 	b"attribute" => KeywordKind::Attribute,

// 	// --- Логические операторы (текстовые) ---
// 	b"and" => KeywordKind::And,
// 	b"or" => KeywordKind::Or,
// 	b"not" => KeywordKind::Not,

// 	// --- Константы и Маркеры ---
// 	b"infinity" | b"Infinity" => KeywordKind::NumberInfinity,
// 	b"delta" => KeywordKind::Delta,
// 	b"xor" => KeywordKind::Xor,
// 	b"bitwise" => KeywordKind::Bitwise,
// 	b"section"=> KeywordKind::SectionMaker,

// 	b"Marker" => KeywordKind::Marker,
// };

// pub fn get_keyword_token(identifier: &[u8]) -> Option<KeywordKind> {
// 	let len = identifier.len();
// 	if !(2..=11).contains(&len) {
// 		return None;
// 	}
// 	KEYWORDS.get(identifier).cloned()
// }
