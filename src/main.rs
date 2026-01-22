/* 2026 - @furry_onko */
use std::env;

mod argv;
mod visual;
mod file;
mod interpreter;

fn main() {
	let file = argv::proc(env::args());
	interpreter::initial_file_summary_check(&file);
}