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
	let mut program = Program(Vec::new());

	for line in &content {
		let mut ch_counter: u32 = 0;
		line_counter += 1;

		// Symbol stack to identify unclosed brackets
		let mut symbol_stack: Vec<char> = Vec::new();

		// Line containing tokens
		let mut tokens: Vec<Tokens> = Vec::new();
		let mut expect_token = false;

		let mut word = String::new();

		let mut number = String::new();
		let mut hex_number = String::new();
		let mut float = String::new();

		let mut in_number = false;  // if char=0-9
		let mut expect_hex = false;	// if char=0 -> expect `x`
		let mut in_float = false;   // if char=. & in_number
		let mut in_hex = false;

		let mut string = String::new();
		let mut in_string: bool = false;
		
		let mut in_comment: bool = false;

		for ch in line {
			ch_counter += 1;

			if *ch == '/' {
				if let Some('/') = symbol_stack.last() {
					symbol_stack.pop();
					in_comment = true;
					break;
				}
				else {
					symbol_stack.push(*ch);
				}
				continue;
			}

			if *ch == '\'' {
				if expect_token { expect_token = false; }
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
					break;
				},
				',' => {
					if in_hex && !hex_number.is_empty() {
						if let Ok(hex_val) = i64::from_str_radix(&hex_number, 16) {
							tokens.push(Tokens::NumberHex(hex_val));
							expect_token = false;
						}
						hex_number.clear();
						in_hex = false;
						number.clear();
					}

					expect_token = true;
					in_hex = false;

					tokens.push(Tokens::Comma);
					continue;
				},
				'(' => {
					symbol_stack.push(*ch);
					tokens.push(Tokens::BracketOpen);
					expect_token = false;
					continue;
				}
				')' => {
					if in_hex && !hex_number.is_empty() {
						if let Ok(hex_val) = i64::from_str_radix(&hex_number, 16) {
							tokens.push(Tokens::NumberHex(hex_val));
							expect_token = false;
						}
						hex_number.clear();
						in_hex = false;
						number.clear();
					}

					in_hex = false;
					if expect_token {
						visual::report(
							line,
							line_counter,
							ch_counter-1,
							"Tokenizer",
							SyntaxError::UnexpectedToken,
						);
					}

					if let Some('(') = symbol_stack.last() {
						symbol_stack.pop();
						tokens.push(Tokens::BracketClose);
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

				'[' => {
					symbol_stack.push(*ch);
					tokens.push(Tokens::SqBracketOpen);
					expect_token = false;
					continue;
				},
				']' => {
					if in_hex && !hex_number.is_empty() {
						if let Ok(hex_val) = i64::from_str_radix(&hex_number, 16) {
							tokens.push(Tokens::NumberHex(hex_val));
							expect_token = false;
						}
						hex_number.clear();
						in_hex = false;
						number.clear();
					}

					in_hex = false;

					if expect_token {
						visual::report(
							line,
							line_counter,
							ch_counter-1,
							"Tokenizer",
							SyntaxError::UnexpectedToken,
						);
					}

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

				'{' => {
					symbol_stack.push(*ch);
					tokens.push(Tokens::CuBracketOpen);
					expect_token = false;
					continue;
				},
				'}' => {
					if in_hex && !hex_number.is_empty() {
						if let Ok(hex_val) = i64::from_str_radix(&hex_number, 16) {
							tokens.push(Tokens::NumberHex(hex_val));
							expect_token = false;
						}
						hex_number.clear();
						in_hex = false;
						number.clear();
					}

					in_hex = false;

					if expect_token {
						visual::report(
							line,
							line_counter,
							ch_counter-1,
							"Tokenizer",
							SyntaxError::UnclosedBracket,
						);
					}

					if let Some('{') = symbol_stack.last() {
						symbol_stack.pop();
						tokens.push(Tokens::CuBracketClose);
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
				'0' => {
					if !in_hex {
						number.push(*ch);
					}
					expect_hex = true;
					continue;
				},
				'x' if expect_hex => {
					in_hex = true;
					expect_hex = false;
					continue;
				},
				('0'..='9' | 'a'..='f' | 'A'..='F') if in_hex => {
					hex_number.push(*ch);
					continue;
				},
				('g'..='z' | 'G'..='Z') if in_hex => {
					visual::report(
						line,
						line_counter,
						ch_counter,
						"Tokenizer",
						SyntaxError::UnexpectedToken,
					);
				},
				_ => {
					if in_hex && !hex_number.is_empty() {
						if let Ok(hex_val) = i64::from_str_radix(&hex_number, 16) {
							tokens.push(Tokens::NumberHex(hex_val));
							expect_token = false;
						}
						hex_number.clear();
						in_hex = false;
						number.clear();
						continue;
					}
				},
			}
		}

		if in_hex && !hex_number.is_empty() {
			if let Ok(hex_val) = i64::from_str_radix(&hex_number, 16) {
				tokens.push(Tokens::NumberHex(hex_val));
				expect_token = false;
			}
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
				SyntaxError::UnclosedBracket,
			);
		}

		if expect_token {
			visual::report(
				line,
				line_counter,
				ch_counter,
				"Tokenizer",
				SyntaxError::UnexpectedToken,
			);
		}
		
		println!("{:?}", tokens);
		program.0.push(tokens);
	}
	// println!("{:#?}", program);

	process::exit(0); // to make compiler stfu
}

#[derive(Debug)]
struct Program(Vec<Vec<Tokens>>);

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

impl Tokens {
	fn identify(ch: char) -> Option<Self> {
		use Tokens::*;
		match ch {
			'_' => Some(Underscore),
			'+' => Some(Plus),
			'-' => Some(Minus),
			'*' => Some(Asterisk),
			'=' => Some(Equals),
			'(' => Some(BracketOpen),
			')' => Some(BracketClose),
			'[' => Some(SqBracketOpen),
			']' => Some(SqBracketClose),
			'{' => Some(CuBracketOpen),
			'}' => Some(CuBracketClose),
			'>' => Some(Returns),
			':' => Some(Colon),
			'.' => Some(Dot),
			',' => Some(Comma),
			'#' => Some(Hash),
			'$' => Some(Dollar),
			'%' => Some(Percent),
			_ => None,
		}
	}
}