mod strings;
use strings::*;
fn main() {
	println!("Hello, world!");
}

#[cfg(test)]
mod lexer_tests {
	use super::*;
	use lacon_core::frontend::lexer::Scanner;
	use lacon_core::frontend::lexer::TokenFlags;
	use lacon_core::shared::unit::{UnitArena, UnitContext};
	use memory_stats::memory_stats;
	use std::fs::{self, File};
	use std::io::Write;
	use std::path::Path;
	use std::sync::LazyLock;
	use std::time::Instant;

	static LACON_FILES_DIR: LazyLock<String> = LazyLock::new(|| {
		let manifest_dir = env!("CARGO_MANIFEST_DIR");
		format!("{}/lacon_files", manifest_dir)
	});
	static LEXER_RESULTS_DIR: LazyLock<String> = LazyLock::new(|| format!("{}/lexer_results", LACON_FILES_DIR.as_str()));

	#[test]
	fn lexer_check() {
		let start_time = Instant::now();
		let mem_before = memory_stats().map_or(0, |m| m.physical_mem);

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
		let arena = UnitArena::new();
		let ctx = UnitContext::new(&arena);
		println!("Обработка файла: {:?}", file_path);

		let source = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Не удалось прочитать файл {:?}", file_path));

		let mut scanner = Scanner::new(&source, &ctx);
		let tokens = scanner.scan_tokens();

		let file_name = file_path.file_name().unwrap().to_str().unwrap();
		let output_file_name = format!("{}.tokens", file_name);
		let output_path = Path::new(LEXER_RESULTS_DIR.as_str()).join(output_file_name);

		let mut file = File::create(&output_path).unwrap_or_else(|_| panic!("Не удалось создать файл {:?}", output_path));

		writeln!(file, "{:<55} | {:<40} | {:<30} | {:<15} | {:<10} | {:<10}", "TYPE", "LEXEME", "LITERAL", "POSITION", "LINE START", "WHITESPACE").unwrap();
		writeln!(file, "{}", "-".repeat(170)).unwrap();

		for token in tokens {
			let literal_str = match &token.literal {
				Some(l) => *l,
				None => "",
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

	#[test]
	fn lexer_speed_test() {
		let arena = UnitArena::new();
		let ctx = UnitContext::new(&arena);

		let source_str = *FILE_STRINGS_CYR;
		let iterations = 1000000; // Увеличим до 1000 для более стабильной статистики
		let warmup_iterations = 10;

		println!("\n Запуск оптимизированного теста скорости...");

		// 1. Предварительная подготовка (Warmup)
		// Позволяет CPU "разогреться", а кэшам заполниться
		for _ in 0..warmup_iterations {
			let mut scanner = Scanner::new(source_str, &ctx);
			let _ = scanner.scan_tokens();
		}

		// 2. Подготовка данных
		// Если Scanner требует String, мы создаем одну строку заранее.
		// Если Scanner поддерживает &str, используй source_str напрямую.
		let source_owned = source_str;

		let mem_before = memory_stats().map_or(0, |m| m.physical_mem);
		let start_time = Instant::now();

		let mut total_tokens = 0;

		for _ in 0..iterations {
			// Мы используем clone(), так как это минимальная задержка
			// по сравнению с парсингом, если лексер забирает владение.
			let mut scanner = Scanner::new(&source_owned, &ctx);
			let tokens = scanner.scan_tokens();
			total_tokens += tokens.len();

			// Предотвращаем оптимизацию "пустого цикла" компилятором
			std::hint::black_box(&tokens);
		}

		let duration = start_time.elapsed();
		let mem_after = memory_stats().map_or(0, |m| m.physical_mem);

		let total_chars = source_str.len() * iterations;
		let avg_duration_ns = duration.as_nanos() as f64 / iterations as f64;
		let mem_diff = (mem_after as f64 - mem_before as f64) / 1024.0 / 1024.0;

		println!("========================================");
		println!("Итераций (после разогрева): {}", iterations);
		println!("Всего токенов (Σ):          {}", total_tokens);
		println!("Всего символов (Σ):         {}", total_chars);
		println!("----------------------------------------");
		println!("Общее время:                {:.3}ms", duration.as_secs_f64() * 1000.0);
		println!("Среднее на прогон:          {:.3}µs", avg_duration_ns / 1000.0);
		println!("Память (diff):              {:.2}MB", mem_diff);
		println!("РЕАЛЬНАЯ СКОРОСТЬ:          {:.2} млн симв/сек", (total_chars as f64 / 1_000_000.0) / duration.as_secs_f64());
		println!("========================================\n");
	}
}
