/* 2026 - @furry_onko */
#![allow(unused)]
use colored::Colorize;
use std::io::{self, Write};
use std::process;

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

pub fn report(
	context: &[char],	// Line
	line: u32,			// Line number
	ch: u32,			// Character number
	at: &str,			// Context (section > label)
	error: SyntaxError,	// Error type
) {
	use SyntaxError::*;
	let context: &[char] =
		if context.is_empty() { &[] }
		else if context[context.len() - 1] == '\n' {
			&context[..context.len() - 1]
		}
		else {context};

	println!(
		"{}: {}",
		"Error".red(),
		format!("{:?}", error).bright_yellow()
	);

	println!(
		"   {}: {}\n",
		"At".red(),
		at.bright_blue()
	);

	let context_format: String = context.iter().collect();
	println!("{} | {}", line, context_format);

	let line_number_len: u32 = (line.to_string().len() as u32) + 3;

	println!(
		"{}^ {}\n",
		" ".repeat((ch + line_number_len - 1) as usize),
		error.get_fix()
	);

	println!(
		"{} {} {}",
		"Type".bright_blue(),
		"dl man error [error name]".bright_magenta(),
		"to get help".bright_blue()
	);

	process::exit(1);
}

#[derive(Debug)]
pub enum SyntaxError {
	UnclosedBracket,		// Unclosed bracket
	NegativeReturn,			// `ret` called but no previous context was given
	JumpTargetNotFound,		// jump to non-existent label / section
	InclTargetNotFound,		// `incl` instruction contains bad path
	InvalidUse,				// `use` instruction points to invalid location
	NameClash,				// two or more names are the same
	IncompatibleValue,		// return value or move value is incompatible
	IncorrectCast,			// value casted is incorrect (like string to char)
	ZeroDivision,			// dividing by zero
	VariableNotFound,		// trying to access non-existent variable
	IncorrectOperands,		// trying to execute an instruction with incorrect operands (like `rsh` on a boolean)
	UnexpectedToken,		// unexpected token (like a comma)
	IndexOutOfBounds,		// trying to index a value out of bounds
	IncompatibleIndex,		// trying to index an incompatible value
}
impl SyntaxError {
	fn get_fix(&self) -> String {
		use SyntaxError::*;

		match self {
			UnclosedBracket 	=> String::from("Replace the opening token with a corresponding token."),
			NegativeReturn  	=> String::from("Remove the `ret` instruction."),
			JumpTargetNotFound	=> String::from("Change the jump target to existing one or remove the instruction."),
			InclTargetNotFound	=> String::from("Check the `incl` target."),
			InvalidUse			=> String::from("Check the `use` instruction target."),
			NameClash			=> String::from("Change the names of clashing elements."),
			IncompatibleValue	=> String::from("Perform a cast or convert a value."),
			IncorrectCast		=> String::from("Perform a conversion instead of casting."),
			ZeroDivision		=> String::from("Do a check or use a `divz` instruction."),
			VariableNotFound	=> String::from("Create a variable with that name."),
			IncorrectOperands	=> String::from("Perform a cast or convert a value."),
			UnexpectedToken		=> String::from("Remove or replace this token."),
			IndexOutOfBounds	=> String::from("Do a bounds check before indexing."),
			IncompatibleIndex	=> String::from("Remove this index statement."),
		}
	}

	fn explain(&self) -> String {
		use SyntaxError::*;

		match self {
			UnclosedBracket		=> String::from("You have an unclosed bracket or string quote."),
			NegativeReturn		=> String::from("You have a `ret` instruction in the starting location."),
			JumpTargetNotFound	=> String::from("Can't jump to a non-existing label."),
			InclTargetNotFound	=> String::from("Can't include a non-existing header."),
			InvalidUse			=> String::from("You are trying to bring a non-existing target into the scope."),
			NameClash			=> String::from("There are 2 or more of the same names in your program."),
			IncompatibleValue	=> String::from("You are trying to move an incompatible value to a target."),
			IncorrectCast		=> String::from("Some values cannot be casted."),
			ZeroDivision		=> String::from("Division by zero is not allowed. You can use `divz` to make division by zero to return it."),
			VariableNotFound	=> String::from("You are trying to use a variable that does not exist"),
			IncorrectOperands	=> String::from("You provided incorrect operands for an operation or the amount of operands is incorrect.0"),
			UnexpectedToken		=> String::from("You have a token that breaks the syntax rules."),
			IndexOutOfBounds	=> String::from("You are trying to access a value out of bounds."),
			IncompatibleIndex	=> String::from("You are trying to index an incompatible value."),
		}
	}
}