/* 2026 - @furry_onko */
#![allow(unused)]

use crate::{visual, file};
use std::process;
use std::env;

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
	Man,     // Manual pages
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

	let curr_dir = env::current_dir().unwrap();

	// Check mode
	let mode = match argv.next().as_deref() {
		Some("run") => Run,
		Some("new") => New,
		Some("check") => Check,
		Some("help") | Some("-h") => Help,
		Some("version") | Some("-v") => Version,
		Some("man") => Man,
		Some(unkn) => Unknown(unkn.to_string()),
		None => Empty,
	};

	match mode {
		Run => { todo!(); },
		Check => { todo!(); },
		New => {
			let opt_or_name = argv.next().
				unwrap_or_else(|| {
					visual::error("No option or name found.");
					process::exit(1);
				});

			if opt_or_name == "lib" || opt_or_name == "header" {
				let lib_name = argv.next().
					unwrap_or_else(|| {
						visual::error("No header name found.");
						process::exit(1);
					});

				visual::info(&format!("Creating header named \"{}\".", lib_name));

				file::create_file_with_content(
					&format!("{}.dh", lib_name),
					&[	"int     cat32",
						"str     ASCII\n",
						&format!("exp     '{}.dh'", lib_name)
					]
				);
				visual::info_green("Header created.");
				process::exit(0);
			}
		},
		Man => {
			todo!();
		},
		_ => execute_in_place(&mode),
	}

	Ok(FileSummary::new("/path/to/x/y/z", Run))
}

fn execute_in_place(mode: &Mode) {
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
				"   lib | header Creates new dragon library file (.dh)",
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