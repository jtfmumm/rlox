use crate::cerror::cerror;
use crate::expr::Expr;
use crate::token::{Token, TokenType};

use std::error::Error;
use std::rc::Rc;

pub struct Parser {
// token_iter: std::iter::Peekable<std::slice::Iter<'a, Token<'a>>>,
	tokens: Vec<Token>,
	current: usize,
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		let current = 0;
		Parser { tokens, current }
	}

	pub fn parse(&mut self) -> Result<Rc<Expr>, String> {
		self.expression()
	}

	fn is_at_end(&self) -> bool {
		self.current >= self.tokens.len()
	}

	fn peek(&mut self) -> Token {
		self.tokens[self.current].clone()
	}

	fn peek_prev(&mut self) -> Token {
		self.tokens[self.current - 1].clone()
	}

	// fn peek_next(&mut self) -> Token {
	// 	self.tokens[self.current + 1].clone()
	// }

	fn advance(&mut self) -> Token {
		let t = self.tokens[self.current].clone();
		self.current += 1;
		t
	}

	fn match_advance(&mut self, ts: &[TokenType]) -> bool {
		if self.is_at_end() { return false }

		let cur_ttype = &self.peek().ttype;
		if ts.iter().any(|tt| tt == cur_ttype) {
			self.current += 1;
			true
		} else {
			false
		}
	}

	fn check(&mut self, t: TokenType) -> bool {
		if self.is_at_end() { return false }
		self.peek().ttype == t
	}

	fn consume(&mut self, t: TokenType, msg: &str) -> Result<(), String> {
		if self.check(t) {
			self.advance();
			Ok(())
		} else {
			Err(cerror(self.peek().line, msg))
		}
	}

	pub fn expression(&mut self) -> Result<Rc<Expr>, String> {
		Ok(self.equality()?)
	}

	pub fn equality(&mut self) -> Result<Rc<Expr>, String> {
		let mut expr = self.comparison()?;

		while self.match_advance(&[TokenType::BangEqual, TokenType::EqualEqual]) {
			let op = self.peek_prev();
			let right = self.comparison()?;
			expr = Expr::binary(expr, op.clone(), right);
		}
		Ok(expr)
	}

	pub fn comparison(&mut self) -> Result<Rc<Expr>, String> {
		let mut expr = self.term()?;

		while self.match_advance(&[TokenType::Greater, TokenType::GreaterEqual,
								   TokenType::Less, TokenType::LessEqual]) {
			let op = self.peek_prev();
			let right = self.term()?;
			expr = Expr::binary(expr, op.clone(), right);
		}
		Ok(expr)
	}

	pub fn term(&mut self) -> Result<Rc<Expr>, String> {
		let mut expr = self.factor()?;

		while self.match_advance(&[TokenType::Minus, TokenType::Plus]) {
			let op = self.peek_prev();
			let right = self.factor()?;
			expr = Expr::binary(expr, op.clone(), right);
		}
		Ok(expr)
	}

	pub fn factor(&mut self) -> Result<Rc<Expr>, String> {
		let mut expr = self.unary()?;

		while self.match_advance(&[TokenType::Slash, TokenType::Star]) {
			let op = self.peek_prev();
			let right = self.unary()?;
			expr = Expr::binary(expr, op.clone(), right);
		}
		Ok(expr)
	}

	pub fn unary(&mut self) -> Result<Rc<Expr>, String> {
		if self.match_advance(&[TokenType::Bang, TokenType::Minus]) {
			let op = self.peek_prev();
			let right = self.unary()?;
			Ok(Expr::unary(op.clone(), right))
		} else {
			Ok(self.primary()?)
		}
	}

	pub fn primary(&mut self) -> Result<Rc<Expr>, String> {
		if self.is_at_end() {
			return Err(cerror(self.peek().line, "Expected primary!"))
		}
		match self.advance().ttype {
			TokenType::False => Ok(Expr::literal("false".to_string())),
			TokenType::True => Ok(Expr::literal("true".to_string())),
			TokenType::Nil => Ok(Expr::literal("nil".to_string())),
			TokenType::Number(_) | TokenType::StringLit(_) => {
				Ok(Expr::literal(self.peek_prev().literal))
			},
			TokenType::LeftParen => {
				let expr = self.expression()?;
				self.consume(TokenType::RightParen, "Expected )!");
				Ok(Expr::grouping(expr))
			},
			_ => {
				Err(cerror(self.peek().line, "Something went wrong!"))
			}
		}
	}
}
