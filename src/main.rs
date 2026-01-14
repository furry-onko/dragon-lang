/* 2026 - @furry_onko */
use std::env;

mod argv;
mod visual;
mod file;

fn main() {
	let file = argv::proc(env::args());

	if let Ok(f_sum) = &file {
		file::check_file(f_sum);
	}
}