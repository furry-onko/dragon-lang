#![allow(unused)]
use crate::file;
use crate::visual::{self, SyntaxError};
use std::process;

pub fn start(path: &str) {
	let file_content: Vec<String> = file::read_file(&format!("{path}/src/main.drg"));
	tokenize(file_content);
}
/*
A - A-Z a-z
1 - 0-9
X - Variable / Register
P - Parameter
0 - New Line
C - Constant
T - Type

- `,` A 1 [ { ( ' X C P *
- `.` A 1
- `{` A 1 [ { } X C 
- `}` 0
- `(` P X 1 C ) T
- `)` 0 :
- `[` ' 1 C X ]
- `]` , 0
- `>` X [
- `_` A 9
- `=` C [ X * '
- `:` ( 0 T
- `'`(begin) everything
- `'`(close) , } ] )
- `%` (define, macro, endmacro)
- `//`, `**`, `;` everything
*/

fn tokenize(content: Vec<String>) -> Program {
	let mut line = Line(vec![]);
	for (line_num, line) in content.iter().enumerate() {
		line_check(line, line_num);
	}

	process::exit(0);
}

fn line_check(line: &str, line_num: usize) -> Option<Line> {
	use State::*;
	use visual::SyntaxError::*;

	let mut state = Default;
	let mut expect_next = Default;

	let mut line_ret = Line(Vec::new());
	let mut symbol_stack = Vec::<Token>::new();
	let mut string_buffer = String::new();

	let mut cnum: usize = 0;

	for (ch_num, ch) in line.chars().enumerate() {
		cnum = ch_num;

		if ch == ';' {
			if state != InString {
				if symbol_stack.len() > 0 {
					visual::report(
						line,
						line_num,
						ch_num,
						"Tokenizer",
						UnclosedBracket,
					);
				}
				return Some(line_ret);
			}
		}

		match ch {
			'.' => line_ret.0.push(Token::Dot),
			'>' => line_ret.0.push(Token::Returns),
			':' => line_ret.0.push(Token::Colon),
			',' => line_ret.0.push(Token::Comma),
			'#' => line_ret.0.push(Token::Hash),
			'$' => line_ret.0.push(Token::Dollar),
			'%' => line_ret.0.push(Token::Percent),
			'+' => line_ret.0.push(Token::Plus),		
			'-' => line_ret.0.push(Token::Minus),
			'*' => line_ret.0.push(Token::Asterisk),
			'=' => line_ret.0.push(Token::Equals),
			'_' => line_ret.0.push(Token::Underscore),
			_ => (),
		}

		if ch == '\'' {
			match state {
				Default => {
					state = InString;
					symbol_stack.push(Token::Quote);
					continue;
				},
				InString => {
					line_ret.0.push(
						Token::String(
							std::mem::take(&mut string_buffer)
						)
					);
					state = Default;
					symbol_stack.pop();
					continue;
				},
				_ => {
					visual::report(
						line,
						line_num,
						ch_num,
						"Tokenizer",
						UnexpectedToken,
					);
				}
			}
		}

		if ch.is_ascii_alphabetic() {
			match state {
				InString => {
					string_buffer.push(ch);
					continue;
				}

				Default => {

				}

				_ => ()
			}
		}

		match state {
			InString => {
				string_buffer.push(ch);
			}
			
			_ => ()
		}
	}

	if symbol_stack.len() > 0 {
		visual::report(
			line,
			line_num,
			cnum,
			"Tokenizer",
			UnclosedBracket,
		);
	}

	println!("{:#?}", line_ret);

	Some(line_ret)
}

#[derive(PartialEq)]
enum State {
    Default,
    InNumber,
    InHex,
    InWord,
    InString,
}

#[derive(Debug)]
struct Program(Vec<Line>);

#[derive(Debug)]
struct Line(Vec<Token>);

#[derive(Debug)]
enum Token {
	Number(i64),	// 0-9
	NumberHex(i64),	// 0x[0-9a-fA-F] / [0-9a-fA-F]h
	Float(f64),		// 0.[0-9]+
	Word(String),	// A-Z
	String(String),	

	Underscore,		// _
	
	Plus, Minus, Asterisk, Equals, // + - * =

	BracketOpen, BracketClose,		// ()
	SqBracketOpen, SqBracketClose,	// []
	CuBracketOpen, CuBracketClose,	// {}

	Quote,		// '
	Returns,	// >
	Colon,		// :
	Dot,		// .
	Comma,		// ,
	Hash,		// #
	Dollar,		// $
	Percent,	// %
	Unknown(char),
}

impl Token {
	fn identify(ch: char) -> Option<Self> {
		use Token::*;
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

/*
to modify !

	// Lines iter:
	for (ln_idx, line) in content.iter().enumerate() {
		let mut state = Default;
		let mut buffer = String::new();
		let mut symbol_stack: Vec<char> = Vec::new();
		let mut tokens: Vec<Tokens> = Vec::new();

		// Char iter:
		for (ch_idx, ch) in line.chars().enumerate() {
			match state {
				Default => {
					if ch.is_ascii_digit() { // 0-9
						buffer.push(ch);
						state = InNumber;
					}
					else if ch.is_ascii_alphabetic() { // a-z A-Z
						buffer.push(ch);
						state = InWord;
					}
					else if ch == '\'' {
						state = InString;
					}
					else if let Some(token) = 
				}
			}
		}
	}
*/