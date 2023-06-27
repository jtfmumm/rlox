use crate::token::{Token, TokenType};

use std::error::Error;
use std::fmt;

pub fn scerror(line_n: u32, msg: &str) -> ScannerError {
	report(line_n, "", msg);
	ScannerError::new(line_n, msg)
}

pub fn perror(token: Token, msg: &str) -> ParseError {
	let location = location_for(&token);
	let line_n = token.line;
	report(line_n, &location, msg);
	ParseError::new(line_n, &location, token, msg)
}

pub fn report(line_n: u32, location: &str, msg: &str) {
	eprintln!("\x1b[1;31m>>\x1b[0m[Line: {:}] {}: {:}", line_n, location, msg);
}

fn location_for(token: &Token) -> String {
	let lexeme = token.lexeme.to_owned();
	let location = match token.ttype {
		TokenType::Eof => "end".to_string(),
		_ => format!("'{:}'", lexeme)
	};
	location
}

#[derive(Debug)]
pub struct ScannerError {
	line: u32,
	msg: String
}

impl ScannerError {
	pub fn new(line: u32, msg: &str) -> Self {
		ScannerError { line, msg: msg.to_string() }
	}
}

impl Error for ScannerError {}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Line {:}] Scanner error: {:}", self.line, &self.msg)
    }
}


#[derive(Debug)]
pub struct ParseError {
	line: u32,
	location: String,
	token: Token,
	msg: String
}

impl ParseError {
	pub fn new(line: u32, location: &str, token: Token, msg: &str) -> Self {
		ParseError { line, location: location.to_string(), token, msg: msg.to_string() }
	}
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Line {:}] Parse error at {:}: {:}", self.line, &self.location, &self.msg)
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
