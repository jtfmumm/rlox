use crate::object::Object;
use crate::token::{Token, TokenType};

use std::error::Error;
use std::fmt;
use std::rc::Rc;

pub enum LoxError {
	Parse,
	Runtime,
	Scan,
}

pub fn scerror(line_n: u32, msg: &str) -> ScannerError {
	eprintln!("[line {}] Error: {}\n", line_n, msg);
	ScannerError::new(msg)
}

pub fn perror(token: Token, msg: &str) -> ParseError {
	let location = location_for(&token);
	let line_n = token.line;
	preport(line_n, &location, msg);
	ParseError::new(msg)
}

pub fn preport(line_n: u32, location: &str, msg: &str) {
	eprintln!("[line {}] Error {}: {}\n", line_n, location, msg);
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
	msg: String
}

impl ParseError {
	pub fn new(msg: &str) -> Self {
		ParseError { msg: msg.to_string() }
	}
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.msg)
    }
}

#[derive(Debug)]
pub enum EvalError {
	Return(Rc<Object>),
	Fail(String)
}

impl EvalError {
	pub fn new(msg: &str) -> Self {
		EvalError::Fail(msg.to_string())
	}

	pub fn new_with_context(token: Token, expr_str: &str, msg: &str) -> Self {
		let location = expr_str.to_string();
		let msg = format!("[line {}] Error at {}: {}", token.line, location, msg);
		EvalError::Fail(msg)
	}

	pub fn new_return(obj: Rc<Object>) -> Self {
		EvalError::Return(obj)
	}

	pub fn with_context(self, token: Token, expr_str: &str) -> Self {
		match self {
			EvalError::Return(_) => self,
			EvalError::Fail(old_msg) => {
				let location = expr_str.to_string();
				let msg = old_msg.clone() + &format!("\n[line {}] Error at {}", token.line, location);
				EvalError::Fail(msg)
			}
		}
	}

	pub fn report(&self) {
		match self {
			EvalError::Return(_) => {},
			EvalError::Fail(msg) => {
				eprintln!("{}\n", msg);
			}
		}
	}
}

impl Error for EvalError {}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			EvalError::Return(obj) => write!(f, "Return {}", obj),
			EvalError::Fail(msg) => write!(f, "{}", msg),
		}
    }
}
