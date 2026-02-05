fn main() {
	println!("Hello, world!");
}

#[cfg(test)]
mod lexer_tests {
	use lacon_core::frontend::lexer::Scanner;
	use lacon_core::frontend::lexer::TokenFlags;
	use std::fs::{self, File};
	use std::io::Write;
	use std::path::Path;
	use std::sync::LazyLock;

	static LACON_FILES_DIR: LazyLock<String> = LazyLock::new(|| {
		let manifest_dir = env!("CARGO_MANIFEST_DIR");
		format!("{}/lacon_files", manifest_dir)
	});
	static LEXER_RESULTS_DIR: LazyLock<String> = LazyLock::new(|| format!("{}/lexer_results", LACON_FILES_DIR.as_str()));

	#[test]
	fn lexer_check() {
		fs::create_dir_all(LEXER_RESULTS_DIR.as_str()).expect("Не удалось создать директорию для результатов");

		let lacon_dir = Path::new(LACON_FILES_DIR.as_str());
		if !lacon_dir.exists() {
			panic!("Директория {} не существует", LACON_FILES_DIR.as_str());
		}

		let entries = fs::read_dir(lacon_dir).expect("Не удалось прочитать директорию lacon_files");

		let mut processed_count = 0;

		for entry in entries {
			let entry = entry.expect("Не удалось прочитать элемент директории");
			let path = entry.path();

			if let Some(extension) = path.extension() {
				let ext = extension.to_str().unwrap_or("");
				if ext == "lacon" || ext == "llacon" || ext == "slacon" {
					process_file(&path);
					processed_count += 1;
				}
			}
		}

		println!("Обработано файлов: {}", processed_count);
		assert!(processed_count > 0, "Не найдено ни одного .lacon/.llacon/.slacon файла");
	}

	fn process_file(file_path: &Path) {
		println!("Обработка файла: {:?}", file_path);

		let source = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Не удалось прочитать файл {:?}", file_path));

		let mut scanner = Scanner::new(source);
		let tokens = scanner.scan_tokens();

		let file_name = file_path.file_name().unwrap().to_str().unwrap();
		let output_file_name = format!("{}.tokens", file_name);
		let output_path = Path::new(LEXER_RESULTS_DIR.as_str()).join(output_file_name);

		let mut file = File::create(&output_path).unwrap_or_else(|_| panic!("Не удалось создать файл {:?}", output_path));

		writeln!(file, "{:<55} | {:<40} | {:<30} | {:<15} | {:<10} | {:<10}", "TYPE", "LEXEME", "LITERAL", "POSITION", "LINE START", "WHITESPACE").unwrap();
		writeln!(file, "{}", "-".repeat(170)).unwrap();

		for token in tokens {
			let literal_str = match &token.literal {
				Some(l) => l.clone(),
				None => "".to_string(),
			};

			writeln!(
				file,
				"{:<55} | {:<40} | {:<30} | {:<15} | {:<10} | {:<10}",
				format!("{:?}", token.token_kind),
				token.lexeme.replace("\n", "\\n"),
				literal_str,
				token.position.to_string(),
				if token.flags.contains(TokenFlags::AT_LINE_START) { "True" } else { "" },
				if token.flags.contains(TokenFlags::HAS_PRECEDING_WHITESPACE) { "WS" } else { "" }
			)
			.unwrap();
		}

		println!("Результат сохранён в: {:?}", output_path);
	}
}
