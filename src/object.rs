use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
	Nil,
	Str(String),
	Num(f64),
	Bool(bool),
	// Variable { name: String },
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Object::*;
        match self {
        	Nil => write!(f, "nil"),
        	Str(s) => write!(f, "\"{}\"", s),
        	Num(n) => write!(f, "{}", n),
        	Bool(b) => write!(f, "{}", b),
        	// Variable { name } => write!(f, "Var({})", name),
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
