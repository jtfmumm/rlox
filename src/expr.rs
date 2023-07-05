///////////////////////
// This file is 
// auto-generated code
///////////////////////

use crate::object::Object;
use crate::token::Token;
use std::rc::Rc;

use std::fmt;
	
#[derive(Debug)]
pub enum Expr {
	Assign { variable: Box<Expr>, value: Box<Expr> },
	Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
	Call { callee: Rc<Expr>, paren: Token, args: Rc<Vec<Expr>> },
	Grouping { expression: Box<Expr> },
	Literal { value: Object },
	Logic { left: Box<Expr>, operator: Token, right: Box<Expr> },
	Unary { operator: Token, right: Box<Expr> },
	Variable { name: Token, depth: Option<u32> },
}

impl Expr {
	pub fn assign(variable: Box<Expr>, value: Box<Expr>) -> Expr {
		Expr::Assign { variable, value }
	}

	pub fn binary(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Expr {
		Expr::Binary { left, operator, right }
	}

	pub fn call(callee: Rc<Expr>, paren: Token, args: Rc<Vec<Expr>>) -> Expr {
		Expr::Call { callee, paren, args }
	}

	pub fn grouping(expression: Box<Expr>) -> Expr {
		Expr::Grouping { expression }
	}

	pub fn literal(value: Object) -> Expr {
		Expr::Literal { value }
	}

	pub fn logic(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Expr {
		Expr::Logic { left, operator, right }
	}

	pub fn unary(operator: Token, right: Box<Expr>) -> Expr {
		Expr::Unary { operator, right }
	}

	pub fn variable(name: Token, depth: Option<u32>) -> Expr {
		Expr::Variable { name, depth }
	}

}

impl fmt::Display for Expr{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

		let s = format!("{:?}", self);
		write!(f, "{}", s)
	}
}
	