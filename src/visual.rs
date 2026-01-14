/* 2026 - @furry_onko */
#![allow(unused)]
use colored::Colorize;

pub fn info(text: &str) {
	println!("{}", text.bright_blue());
}

pub fn info_green(text: &str) {
	println!("{}", text.bright_green());
}

pub fn warn(text: &str) {
	println!("{}", text.bright_yellow());
}

pub fn error(text: &str) {
	println!("{}", text.red());
}