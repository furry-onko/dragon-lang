/* 2026 - @furry_onko */
#![allow(unused)]

use crate::visual;
use std::process;

#[derive(Debug, PartialEq)]
#[allow(clippy::enum_variant_names)]
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
	Unknown(String), // Unknown Parameter
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
			mode,
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
		Some(cmd)       => Unknown(cmd.to_string()),
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
				"Usage: [help | -h] [run (-f <file>)] [check (-f <file> | -d <dir>)]",
				"       [new (lib | link | project | prg) <name>]",
				"       [version | -v] [man (article)]",
				"  ---\n",
				
				"Common commands used in various situations:\n",
				
				"Code processing:",
				"   run          Executes the `main.drg` program file.",
				"   run -f       Executes the file provided by the User.",
				"   check        Checks every file in the project directory.",
				"   check -f     Checks the file given by the User.",
				"   check -d     Checks the project given by User\n",

				"Create a new file: (using `new`)",
				"   lib          Creates new dragon library file (.dh)",
				"   link         Creates new dragon dynamic library link (.ddl)",
				"   project      Creates new dragon project (Directory)",
				"   prg          Creates new dragon program file (.drg)",
				"   (By default `prg` is the default option of `new`.",
				"    Names `lib`, `link` and `project` are therefore reserved.)\n",
				
				"Learn more:",
				"   help | -h    Shows the help page.",
				"   version | -v Shows the interpreter version.",
				"   man          Shows the language documenation.\n",
			];

			for line in h_page {
				println!("{}", line);
			}

			process::exit(0);
		},
		Version => {
			use colored::Colorize;
			
			visual::info_green("Dragon Interpreter v0.1.0");
			println!("By: {}", "Onko Aikuu (@furry_onko) 2026".truecolor(0xFF, 0xAF, 0xFA));
			process::exit(0);
		},
		Unknown(text) => {
			visual::error(&format!("Unknown option: {}", text));
			process::exit(1);
		},
		Empty => {
			visual::error("No options found. Use `help` option to show the help page.");
			process::exit(1);
		}
		_ => (),
	}
}