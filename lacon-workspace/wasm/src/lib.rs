use js_sys::{Array, Object, Reflect};
use lacon_core::frontend::lexer::Scanner;
use lacon_core::shared::unit::{UnitArena, UnitContext};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn lex(source: &str) -> JsValue {
	let arena = UnitArena::new();
	let ctx = UnitContext::new(&arena);
	let mut scanner = Scanner::new(source, &ctx);
	let tokens = scanner.scan_tokens();

	let result = Array::new();

	for token in tokens.iter() {
		let obj = Object::new();

		Reflect::set(&obj, &"token_type".into(), &format!("{:?}", token.token_kind).into()).unwrap();
		Reflect::set(&obj, &"lexeme".into(), &token.lexeme.clone().into()).unwrap();
		Reflect::set(&obj, &"literal".into(), &token.literal.clone().into()).unwrap();
		Reflect::set(&obj, &"position".into(), &token.position.to_string().into()).unwrap();
		Reflect::set(&obj, &"flags".into(), &(token.flags.bits() as u32).into()).unwrap();

		result.push(&obj);
	}

	result.into()
}
