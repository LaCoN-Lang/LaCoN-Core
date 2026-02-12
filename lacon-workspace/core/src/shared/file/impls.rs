use crate::shared::{SourceCodeReadModes, SourceId};

use super::SourceFile;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io;
use std::path::{Path, PathBuf};

fn hash_str(s: &str) -> u64 {
	let mut hasher = DefaultHasher::new();
	s.hash(&mut hasher);
	hasher.finish()
}

fn resolve_path(path: impl AsRef<Path>) -> io::Result<PathBuf> {
	let path = path.as_ref();

	if path.is_absolute() {
		return Ok(path.to_path_buf());
	}

	let current_dir = std::env::current_dir()?;
	let resolved = current_dir.join(path);

	print!("{} -> {}\n", path.display(), resolved.display());
	resolved.canonicalize()
}

impl SourceFile {
	pub fn load(path: impl Into<PathBuf>) -> io::Result<Self> {
		let path = path.into();
		let resolved_path = resolve_path(&path)?;
		let source = std::fs::read_to_string(&resolved_path)?;
		let source_id = SourceId::File(resolved_path);

		Ok(Self { source_id, source })
	}

	pub fn from_uri(uri: String) -> io::Result<Self> {
		let source = if uri.starts_with("file://") {
			let path = PathBuf::from(uri.strip_prefix("file://").unwrap());
			let resolved_path = resolve_path(&path)?;
			std::fs::read_to_string(resolved_path)?
		} else {
			String::new()
		};

		let source_id = SourceId::Uri(uri);

		Ok(Self { source_id, source })
	}

	pub fn as_virtual(name: impl Into<String>, content: impl Into<String>) -> Self {
		Self {
			source_id: SourceId::Virtual(name.into()),
			source: content.into(),
		}
	}
	pub fn hash(&self) -> u64 {
		hash_str(&self.source)
	}

	pub fn name(&self) -> Option<&str> {
		match &self.source_id {
			SourceId::File(path) => path.file_name()?.to_str(),
			SourceId::Uri(uri) => Some(uri),
			SourceId::Virtual(name) => Some(name),
		}
	}

	pub fn extension(&self) -> Option<&str> {
		match &self.source_id {
			SourceId::File(path) => path.extension()?.to_str(),
			_ => None,
		}
	}

	pub fn code_mode(&self) -> Option<SourceCodeReadModes> {
		match &self.source_id {
			SourceId::File(path) => {
				let ext = path.extension()?.to_str()?;
				match ext {
					"llacon" => Some(SourceCodeReadModes::DynamicData),
					"slacon" => Some(SourceCodeReadModes::StaticData),
					_ => Some(SourceCodeReadModes::None),
				}
			}
			SourceId::Uri(uri) => {
				let ext = uri.split('/').last()?.split('.').last()?.to_lowercase();
				match ext.as_str() {
					"llacon" => Some(SourceCodeReadModes::DynamicData),
					"slacon" => Some(SourceCodeReadModes::StaticData),
					_ => Some(SourceCodeReadModes::None),
				}
			}
			_ => Some(SourceCodeReadModes::None),
		}
	}

	pub fn parent(&self) -> Option<&Path> {
		match &self.source_id {
			SourceId::File(path) => path.parent(),
			_ => None,
		}
	}

	pub fn full_path(&self) -> &str {
		match &self.source_id {
			SourceId::File(path) => path.to_str().unwrap_or(""),
			SourceId::Uri(uri) => uri,
			SourceId::Virtual(name) => name,
		}
	}

	pub fn count_lines(&self) -> usize {
		self.source.as_bytes().iter().filter(|&&b| b == b'\n').count() + 1
	}

	pub fn size(&self) -> usize {
		self.source.len()
	}
}
