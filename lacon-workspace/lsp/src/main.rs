mod server;
mod types;

use lacon_core::frontend::lexer::Scanner;
use lacon_core::frontend::lexer::TokenKind;
use serde_json::{Value, json};
use std::io::{self, BufRead, Read, Write};

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let mut stdin_lock = stdin.lock();

	loop {
		let mut content_length: usize = 0;

		loop {
			let mut header = String::new();
			if stdin_lock.read_line(&mut header)? == 0 {
				return Ok(());
			}
			let trimmed = header.trim();
			if trimmed.is_empty() {
				break;
			}
			if trimmed.to_lowercase().starts_with("content-length:") {
				if let Some(len_str) = trimmed.split(':').nth(1) {
					content_length = len_str.trim().parse().unwrap_or(0);
				}
			}
		}

		if content_length == 0 {
			continue;
		}

		let mut body = vec![0u8; content_length];
		stdin_lock.read_exact(&mut body)?;

		let msg: Value = match serde_json::from_slice(&body) {
			Ok(v) => v,
			Err(_) => continue,
		};

		let method = msg["method"].as_str().unwrap_or("");
		let id = &msg["id"];

		match method {
			"initialize" => {
				let response = json!({
								"jsonrpc": "2.0",
								"id": id,
								"result": {
												"capabilities": {
																"hoverProvider": true,
																"textDocumentSync": 1
												}
								}
				});
				send_response(response)?;
			}

			"lacon/parseRaw" => {
				let content = msg["params"]["content"].as_str().unwrap_or("");
				let mut scanner = Scanner::new(content);
				let tokens = scanner.scan_tokens();

				let tokens_json: Vec<Value> = tokens
					.iter()
					.map(|t| {
						let mut dimension = None;

						// Используем твой метод is_unit() из TokenType
						if t.token_kind.is_unit() {
							dimension = Some(server::get_unit_formula(&t.lexeme));
						}

						json!({
										"token_type": format!("{:?}", t.token_kind),
										"lexeme": t.lexeme,
										"literal": t.literal,
										"position": t.position.to_string(),
										"flags": t.flags.bits() as u32,
										"dimension": dimension
						})
					})
					.collect();

				send_response(json!({
								"jsonrpc": "2.0",
								"id": id,
								"result": {
												"status": "success",
												"tokens": tokens_json
								}
				}))?;
			}

			_ => {
				if !id.is_null() {
					send_response(json!({
									"jsonrpc": "2.0",
									"id": id,
									"error": { "code": -32601, "message": "Method not found" }
					}))?;
				}
			}
		}
	}
}

fn send_response(res: Value) -> io::Result<()> {
	let out = res.to_string();
	let msg = format!("Content-Length: {}\r\n\r\n{}", out.len(), out);
	let mut stdout = io::stdout().lock();
	stdout.write_all(msg.as_bytes())?;
	stdout.flush()
}
