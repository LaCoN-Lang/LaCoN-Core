#[cfg(test)]
mod lexer_tests {
	use super::super::{LACON_FILES_DIR, LEXER_RESULTS_DIR};
	use lacon_core::frontend::lexer::Scanner;
	use lacon_core::frontend::lexer::TokenFlags;
	use lacon_core::shared::{ErrorStorage, UnitArena, UnitContext};
	use memory_stats::memory_stats;
	use std::fs::{self, File};
	use std::io::{BufWriter, Write};
	use std::path::Path;
	use std::time::{Duration, Instant};

	#[test]
	fn lexer_check() {
		let mem_before = memory_stats().map_or(0, |m| m.physical_mem);

		fs::create_dir_all(LEXER_RESULTS_DIR.as_path()).expect("Не удалось создать директорию");

		let entries = fs::read_dir(LACON_FILES_DIR.as_path()).expect("Не удалось прочитать директорию");

		let mut processed_count = 0;
		let mut total_scan_time = Duration::ZERO; // Суммируем ТОЛЬКО чистое время лексера

		for entry in entries {
			let entry = entry.expect("Ошибка элемента директории");
			let path = entry.path();

			if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
				if matches!(ext, "lacon" | "llacon" | "slacon") {
					let scan_duration = process_file_optimized(&path);
					total_scan_time += scan_duration;
					processed_count += 1;
				}
			}
		}

		let mem_after = memory_stats().map_or(0, |m| m.physical_mem);
		let mem_diff = (mem_after as f64 - mem_before as f64) / 1024.0 / 1024.0;

		println!("\n========================================");
		println!("Обработано файлов: {}", processed_count);
		println!("ЧИСТОЕ время лексера (без I/O): {:.2}ms", total_scan_time.as_secs_f64() * 1000.0);
		println!("Использование памяти: {:.2}MB", mem_diff);
		println!("========================================\n");

		assert!(processed_count > 0);
	}

	fn process_file_optimized(file_path: &Path) -> Duration {
		let mut error_store = ErrorStorage::new();
		let arena = UnitArena::new();
		let ctx = UnitContext::new(&arena);

		// 1. Читаем файл целиком (не входит в замер времени лексера)
		let source_bytes = fs::read(file_path).unwrap();

		// 2. ЗАМЕРЯЕМ ТОЛЬКО ЭТОТ БЛОК
		let mut scanner = Scanner::new(&source_bytes, &ctx, &mut error_store);
		let start_scan = Instant::now();
		let tokens = scanner.scan_tokens();
		let scan_duration = start_scan.elapsed();

		// 3. Оптимизированная запись (вне замера времени)
		let output_path = LEXER_RESULTS_DIR.join(format!("{}.tokens", file_path.file_name().unwrap().to_str().unwrap()));
		let file = File::create(&output_path).unwrap();
		let mut writer = BufWriter::with_capacity(128 * 1024, file); // Буфер 128КБ

		writeln!(writer, "{:<55} | {:<40} | {:<15} | {:<10} | {:<10}", "TYPE", "LEXEME", "POSITION", "LINE START", "WHITESPACE").unwrap();
		writeln!(writer, "{}", "-".repeat(170)).unwrap();

		// Используем временный буфер для строк, чтобы не аллоцировать на каждой итерации
		let mut kind_buf = String::with_capacity(64);

		for token in tokens {
			kind_buf.clear();
			use std::fmt::Write as _;
			write!(&mut kind_buf, "{:?}", token.kind).unwrap();

			// Печатаем напрямую в BufWriter, конвертируя байты «на лету» через lossy
			// Это гораздо быстрее, чем .into_owned()
			let lexeme_view = token.lexeme.map(|b| String::from_utf8_lossy(b)).unwrap_or_default();

			writeln!(
				writer,
				"{:<55} | {:<40} | {:<15} | {:<10} | {:<10}",
				kind_buf,
				lexeme_view,
				token.position.to_string(),
				if token.flags.contains(TokenFlags::AT_LINE_START) { "TRUE" } else { "" },
				if token.flags.contains(TokenFlags::HAS_PRECEDING_WHITESPACE) { "TRUE" } else { "" }
			)
			.unwrap();
		}

		// Принудительно сбрасываем буфер на диск
		writer.flush().unwrap();

		scan_duration // Возвращаем только время работы алгоритма
	}
}
