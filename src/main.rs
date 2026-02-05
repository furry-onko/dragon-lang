/* 2026 - @furry_onko */
use std::env;

mod argv;
mod visual;
mod file;
mod interpreter;
mod toml;

fn main() {
	let file = argv::proc(env::args());
	interpreter::initial_file_summary_check(&file);
}

/*
#[cfg(test)]
mod tests {
	use super::*;
	use crate::visual::*;

	#[test]
	fn error_trace() {
		visual::report(
			vec!['t', 'e', 's', 't', ' ', 'l', 'i', 'n', 'e'],
			8, 3,
			".section label",
			SyntaxError::UnexpectedToken
		)
	}
}
*/