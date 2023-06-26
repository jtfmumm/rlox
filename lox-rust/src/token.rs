#[derive(Debug, PartialEq, Clone)]
pub struct Token {
	pub ttype: TokenType,
	pub lexeme: String,
	pub literal: String,
	pub line: u32,
}

impl Token {
	pub fn new(ttype: TokenType, lexeme: String, literal: String, line: u32) -> Self {
		// let literal = lit.to_string();
		Token { ttype, lexeme, literal, line }
	}

	// pub fn to_string(&self) -> String {
	// 	// format!("{:?} {:?}", self.ttype, self.lexeme/*, literal*/)
	// 	format!("{:}", self.lexeme/*, literal*/)
	// }
}

#[derive(Debug, PartialEq, Clone)]
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

	Eof, Sof,

	// temporary error one,
	Error,
}
