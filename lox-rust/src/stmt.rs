use crate::cerror::EvalError;
use crate::environment::Environment;
use crate::expr::Expr;
use crate::object::{Object, stringify_cli_result};

use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub enum Stmt {
	VarDeclStmt { variable: Rc<Expr>, value: Rc<Expr> },
	AssignStmt { variable: Rc<Expr>, value: Rc<Expr> },
	ExprStmt { expr: Rc<Expr> },
	PrintStmt { expr: Rc<Expr> },
	BlockStmt { stmts: Rc<Vec<Rc<Stmt>>> }
}

impl Stmt {
	pub fn declare(variable: Rc<Expr>, value: Rc<Expr>) -> Rc<Stmt> {
		Rc::new(Stmt::VarDeclStmt { variable, value })
	}

	pub fn assign(variable: Rc<Expr>, value: Rc<Expr>) -> Rc<Stmt> {
		Rc::new(Stmt::AssignStmt { variable, value })
	}

	pub fn expr(expr: Rc<Expr>) -> Rc<Stmt> {
		Rc::new(Stmt::ExprStmt { expr })
	}

	pub fn print(expr: Rc<Expr>) -> Rc<Stmt> {
		Rc::new(Stmt::PrintStmt { expr })
	}

	pub fn block(stmts: Rc<Vec<Rc<Stmt>>>) -> Rc<Stmt> {
		Rc::new(Stmt::BlockStmt { stmts })
	}
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use Stmt::*;

		let s = match *self {
			VarDeclStmt { ref variable, ref value } => "var ".to_string() + &variable.to_string() + " = " + &value.to_string(),
			AssignStmt { ref variable, ref value } => variable.to_string() + " = " + &value.to_string(),
			ExprStmt { ref expr } => expr.to_string(),
			PrintStmt { ref expr } => expr.to_string(),
			BlockStmt { ref stmts } => {
				let mut s = "".to_string();
				for stmt in stmts.iter() {
					s += &stmt.to_string();
					s += ";\n";
				}
				s
			}
		};
		write!(f, "{}", s)
	}
}
