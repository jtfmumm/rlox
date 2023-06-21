// use argparse::{ArgumentParser, StoreTrue};
use std::env;
use std::error::Error;
use std::fmt;
use std::io;
use std::io::Write;
use std::mem;
use std::process;

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
	run(arg.to_string());
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

		self.tokens.push(Token::new(TokenType::EOF, "".to_string(), /*null,*/ self.line));
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

	fn scan_token(&mut self) {
		let ch = self.advance();
		let token_type = match ch {
			'(' => TokenType::LEFT_PAREN,
			')' => TokenType::RIGHT_PAREN,
			'{' => TokenType::LEFT_BRACE,
			'}' => TokenType::RIGHT_BRACE,
			',' => TokenType::COMMA,
			'.' => TokenType::DOT,
			'-' => TokenType::MINUS,
			'+' => TokenType::PLUS,
			';' => TokenType::SEMICOLON,
			'*' => TokenType::STAR,
			_ => { error(self.line, "Unexpected character."); TokenType::ERROR }
		};
		self.add_token(token_type);
	}

	fn add_token(&mut self, ttype: TokenType) {
		self.tokens.push(Token::new(ttype, "".to_string(), /*null,*/ self.line));
	}
}


#[derive(Debug)]
pub enum TokenType {
  // Single-character tokens.
  LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
  COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

  // One or two character tokens.
  BANG, BANG_EQUAL,
  EQUAL, EQUAL_EQUAL,
  GREATER, GREATER_EQUAL,
  LESS, LESS_EQUAL,

  // Literals.
  IDENTIFIER, STRING, NUMBER,

  // Keywords.
  AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
  PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

  EOF,

  // Temporary Error one
  ERROR
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
