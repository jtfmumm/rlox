use crate::object::Object;
use crate::token::{Token, TokenType};

use std::error::Error;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub enum LoxError {
    Parse,
    Runtime,
    Scan,
}

impl Error for LoxError {}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn scerror(line_n: u32, msg: &str) -> ScanError {
    eprintln!("[line {}] Error: {}\n", line_n, msg);
    ScanError::new(msg)
}

pub fn perror(token: Token, msg: &str) -> ParseError {
    let location = location_for(&token);
    let line_n = token.line;
    eprintln!("[line {}] Error {}: {}\n", line_n, location, msg);
    ParseError::new(msg)
}

fn location_for(token: &Token) -> String {
    let lexeme = token.lexeme.to_owned();
    match token.ttype {
        TokenType::Eof => "at end".to_string(),
        _ => format!("at '{:}'", lexeme),
    }
}

#[derive(Debug)]
pub struct ScanError {
    pub msg: String,
}

impl ScanError {
    pub fn new(msg: &str) -> Self {
        ScanError {
            msg: msg.to_string(),
        }
    }
}

impl Error for ScanError {}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.msg)
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub msg: String,
}

impl ParseError {
    pub fn new(msg: &str) -> Self {
        ParseError {
            msg: msg.to_string(),
        }
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
    Runtime(String),
}

impl EvalError {
    pub fn new(msg: &str) -> Self {
        EvalError::Runtime(msg.to_string())
    }

    pub fn new_with_context(token: Token, expr_str: &str, msg: &str) -> Self {
        let location = expr_str.to_string();
        let msg = format!("[line {}] Error at {}: {}", token.line, location, msg);
        EvalError::Runtime(msg)
    }

    pub fn new_return(obj: Rc<Object>) -> Self {
        EvalError::Return(obj)
    }

    pub fn with_context(self, token: Token, expr_str: &str) -> Self {
        match self {
            EvalError::Return(_) => self,
            EvalError::Runtime(old_msg) => {
                let location = expr_str.to_string();
                let msg = old_msg + &format!("\n[line {}] Error at {}", token.line, location);
                EvalError::Runtime(msg)
            }
        }
    }

    pub fn report(&self) {
        match self {
            EvalError::Return(_) => {}
            EvalError::Runtime(msg) => eprintln!("{}\n", msg),
        }
    }
}

impl Error for EvalError {}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvalError::Return(obj) => write!(f, "Return {}", obj),
            EvalError::Runtime(msg) => write!(f, "{}", msg),
        }
    }
}
