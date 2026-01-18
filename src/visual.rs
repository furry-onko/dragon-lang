/* 2026 - @furry_onko */
#![allow(unused)]
use colored::Colorize;
use std::io::{self, Write};

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

pub fn await_input(msg: Option<&str>) -> String {
	let mut user_input = String::new();

	if let Some(s_msg) = msg {
		print!("{}", s_msg);
		io::stdout().
			flush().
			expect("Flush fail");
	}

	io::stdin().
		read_line(&mut user_input).
		expect("ReadLine fail");

	user_input
}

