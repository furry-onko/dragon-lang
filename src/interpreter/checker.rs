#![allow(unused)]

use std::process;
use crate::argv::{self, FileSummary, Mode};
use crate::{file, visual, toml};
use crate::interpreter::tokenizer;

pub fn initial_file_summary_check(summary: &FileSummary) {
	match summary.mode {
		Mode::Run => {
			let path: &str = &format!("{}/src/main.drg", summary.path);

			if !file::location_exists(path) {
				visual::error("Could not find main.drg file in src.");
				process::exit(1);
			}
			tokenizer::start(&summary.path);
		},
		Mode::RunF => {
			if !file::location_exists(&summary.path) {
				visual::error("Could not find file specified.");
				process::exit(1);
			}
			tokenizer::start(&summary.path);
		},
		Mode::Check => {},
		_ => unreachable!(),
	}
}