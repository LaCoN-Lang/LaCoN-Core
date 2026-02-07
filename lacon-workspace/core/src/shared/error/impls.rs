use super::{Error, ErrorKind, ErrorPos, ErrorReporter, ErrorReporterFlag, Position};
use std::fmt;

impl Error {
	pub fn global(error_kind: ErrorKind) -> Self {
		Self { kind: error_kind, pos: None }
	}
	pub fn at(error_kind: ErrorKind, start: Position) -> Self {
		Self {
			kind: error_kind,
			pos: Some(ErrorPos { start, end: None }),
		}
	}
	pub fn span(kind: ErrorKind, start: Position, end: Position) -> Self {
		Self {
			kind,
			pos: Some(ErrorPos { start, end: Some(end) }),
		}
	}
}

impl ErrorReporter {
	pub fn report(&self, error: &Error) {
		use ErrorReporter::*;
		match self {
			Silent => {}
			Console => println!("{}", error),
			File(path, flag) => {
				use std::fs::OpenOptions;
				use std::io::Write;

				let flag = *flag;

				let mut file = OpenOptions::new()
					.create(true)
					.write(true)
					.truncate(flag == ErrorReporterFlag::Truncate)
					.append(flag == ErrorReporterFlag::Append)
					.open(path)
					.expect("Cannot open file to write error");

				writeln!(file, "{}", error).expect("Cannot write error to file");
			}
			Log(path, flag) => {
				let flag = *flag;

				Console.report(error);
				File(path.clone(), flag).report(error);
			}
		}
	}
}

// Defaults

impl Default for ErrorReporter {
	fn default() -> Self {
		Self::Console
	}
}

impl Default for ErrorReporterFlag {
	fn default() -> Self {
		Self::Append
	}
}

// Displays

impl fmt::Display for ErrorPos {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "on position at ({})", self.start)?;

		if let Some(end) = &self.end {
			write!(f, " to ({})", end)?;
		}

		Ok(())
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[{}]", self.kind)?;

		if let Some(start_pos) = &self.pos {
			write!(f, " {}", start_pos)?;
		}

		Ok(())
	}
}
