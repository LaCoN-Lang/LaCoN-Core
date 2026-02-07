use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorFlag {
	Common,
	Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorReporterFlag {
	Truncate,
	Append,
}

pub enum ErrorReporter {
	Silent,
	Console,
	File(PathBuf, ErrorReporterFlag),
	Log(PathBuf, ErrorReporterFlag),
}
