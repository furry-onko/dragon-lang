#![allow(unused)]
use crate::file;
use crate::visual::{self, SyntaxError};
use std::process;

pub fn start(path: &str) {
	let file_content: Vec<Vec<char>> = file::read_file_vec(&format!("{path}/src/main.drg"));
	tokenize(file_content);
}

fn tokenize(content: Vec<Vec<char>>) -> Program {
	let mut line_counter: u32 = 0;

	for line in &content {
		let mut ch_counter: u32 = 0;
		line_counter += 1;

		let mut tokens: Vec<Tokens> = Vec::new();
		let mut word = String::new();
		let mut number = String::new();
		let mut string = String::new();
		
		let mut expect_token = false;
		let mut in_string: bool = false;
		let mut in_comment: bool = false;
		let mut symbol_stack: Vec<char> = Vec::new();

		for ch in line {
			ch_counter += 1;

			if *ch == '/' {
				if let Some('/') = symbol_stack.last() {
					symbol_stack.pop();
					in_comment = true;
				}
				else {
					symbol_stack.push(*ch);
				}
				continue;
			}

			if *ch == '\'' {
				if in_string {
					tokens.push(Tokens::String(
						std::mem::take(&mut string)
					));
					in_string = false;
				}
				else {
					in_string = true;
				}
				continue;
			}
			if in_string {
				string.push(*ch);
				continue;
			}


			match *ch {
				';' => {
					in_comment = true;
					continue;
				},
				',' => {
					expect_token = true;
					tokens.push(Tokens::Comma);
					continue;
				},
				'[' => {
					symbol_stack.push(*ch);
					tokens.push(Tokens::SqBracketOpen);
					continue;
				},
				']' => {
					if let Some('[') = symbol_stack.last() {
						symbol_stack.pop();
						tokens.push(Tokens::SqBracketClose);
						continue;
					}
					else {
						visual::report(
							line,
							line_counter,
							ch_counter,
							"Tokenizer",
							SyntaxError::UnclosedBracket,
						);
					}
				},
				_ => (),
			}
		}

		if in_comment {
			in_comment = false;
			continue;
		}

		if !symbol_stack.is_empty() {
			visual::report(
				line,
				line_counter,
				ch_counter+1,
				"Tokenizer",
				SyntaxError::UnclosedBracket,
			)
		}

		if in_string && !in_comment {
			visual::report(
				line,
				line_counter,
				ch_counter,
				"Tokenizer",
				visual::SyntaxError::UnclosedBracket,
			);
		}
		
		println!("{:?}", tokens);
	}

	process::exit(0); // to make compiler stfu
}

struct Program {
	lines: Vec<Line>,
}

struct Line {
	content: Vec<Tokens>,
}

#[derive(Debug)]
enum Tokens {
	Number(i64),	// 0-9
	NumberHex(i64),	// 0x[0-9a-fA-F] / [0-9a-fA-F]h
	Word(String),	// A-Z
	String(String),	

	Underscore,		// _
	
	Plus, Minus, Asterisk, Equals, // + - * =

	BracketOpen, BracketClose,		// ()
	SqBracketOpen, SqBracketClose,	// []
	CuBracketOpen, CuBracketClose,	// {}

	Returns,	// >
	Colon,		// :
	Dot,		// .
	Comma,		// ,
	Hash,		// #
	Dollar,		// $
	Percent,	// %
	Unknown,
}