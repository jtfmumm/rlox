use crate::expr::Expr;
use crate::evaluate::evaluate;
use crate::literal::Literal;

use std::rc::Rc;

pub fn interpret(expr: Rc<Expr>) {
	match evaluate(&expr) {
		Ok(lit) => println!("{}", stringify_result(lit)),
		Err(_) => println!("\n\x1b[1;31merror\x1b[0m: could not interpret due to previous error") //println!("\nError: {:}", err)
	}
}

fn stringify_result(lit: Literal) -> String {
	let s = format!("{}", lit);
	if s.ends_with(".0") {
		s[0..s.len() - 2].to_string()
	} else {
		s
	}
}
