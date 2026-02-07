use std::path::PathBuf;

fn workspace_root() -> PathBuf {
	let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

	loop {
		let cargo = dir.join("Cargo.toml");
		if cargo.exists() {
			if std::fs::read_to_string(&cargo).map(|s| s.contains("[workspace]")).unwrap_or(false) {
				return dir;
			}
		}
		dir.pop();
	}
}

const FILE_STRINGS_BIG: &str = include_str!("../../files/big.lacon");

#[cfg(test)]
mod single_test {
	use super::{FILE_STRINGS_BIG, workspace_root};
	use lacon_core::frontend::lexer::Scanner;
	use lacon_core::shared::{ErrorStorage, UnitArena, UnitContext};

	#[allow(dead_code, unused_imports)]
	use flame as f;
	use flamer::flame;
	use std::fs::File;

	#[test]
	fn compare_simple_vs_complex() {
		let mut error_store = ErrorStorage::new();
		let source_str = FILE_STRINGS_BIG;
		let arena = UnitArena::new();
		let ctx = UnitContext::new(&arena);
		flame::start("profile performance");
		let mut scanner = Scanner::new(&source_str, &ctx, &mut error_store);
		scanner.scan_tokens();
		flame::end("profile performance");
		let path = workspace_root().join("tests").join("_logs").join("single.json");
		flame::dump_json(&mut File::create(path).unwrap()).unwrap();
	}
}

#[cfg(test)]
mod lexer_tests {
	use super::{FILE_STRINGS_BIG, workspace_root};
	use lacon_core::frontend::lexer::Scanner;
	use lacon_core::shared::{ErrorReporter, ErrorStorage, UnitArena, UnitContext};
	use memory_stats::memory_stats;
	use std::time::Instant;

	#[allow(dead_code, unused_imports)]
	use flame as f;
	use flamer::flame;
	use std::fs::File;
	#[test]
	fn lexer_speed_test_full() {
		flame::start("profile performance");
		let mut error_store = ErrorStorage::new();

		let arena = UnitArena::new();
		let ctx = UnitContext::new(&arena);

		let source_str = FILE_STRINGS_BIG;
		let iterations = 10000;
		let warmup_iterations = 10;

		let multiple_files = true;

		let bench_source = if multiple_files { source_str.to_string() } else { source_str.repeat(iterations) };
		let iters_to_run = if multiple_files { iterations } else { 1 };
		let source_ref = if multiple_files { source_str } else { &bench_source };

		let mut scanner = Scanner::new(source_ref, &ctx, &mut error_store);

		for _ in 0..warmup_iterations {
			scanner.reset(source_ref);
			let tokens = scanner.scan_tokens();
			std::hint::black_box(tokens);
		}

		let mem_before = memory_stats().map_or(0, |m| m.physical_mem);
		let start_time = Instant::now();
		let mut total_tokens = 0;

		for _ in 0..iters_to_run {
			scanner.reset(source_ref);

			let tokens = scanner.scan_tokens();
			total_tokens += tokens.len();

			std::hint::black_box(tokens);
		}

		let duration = start_time.elapsed();
		let mem_after = memory_stats().map_or(0, |m| m.physical_mem);

		let total_chars = source_ref.len() * iters_to_run;
		let actual_avg_tokens = total_tokens as f64 / iters_to_run as f64;
		let duration_secs = duration.as_secs_f64();

		let char_speed = if duration_secs > 0.0 { (total_chars as f64 / 1_000_000.0) / duration_secs } else { 0.0 };
		let token_speed = if duration_secs > 0.0 { (total_tokens as f64 / 1_000_000.0) / duration_secs } else { 0.0 };

		println!("========================================");
		println!("ТЕСТОВЫЕ ДАННЫЕ:");
		println!("Режим:             {}", if multiple_files { "Множество файлов (reset)" } else { "Один гигантский файл" });
		println!("Длина исходника:   {} байт", source_ref.len());
		println!("Итераций:          {}", iters_to_run);
		println!("----------------------------------------");
		println!("РЕЗУЛЬТАТЫ БЕНЧМАРКА:");
		println!("Всего токенов (Σ):  {}", total_tokens);
		println!("Всего символов (Σ): {}", total_chars);
		println!("Среднее токенов/итерацию: {:.2}", actual_avg_tokens);
		println!("----------------------------------------");
		println!("ПРОИЗВОДИТЕЛЬНОСТЬ:");
		println!("Общее время:        {:.3}ms", duration_secs * 1000.0);
		println!("Среднее на прогон:  {:.3}µs", duration.as_nanos() as f64 / iters_to_run as f64 / 1000.0);
		println!("Память (diff):      {:.2}MB", (mem_after as f64 - mem_before as f64) / 1024.0 / 1024.0);
		println!("СКОРОСТЬ (символы): {:.2} млн/сек", char_speed);
		println!("СКОРОСТЬ (токены):  {:.2} млн/сек", token_speed);
		println!("========================================\n");

		for (index, error) in error_store.all().iter().enumerate() {
			if index >= 5 {
				break;
			}
			ErrorReporter::Console.report(&error.error);
		}

		flame::end("profile performance");
		let path = workspace_root().join("tests").join("_logs").join("full.json");
		if let Ok(mut file) = File::create(path) {
			let _ = flame::dump_json(&mut file);
		}
	}
}
