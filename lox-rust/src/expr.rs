use crate::object::Object;
use crate::token::Token;

use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub enum Expr {
	Binary { left: Rc<Expr>, operator: Token, right: Rc<Expr> },
	Grouping { expression: Rc<Expr> },
	Literal { value: Object },
	Variable { name: String },
	Unary { operator: Token, right: Rc<Expr> },
}

impl Expr {
	pub fn binary(left: Rc<Expr>, operator: Token, right: Rc<Expr>) -> Rc<Expr> {
		Rc::new(Expr::Binary { left, operator, right })
	}

	pub fn grouping(expression: Rc<Expr>) -> Rc<Expr> {
		Rc::new(Expr::Grouping { expression })
	}

	pub fn literal(value: Object) -> Rc<Expr> {
		Rc::new(Expr::Literal { value })
	}

	pub fn variable(name: &str) -> Rc<Expr> {
		Rc::new(Expr::Variable { name: name.to_string() })
	}

	pub fn unary(operator: Token, right: Rc<Expr>) -> Rc<Expr> {
		Rc::new(Expr::Unary { operator, right })
	}

	fn parens(left: String, right: String) -> String {
		format!("({:} {:})", left, right)
	}
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use Expr::*;

		let s = match *self {
			Binary { ref left, ref operator, ref right } => {
				operator.to_string() + " " + &Expr::parens(left.to_string(), right.to_string())
			},
			Grouping { ref expression } => expression.to_string(),
			Literal { ref value } => value.to_string(),
			Variable { ref name } => name.to_string(),
			Unary { ref operator, ref right } => {
				Expr::parens(operator.to_string(), right.to_string())
			},
		};
		write!(f, "{}", s)
	}
}
