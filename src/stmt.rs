///////////////////////
// This file is 
// auto-generated code
///////////////////////

use crate::expr::Expr;
use crate::token::Token;

use std::fmt;
use std::rc::Rc;
	
#[derive(Debug)]
pub enum Stmt {
	BlockStmt { stmts: Rc<Vec<Rc<Stmt>>> },
	ExprStmt { expr: Rc<Expr> },
	ForStmt { init: Rc<Option<Rc<Stmt>>>, condition: Rc<Option<Rc<Expr>>>, inc: Rc<Option<Rc<Expr>>>, block: Rc<Stmt> },
	FunStmt { name: Token, params: Rc<Vec<Token>>, body: Rc<Stmt> },
	IfStmt { conditionals: Rc<Vec<(Rc<Expr>,Rc<Stmt>)>>, else_block: Rc<Option<Rc<Stmt>>> },
	PrintStmt { expr: Rc<Expr> },
	ReturnStmt { expr: Rc<Expr> },
	VarDeclStmt { variable: Rc<Expr>, value: Rc<Expr> },
	WhileStmt { condition: Rc<Expr>, block: Rc<Stmt> },
}

impl Stmt {
	pub fn block_stmt(stmts: Rc<Vec<Rc<Stmt>>>) -> Rc<Stmt> {
		Rc::new(Stmt::BlockStmt { stmts })
	}

	pub fn expr_stmt(expr: Rc<Expr>) -> Rc<Stmt> {
		Rc::new(Stmt::ExprStmt { expr })
	}

	pub fn for_stmt(init: Rc<Option<Rc<Stmt>>>, condition: Rc<Option<Rc<Expr>>>, inc: Rc<Option<Rc<Expr>>>, block: Rc<Stmt>) -> Rc<Stmt> {
		Rc::new(Stmt::ForStmt { init, condition, inc, block })
	}

	pub fn fun_stmt(name: Token, params: Rc<Vec<Token>>, body: Rc<Stmt>) -> Rc<Stmt> {
		Rc::new(Stmt::FunStmt { name, params, body })
	}

	pub fn if_stmt(conditionals: Rc<Vec<(Rc<Expr>,Rc<Stmt>)>>, else_block: Rc<Option<Rc<Stmt>>>) -> Rc<Stmt> {
		Rc::new(Stmt::IfStmt { conditionals, else_block })
	}

	pub fn print_stmt(expr: Rc<Expr>) -> Rc<Stmt> {
		Rc::new(Stmt::PrintStmt { expr })
	}

	pub fn return_stmt(expr: Rc<Expr>) -> Rc<Stmt> {
		Rc::new(Stmt::ReturnStmt { expr })
	}

	pub fn var_decl_stmt(variable: Rc<Expr>, value: Rc<Expr>) -> Rc<Stmt> {
		Rc::new(Stmt::VarDeclStmt { variable, value })
	}

	pub fn while_stmt(condition: Rc<Expr>, block: Rc<Stmt>) -> Rc<Stmt> {
		Rc::new(Stmt::WhileStmt { condition, block })
	}

}

impl fmt::Display for Stmt{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

		let s = format!("{:?}", self);
		write!(f, "{}", s)
	}
}
	