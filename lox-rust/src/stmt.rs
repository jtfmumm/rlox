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
	// ConditionalStmt { condition: Rc<Expr>, stmt: Rc<Stmt> },
	// IfStmt { conditionals: Rc<Vec<Rc<ConditionalStmt>>>, else_stmt: Rc<Stmt> },
	IfStmt { conditionals: Rc<Vec<(Rc<Expr>, Rc<Stmt>)>>,
			 else_block: Rc<Option<Rc<Stmt>>>
		   },
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

	// pub fn conditional(condition: Rc<Expr>, stmt: Rc<Expr>) -> Rc<Stmt> {
	// 	Rc::new(Stmt::ConditionalStmt { condition, stmt })
	// }

	// pub fn if(conditions: Rc<Vec<Rc<ConditionalStmt>>>, else_stmt: Rc<Stmt>) -> Rc<Stmt> {
	// 	Rc::new(Stmt::IfStmt { conditions, else_stmt })
	// }

	pub fn ifstmt(conditionals: Rc<Vec<(Rc<Expr>, Rc<Stmt>)>>,
			  else_block: Rc<Option<Rc<Stmt>>>) -> Rc<Stmt> {
		Rc::new(Stmt::IfStmt { conditionals, else_block })
	}

	pub fn block(stmts: Rc<Vec<Rc<Stmt>>>) -> Rc<Stmt> {
		Rc::new(Stmt::BlockStmt { stmts })
	}
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use Stmt::*;

		let s = match *self {
			VarDeclStmt { ref variable, ref value } => {
				"var ".to_string() + &variable.to_string() + " = " + &value.to_string()
			},
			AssignStmt { ref variable, ref value } => {
				variable.to_string() + " = " + &value.to_string()
			},
			ExprStmt { ref expr } => expr.to_string(),
			PrintStmt { ref expr } => expr.to_string(),
			// ConditionalStmt { ref condition, ref stmt } => {
			// 	"if ".to_string() + &condition.to_string() + " then " + &stmt.to_string()
			// },
			// IfStmt { ref conditions, ref else_stmt } => {
			// 	let mut s = "".to_string();
			// 	for c in conditions.iter() {
			// 		s += &c.to_string();
			// 		s += ",\n";
			// 	}
			// 	s += "else ";
			// 	s += &else_stmt.to_string();
			// 	s
			// }
			IfStmt { ref conditionals, ref else_block } => {
				let mut s = "".to_string();
				for (c, blk) in conditionals.iter() {
					s += "if ";
					s += &c.to_string();
					s += " then ";
					s += &blk.to_string();
					s += ",\n";
				}
				if let Some(block) = &*else_block.clone() {
					s += "else ";
					s += &block.to_string();
				}
				s
			}
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
