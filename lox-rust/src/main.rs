// use argparse::{ArgumentParser, StoreTrue};
use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::io::Write;
use std::mem;
use std::process;

const KEYWORDS: [&'static str; 16] = ["and", "class", "else", "false", "fun", "for", "if",
                     		 "nil", "or", "print", "return", "super", "this", "true",
                     		 "var", "while"];

fn main() {
  println!("\n");
  let args: Vec<_> = env::args().collect();
  if args.len() > 2 {
  	println!("Usage: jlox [script]");
  	process::exit(64);
  } else if args.len() == 2 {
  	run_file(&args[1]);
  } else {
  	match run_prompt() {
  		Ok(_) => println!("Tot ziens"),
  		Err(_) => error(0, "Something went wrong!")
  	}
  }
}

fn run_file(arg: &str) {
    let contents = fs::read_to_string(arg)
        .expect("Should have been able to read the file");

	run(contents);
}

fn run_prompt() -> io::Result<()> {
    let exit_string = "x\n".to_string();
	loop {
		print!("> ");
		io::stdout().flush()?;
	    let mut user_input = String::new();
	    let stdin = io::stdin();
	    stdin.read_line(&mut user_input)?;
	    match user_input {
	    	s if s == exit_string => break,
	    	_ => {}
	    }
	    run(user_input)
	}

    Ok(())
}

fn run(source: String) {
	let mut scanner = Scanner::new(source);
	let tokens = scanner.scan_tokens();
	for t in tokens {
		println!("{:?}", t);
	}
}

#[derive(Debug)]
pub struct Token {
	ttype: TokenType,
	lexeme: String,
	// literal: ...,
	line: u32,
}

impl Token {
	fn new(ttype: TokenType, lexeme: String, /*lit: ...,*/ line: u32) -> Self {
		// let literal = lit.to_string();
		Token { ttype, lexeme, /*literal,*/ line }
	}

	fn to_string(&self) -> String {
		format!("{:?} {:?}", self.ttype, self.lexeme/*, literal*/)
	}
}


pub struct Scanner {
	source_string: String,
	source: Vec<char>,
	tokens: Vec<Token>,
	start: usize,
	current: usize,
	line: u32,
}

impl Scanner {
	fn new(s: String) -> Self {
		let source = s.chars().collect();
		let start = 0;
		let current = 0;
		let line = 1;
		Scanner { source_string: s, source, tokens: Vec::new(), start, current, line }
	}

	fn scan_tokens(&mut self) -> Vec<Token> {
		while !self.is_at_end() {
			self.start = self.current;
			self.scan_token()
		}

		self.tokens.push(Token::new(TokenType::Eof, "".to_string(), /*null,*/ self.line));
		mem::replace(&mut self.tokens, Vec::new())
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
				} else {
					TokenType::Slash
				}
			}
			'"' => self.scan_string(),
			a if a.is_alphabetic() => self.scan_word(),
			d if d.is_digit(10) => self.scan_number(),
			' ' | '\r' | '\t' => return,
			'\n' => { self.line += 1; return },
			_ => { error(self.line, "Unexpected character."); TokenType::Error }
		};
		self.add_token(token_type);
	}

	fn scan_string(&mut self) -> TokenType {
		while !self.match_advance('"') {
			if self.is_at_end() { error(self.line, "Unterminated string!"); return TokenType::Error }
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
			if !self.peek().is_digit(10) { error(self.line, "Number has trailing ."); return TokenType::Error }
			while self.peek().is_digit(10) {
				self.current += 1;
			}
		}
		let n = self.source_substr().parse::<f64>().unwrap();
		TokenType::Number(n)
	}
	// // literals.,
	// Number,

	fn add_token(&mut self, ttype: TokenType) {
		let s = self.source_substr();
		self.tokens.push(Token::new(ttype, s, /*null,*/ self.line));
	}

	fn source_substr(&self) -> String {
		self.source_string[self.start..self.current].to_string()
	}

	fn keyword_token(&self, keyword: &str) -> TokenType {
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
			"print" => TokenType::Print,
			"return" => TokenType::Return,
			"super" => TokenType::Super,
			"this" => TokenType::This,
			"true" => TokenType::True,
			"var" => TokenType::Var,
			"while" => TokenType::While,
			_ => { error(self.line, "Invalid keyword!"); TokenType::Error }
		}
	}
}



#[derive(Debug)]
pub enum TokenType {
	// single-character tokens.,
	LeftParen, RightParen, LeftBrace, RightBrace,
	Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

	// one or two character tokens.,
	Bang, BangEqual,
	Equal, EqualEqual,
	Greater, GreaterEqual,
	Less, LessEqual,

	// literals.,
	Identifier(String), StringLit(String), Number(f64),

	// keywords.,
	And, Class, Else, False, Fun, For, If, Nil, Or,
	Print, Return, Super, This, True, Var, While,

	Eof,

	// temporary error one,
	Error,
}


fn error(line_n: u32, msg: &str) {
	report(line_n, "", msg);
}

fn report(line_n: u32, location: &str, msg: &str) {
	eprintln!("[line: {:?}] Error {:?}: {:?}", line_n, location, msg);
}


#[derive(Debug, Clone)]
struct GenError;

impl fmt::Display for GenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fake error!")
    }
}

impl Error for GenError {}
