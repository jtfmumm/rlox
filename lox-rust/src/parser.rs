use crate::cerror::{perror, ParseError};
use crate::expr::Expr;
use crate::object::Object;
use crate::stmt::Stmt;
use crate::token::{Token, TokenType};

use std::iter::Peekable;
use std::rc::Rc;
use std::vec::IntoIter;

pub struct Parser {
	tokens: Peekable<IntoIter<Token>>,
	prev: Token,
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		let prev = Token::new(TokenType::Sof, "".to_string(), "".to_string(), 0);
		Parser { tokens: tokens.into_iter().peekable(), prev }
	}

	pub fn parse(&mut self) -> Result<Vec<Rc<Stmt>>, String> {//Result<Rc<Expr>, String> {
		let mut stmts = Vec::new();
		let mut failed = false;
		while self.tokens.peek().unwrap().ttype != TokenType::Eof {
			match self.statement() {
				Ok(stmt) => {
					stmts.push(stmt);
				},
				Err(_) => {
					failed = true;
					self.synchronize();
				}
			}
		}
		if failed {
			Err("Parsing failed!".to_string())
		} else {
			Ok(stmts)
		}
	}

	fn statement(&mut self) -> Result<Rc<Stmt>, ParseError> {
		if self.match_advance(&[TokenType::Var]) {
			self.var_statement()
		} else if self.match_advance(&[TokenType::Print]) {
			self.print_statement()
		} else if self.match_advance(&[TokenType::LeftBrace]) {
			self.block()
		} else if self.match_advance(&[TokenType::If]) {
			self.if_statement()
		} else if self.identifier_match() {
			self.assign_statement()
		} else {
			self.expr_statement()
		}
	}

	fn block(&mut self) -> Result<Rc<Stmt>, ParseError> {
		let mut stmts = Vec::new();
		let mut failed = false;

		while !self.match_advance(&[TokenType::RightBrace]) {
			match self.statement() {
				Ok(stmt) => {
					stmts.push(stmt);
				},
				Err(_) => {
					failed = true;
					self.synchronize();
				}
			}
		}
		if failed {
			Err(perror(self.peek_prev().clone(), "Error while parsing block."))
		} else {
			Ok(Stmt::block(Rc::new(stmts)))
		}
	}

	fn var_statement(&mut self) -> Result<Rc<Stmt>, ParseError> {
		if self.check_identifier() {
			let variable = self.expression()?;
			self.consume(TokenType::Equal, "Expect '=' for variable declaration.");
			let value = self.expression()?;
			self.advance_end_of_statement();
			Ok(Stmt::declare(variable, value))
		} else {
			Err(perror(self.tokens.peek().unwrap().clone(), "Expect identifier after var."))
		}

	}

	fn print_statement(&mut self) -> Result<Rc<Stmt>, ParseError> {
		let expr = self.expression()?;
		self.advance_end_of_statement();
		Ok(Stmt::print(expr))
	}

	fn if_statement(&mut self) -> Result<Rc<Stmt>, ParseError> {
		let mut conditionals = Vec::new();
		let mut else_block = None;
		loop {
			let conditional = self.expression()?;
			self.consume(TokenType::LeftBrace, "Expect blocks for conditional statements.");
			let blk = self.block()?;
			conditionals.push((conditional, blk));
			if !self.match_advance(&[TokenType::Elif]) { break }
		}
		if self.match_advance(&[TokenType::Else]) {
			self.consume(TokenType::LeftBrace, "Expect blocks for else statements.");
			let blk = self.block()?;
			else_block = Some(blk);
		}
		// self.advance_end_of_statement();
		Ok(Stmt::ifstmt(Rc::new(conditionals), Rc::new(else_block)))
	}

	fn assign_statement(&mut self) -> Result<Rc<Stmt>, ParseError> {
		let expr = self.expression()?;

		if self.match_advance(&[TokenType::Equal]) {
			let value = self.expression()?;
			self.advance_end_of_statement();
			return Ok(Stmt::assign(expr, value))
		}

		// Not an assignment
		self.advance_end_of_statement();
		Ok(Stmt::expr(expr))
	}

	fn expr_statement(&mut self) -> Result<Rc<Stmt>, ParseError> {
		let expr = self.expression()?;
		self.advance_end_of_statement();
		Ok(Stmt::expr(expr))
	}

	fn peek(&mut self) -> Result<&Token, ParseError> {
		if let Some(t) = self.tokens.peek() {
			Ok(t)
		} else {
			Err(perror(self.prev.clone(), "peek() Expected another token!"))
		}
	}

	fn peek_prev(&self) -> &Token {
		&self.prev
	}

	fn advance(&mut self) -> Result<&Token, ParseError> {
		if let Some(t) = self.tokens.next().take() {
			self.prev = t;
			Ok(&self.prev)
		} else {
			Err(perror(self.prev.clone(), "advance() Expected another token!"))
		}
	}

	fn match_advance(&mut self, matches: &[TokenType]) -> bool {
		let is_match = self.tokens.peek().map(|t| {
				matches.iter().any(|mtt| *mtt == t.ttype)
			}).unwrap_or(false);
		if is_match { self.prev = self.tokens.next().take().unwrap(); }
		is_match
	}

	fn identifier_match(&mut self) -> bool {
		self.tokens.peek().map(|t| {
			match t.ttype {
				TokenType::Identifier(_) => true,
				_ => false
			}}).unwrap_or(false)
	}

	fn check(&mut self, mtt: TokenType) -> bool {
		self.tokens.peek().map(|t| {
			t.ttype == mtt
		}).unwrap_or(false)
	}

	fn check_identifier(&mut self) -> bool {
		if let Some(t) = self.tokens.peek() {
			match t.ttype {
				TokenType::Identifier(_) => true,
				_ => false
			}
		} else {
			false
		}
	}

	fn consume(&mut self, t: TokenType, msg: &str) -> Result<(), ParseError> {
		if self.check(t) {
			self.advance()?;
			Ok(())
		} else {
			Err(perror(self.peek()?.clone(), msg))
		}
	}

	fn advance_end_of_statement(&mut self) -> Result<(), ParseError> {
		self.consume(TokenType::Semicolon, "Expect ';' at end of statement.")
	}

	fn expression(&mut self) -> Result<Rc<Expr>, ParseError> {
		Ok(self.equality()?)
	}

	fn equality(&mut self) -> Result<Rc<Expr>, ParseError> {
		let mut expr = self.comparison()?;

		while self.match_advance(&[TokenType::BangEqual, TokenType::EqualEqual]) {
			let op = self.peek_prev().clone();
			let right = self.comparison()?;
			expr = Expr::binary(expr, op, right);
		}
		Ok(expr)
	}

	fn comparison(&mut self) -> Result<Rc<Expr>, ParseError> {
		let mut expr = self.term()?;

		while self.match_advance(&[TokenType::Greater, TokenType::GreaterEqual,
								   TokenType::Less, TokenType::LessEqual]) {
			let op = self.peek_prev().clone();
			let right = self.term()?;
			expr = Expr::binary(expr, op, right);
		}
		Ok(expr)
	}

	fn term(&mut self) -> Result<Rc<Expr>, ParseError> {
		let mut expr = self.factor()?;

		while self.match_advance(&[TokenType::Minus, TokenType::Plus]) {
			let op = self.peek_prev().clone();
			let right = self.factor()?;
			expr = Expr::binary(expr, op, right);
		}
		Ok(expr)
	}

	fn factor(&mut self) -> Result<Rc<Expr>, ParseError> {
		let mut expr = self.unary()?;

		while self.match_advance(&[TokenType::Slash, TokenType::Star]) {
			let op = self.peek_prev().clone();
			let right = self.unary()?;
			expr = Expr::binary(expr, op, right);
		}
		Ok(expr)
	}

	fn unary(&mut self) -> Result<Rc<Expr>, ParseError> {
		if self.match_advance(&[TokenType::Bang, TokenType::Minus]) {
			let op = self.peek_prev().clone();
			let right = self.unary()?;
			Ok(Expr::unary(op, right))
		} else {
			Ok(self.primary()?)
		}
	}

	fn primary(&mut self) -> Result<Rc<Expr>, ParseError> {
		use crate::token::TokenType::*;
		if self.check(TokenType::Semicolon) {
			return Err(perror(self.peek()?.clone(), "Expect primary!"))
		}
		match &self.advance()?.ttype {
			False => Ok(Expr::literal(Object::Bool(false))),
			True => Ok(Expr::literal(Object::Bool(true))),
			Nil => Ok(Expr::literal(Object::Nil)),
			Number(n) => Ok(Expr::literal(Object::Num(*n))),
			StringLit(_) => {
				let s = self.peek_prev().literal.clone();
				// TODO: Why does the StringLit string pick up quotes
				// at beginning and end? I'm removing them here.
				let s2 = s[1..s.len() - 1].to_string();
				Ok(Expr::literal(Object::Str(s2)))
			},
			LeftParen => {
				let expr = self.expression()?;
				self.consume(TokenType::RightParen, "Expect )!")?;
				Ok(Expr::grouping(expr))
			},
			LeftBrace => {
				let expr = self.expression()?;
				self.consume(TokenType::RightBrace, "Expect }!")?;
				Ok(Expr::block(expr))
			},
			Identifier(name) => Ok(Expr::variable(&name)),
			_ => {
				Err(perror(self.peek_prev().clone(), "Supposed primary not found!"))
			}
		}
	}

	// Skip the remaining tokens in the current statemet
	// and continue parsing the next statement.
	fn synchronize(&mut self) -> Result<(), ParseError> {
		self.advance()?;
		use crate::token::TokenType::*;
		loop {
			if self.peek_prev().ttype == Semicolon {
				return Ok(())
			}

			match self.peek()?.ttype {
				Class | Fun | Var | For | If
				| While | Print | Return => return Ok(()),
				_ => {}
			}
			self.advance()?;
		}
	}
}
