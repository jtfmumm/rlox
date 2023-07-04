use crate::lox_error::{LoxError, ScanError, scerror};
use crate::token::{Token, TokenType};

use std::mem;

const KEYWORDS: [&'static str; 17] = ["and", "class", "elif", "else", "false", "fun", "for",
							 "if", "nil", "or", "print", "return", "super", "this",
							 "true", "var", "while"];

pub struct Scanner {
	source: Vec<char>,
	tokens: Vec<Token>,
	start: usize,
	current: usize,
	line: u32,
}

impl Scanner {
	pub fn new(s: String) -> Self {
		let source = s.chars().collect();
		let start = 0;
		let current = 0;
		let line = 1;
		Scanner { source, tokens: Vec::new(), start, current, line }
	}

	pub fn scan_tokens(&mut self) -> Result<Vec<Token>,LoxError> {
		while !self.is_at_end() {
			self.start = self.current;
			if self.scan_token().is_err() { return Err(LoxError::Scan) }
		}

		self.tokens.push(Token::new(TokenType::Eof, "".to_string(), "".to_string(), self.line));
		Ok(mem::replace(&mut self.tokens, Vec::new()))
	}

	fn is_at_end(&self) -> bool {
		self.current >= self.source.len()
	}

	fn advance(&mut self) -> char {
		let ch = self.source[self.current];
		self.current += 1;
		ch
	}

	fn peek(&self) -> char {
		self.source[self.current]
	}

	fn peek_next(&self) -> char {
		self.source[self.current + 1]
	}

	fn match_advance(&mut self, m: char) -> bool {
		if self.is_at_end() { return false }
		if self.peek() == m {
			self.current += 1;
			true
		} else {
			false
		}
	}

	fn report_error(&mut self, msg: &str) -> ScanError {
		scerror(self.line, msg)
	}

	fn scan_token(&mut self) -> Result<(),ScanError> {
		let ch = self.advance();
		let token_type = match ch {
			'(' => TokenType::LeftParen,
			')' => TokenType::RightParen,
			'{' => TokenType::LeftBrace,
			'}' => TokenType::RightBrace,
			',' => TokenType::Comma,
			'.' => TokenType::Dot,
			'-' => TokenType::Minus,
			'+' => TokenType::Plus,
			';' => TokenType::Semicolon,
			'*' => TokenType::Star,
			'>' => {
				if self.match_advance('=') { TokenType::GreaterEqual } else { TokenType::Greater }
			},
			'<' => {
				if self.match_advance('=') { TokenType::LessEqual } else { TokenType::Less }
			},
			'=' => {
				if self.match_advance('=') { TokenType::EqualEqual } else { TokenType::Equal }
			},
			'!' => {
				if self.match_advance('=') { TokenType::BangEqual } else { TokenType::Bang }
			},
			'/' => {
				if self.match_advance('/') {
					while !(self.is_at_end() || self.peek() == '\n')  { self.current += 1; };
					return Ok(())
				} else if self.match_advance('*') {
					while !self.is_at_end() && !(self.peek() == '*' && self.peek_next() == '/') {
						self.current += 1;
					}
					if self.is_at_end() {
						return Err(self.report_error("You must close multiline comments with */"))
					} else {
						self.current += 2;
					}
					return Ok(())
				} else {
					TokenType::Slash
				}
			}
			'"' => {
				let (t, s) = self.scan_string()?;
				self.tokens.push(Token::new(t, s.clone(), s, self.line));
				return Ok(())
			},
			a if a.is_alphabetic() => self.scan_word()?,
			d if d.is_digit(10) => self.scan_number()?,
			' ' | '\r' | '\t' => return Ok(()),
			'\n' => { self.line += 1; return Ok(()) },
			_ => { return Err(self.report_error("Unexpected character.")) }
		};
		self.add_token(token_type);
		Ok(())
	}

	fn scan_string(&mut self) -> Result<(TokenType, String),ScanError> {
		let mut s = "".to_string();
		while !self.match_advance('"') {
			if self.is_at_end() { return Err(self.report_error("Unterminated string.")) }
			if self.peek() == '\n' { self.line += 1;  }
			// Add one by one so that Unicode can also be handled correctly.
			s.push(self.peek());
			self.current += 1;
		}
		Ok((TokenType::StringLit(s.clone()), s))
	}

	fn scan_word(&mut self) -> Result<TokenType,ScanError> {
		while !self.is_at_end() && (self.peek().is_alphanumeric() || self.peek() == '_') {
			self.current += 1;
		}
		let substr = self.source_substr();
		Ok(if KEYWORDS.contains(&&substr[..]) {
			self.keyword_token(&substr)?
		} else {
			TokenType::Identifier(substr)
		})
	}

	fn scan_number(&mut self) -> Result<TokenType,ScanError> {
		while !self.is_at_end() && (self.peek().is_digit(10)) {
			self.current += 1
		}
		if self.match_advance('.') {
			if !self.peek().is_digit(10) { return Err(self.report_error("Number has trailing .")) }
			while self.peek().is_digit(10) {
				self.current += 1;
			}
		}
		let n = self.source_substr().parse::<f64>().unwrap();
		Ok(TokenType::Number(n))
	}

	fn add_token(&mut self, ttype: TokenType) {
		let s = self.source_substr();
		self.tokens.push(Token::new(ttype, s.clone(), s, self.line));
	}

	fn source_substr(&self) -> String {
		let mut s = "".to_string();
		for i in self.start..self.current {
			s.push(self.source[i]);
		}
		s
	}

	fn keyword_token(&mut self, keyword: &str) -> Result<TokenType,ScanError> {
		Ok(match keyword {
			"and" => TokenType::And,
			"class" => TokenType::Class,
			"elif" => TokenType::Elif,
			"else" => TokenType::Else,
			"false" => TokenType::False,
			"fun" => TokenType::Fun,
			"for" => TokenType::For,
			"if" => TokenType::If,
			"nil" => TokenType::Nil,
			"or" => TokenType::Or,
			"print" => TokenType::Print,
			"return" => TokenType::Return,
			"super" => TokenType::Super,
			"this" => TokenType::This,
			"true" => TokenType::True,
			"var" => TokenType::Var,
			"while" => TokenType::While,
			_ => { return Err(self.report_error("Invalid keyword!")) }
		})
	}
}
