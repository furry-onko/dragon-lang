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
		file_handle.write_all(
			format!("{}\n", line).as_bytes()
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

pub fn read_file_string(path: &str) -> String {
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
		expect("Failed to read a file.");

	content
}

pub fn read_file(path: &str) -> Vec<String> {
	let content: String = read_file_string(path);

	let result: Vec<String> = content.split("\n").
		map(|item: &str| item.to_string()).
		collect();

	result
}

pub fn read_file_vec(path: &str) -> Vec<Vec<char>> {
	let content: Vec<String> = read_file(path);	// Load file content
	content.into_iter().						// Iterate over file content (lines)
		map(|line: String| {					// Map lines
			line.chars().						// Get characters iter
				// map(|ch: char| ch.to_string()).	// Convert them to Strings
				collect::<Vec<char>>()			// Collect to Vec<char>
		}).collect()							// Collect to Vec<Vec<char>>
}

pub fn extract_file(path: &str) -> Option<&str> {
	Path::new(path).
		file_name()?.
		to_str()
}

pub fn extract_file_extension(path: &str) -> Option<&str> {
	Path::new(path).
		extension()?.
		to_str()
}

pub fn extract_file_name(path: &str) -> Option<&str> {
	Path::new(path).
		file_stem()?.
		to_str()
}

pub fn get_cwd() -> String {
	env::current_dir().unwrap(). // get PathBuf
		into_os_string().		 // into OsString
		into_string().unwrap()	 // into String
}

/* pub fn check_and_read<F>(path: &str, mut err_action: F) -> Vec<String>
where F: FnMut() -> () {
	if !location_exists(path) {
		err_action();
		process::exit(1);
	}

	read_file(path)
}*/