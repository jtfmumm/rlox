use crate::lox_error::{LoxError, perror, ParseError};
use crate::expr::Expr;
use crate::object::Object;
use crate::stmt::Stmt;
use crate::token::{Token, TokenType};

use std::collections::HashMap;
use std::iter::Peekable;
use std::rc::Rc;
use std::vec::IntoIter;

pub struct Parser {
	tokens: Peekable<IntoIter<Token>>,
	prev: Token,
	// This represents all scopes except the global scope.
	scopes: Vec<HashMap<String, bool>>,
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		let prev = Token::new(TokenType::Sof, "".to_string(), "".to_string(), 0);
		Parser { tokens: tokens.into_iter().peekable(), prev,
				 scopes: Vec::new() }
	}

	pub fn parse(&mut self) -> Result<Vec<Rc<Stmt>>, LoxError> {
		let mut stmts = Vec::new();
		let mut failed = false;
		while self.tokens.peek().unwrap().ttype != TokenType::Eof {
			match self.declaration() {
				Ok(stmt) => {
					stmts.push(stmt);
				},
				Err(_) => {
					failed = true;
					let _ = self.synchronize();
				}
			}
			if self.tokens.peek().is_none() { break }
		}
		if failed {
			Err(LoxError::Parse)
		} else {
			Ok(stmts)
		}
	}

	fn declaration(&mut self) -> Result<Rc<Stmt>, ParseError> {
		if self.match_advance(&[TokenType::Fun]) {
			self.fun_statement()
		} else if self.match_advance(&[TokenType::Var]) {
			self.var_statement()
		} else {
			self.statement()
		}
	}

	fn statement(&mut self) -> Result<Rc<Stmt>, ParseError> {
		if self.match_advance(&[TokenType::Print]) {
			self.print_statement()
		} else if self.check(&[TokenType::LeftBrace]) {
			self.block()
		} else if self.match_advance(&[TokenType::If]) {
			self.if_statement()
		} else if self.match_advance(&[TokenType::While]) {
			self.while_statement()
		} else if self.match_advance(&[TokenType::For]) {
			self.for_statement()
		} else if self.match_advance(&[TokenType::Return]) {
			self.return_statement()
		} else {
			self.expr_statement()
		}
	}

	fn block(&mut self) -> Result<Rc<Stmt>, ParseError> {
		self.scopes.push(HashMap::new());
		let mut stmts = Vec::new();
		let mut failed = false;

		if self.match_advance(&[TokenType::LeftBrace]) {
			while !self.match_advance(&[TokenType::RightBrace]) {
				match self.declaration() {
					Ok(stmt) => {
						stmts.push(stmt);
					},
					Err(_) => {
						failed = true;
						let _ = self.synchronize();
					}
				}
				if self.check(&[TokenType::Eof]) {
					break
				}
			}
		} else {
			stmts.push(self.statement()?)
		}

		self.scopes.pop();

		if failed {
			Err(ParseError::new("Failed while parsing block."))
		} else {
			Ok(Stmt::block_stmt(Rc::new(stmts)))
		}
	}

	fn for_statement(&mut self) -> Result<Rc<Stmt>, ParseError> {
		self.scopes.push(HashMap::new());
		self.consume(TokenType::LeftParen, "Expect ( for condition.")?;
		let init = if self.match_advance(&[TokenType::Semicolon]) {
			None
		} else {
			if !(self.check_var() || self.check_identifier()) {
				return Err(perror(self.peek()?.clone(), "Expect expression."))
			}
			Some(self.declaration()?)
		};
		let condition = if self.match_advance(&[TokenType::Semicolon]) {
			None
		} else {
			let exp = Some(self.expression()?);
			self.consume(TokenType::Semicolon, "Expect ; after for condition.")?;
			exp
		};
		let inc = if self.match_advance(&[TokenType::RightParen]) {
			None
		} else {
			if !self.check_identifier() {
				return Err(perror(self.peek()?.clone(), "Expect expression."))
			}
			let expr = Some(self.expression()?);
			self.consume(TokenType::RightParen, "Expect ) for end of for.")?;
			expr
		};
		let blk = self.block()?;
		self.scopes.pop();
		Ok(Stmt::for_stmt(Rc::new(init), Rc::new(condition), Rc::new(inc), blk))
	}

	fn fun_statement(&mut self) -> Result<Rc<Stmt>, ParseError> {
		let name = if self.check_identifier() {
			self.advance()?.clone()
		} else {
			return Err(perror(self.peek()?.clone(), "Expect function name."))
		};
		self.define_var(&name)?;
		self.consume(TokenType::LeftParen, "Expect '(' after function name.")?;
		self.scopes.push(HashMap::new());
		let mut params = Vec::new();
		if !self.check(&[TokenType::RightParen]) {
			loop {
				if self.check_identifier() {
					match &*self.expression()?.clone() {
						Expr::Variable { name, .. } => {
							self.declare_var(&name)?;
							self.define_var(&name)?;
							params.push(name.clone());
						},
						_ => unreachable!()
					}
				} else {
					return Err(perror(self.peek()?.clone(), "Expect parameter name."))
				}
				if !self.match_advance(&[TokenType::Comma]) { break }
			}
			if params.len() >= 255 {
				return Err(perror(self.peek_prev().clone(), "Can't have more than 255 parameters."))
			}
		}
		self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;
		if !self.check(&[TokenType::LeftBrace]) {
			return Err(perror(self.peek()?.clone(), "Expect '{' before function body."))
		}
		let body = self.block()?;
		self.scopes.pop();
		let fun_depth = Rc::new(if self.scopes.is_empty() {
			None
		} else {
			Some(0)
		});
		Ok(Stmt::fun_stmt(name.clone(), Rc::new(params), body.clone(), fun_depth))
	}

	fn var_statement(&mut self) -> Result<Rc<Stmt>, ParseError> {
		if self.check_identifier() {
			let vname = self.peek()?.clone();
			self.declare_var(&vname)?;
			let (vr, vl) = match &*self.expression()?.clone() {
				Expr::Assign { variable, value } => {
					if let Expr::Variable { name, .. } = &*value.clone() {
						if &vname == name && self.depth_for(&vname)?.is_some() {
							return Err(perror(self.peek_prev().clone(),
								"Can't read local variable in its own initializer."));
						}
					}
					self.define_var(&vname)?;
					(variable.clone(), value.clone())
				},
				Expr::Variable { name, depth } => {
					self.define_var(&name)?;
					(
						Expr::variable(name.clone(), depth.clone()),
					 	Expr::literal(Rc::new(Object::Nil))
				 	)
				}
				_ => return Err(perror(self.peek_prev().clone(), "Invalid declaration"))
			};
			self.advance_end_of_statement()?;
			Ok(Stmt::var_decl_stmt(vr, vl))
		} else {
			Err(perror(self.tokens.peek().unwrap().clone(), "Expect variable name."))
		}
	}

	fn print_statement(&mut self) -> Result<Rc<Stmt>, ParseError> {
		let expr = self.expression()?;
		self.advance_end_of_statement()?;
		Ok(Stmt::print_stmt(expr))
	}

	fn if_statement(&mut self) -> Result<Rc<Stmt>, ParseError> {
		let mut conditionals = Vec::new();
		let mut else_block = None;
		loop {
			self.consume(TokenType::LeftParen, "Expect ( for condition.")?;
			let conditional = self.expression()?;
			self.consume(TokenType::RightParen, "Expect ) for condition.")?;
			let blk = self.block()?;
			conditionals.push((conditional, blk));
			if !self.match_advance(&[TokenType::Elif]) { break }
		}
		if self.match_advance(&[TokenType::Else]) {
			let stmt = self.statement()?;
			else_block = Some(stmt);
		}
		Ok(Stmt::if_stmt(Rc::new(conditionals), Rc::new(else_block)))
	}

	fn while_statement(&mut self) -> Result<Rc<Stmt>, ParseError> {
		self.consume(TokenType::LeftParen, "Expect ( for condition.")?;
		let condition = self.expression()?;
		self.consume(TokenType::RightParen, "Expect ) for condition.")?;
		let blk = self.block()?;
		Ok(Stmt::while_stmt(condition, blk))
	}

	fn return_statement(&mut self) -> Result<Rc<Stmt>, ParseError> {
		if self.scopes.is_empty() {
			return Err(perror(self.peek_prev().clone(), "Can't return from top-level code."))
		}
		if self.match_advance(&[TokenType::Semicolon]) {
			return Ok(Stmt::return_stmt(Expr::literal(Rc::new(Object::Nil))))
		}
		let expr = self.expression()?;
		self.advance_end_of_statement()?;
		Ok(Stmt::return_stmt(expr))
	}

	fn expr_statement(&mut self) -> Result<Rc<Stmt>, ParseError> {
		let expr = self.expression()?;
		self.advance_end_of_statement()?;
		Ok(Stmt::expr_stmt(expr))
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
		let is_match = self.check(matches);
		if is_match { self.prev = self.tokens.next().take().unwrap(); }
		is_match
	}

	fn check(&mut self, matches: &[TokenType]) -> bool {
		self.tokens.peek().map(|t| {
			matches.iter().any(|mtt| *mtt == t.ttype)
		}).unwrap_or(false)
	}

	fn check_var(&mut self) -> bool {
		self.check(&[TokenType::Var])
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
		if self.check(&[t]) {
			self.advance()?;
			Ok(())
		} else {
			Err(perror(self.peek()?.clone(), msg))
		}
	}

	fn advance_end_of_statement(&mut self) -> Result<(), ParseError> {
		self.consume(TokenType::Semicolon, "Expect ';' after expression.")
	}

	fn expression(&mut self) -> Result<Rc<Expr>, ParseError> {
		Ok(self.logic_or()?)
	}

	fn logic_or(&mut self) -> Result<Rc<Expr>, ParseError> {
		let mut expr = self.logic_and()?;

		while self.match_advance(&[TokenType::Or]) {
			let op = self.peek_prev().clone();
			let right = self.logic_and()?;
			expr = Expr::logic(expr, op, right);
		}
		Ok(expr)
	}

	fn logic_and(&mut self) -> Result<Rc<Expr>, ParseError> {
		let mut expr = self.assignment()?;

		while self.match_advance(&[TokenType::And]) {
			let op = self.peek_prev().clone();
			let right = self.assignment()?;
			expr = Expr::logic(expr, op, right);
		}
		Ok(expr)
	}

	fn assignment(&mut self) -> Result<Rc<Expr>, ParseError> {
		let mut expr = self.equality()?;

		if self.match_advance(&[TokenType::Equal]) {
			let vname = match &*expr.clone() {
				Expr::Variable { name, .. } => name.clone(),
				_ => return Err(perror(self.peek_prev().clone(), "Invalid assignment target."))
			};
			self.assign_var(&vname)?;
			let value = self.expression()?;
			expr = Expr::assign(expr, value)
		}
		Ok(expr)
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
			Ok(self.call()?)
		}
	}

	fn call(&mut self) -> Result<Rc<Expr>, ParseError> {
		let mut expr = self.primary()?;

		loop {
			if self.match_advance(&[TokenType::LeftParen]) {
				expr = self.build_call(expr)?;
			} else {
				break
			}
		}
		Ok(expr)
	}

	fn build_call(&mut self, expr: Rc<Expr>) -> Result<Rc<Expr>, ParseError> {
		let args = if self.check(&[TokenType::RightParen]) {
			Rc::new(Vec::new())
		} else {
			self.call_args()?
		};
		if self.check(&[TokenType::RightParen]) {
			Ok(Expr::call(expr, self.advance()?.clone(), args))
		} else {
			Err(perror(self.peek()?.clone(), "Expect )."))
		}
	}

	fn call_args(&mut self) -> Result<Rc<Vec<Rc<Expr>>>, ParseError> {
		let mut args = Vec::new();
		loop {
			args.push(self.expression()?);
			if !self.match_advance(&[TokenType::Comma]) { break }
		}
		if args.len() >= 255 {
			Err(perror(self.peek_prev().clone(), "Can't have more than 255 arguments."))
		} else {
			Ok(Rc::new(args))
		}
	}

	fn primary(&mut self) -> Result<Rc<Expr>, ParseError> {
		use crate::token::TokenType::*;
		if self.check(&[TokenType::Semicolon]) {
			return Err(perror(self.peek()?.clone(), "Expect expression."))
		}
		let token = &self.advance()?.clone();
		match &token.ttype {
			False => Ok(Expr::literal(Rc::new(Object::Bool(false)))),
			Identifier(_) => {
				let depth = self.depth_for(&token)?;
				Ok(Expr::variable(token.clone().clone(), depth))
			},
			LeftParen => {
				let expr = self.expression()?;
				self.consume(TokenType::RightParen, "Expect )!")?;
				Ok(Expr::grouping(expr))
			},
			Nil => Ok(Expr::literal(Rc::new(Object::Nil))),
			Number(n) => Ok(Expr::literal(Rc::new(Object::Num(*n)))),
			StringLit(_) => {
				let s = self.peek_prev().literal.clone();
				let s2 = s[0..s.len()].to_string();
				Ok(Expr::literal(Rc::new(Object::Str(s2))))
			},
			True => Ok(Expr::literal(Rc::new(Object::Bool(true)))),
			_ => {
				Err(perror(self.peek_prev().clone(), "Expect expression."))
			}
		}
	}

	fn declare_var(&mut self, name: &Token) -> Result<(), ParseError> {
		self.set_var(name, false)
	}

	fn define_var(&mut self, name: &Token) -> Result<(), ParseError> {
		self.set_var(name, true)
	}

	fn set_var(&mut self, name: &Token, define: bool) -> Result<(), ParseError> {
		if self.scopes.is_empty() { return Ok(()) }
		if let TokenType::Identifier(ref vname) = name.ttype {
			let scope = self.scopes.last_mut().unwrap();
			if !define && scope.contains_key(vname) {
				return Err(perror(name.clone(),
					"Already a variable with this name in this scope."))
			}
			scope.insert(vname.to_string(), define);
			Ok(())
		} else {
			Err(perror(self.peek_prev().clone(), "Expect variable."))
		}
	}

	fn assign_var(&mut self, name: &Token) -> Result<(), ParseError> {
		if self.scopes.is_empty() { return Ok(()) }
		if let TokenType::Identifier(ref vname) = name.ttype {
			for i in (0..self.scopes.len()).rev() {
				let scope = self.scopes.get_mut(i).unwrap();
				if scope.contains_key(vname) {
					scope.insert(vname.to_string(), true);
				}
			}
			Ok(())
		} else {
			Err(perror(self.peek_prev().clone(), "Expect variable."))
		}
	}

	fn depth_for(&self, identifier: &Token) -> Result<Rc<Option<u32>>, ParseError> {
		if let TokenType::Identifier(ref vname) = identifier.ttype {
			for i in (0..self.scopes.len()).rev() {
				if self.scopes.get(i).unwrap().contains_key(vname) {
					let depth = (self.scopes.len() - 1) - i;
					return Ok(Rc::new(Some(depth as u32)))
				}
			}
			Ok(Rc::new(None))
		} else {
			Err(perror(self.peek_prev().clone(), "Expect identifier."))
		}
	}

	// Skip the remaining tokens in the current statemet
	// and continue parsing the next statement.
	fn synchronize(&mut self) -> Result<(), ParseError> {
		if self.check(&[Eof]) { return Ok(()) }
		self.advance()?;
		use crate::token::TokenType::*;
		loop {
			if self.peek_prev().ttype == LeftBrace {
				loop {
					let next = self.advance()?;
					if next.ttype == RightBrace || next.ttype == Eof {
						break
					}
				}
			}

			if self.peek_prev().ttype == Semicolon {
				return Ok(())
			}

			if self.check(&[Eof, Class, Fun, Var, For, If, While, Print, Return]) {
				return Ok(())
			}

			self.advance()?;
		}
	}
}
