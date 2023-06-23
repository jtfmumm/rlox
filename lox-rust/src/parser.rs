use crate::cerror::error;
use crate::expr::Expr;
use crate::token::{Token, TokenType};

use std::error::Error;
use std::fmt;
use std::mem;
use std::rc::Rc;

pub struct Parser {
	tokens: Vec<Token>,
	current: usize,
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		let current = 0;
		Parser { tokens, current }
	}

	fn is_at_end(&self) -> bool {
		self.current >= self.tokens.len()
	}

	fn advance(&mut self) -> Token {
		let t = self.tokens[self.current].clone();
		self.current += 1;
		t
	}

	fn peek(&mut self) -> Token {
		self.tokens[self.current].clone()
	}

	fn peek_prev(&mut self) -> Token {
		self.tokens[self.current - 1].clone()
	}

	fn peek_next(&mut self) -> Token {
		self.tokens[self.current + 1].clone()
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

	pub fn expression(&mut self) -> Rc<Expr> {
		self.equality()
	}

	pub fn equality(&mut self) -> Rc<Expr> {
		let mut expr = self.comparison();

		while self.match_advance(&[TokenType::BangEqual, TokenType::EqualEqual]) {
			let op = self.peek_prev();
			let right = self.comparison();
			expr = Expr::binary(expr, op.clone(), right);
		}
		expr
	}

	pub fn comparison(&mut self) -> Rc<Expr> {
		let mut expr = self.term();

		while self.match_advance(&[TokenType::Greater, TokenType::GreaterEqual,
								   TokenType::Less, TokenType::LessEqual]) {
			let op = self.peek_prev();
			let right = self.term();
			expr = Expr::binary(expr, op.clone(), right);
		}
		expr
	}

	pub fn term(&mut self) -> Rc<Expr> {
		let mut expr = self.factor();

		while self.match_advance(&[TokenType::Minus, TokenType::Plus]) {
			let op = self.peek_prev();
			let right = self.factor();
			expr = Expr::binary(expr, op.clone(), right);
		}
		expr
	}

	pub fn factor(&mut self) -> Rc<Expr> {
		let mut expr = self.unary();

		while self.match_advance(&[TokenType::Slash, TokenType::Star]) {
			let op = self.peek_prev();
			let right = self.unary();
			expr = Expr::binary(expr, op.clone(), right);
		}
		expr
	}

	pub fn unary(&mut self) -> Rc<Expr> {
		if self.match_advance(&[TokenType::Bang, TokenType::Minus]) {
			let op = self.peek_prev();
			let right = self.unary();
			Expr::unary(op.clone(), right)
		} else {
			self.primary()
		}
	}

	pub fn primary(&mut self) -> Rc<Expr> {
		// if self.is_at_end() { return false }
		match self.advance().ttype {
			TokenType::False => Expr::literal("false".to_string()),
			TokenType::True => Expr::literal("true".to_string()),
			TokenType::Nil => Expr::literal("nil".to_string()),
			TokenType::Number(_) | TokenType::StringLit(_) => {
				Expr::literal(self.peek_prev().literal)
			},
			TokenType::LeftParen => {
				let mut expr = self.expression();
				if !self.match_advance(&[TokenType::RightParen]) {
					error(self.peek().line, "Something went wrong!");
					Expr::literal("SOMETHING WENT WRONG".to_string())
				} else {
					Expr::grouping(expr)
				}
			},
			_ => {
				error(self.peek().line, "Something went wrong!");
				Expr::literal("SOMETHING WENT WRONG".to_string())
			}
		}
	}
}
