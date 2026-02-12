use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SourceId {
	File(PathBuf),
	Uri(String),
	Virtual(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceCodeReadModes {
	None,
	DynamicData,
	StaticData,
}
