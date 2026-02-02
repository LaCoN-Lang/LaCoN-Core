use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
	pub line: u32,
	pub character: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Range {
	pub start: Position,
	pub end: Position,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextDocumentIdentifier {
	pub uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HoverParams {
	pub text_document: TextDocumentIdentifier,
	pub position: Position,
}
