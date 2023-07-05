///////////////////////
// This file is 
// auto-generated code
///////////////////////

use crate::expr::Expr;
use crate::token::Token;
use std::rc::Rc;

use std::fmt;
	
#[derive(Debug)]
pub enum Stmt {
	BlockStmt { stmts: Box<Vec<Stmt>> },
	ExprStmt { expr: Expr },
	ForStmt { init: Option<Box<Stmt>>, condition: Option<Expr>, inc: Option<Expr>, block: Box<Stmt> },
	FunStmt { name: Token, params: Vec<Token>, body: Rc<Vec<Stmt>>, depth: Option<u32> },
	IfStmt { conditionals: Vec<(Expr,Box<Stmt>)>, else_block: Option<Box<Stmt>> },
	PrintStmt { expr: Expr },
	ReturnStmt { expr: Expr },
	VarDeclStmt { variable: Expr, value: Expr },
	WhileStmt { condition: Expr, block: Box<Stmt> },
}

impl Stmt {
	pub fn block_stmt(stmts: Box<Vec<Stmt>>) -> Stmt {
		Stmt::BlockStmt { stmts }
	}

	pub fn expr_stmt(expr: Expr) -> Stmt {
		Stmt::ExprStmt { expr }
	}

	pub fn for_stmt(init: Option<Box<Stmt>>, condition: Option<Expr>, inc: Option<Expr>, block: Box<Stmt>) -> Stmt {
		Stmt::ForStmt { init, condition, inc, block }
	}

	pub fn fun_stmt(name: Token, params: Vec<Token>, body: Rc<Vec<Stmt>>, depth: Option<u32>) -> Stmt {
		Stmt::FunStmt { name, params, body, depth }
	}

	pub fn if_stmt(conditionals: Vec<(Expr,Box<Stmt>)>, else_block: Option<Box<Stmt>>) -> Stmt {
		Stmt::IfStmt { conditionals, else_block }
	}

	pub fn print_stmt(expr: Expr) -> Stmt {
		Stmt::PrintStmt { expr }
	}

	pub fn return_stmt(expr: Expr) -> Stmt {
		Stmt::ReturnStmt { expr }
	}

	pub fn var_decl_stmt(variable: Expr, value: Expr) -> Stmt {
		Stmt::VarDeclStmt { variable, value }
	}

	pub fn while_stmt(condition: Expr, block: Box<Stmt>) -> Stmt {
		Stmt::WhileStmt { condition, block }
	}

}

impl fmt::Display for Stmt{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

		let s = format!("{:?}", self);
		write!(f, "{}", s)
	}
}
	