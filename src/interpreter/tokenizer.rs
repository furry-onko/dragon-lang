#![allow(unused)]
use crate::file;
use std::process;

pub fn start(path: &str) {
	let file_content: Vec<Vec<char>> = file::read_file_vec(path);
}

fn tokenize(content: Vec<Vec<char>>) -> Program {
	for line in content {
		continue;
	}

	process::exit(0); // to make compiler stfu
}

struct Program {
	lines: Vec<Line>,
}

struct Line {
	content: Vec<Tokens>,
}

enum Tokens {
	Number(i32),	// 0-9
	Word(String),	// A-Z
	
	Underscore,		// _
	
	Plus, Minus, Slash, Asterisk, Equals, // + - / * =

	BracketOpen, BracketClose,		// ()
	SqBracketOpen, SqBracketClose,	// []
	CuBracketOpen, CuBracketClose,	// {}

	Returns,	// >
	Colon,		// :
	Quote,		// '
	Dot,		// .
	Comma,		// ,
	Hash,		// #
	Dollar,		// $
	Percent,	// %
}