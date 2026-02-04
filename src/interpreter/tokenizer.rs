#![allow(unused)]
use crate::file;
use std::process;

pub fn start(path: &str) {
	let file_content: Vec<Vec<char>> = file::read_file_vec(&format!("{path}/src/main.drg"));
	tokenize(file_content);
}

fn tokenize(content: Vec<Vec<char>>) -> Program {
	for line in content {
		let mut tokens: Vec<Tokens> = Vec::new();
		let mut word = String::new();
		let mut number = String::new();
		let mut string = String::new();
		
		let mut in_string: bool = false;
		let mut symbol_stack: Vec<char> = Vec::new();

		for ch in line {
			if ch == '\'' {
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
				string.push(ch);
				continue;
			}

			if ch == '/' {
				if let Some('/') = symbol_stack.last() {
					symbol_stack.pop();
				}
				else {
					symbol_stack.push(ch);
				}
				continue;
			}

			match ch {
				';' => {continue;}
				'[' => {
					symbol_stack.push(ch);
					tokens.push(Tokens::SqBracketOpen);
					continue;
				}
				']' => {
					if let Some('[') = symbol_stack.last() {
						symbol_stack.pop();
						tokens.push(Tokens::SqBracketClose);
						continue;
					}
					else {
						todo!();
						/* REFACTORING NEEDED
							- loops from for to while
							- add an error reporter/handler (lines, error types etc)
						*/
					}
				},
				// '' => {}
				_ => (),
			}
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