/* 2026 - @furry_onko */
use std::env;

mod argv;
mod visual;
mod file;
mod interpreter;

fn main() {
	let file = argv::proc(env::args());

	if let Ok(f_sum) = &file {
		interpreter::checker::initial_file_summary_check(f_sum);
	}
}