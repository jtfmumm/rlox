use crate::cerror::{perror, ParseError};
use crate::expr::Expr;
use crate::literal::Literal;
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

	pub fn parse(&mut self) -> Result<Rc<Expr>, String> {
		match self.expression() {
			Ok(expr) => Ok(expr),
			// We need to handle syntax errors here.
			Err(_) => {
				// println!("\n{:}", err);
				Err("Parsing failed!".to_string())
			}
		}
	}

	fn peek(&mut self) -> Result<&Token, ParseError> {
		if let Some(t) = self.tokens.peek() {
			Ok(t)
		} else {
			Err(perror(self.prev.line, self.prev.clone(), "Expected another token!"))
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
			Err(perror(self.prev.line, self.prev.clone(), "Expected another token!"))
		}
	}

	fn match_advance(&mut self, matches: &[TokenType]) -> bool {
		let is_match = self.tokens.peek().map(|t| {
				matches.iter().any(|mtt| *mtt == t.ttype)
			}).unwrap_or(false);
		if is_match { self.prev = self.tokens.next().take().unwrap(); }
		is_match
	}

	fn check(&mut self, mtt: TokenType) -> bool {
		self.tokens.peek().map(|t| {
			t.ttype == mtt
		}).unwrap_or(false)
	}

	fn consume(&mut self, t: TokenType, msg: &str) -> Result<(), ParseError> {
		if self.check(t) {
			self.advance()?;
			Ok(())
		} else {
			Err(perror(self.peek()?.line, self.peek()?.clone(), msg))
		}
	}

	pub fn expression(&mut self) -> Result<Rc<Expr>, ParseError> {
		Ok(self.equality()?)
	}

	pub fn equality(&mut self) -> Result<Rc<Expr>, ParseError> {
		let mut expr = self.comparison()?;

		while self.match_advance(&[TokenType::BangEqual, TokenType::EqualEqual]) {
			let op = self.peek_prev().clone();
			let right = self.comparison()?;
			expr = Expr::binary(expr, op, right);
		}
		Ok(expr)
	}

	pub fn comparison(&mut self) -> Result<Rc<Expr>, ParseError> {
		let mut expr = self.term()?;

		while self.match_advance(&[TokenType::Greater, TokenType::GreaterEqual,
								   TokenType::Less, TokenType::LessEqual]) {
			let op = self.peek_prev().clone();
			let right = self.term()?;
			expr = Expr::binary(expr, op, right);
		}
		Ok(expr)
	}

	pub fn term(&mut self) -> Result<Rc<Expr>, ParseError> {
		let mut expr = self.factor()?;

		while self.match_advance(&[TokenType::Minus, TokenType::Plus]) {
			let op = self.peek_prev().clone();
			let right = self.factor()?;
			expr = Expr::binary(expr, op, right);
		}
		Ok(expr)
	}

	pub fn factor(&mut self) -> Result<Rc<Expr>, ParseError> {
		let mut expr = self.unary()?;

		while self.match_advance(&[TokenType::Slash, TokenType::Star]) {
			let op = self.peek_prev().clone();
			let right = self.unary()?;
			expr = Expr::binary(expr, op, right);
		}
		Ok(expr)
	}

	pub fn unary(&mut self) -> Result<Rc<Expr>, ParseError> {
		if self.match_advance(&[TokenType::Bang, TokenType::Minus]) {
			let op = self.peek_prev().clone();
			let right = self.unary()?;
			Ok(Expr::unary(op, right))
		} else {
			Ok(self.primary()?)
		}
	}

	pub fn primary(&mut self) -> Result<Rc<Expr>, ParseError> {
		use crate::token::TokenType::*;
		match self.advance()?.ttype {
			False => Ok(Expr::literal(Literal::Bool(false))),
			True => Ok(Expr::literal(Literal::Bool(true))),
			Nil => Ok(Expr::literal(Literal::Nil)),
			Number(n) => Ok(Expr::literal(Literal::Num(n))),
			StringLit(_) => {
				let s = self.peek_prev().literal.clone();
				Ok(Expr::literal(Literal::Str(s)))
			},
			LeftParen => {
				let expr = self.expression()?;
				self.consume(TokenType::RightParen, "Expected )!")?;
				Ok(Expr::grouping(expr))
			},
			_ => {
				Err(perror(self.peek()?.line, self.peek()?.clone(), "Expected expression!"))
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
