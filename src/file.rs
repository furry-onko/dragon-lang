/* 2026 - @furry_onko */
#![allow(unused)]

use crate::argv;
use crate::visual;
use std::fs::File;
use std::io::Write;

pub fn create_file_with_content (
	name: &str,
	content: &[&str]
) {
	let mut file_handle = File::create(name).expect("Failed to create a file.");

	for line in content {
		file_handle.write(
			&format!("{}\n", line).as_bytes()
		).expect("Failed to write a line to a file.");
	}
}

pub fn check_file(file_summary: &argv::FileSummary) {
	println!("{:?}", file_summary);
}