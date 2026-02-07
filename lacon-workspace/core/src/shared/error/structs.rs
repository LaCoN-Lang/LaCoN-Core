use super::{ErrorFlag, ErrorKind, Position};

#[derive(Debug, Clone)]
pub struct ErrorPos {
	pub start: Position,
	pub end: Option<Position>,
}

#[derive(Debug, Clone)]
pub struct Error {
	pub kind: ErrorKind,
	pub pos: Option<ErrorPos>,
}

#[derive(Debug)]
pub struct StoredError {
	pub id: u16,
	pub error: Error,
	pub flag: ErrorFlag,
}

#[derive(Debug)]
pub struct ErrorStorage {
	errors: Vec<StoredError>,
	next_id: u16,
}

impl ErrorStorage {
	pub fn new() -> Self {
		Self { errors: Vec::new(), next_id: 0 }
	}

	pub fn add(&mut self, error: Error, flag: ErrorFlag) -> u16 {
		let id = self.next_id;
		self.next_id = self.next_id.wrapping_add(1);

		self.errors.push(StoredError { id, error, flag });
		id
	}

	pub fn get(&self, id: u16) -> Option<&StoredError> {
		self.errors.iter().find(|e| e.id == id)
	}

	pub fn filter_by_kind(&self, kind_to_match: fn(&ErrorKind) -> bool) -> impl Iterator<Item = &StoredError> {
		self.errors.iter().filter(move |e| kind_to_match(&e.error.kind))
	}

	pub fn filter_by_flag(&self, flag: ErrorFlag) -> impl Iterator<Item = &StoredError> {
		self.errors.iter().filter(move |e| e.flag == flag)
	}

	pub fn all(&self) -> &[StoredError] {
		&self.errors
	}
}
