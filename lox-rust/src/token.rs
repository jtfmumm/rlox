#[derive(Debug)]
pub struct Token {
	ttype: TokenType,
	lexeme: String,
	// literal: ...,
	line: u32,
}

impl Token {
	pub fn new(ttype: TokenType, lexeme: String, /*lit: ...,*/ line: u32) -> Self {
		// let literal = lit.to_string();
		Token { ttype, lexeme, /*literal,*/ line }
	}

	pub fn to_string(&self) -> String {
		format!("{:?} {:?}", self.ttype, self.lexeme/*, literal*/)
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
