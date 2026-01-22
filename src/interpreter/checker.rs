#![allow(unused)]

use std::process;
use crate::argv::{self, FileSummary};
use crate::file;
use crate::visual;

pub fn initial_file_summary_check(summary: &FileSummary) {
	println!("{:?}", summary);
}