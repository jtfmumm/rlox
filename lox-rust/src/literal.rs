use std::fmt;

#[derive(Debug, Clone)]
pub enum Literal {
	Nil,
	Str(String),
	Num(f64),
	Bool(bool),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Literal::*;
        match self {
        	Nil => write!(f, "nil"),
        	Str(s) => write!(f, "\"{}\"", s),
        	Num(n) => write!(f, "{}", n),
        	Bool(b) => write!(f, "{}", b)
        }
    }
}
