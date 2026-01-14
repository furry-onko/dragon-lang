/* 2026 - @furry_onko */
#![allow(unused)]

use crate::visual;
use std::process;

#[derive(Debug, PartialEq)]
pub enum ProcErrors {
	FileNotFound(String),
	FileAccessDenied(String),
	FileNotSpecified,
}

#[derive(Debug, PartialEq)]
pub enum Mode {
	Run,     // Run Program
	New,     // New File
	Check,   // Check Code
	Help,    // Show Help Page
	Version, // Show Interpreter Version
	Unknown, // Unknown Parameter
	Empty,   // No Parameter
}

#[derive(Debug)]
pub struct FileSummary {
	pub path: String,
	pub mode: Mode,
}

impl FileSummary {
	fn new(path: &str, mode: Mode) -> Self {
		Self {
			path: String::from(path),
			mode: mode,
		}
	}
}

pub fn proc<I>(mut argv: I) -> Result<FileSummary, ProcErrors>
where I: Iterator<Item = String> {
	use Mode::*;

	// Discard file path
	argv.next();

	// Check mode
	let interpreter_mode = match argv.next().as_deref() {
		Some("run")     => Run,
		Some("new")     => New,
		Some("check")   => Check,
		Some("help")    => Help,
		Some("version") => Version,
		Some(_)         => Unknown,
		None            => Empty,
	};
	check_interpreter_mode(&interpreter_mode);

	// Check file
	match argv.next() {
		Some(path) => Ok(FileSummary::new(&path, interpreter_mode)),
		None => Err(ProcErrors::FileNotSpecified),
	}
}

fn check_interpreter_mode(mode: &Mode) {
	use Mode::*;

	match mode {
		Help => {
			let h_page = [
				"Usage: [help | -h] [run [-f <file>]] [check [-f <file>]]",
				"       [new <name>] [version | -v]",
			];

			for line in h_page {
				println!("{}", line);
			}

			process::exit(0);
		},
		Version => {
			use colored::Colorize;
			visual::info("Dragon Interpreter v0.1.0", 1);
			println!("By: {}", "Onko Aikuu (@furry_onko)".truecolor(0xFF, 0xAF, 0xAA));
			process::exit(0);
		}

		_ => (),
	}
}