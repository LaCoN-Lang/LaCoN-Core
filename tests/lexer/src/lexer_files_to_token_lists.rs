#[cfg(test)]
mod lexer_tests {
	use super::super::{LACON_FILES_DIR, LEXER_RESULTS_DIR};
	use lacon_core::frontend::lexer::Scanner;
	use lacon_core::frontend::lexer::TokenFlags;
	use lacon_core::shared::{ErrorStorage, UnitArena, UnitContext};
	use memory_stats::memory_stats;
	use std::fs::{self, File};
	use std::io::Write;
	use std::path::Path;
	use std::time::Instant;

	#[test]
	fn lexer_check() {
		let start_time = Instant::now();
		let mem_before = memory_stats().map_or(0, |m| m.physical_mem);

		// Создаём директорию для результатов
		fs::create_dir_all(LEXER_RESULTS_DIR.as_path()).expect("Не удалось создать директорию для результатов");

		// Проверяем существование исходной папки с файлами
		if !LACON_FILES_DIR.as_path().exists() {
			panic!("Директория {:?} не существует", LACON_FILES_DIR);
		}

		let entries = fs::read_dir(LACON_FILES_DIR.as_path()).expect("Не удалось прочитать директорию lacon_files");

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

		let duration = start_time.elapsed();
		let mem_after = memory_stats().map_or(0, |m| m.physical_mem);
		let mem_diff = (mem_after as f64 - mem_before as f64) / 1024.0 / 1024.0;

		println!("\n========================================");
		println!("Обработано файлов: {}", processed_count);
		println!("Время выполнения: {:.2}ms", duration.as_secs_f64() * 1000.0);
		println!("Использование памяти: {:.2}MB", mem_diff);
		println!("========================================\n");

		assert!(processed_count > 0, "Не найдено ни одного .lacon/.llacon/.slacon файла");
	}

	fn process_file(file_path: &Path) {
		let mut error_store = ErrorStorage::new();

		let arena = UnitArena::new();
		let ctx = UnitContext::new(&arena);
		println!("Обработка файла: {:?}", file_path);

		let source = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Не удалось прочитать файл {:?}", file_path));

		let mut scanner = Scanner::new(&source, &ctx, &mut error_store);
		let tokens = scanner.scan_tokens();

		let file_name = file_path.file_name().unwrap().to_str().unwrap();
		let output_file_name = format!("{}.tokens", file_name);
		let output_path = LEXER_RESULTS_DIR.join(output_file_name);

		let mut file = File::create(&output_path).unwrap_or_else(|_| panic!("Не удалось создать файл {:?}", output_path));

		writeln!(file, "{:<55} | {:<40} | {:<15} | {:<10} | {:<10}", "TYPE", "LEXEME", "POSITION", "LINE START", "WHITESPACE").unwrap();

		writeln!(file, "{}", "-".repeat(170)).unwrap();

		for token in tokens {
			writeln!(
				file,
				"{:<55} | {:<40} | {:<15} | {:<10} | {:<10}",
				format!("{:?}", token.token_kind),
				token.lexeme.unwrap_or(""),
				token.position.to_string(),
				if token.flags.contains(TokenFlags::AT_LINE_START) { "TRUE" } else { "" },
				if token.flags.contains(TokenFlags::HAS_PRECEDING_WHITESPACE) { "TRUE" } else { "" }
			)
			.unwrap();
		}

		println!("Результат сохранён в: {:?}", output_path);
	}
}
