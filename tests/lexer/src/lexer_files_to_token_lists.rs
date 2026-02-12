#[cfg(test)]
mod lexer_tests {
	use super::super::{LACON_FILES_DIR, LEXER_RESULTS_DIR};
	use lacon_core::frontend::lexer::Scanner;
	use lacon_core::frontend::lexer::TokenFlags;
	use lacon_core::shared::{ErrorStorage, SourceCodeReadModes, SourceFile, UnitArena, UnitContext};
	use memory_stats::memory_stats;
	use std::fs::{self, File};
	use std::io::{BufWriter, Write};
	use std::time::{Duration, Instant};

	#[test]
	fn lexer_check() {
		let mem_before = memory_stats().map_or(0, |m| m.physical_mem);

		fs::create_dir_all(LEXER_RESULTS_DIR.as_path()).expect("Не удалось создать директорию");

		let entries = fs::read_dir(LACON_FILES_DIR.as_path()).expect("Не удалось прочитать директорию");

		let mut processed_count = 0;
		let mut processed_count_default = 0;
		let mut processed_count_list = 0;
		let mut processed_count_static = 0;
		let mut total_scan_time = Duration::ZERO;

		let arena = UnitArena::new();
		let ctx = UnitContext::new(&arena);

		for entry in entries {
			let entry = entry.expect("Ошибка элемента директории");
			let path = entry.path();

			if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
				if matches!(ext, "lacon" | "llacon" | "slacon") {
					let source_file = match SourceFile::load(&path) {
						Ok(sf) => sf,
						Err(e) => {
							eprintln!("Ошибка загрузки файла {:?}: {}", path, e);
							continue;
						}
					};

					let code_mode = source_file.code_mode();
					let mut error_store = ErrorStorage::new();

					let scan_duration = process_file_optimized(&source_file, &mut error_store, &ctx);

					total_scan_time += scan_duration;
					processed_count += 1;

					match code_mode {
						Some(SourceCodeReadModes::None) => processed_count_default += 1,
						Some(SourceCodeReadModes::DynamicData) => processed_count_list += 1,
						Some(SourceCodeReadModes::StaticData) => processed_count_static += 1,
						None => processed_count_default += 1,
					}
				}
			}
		}

		let mem_after = memory_stats().map_or(0, |m| m.physical_mem);
		let mem_diff = (mem_after as f64 - mem_before as f64) / 1024.0 / 1024.0;

		println!("\n========================================");
		println!("Обработано файлов: {processed_count}");
		println!("LaCoN: {processed_count_default}");
		println!("LLaCoN: {processed_count_list}");
		println!("SLaCoN: {processed_count_static}");
		println!("ЧИСТОЕ время лексера (без I/O): {:.2}ms", total_scan_time.as_secs_f64() * 1000.0);
		println!("Использование памяти: {:.2}MB", mem_diff);
		println!("========================================\n");

		assert!(processed_count > 0);
	}

	fn process_file_optimized(source_file: &SourceFile, error_store: &mut ErrorStorage, ctx: &UnitContext) -> Duration {
		let source_bytes = source_file.source.as_bytes();
		let code_mode = source_file.code_mode();

		let mut scanner = Scanner::new(source_bytes, ctx, error_store, code_mode);
		let start_scan = Instant::now();
		let tokens = scanner.scan_tokens();
		let scan_duration = start_scan.elapsed();

		let file_name = source_file.name().unwrap_or("unknown");
		let output_path = LEXER_RESULTS_DIR.join(format!("{}.tokens", file_name));

		let file = File::create(&output_path).unwrap();
		let mut writer = BufWriter::with_capacity(128 * 1024, file);

		writeln!(writer, "{:<15} | {:<55} | {:<55} | {:<10} | {:<10}", "POSITION", "TYPE", "LEXEME", "LINE START", "WHITESPACE").unwrap();
		writeln!(writer, "{}", "-".repeat(170)).unwrap();

		let mut kind_buf = String::with_capacity(64);

		for token in tokens {
			kind_buf.clear();
			use std::fmt::Write as _;
			write!(&mut kind_buf, "{:?}", token.kind).unwrap();

			let lexeme_view = token.lexeme.map(|b| String::from_utf8_lossy(b)).unwrap_or_default();

			writeln!(
				writer,
				"{:<15} | {:<55} | {:<40} | {:<10} | {:<10}",
				token.position.to_string(),
				kind_buf,
				lexeme_view,
				if token.flags.contains(TokenFlags::AT_LINE_START) { "TRUE" } else { "" },
				if token.flags.contains(TokenFlags::HAS_PRECEDING_WHITESPACE) { "TRUE" } else { "" }
			)
			.unwrap();
		}

		writer.flush().unwrap();

		scan_duration
	}
}
