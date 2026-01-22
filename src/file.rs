/* 2026 - @furry_onko */
#![allow(unused)]

use crate::argv;
use crate::visual;
use std::fs::{self, File};
use std::io::{self, Read, Write, ErrorKind};
use std::process;
use std::env;
use std::path::Path;

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

pub fn mkdir(dir_name: &str) {
	match fs::create_dir(dir_name) {
		Err(e) if e.kind() == ErrorKind::AlreadyExists => {
			visual::error(&format!("The directory {} already exists", dir_name));
			process::exit(1);
		},
		Err(e) => {
			visual::error(&format!("Unknown error: {}", e));
			process::exit(0);
		},
		_ => (),
	}
}

pub fn location_exists(location: &str) -> bool {
	Path::new(location).exists()
}

pub fn read_file(path: &str) -> Vec<String> {
	if !location_exists(path) {
		visual::error("File not found");
		process::exit(1);
	}

	let mut file = match File::open(path) {
		Ok(file) => file,
		Err(e) => {
			visual::error(&format!("An unknown error has occured: {}", e));
			process::exit(1);
		}
	};

	let mut content = String::new();
	file.read_to_string(&mut content).
		ok().
		expect("Failed to read a file.");

	let result: Vec<String> = content.split("\n").
		map(|item: &str| item.to_string()).
		collect();

	result
}

/* pub fn check_and_read<F>(path: &str, mut err_action: F) -> Vec<String>
where F: FnMut() -> () {
	if !location_exists(path) {
		err_action();
		process::exit(1);
	}

	read_file(path)
}*/