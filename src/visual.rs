/* 2026 - @furry_onko */
#![allow(unused)]
use colored::Colorize;

pub fn info(text: &str, variant: u8) {
	if variant == 0 {
		println!("{}", text.blue());
	}
	else {
		println!("{}", text.bright_green());
	}
}

pub fn error(text: &str) {
	println!("{}", text.red());
}