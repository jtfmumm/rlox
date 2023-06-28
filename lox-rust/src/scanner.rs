use crate::cerror::scerror;
use crate::token::{Token, TokenType};

use std::mem;

const KEYWORDS: [&'static str; 17] = ["and", "class", "else", "false", "fun", "for", "if",
                     		 "nil", "or", "p", "print", "return", "super", "this", "true",
                     		 "var", "while"];

pub struct Scanner {
	source_string: String,
	source: Vec<char>,
	tokens: Vec<Token>,
	start: usize,
	current: usize,
	line: u32,
	hit_error: bool,
}

impl Scanner {
	pub fn new(s: String) -> Self {
		let source = s.chars().collect();
		let start = 0;
		let current = 0;
		let line = 1;
		Scanner { source_string: s, source, tokens: Vec::new(), start, current, line, hit_error: false }
	}

	pub fn scan_tokens(&mut self) -> Result<Vec<Token>,()> {
		while !self.is_at_end() {
			self.start = self.current;
			self.scan_token()
		}

		self.tokens.push(Token::new(TokenType::Eof, "".to_string(), "".to_string(), self.line));
		if !self.hit_error {
			Ok(mem::replace(&mut self.tokens, Vec::new()))
		} else {
			Err(())
		}
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

	fn report_error(&mut self, msg: &str) {
		self.hit_error = true;
		scerror(self.line, msg);
	}

	fn scan_token(&mut self) {
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
					while !(self.peek() == '\n' || self.is_at_end())  { self.current += 1; }; return
				} else if self.match_advance('*') {
					while !self.is_at_end() && !(self.peek() == '*' && self.peek_next() == '/') {
						self.current += 1;
					}
					if self.is_at_end() {
						self.report_error("You must close multiline comments with */");
					} else {
						self.current += 2;
					}
					return
				} else {
					TokenType::Slash
				}
			}
			'"' => self.scan_string(),
			a if a.is_alphabetic() => self.scan_word(),
			d if d.is_digit(10) => self.scan_number(),
			' ' | '\r' | '\t' => return,
			'\n' => { self.line += 1; return },
			_ => { self.report_error("Unexpected character."); TokenType::Error }
		};
		self.add_token(token_type);
	}

	fn scan_string(&mut self) -> TokenType {
		while !self.match_advance('"') {
			if self.is_at_end() { self.report_error("Unterminated string!"); return TokenType::Error }
			self.current += 1
		}
		TokenType::StringLit(self.source_string[self.start + 1..self.current - 1].to_string())
	}

	fn scan_word(&mut self) -> TokenType {
		while !self.is_at_end() && (self.peek().is_alphanumeric() || self.peek() == '_') {
			self.current += 1
		}
		let substr = self.source_substr();
		if KEYWORDS.contains(&&substr[..]) {
			self.keyword_token(&substr)
		} else {
			TokenType::Identifier(substr)
		}
	}

	fn scan_number(&mut self) -> TokenType {
		while !self.is_at_end() && (self.peek().is_digit(10)) {
			self.current += 1
		}
		if self.match_advance('.') {
			if !self.peek().is_digit(10) { self.report_error("Number has trailing ."); return TokenType::Error }
			while self.peek().is_digit(10) {
				self.current += 1;
			}
		}
		let n = self.source_substr().parse::<f64>().unwrap();
		TokenType::Number(n)
	}

	fn add_token(&mut self, ttype: TokenType) {
		let s = self.source_substr();
		self.tokens.push(Token::new(ttype, s.clone(), s, self.line));
	}

	fn source_substr(&self) -> String {
		self.source_string[self.start..self.current].to_string()
	}

	fn keyword_token(&mut self, keyword: &str) -> TokenType {
		match keyword {
			"and" => TokenType::And,
			"class" => TokenType::Class,
			"else" => TokenType::Else,
			"false" => TokenType::False,
			"fun" => TokenType::Fun,
			"for" => TokenType::For,
			"if" => TokenType::If,
			"nil" => TokenType::Nil,
			"or" => TokenType::Or,
			// Save myself repl typing by adding a shortened print
			"p" => TokenType::Print,
			"print" => TokenType::Print,
			"return" => TokenType::Return,
			"super" => TokenType::Super,
			"this" => TokenType::This,
			"true" => TokenType::True,
			"var" => TokenType::Var,
			"while" => TokenType::While,
			_ => { self.report_error("Invalid keyword!"); TokenType::Error }
		}
	}
}
