pub mod big_strings;
pub mod fields {
	use std::path::PathBuf;
	use std::sync::LazyLock;

	pub static ROOT: LazyLock<PathBuf> = LazyLock::new(|| {
		std::env::var("LACON_WORKSPACE_ROOT").map(PathBuf::from).unwrap_or_else(|_| {
			let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
			crate_root.parent().unwrap_or(&crate_root).to_path_buf()
		})
	});

	pub static LACON_FILES_DIR: LazyLock<PathBuf> = LazyLock::new(|| ROOT.join("files"));

	pub static LEXER_RESULTS_DIR: LazyLock<PathBuf> = LazyLock::new(|| LACON_FILES_DIR.join("_lexer_results"));
}
pub use fields::*;

#[cfg(test)]
mod lexer_direct_0;

#[cfg(test)]
mod lexer_files_to_token_lists;
