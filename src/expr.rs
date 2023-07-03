///////////////////////
// This file is 
// auto-generated code
///////////////////////

use crate::object::Object;
use crate::token::Token;

use std::fmt;
use std::rc::Rc;
	
#[derive(Debug)]
pub enum Expr {
	Assign { variable: Rc<Expr>, value: Rc<Expr> },
	Binary { left: Rc<Expr>, operator: Token, right: Rc<Expr> },
	Call { callee: Rc<Expr>, paren: Token, args: Rc<Vec<Rc<Expr>>> },
	Grouping { expression: Rc<Expr> },
	Literal { value: Rc<Object> },
	Logic { left: Rc<Expr>, operator: Token, right: Rc<Expr> },
	Unary { operator: Token, right: Rc<Expr> },
	Variable { name: Token, depth: Rc<Option<u32>> },
}

impl Expr {
	pub fn assign(variable: Rc<Expr>, value: Rc<Expr>) -> Rc<Expr> {
		Rc::new(Expr::Assign { variable, value })
	}

	pub fn binary(left: Rc<Expr>, operator: Token, right: Rc<Expr>) -> Rc<Expr> {
		Rc::new(Expr::Binary { left, operator, right })
	}

	pub fn call(callee: Rc<Expr>, paren: Token, args: Rc<Vec<Rc<Expr>>>) -> Rc<Expr> {
		Rc::new(Expr::Call { callee, paren, args })
	}

	pub fn grouping(expression: Rc<Expr>) -> Rc<Expr> {
		Rc::new(Expr::Grouping { expression })
	}

	pub fn literal(value: Rc<Object>) -> Rc<Expr> {
		Rc::new(Expr::Literal { value })
	}

	pub fn logic(left: Rc<Expr>, operator: Token, right: Rc<Expr>) -> Rc<Expr> {
		Rc::new(Expr::Logic { left, operator, right })
	}

	pub fn unary(operator: Token, right: Rc<Expr>) -> Rc<Expr> {
		Rc::new(Expr::Unary { operator, right })
	}

	pub fn variable(name: Token, depth: Rc<Option<u32>>) -> Rc<Expr> {
		Rc::new(Expr::Variable { name, depth })
	}

}

impl fmt::Display for Expr{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

		let s = format!("{:?}", self);
		write!(f, "{}", s)
	}
}
	