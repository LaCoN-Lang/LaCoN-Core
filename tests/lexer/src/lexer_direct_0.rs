const FILE_STRINGS_BIG: &str = include_str!("../../files/big.lacon");

#[cfg(test)]
mod lexer_tests {
	// use super::super::big_strings::FILE_STRINGS_BIG;
	use super::FILE_STRINGS_BIG;
	use lacon_core::frontend::lexer::{Scanner, TokenKind};
	use lacon_core::shared::{ErrorReporter, ErrorStorage, UnitArena, UnitContext};
	use memory_stats::memory_stats;
	use std::time::Instant;

	#[test]
	fn lexer_speed_test_full() {
		let mut error_store = ErrorStorage::new();

		let arena = UnitArena::new();
		let ctx = UnitContext::new(&arena);

		let source_str = FILE_STRINGS_BIG;
		let iterations = 1000;
		let warmup_iterations = 10;

		let multiple_files = true;

		// Подготавливаем данные как байты
		let bench_source_string = if multiple_files { source_str.to_string() } else { source_str.repeat(iterations) };
		let source_bytes: &[u8] = if multiple_files { source_str.as_bytes() } else { bench_source_string.as_bytes() };

		let iters_to_run = if multiple_files { iterations } else { 1 };

		// Инициализируем сканер (убедись, что Scanner::new принимает &[u8])
		// Если Scanner::new все еще принимает &str внутри, он сам сделает .as_bytes()
		let mut scanner = Scanner::new(source_str.as_bytes(), &ctx, &mut error_store);

		// Прогрев
		for _ in 0..warmup_iterations {
			scanner.reset(source_bytes); // Передаем &[u8]
			let tokens = scanner.scan_tokens();
			std::hint::black_box(tokens);
		}

		let mem_before = memory_stats().map_or(0, |m| m.physical_mem);
		let start_time = Instant::now();
		let mut total_tokens = 0;
		let mut total_lines = 0;

		for _ in 0..iters_to_run {
			scanner.reset(source_bytes);

			let tokens = scanner.scan_tokens();
			total_tokens += tokens.len();
			total_lines += tokens.iter().filter(|t| t.kind == TokenKind::Newline).count();

			std::hint::black_box(tokens);
		}

		let duration = start_time.elapsed();
		let mem_after = memory_stats().map_or(0, |m| m.physical_mem);

		// Расчеты
		let total_bytes = source_bytes.len() * iters_to_run;
		let actual_avg_tokens = total_tokens as f64 / iters_to_run as f64;
		let duration_secs = duration.as_secs_f64();

		let byte_speed = if duration_secs > 0.0 { (total_bytes as f64 / 1_048_576.0) / duration_secs } else { 0.0 };
		let token_speed = if duration_secs > 0.0 { (total_tokens as f64 / 1_000_000.0) / duration_secs } else { 0.0 };
		let lines_speed = if duration_secs > 0.0 { (total_lines as f64 / 1_000_000.0) / duration_secs } else { 0.0 };

		println!("========================================");
		println!("ТЕСТОВЫЕ ДАННЫЕ (u8 MODE):");
		println!("Режим:             {}", if multiple_files { "Множество файлов (reset)" } else { "Один гигантский файл" });
		println!("Длина исходника:   {} байт", source_bytes.len());
		println!("Итераций:          {}", iters_to_run);
		println!("----------------------------------------");
		println!("РЕЗУЛЬТАТЫ БЕНЧМАРКА:");
		println!("Всего токенов (Σ):  {}", total_tokens);
		println!("Всего байт (Σ):    {}", total_bytes);
		println!("Среднее токенов/итерацию: {:.2}", actual_avg_tokens);
		println!("----------------------------------------");
		println!("ПРОИЗВОДИТЕЛЬНОСТЬ:");
		println!("Общее время:        {:.3}ms", duration_secs * 1000.0);
		println!("Среднее на прогон:  {:.3}µs", duration.as_nanos() as f64 / iters_to_run as f64 / 1000.0);
		println!("Память (diff):      {:.2}MB", (mem_after as f64 - mem_before as f64) / 1024.0 / 1024.0);
		println!("СКОРОСТЬ (MiB/s):   {:.2} млн байт/сек", byte_speed);
		println!("СКОРОСТЬ (токены):  {:.2} млн/сек", token_speed);
		println!("СКОРОСТЬ (строки):  {:.2} млн/сек", lines_speed);
		println!("========================================\n");

		for (index, error) in error_store.all().iter().enumerate() {
			if index >= 5 {
				break;
			}
			ErrorReporter::Console.report(&error.error);
		}
	}
}
