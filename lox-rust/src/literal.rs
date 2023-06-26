
#[derive(Debug, Clone)]
pub enum Literal {
	Nil,
	Str(String),
	Num(f64),
	Bool(bool),
}

// impl Literal {
// 	pub fn to_string(&self) -> String {
// 		match self {
// 			Literal::Nil => "nil".to_string(),
// 			Literal::Str(s) => s.clone(),
// 			Literal::Num(n) => n.to_string(),
// 			Literal::Bool(b) => b.to_string(),
// 		}
// 	}
// }
