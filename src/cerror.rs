use crate::token::{Token, TokenType};

use std::error::Error;
use std::fmt;

pub enum LoxError {
	Parse,
	Runtime,
	Scan,
}

pub fn scerror(line_n: u32, msg: &str) -> ScannerError {
	report(line_n, "Syntax error", msg);
	ScannerError::new(msg)
}

pub fn perror(token: Token, msg: &str) -> ParseError {
	let location = location_for(&token);
	let line_n = token.line;
	preport(line_n, &location, msg);
	// ParseError::new(line_n, &location, token, msg)
	ParseError::new(msg)
}

pub fn preport(line_n: u32, location: &str, msg: &str) {
	eprintln!("[line {}] Error {}: {}\n", line_n, location, msg);
	// eprintln!("\x1b[1;31m[{}] Error\x1b[0m{}: {}", line_n, location, msg);
}

pub fn report(line_n: u32, location: &str, msg: &str) {
	eprintln!("{}\n", msg);
	// eprintln!("{}\n\x1b[1;31m[line {:}] Error\x1b[0m {}", msg, line_n, location);
}

fn location_for(token: &Token) -> String {
	let lexeme = token.lexeme.to_owned();
	let location = match token.ttype {
		TokenType::Eof => "at end".to_string(),
		_ => format!("at '{:}'", lexeme)
	};
	location
}

#[derive(Debug)]
pub struct ScannerError {
	msg: String
}

impl ScannerError {
	pub fn new(msg: &str) -> Self {
		ScannerError { msg: msg.to_string() }
	}
}

impl Error for ScannerError {}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.msg)
    }
}


#[derive(Debug)]
pub struct ParseError {
	// line: u32,
	// location: String,
	// token: Token,
	msg: String
}

impl ParseError {
	pub fn new(/*line: u32, location: &str, token: Token,*/ msg: &str) -> Self {
		ParseError { msg: msg.to_string() }
	}
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.msg)
        // write!(f, "[line {:}] Parse error at {:}: {:}", self.line, &self.location, &self.msg)
    }
}

#[derive(Debug, PartialEq)]
pub enum EvalError {
	WithoutContext { msg: String },
	WithContext { line: u32, location: String, token: Token, msg: String }
}

impl EvalError {
	pub fn new(msg: &str) -> Self {
		EvalError::WithoutContext { msg: msg.to_string() }
	}

	// HACK: Has a non-obvious side effect by reporting
	pub fn new_everror(token: Token, expr_str: &str, msg: &str) -> Self {
		let location = expr_str.to_string();//location_for(&token);

		report(token.line, &location, msg);
		EvalError::WithContext { line: token.line, location, token, msg: msg.to_string() }
	}

	// HACK: Has a non-obvious side effect by reporting if context is updated through new_everror
	pub fn with_context(self, token: Token, expr_str: &str) -> Self {
		match self {
			EvalError::WithoutContext { msg } => {
				EvalError::new_everror(token, expr_str, &msg)
			},
			_ => self
		}
	}

	fn get_msg(&self) -> &str {
		match self {
			EvalError::WithoutContext { msg } => {
				&msg
			},
			EvalError::WithContext { msg, .. } => {
				&msg
			}
		}
	}
}

impl Error for EvalError {}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Evaluation error: {:}", self.get_msg())
    }
}
