use super::SourceId;

#[derive(Debug, Clone)]
pub struct SourceFile {
	pub source_id: SourceId,
	pub source: String,
}
