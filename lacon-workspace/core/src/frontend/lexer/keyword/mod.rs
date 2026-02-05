mod impls;
mod kind;
mod structs;

pub use kind::*;
// pub use structs::*;

use phf::phf_map;
use std::collections::HashMap;
use std::sync::RwLock;

static KEYWORDS: phf::Map<&'static str, KeywordKind> = phf_map! {
	// --- Управление потоком (Control Flow) ---
	"if" => KeywordKind::If,
	"else" => KeywordKind::Else,
	"elif" => KeywordKind::Elif,
	"match" => KeywordKind::Match,
	"case" => KeywordKind::Case,
	"default" => KeywordKind::Default,
	"switch" => KeywordKind::Switch,
	"for" => KeywordKind::For,
	"while" => KeywordKind::While,
	"until" => KeywordKind::Until,
	"spread" => KeywordKind::Spread,
	"generate" => KeywordKind::Generate,
	"combine" => KeywordKind::Combine,
	"enumerate" => KeywordKind::Enumerate,
	"filter" => KeywordKind::Filter,
	"flatten" => KeywordKind::Flatten,
	"repeat" => KeywordKind::Repeat,
	"transform" => KeywordKind::Transform,
	"transpose" => KeywordKind::Transpose,
	"loop" => KeywordKind::Loop,
	"break" => KeywordKind::Break,
	"continue" => KeywordKind::Continue,
	"return" => KeywordKind::Return,
	"yield" => KeywordKind::Yield,
	"exit" => KeywordKind::Exit,
	"cancel" => KeywordKind::Cancel,
	"defer" => KeywordKind::Defer,

	// --- Обработка исключений ---
	"try" => KeywordKind::Try,
	"catch" => KeywordKind::Catch,
	"finally" => KeywordKind::Finally,
	"throw" => KeywordKind::Throw,

	// --- Асинхронность ---
	"async" => KeywordKind::Async,
	"await" => KeywordKind::Await,
	"coroutine" => KeywordKind::Coroutine,

	// --- Объявления и Структура ---
	"class" => KeywordKind::Class,
	"interface" => KeywordKind::Interface,
	"enum" => KeywordKind::Enum,
	"cont" | "container" => KeywordKind::Container,
	"func" |"function" => KeywordKind::Function,
	"proc" |"procedure" => KeywordKind::Procedure,
	"let" |"var" |"variable" => KeywordKind::Variable,
	"const" |"constant" => KeywordKind::Constant,
	"entry" => KeywordKind::Entry,
	"struct" |"structure" => KeywordKind::Structure,
	"import" => KeywordKind::Import,
	"export" => KeywordKind::Export,
	"from" => KeywordKind::From,
	"include" => KeywordKind::Include,
	"provide" => KeywordKind::Provide,
	"new" => KeywordKind::New,
	"use" => KeywordKind::Use,
	"schema" =>  KeywordKind::Schema,

	"sanction" => KeywordKind::Sanction,
	"be" => KeywordKind::Be,
	"only" => KeywordKind::Only,
	"context" => KeywordKind::Context,
	"condition" => KeywordKind::Condition,
	"action" => KeywordKind::Action,
	"capability" => KeywordKind::Capability,
	"may" => KeywordKind::May,

	// --- Типовая система ---
	"type" => KeywordKind::Type,
	"alias" => KeywordKind::Alias,
	"as" => KeywordKind::As,
	"is" => KeywordKind::Is,
	"extends" => KeywordKind::Extends,
	"implements" => KeywordKind::Implements,
	"in" => KeywordKind::In,
	"of" => KeywordKind::Of,
	"where" => KeywordKind::Where,
	"when" => KeywordKind::When,
	"contains" => KeywordKind::Contains,
	"with" => KeywordKind::With,

	// --- Литералы-константы ---
	"true" | "false" | "negate" => KeywordKind::Boolean,
	"auto" => KeywordKind::AutoValue,
	"nil" => KeywordKind::NilValue,
	"none" => KeywordKind::NoneValue,
	"undefined" => KeywordKind::UndefinedValue,
	"this" => KeywordKind::This,
	"self" => KeywordKind::SelfScope,
	"super" => KeywordKind::Super,
	"root" => KeywordKind::Root,
	"parent" => KeywordKind::Parent,
	"origin" => KeywordKind::Origin,
	"here" => KeywordKind::Here,

	// --- Модификаторы доступа и ООП ---
	"public" => KeywordKind::Public,
	"private" => KeywordKind::Private,
	"protected" => KeywordKind::Protected,
	"internal" => KeywordKind::Internal,
	"external" => KeywordKind::External,
	"global" => KeywordKind::Global,
	"local" => KeywordKind::Local,
	"static" => KeywordKind::Static,
	"virtual" => KeywordKind::Virtual,
	"abstract" => KeywordKind::Abstract,
	"override" => KeywordKind::Override,
	"final" => KeywordKind::Final,

	// --- Метапрограммирование ---
	"meta" => KeywordKind::Meta,
	"reflect" => KeywordKind::Reflect,
	"attribute" => KeywordKind::Attribute,

	// --- Логические операторы (текстовые) ---
	"and" => KeywordKind::And,
	"or" => KeywordKind::Or,
	"not" => KeywordKind::Not,


	// --- Константы и Маркеры ---
	"infinity" |"Infinity" => KeywordKind::NumberInfinity,
	"delta" => KeywordKind::Delta,
	"xor" => KeywordKind::Xor,
	"bitwise" => KeywordKind::Bitwise,
	"section"=> KeywordKind::SectionMaker,

	"Marker" => KeywordKind::Marker,
};

pub fn get_keyword_token(identifier: &str) -> Option<KeywordKind> {
	KEYWORDS.get(identifier).cloned()
}

lazy_static::lazy_static! {
	pub static ref ALIAS_KEYWORDS: RwLock<HashMap<String, &'static str>> = RwLock::new(HashMap::new());
}

pub fn add_alias(alias: &str, target_keyword: &'static str) {
	let mut map = ALIAS_KEYWORDS.write().unwrap();
	map.insert(alias.to_string(), target_keyword);
}

pub fn resolve_identifier(identifier: &str) -> Option<KeywordKind> {
	let map = ALIAS_KEYWORDS.read().unwrap();
	let key = map.get(identifier).unwrap_or(&identifier);
	KEYWORDS.get(key).cloned()
}
