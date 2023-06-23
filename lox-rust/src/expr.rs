///////////////////////
// This file is 
// auto-generated code
///////////////////////

use crate::token::Token;
use std::rc::Rc;

pub enum Expr {
	Binary { left: Rc<Expr>, operator: Token, right: Rc<Expr> },
	Grouping { expression: Rc<Expr> },
	Literal { value: String },
	Unary { operator: Token, right: Rc<Expr> },
}

impl Expr {
	fn binary(left: Rc<Expr>, operator: Token, right: Rc<Expr>) -> Rc<Expr> {
		Rc::new(Expr::Binary { left, operator, right })
	}

	fn grouping(expression: Rc<Expr>) -> Rc<Expr> {
		Rc::new(Expr::Grouping { expression })
	}

	fn literal(value: String) -> Rc<Expr> {
		Rc::new(Expr::Literal { value })
	}

	fn unary(operator: Token, right: Rc<Expr>) -> Rc<Expr> {
		Rc::new(Expr::Unary { operator, right })
	}

	// Visitor methods
	fn parens(left: String, right: String) -> String {
		format!("({:?} {:?} )", left, right)
	}

	fn to_string(&self) -> String {
		use Expr::*;

		match *self {
			Binary { ref left, ref operator, ref right } => {
				operator.to_string() + &Expr::parens(left.to_string(), right.to_string())
			},
			Grouping { ref expression } => expression.to_string(),
			Literal { ref value } => value.to_owned(),
			Unary { ref operator, ref right } => {
				Expr::parens(operator.to_string(), right.to_string())
			},
		}
	}
}