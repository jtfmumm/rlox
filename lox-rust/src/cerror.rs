use crate::token::{Token, TokenType};

use std::error::Error;
use std::fmt;

pub fn scerror(line_n: u32, msg: &str) -> ScannerError {
	report(line_n, "", msg);
	ScannerError::new(line_n, msg)
}

pub fn perror(line_n: u32, token: Token, msg: &str) -> ParseError {
	let lexeme = token.lexeme.to_owned();
	let location = match token.ttype {
		TokenType::Eof => "end".to_string(),
		_ => format!("'{:}'", lexeme)
	};
	report(line_n, &location, msg);
	ParseError::new(line_n, &location, msg)
	// msg.to_string()
}

pub fn everror(msg: &str) -> EvalError {
	report(0, "", msg);
	EvalError::new(msg)
}

pub fn report(line_n: u32, location: &str, msg: &str) {
	eprintln!("[Line: {:}] Error {:}: {:}", line_n, location, msg);
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
	msg: String
}

impl ParseError {
	pub fn new(line: u32, location: &str, msg: &str) -> Self {
		ParseError { line, location: location.to_string(), msg: msg.to_string() }
	}
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Line {:}] Parse error at {:}: {:}", self.line, &self.location, &self.msg)
    }
}

#[derive(Debug)]
pub struct EvalError {
	msg: String
}

impl EvalError {
	pub fn new(msg: &str) -> Self {
		EvalError { msg: msg.to_string() }
	}
}

impl Error for EvalError {}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Evaluation error: {:}", &self.msg)
    }
}
