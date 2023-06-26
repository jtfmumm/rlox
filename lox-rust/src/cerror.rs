use crate::token::{Token, TokenType};

use std::error::Error;
use std::fmt;

pub fn scerror(line_n: u32, msg: &str) -> ScannerError {
	report(line_n, "", msg);
	ScannerError::new(line_n, msg)
}

pub fn perror(line_n: u32, token: Token, msg: &str) -> ParseError {
	// let lexeme = token.lexeme.to_owned();
	// let location = match token.ttype {
	// 	TokenType::Eof => "end".to_string(),
	// 	_ => format!("'{:}'", lexeme)
	// };
	let location = location_for(&token);
	report(line_n, &location, msg);
	ParseError::new(line_n, &location, msg)
	// msg.to_string()
}

pub fn everror(token: Token, msg: &str) -> EvalError {
	let updated =
// pub fn everror(msg: &str, token: Option<Token>) -> EvalError {
	// let lexeme = token.lexeme.to_owned();
	// let location = match token.ttype {
	// 	TokenType::Eof => "end".to_string(),
	// 	_ => format!("'{:}'", lexeme)
	// };
	report(0, "", msg);
	EvalError::new(msg)
}

pub fn report(line_n: u32, location: &str, msg: &str) {
	eprintln!("[Line: {:}] Error {:}: {:}", line_n, location, msg);
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

#[derive(Debug, PartialEq)]
pub enum EvalError {
	WithoutContext { msg: String },
	WithContext { line: u32, location: String, msg: String }
}
// #[derive(Debug)]
// pub struct EvalError {
// 	// err: Option<EvalError>,
// 	msg: String
// }

impl EvalError {
	pub fn new(msg: &str) -> Self {
		EvalError::WithoutContext { msg: msg.to_string() }
	}

	// HACK: Has a non-obvious side effect by reporting
	pub fn new_everror(token: Token, msg: &str) -> Self {
		let location = location_for(&token);
		report(token.line, &location, msg);
		EvalError::WithContext { line: token.line, location, msg: msg.to_string() }
	}

	// HACK: Has a non-obvious side effect by reporting if context is updated
	pub fn with_context(&self, token: Token) -> Self {
		match self {
			EvalError::WithoutContext { msg } => {
				EvalError::new_everror(token, msg)
			},
			EvalError::WithContext { line, location, msg } => {
				EvalError::WithContext { line: *line, location: location.clone(), msg: msg.clone() }
			}
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
