/* 2026 - @furry_onko */
#![allow(unused)]

use crate::{visual, file};
use std::process;
use std::env;
use std::path::Path;

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
	Link,    // Link a library
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
		Some("link") => Link,
		Some("man") => Man,
		Some(unkn) => Unknown(unkn.to_string()),
		None => Empty,
	};

	match mode {
		Run => {

		},
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

			else if opt_or_name == "project" {
				let project_name = argv.next().
					unwrap_or_else(|| {
						visual::error("No project name found.");
						process::exit(1);
					});

				visual::info(&format!("Creating new project named: {}", &project_name));
				file::mkdir(&project_name);

				file::mkdir(&format!("{}/src", &project_name));
				file::create_file_with_content(
					&format!("{}/src/main.drg", &project_name),
					&[	"int     cat32",
						"str     ASCII\n",
						"section .main:",
						"    hlt     0x00\n"
					]
				);
				file::create_file_with_content(
					&format!("{}/src/release.drg", &project_name),
					&[	"int     cat32",
						"str     ASCII\n",
						"section .main:",
						"    hlt     0x00\n"
					]
				);

				file::mkdir(&format!("{}/lib/", &project_name));
				file::mkdir(&format!("{}/dyn/", &project_name));

				file::create_file_with_content(
					&format!("{}/draco.toml", &project_name),
					&[	"[program]",
						&format!("name=\"{}\"", &project_name.to_lowercase()),
						"entrypoint=\"main.drg\"\n",
						"[release]",
						&format!("name=\"{}\"", &project_name.to_lowercase()),
						"entrypoint=\"release.drg\"",
						"version=\"0.1.0\"\n",
					]
				);
				visual::info_green("Successfully created the project.");
				process::exit(0);
			}

			else if opt_or_name == "prg" {
				let prg_name = argv.next().
					unwrap_or_else(|| {
						visual::error("Program name not found.");
						process::exit(1);
					});

				create_prg(&prg_name);
			}

			else {
				create_prg(&opt_or_name);
			}
		},
		Link => {
			// Check link arguments
			let link_argv = argv.collect::<Vec<String>>();
			let link_path: &str;
			let link_crate: Option<&str>;

			match link_argv.len() {
				0 => {
					visual::error("No entrypoint or directory given.");
					process::exit(1);
				},
				1 => {
					if link_argv[0].contains("/") {
						visual::warn("Source directory was not given. Continuing without it.");
						link_path = &link_argv[0];
						link_crate = None;
					}
					else {
						visual::error("This option accepts only file paths. Not file names.");
						process::exit(1);
					}
				},
				2 => {
					if link_argv[0].contains("/") {
						visual::error("This option accepts only file names. Not file paths.");
						process::exit(1);
					}
					else {
						link_path = &link_argv[0];
						link_crate = Some(&link_argv[1]);
					}
				},
				_ => {
					visual::error("Incorrect amount of operands.");
					process::exit(1);
				},
			}

			// If crate path was given
			if let Some(crate_path) = link_crate {
				if !link_path.ends_with(".dh") {
					visual::error("Linked file must end with `.dh`.");
					process::exit(1);
				}

				if file::location_exists(&format!("{}/{}", crate_path, link_path)) {
					file::create_file_with_content(
						&format!("{}.ddl", link_path),
						&[	"[dyn_link]",
							&format!("root_crate=\"{}\"", crate_path),
							&format!("entrypoint=\"{}\"\n", link_path),
						]
					);
					process::exit(0);
				}
				else {
					visual::error(&format!("File {} at {} not found", link_path, crate_path));
					process::exit(1);
				}
			}

			else {
				if !link_path.ends_with(".dh") {
					visual::error("Linked file must end with `.dh`.");
					process::exit(1);
				}

				if file::location_exists(link_path) {
					let ddl_name = format!("{}.ddl", Path::new(link_path).file_stem().unwrap().to_str().unwrap());
					file::create_file_with_content(
						&ddl_name,
						&[	"[dyn_link]",
							&format!("file=\"{}\"\n", link_path),
						]
					);
					process::exit(0);
				}
				else {
					visual::error(&format!("File at {} not found", link_path));
					process::exit(1);
				}
			}
		},
		Man => {
			todo!();
		},
		_ => execute_in_place(&mode),
	}

	Ok(FileSummary::new("/path/to/x/y/z", Run)) // for debugging purposes
}

fn create_prg(name: &str) {
	visual::info(&format!("Creating file named: {}.drg", name));
	file::create_file_with_content(
		&format!("{}.drg", name),
		&[	"int     cat32",
			"str     ASCII\n",
			"section .main:",
			"    hlt     0x00\n"
		]
	);
	visual::info_green("Successfully created a program.");
	process::exit(0);
}

fn execute_in_place(mode: &Mode) {
	use Mode::*;

	match mode {
		Help => {
			let h_page = [
				"Usage: [help | -h] [run (-f <file>)] [check (-f <file> | -d <dir>)]",
				"       [new (lib | project | prg) <name>] [link <entrypoint> [dir]]",
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
				"   project      Creates new dragon project (Directory)",
				"   prg          Creates new dragon program file (.drg)",
				"   (By default `prg` is the default option of `new`.",
				"    Names `lib`, `link` and `project` are therefore reserved.)\n",
				
				"Library linking:",
				"   link         Creates new dragon dynamic library link (.ddl)",
				
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

fn pre_file_summary_check(summary: FileSummary) {
	if !file::location_exists(&summary.path) {
		todo!();
	}
}