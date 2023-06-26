use crate::cerror::{perror, ParseError};
use crate::expr::Expr;
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

	pub fn parse(&mut self) -> Result<Rc<Expr>, ParseError> {
		self.expression()
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
		match self.advance()?.ttype {
			TokenType::False => Ok(Expr::literal("false".to_string())),
			TokenType::True => Ok(Expr::literal("true".to_string())),
			TokenType::Nil => Ok(Expr::literal("nil".to_string())),
			TokenType::Number(_) | TokenType::StringLit(_) => {
				Ok(Expr::literal(self.peek_prev().literal.clone()))
			},
			TokenType::LeftParen => {
				let expr = self.expression()?;
				self.consume(TokenType::RightParen, "Expected )!")?;
				Ok(Expr::grouping(expr))
			},
			_ => {
				Err(perror(self.peek()?.line, self.peek()?.clone(), "Something went wrong!"))
			}
		}
	}
}
