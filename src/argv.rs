/* 2026 - @furry_onko */
#![allow(unused)]

use crate::visual;

#[derive(Debug, PartialEq)]
pub enum ProcErrors {
	FileNotFound(String),
	FileAccessDenied(String),
	FileNotSpecified,
}

#[derive(Debug, PartialEq)]
pub enum InterpreterMode {
	Run,
	New,
	Check,
	Help,
	Unknown,
}

#[derive(Debug)]
pub struct FileSummary {
	pub path: String,
	pub mode: InterpreterMode,
}

impl FileSummary {
	fn new(path: &str, mode: InterpreterMode) -> Self {
		Self {
			path: String::from(path),
			mode: mode,
		}
	}
}

pub fn proc<I>(mut argv: I) -> Result<FileSummary, ProcErrors>
where I: Iterator<Item = String> {
	// Discard file path
	argv.next();

	// Check mode
	let interpreter_mode = match argv.next().as_deref() {
		Some("run")    => InterpreterMode::Run,
		Some("new")    => InterpreterMode::New,
		Some("check")  => InterpreterMode::Check,
		Some("help")   => InterpreterMode::Help,
		Some(_) | None => InterpreterMode::Unknown,
	};

	// Check file
	match argv.next() {
		Some(path) => Ok(FileSummary::new(&path, interpreter_mode)),
		None => Err(ProcErrors::FileNotSpecified),
	}
}