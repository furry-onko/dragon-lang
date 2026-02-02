#![allow(unused)]

use std::process;
use crate::argv::{self, FileSummary, Mode};
use crate::{file, visual, toml};

pub fn initial_file_summary_check(summary: &FileSummary) {
	match summary.mode {
		Mode::Run => {
			if !file::location_exists(&format!("{}/src/main.drg", summary.path)) {
				visual::error("Could not find main.drg file in src.");
				process::exit(1);
			}
			println!("Mode::Run | Path: {}", summary.path);
			// ...
		},
		Mode::RunF => {
			if !file::location_exists(&summary.path) {
				visual::error("Could not find file specified.");
				process::exit(1);
			}
			println!("Mode::RunF | Path: {}", summary.path);
			// ...
		},
		Mode::Check => {},
		_ => unreachable!(),
	}
}