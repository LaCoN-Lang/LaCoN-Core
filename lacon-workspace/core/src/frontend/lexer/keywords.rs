use crate::frontend::lexer::token_type::TokenType;
use phf::phf_map;

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
	// --- Управление потоком (Control Flow) ---
	"if" => TokenType::If,
	"else" => TokenType::Else,
	"elif" => TokenType::Elif,
	"match" => TokenType::Match,
	"case" => TokenType::Case,
	"default" => TokenType::Default,
	"switch" => TokenType::Switch,
	"for" => TokenType::For,
	"while" => TokenType::While,
	"until" => TokenType::Until,
	"spread" => TokenType::Spread,
	"generate" => TokenType::Generate,
	"combine" => TokenType::Combine,
	"enumerate" => TokenType::Enumerate,
	"filter" => TokenType::Filter,
	"flatten" => TokenType::Flatten,
	"repeat" => TokenType::Repeat,
	"transform" => TokenType::Transform,
	"transpose" => TokenType::Transpose,
	"loop" => TokenType::Loop,
	"break" => TokenType::Break,
	"continue" => TokenType::Continue,
	"return" => TokenType::Return,
	"yield" => TokenType::Yield,
	"exit" => TokenType::Exit,
	"cancel" => TokenType::Cancel,
	"defer" => TokenType::Defer,

	// --- Обработка исключений ---
	"try" => TokenType::Try,
	"catch" => TokenType::Catch,
	"finally" => TokenType::Finally,
	"throw" => TokenType::Throw,

	// --- Асинхронность ---
	"async" => TokenType::Async,
	"await" => TokenType::Await,
	"coroutine" => TokenType::Coroutine,

	// --- Объявления и Структура ---
	"class" => TokenType::Class,
	"interface" => TokenType::Interface,
	"enum" => TokenType::Enum,
	"cont" | "container" => TokenType::Container,
	"func" |"function" => TokenType::Function,
	"proc" |"procedure" => TokenType::Procedure,
	"let" |"var" |"variable" => TokenType::Variable,
	"const" |"constant" => TokenType::Constant,
	"struct" |"structure" => TokenType::Structure,
	"import" => TokenType::Import,
	"export" => TokenType::Export,
	"from" => TokenType::From,
	"include" => TokenType::Include,
	"provide" => TokenType::Provide,
	"new" => TokenType::New,
	"use" => TokenType::Use,
	"schema" =>  TokenType::Schema,

	"sanction" => TokenType::Sanction,
	"be" => TokenType::Be,
	"only" => TokenType::Only,
	"context" => TokenType::Context,
	"condition" => TokenType::Condition,
	"action" => TokenType::Action,
	"capability" => TokenType::Capability,
	"may" => TokenType::May,

	// --- Типовая система ---
	"type" => TokenType::Type,
	"auto" => TokenType::Auto,
	"alias" => TokenType::Alias,
	"as" => TokenType::As,
	"is" => TokenType::Is,
	"extends" => TokenType::Extends,
	"implements" => TokenType::Implements,
	"in" => TokenType::In,
	"of" => TokenType::Of,
	"where" => TokenType::Where,
	"when" => TokenType::When,
	"contains" => TokenType::Contains,
	"with" => TokenType::With,

	// --- Литералы-константы ---
	"true" => TokenType::True,
	"false" => TokenType::False,
	"nil" => TokenType::Nil,
	"none" => TokenType::None,
	"undefined" => TokenType::Undefined,
	"this" => TokenType::This,
	"self" => TokenType::SelfScope,
	"super" => TokenType::Super,
	"root" => TokenType::Root,
	"parent" => TokenType::Parent,
	"origin" => TokenType::Origin,
	"here" => TokenType::Here,

	// --- Модификаторы доступа и ООП ---
	"public" => TokenType::Public,
	"private" => TokenType::Private,
	"protected" => TokenType::Protected,
	"internal" => TokenType::Internal,
	"external" => TokenType::External,
	"global" => TokenType::Global,
	"local" => TokenType::Local,
	"static" => TokenType::Static,
	"virtual" => TokenType::Virtual,
	"abstract" => TokenType::Abstract,
	"override" => TokenType::Override,
	"final" => TokenType::Final,

	// --- Метапрограммирование ---
	"meta" => TokenType::Meta,
	"reflect" => TokenType::Reflect,
	"attribute" => TokenType::Attribute,

	// --- Логические операторы (текстовые) ---
	"and" => TokenType::And,
	"or" => TokenType::Or,
	"not" => TokenType::Not,

	// --- Единицы измерения ---
	"deg" | "rad" => TokenType::Unit,

	// --- Константы и Маркеры ---
	"infinity" |"Infinity" => TokenType::NumberInfinity,
	"delta" => TokenType::Delta,
	"section"=> TokenType::SectionMaker,

	"Marker" => TokenType::Marker,
};

/// Проверяет, является ли идентификатор ключевым словом.
pub fn get_keyword_token(identifier: &str) -> Option<TokenType> {
	KEYWORDS.get(identifier).cloned()
}
