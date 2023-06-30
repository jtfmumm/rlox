use crate::callable::Callable;

use std::fmt;
use std::rc::Rc;

// #[derive(Debug, Clone, PartialEq)]
#[derive(Debug)]
pub enum Object {
	Nil,
	Str(String),
	Num(f64),
	Bool(bool),
	Fun(Rc<dyn Callable>),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Object::*;
        match self {
        	Nil => write!(f, "nil"),
        	Str(s) => write!(f, "{}", s),
        	Num(n) => write!(f, "{}", n),
        	Bool(b) => write!(f, "{}", b),
        	Fun(fun) => write!(f, "{:?}", fun),
        }
    }
}

pub fn stringify_cli_result(obj: &Object) -> String {
	let s = format!("{}", obj);
	if s.ends_with(".0") {
		s[0..s.len() - 2].to_string()
	} else {
		s
	}
}
