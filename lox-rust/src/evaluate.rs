use crate::cerror::{everror, EvalError};
use crate::expr::Expr;
use crate::literal::Literal;
use crate::token::{Token, TokenType};

use std::rc::Rc;

pub fn evaluate(expr: &Expr) -> Result<Literal, EvalError> {
	use Expr::*;

	match expr {
		Binary { ref left, ref operator, ref right } => {
			eval_binary(left, operator, right)
		},
		Grouping { ref expression } => {
			eval_grouping(expression)
		},
		Literal { ref value } => {
			use self::Literal::*;
			Ok(match value {
				Nil => Nil.clone(),
				Bool(b) => Bool(*b),
				Num(n) => Num(*n),
				Str(s) => Str(s.clone()),
			})
		},
		Unary { ref operator, ref right } => {
			eval_unary(operator, right)
		},
	}
}

pub fn eval_grouping(expr: &Expr) -> Result<Literal, EvalError> {
	evaluate(expr)
}

pub fn eval_unary(op: &Token, right: &Expr) -> Result<Literal, EvalError> {
	let r = evaluate(right)?;

	use TokenType::*;
	use self::Literal::*;
	match &op.ttype {
		Minus => Ok(Num(-as_num(r)?)),
		Bang => Ok(Bool(!(is_truthy(r)))),
		tt => Err(everror(&format!("eval_unary: Invalid operator! {:?}", tt)))
	}
}

pub fn eval_binary(left: &Expr, operator: &Token, right: &Expr) -> Result<Literal, EvalError> {
	let l = evaluate(left)?;
	let r = evaluate(right)?;

	use TokenType::*;
	use self::Literal::*;
	match &operator.ttype {
		Minus => Ok(Num(as_num(l)? - as_num(r)?)),
		// TODO: Handle concat for Strings! "3" + "rd"
		Plus => Ok(Num(as_num(l)? + as_num(r)?)),
		Slash => Ok(Num(as_num(l)? / as_num(r)?)),
		Star => Ok(Num(as_num(l)? * as_num(r)?)),
		// TODO: In Lox you can compare different types, but
		// that returns false.
		// BangEqual => ,
		// EqualEqual => ,
		Greater => Ok(Bool(as_num(l)? > as_num(r)?)),
		GreaterEqual => Ok(Bool(as_num(l)? >= as_num(r)?)),
		Less => Ok(Bool(as_num(l)? < as_num(r)?)),
		LessEqual => Ok(Bool(as_num(l)? <= as_num(r)?)),
		tt => Err(everror(&format!("eval_binary: Invalid operator! {:?}", tt)))
	}
}

fn is_truthy(lit: Literal) -> bool {
	use self::Literal::*;
	match lit {
		Bool(b) => b,
		Num(_) | Str(_) => true,
		Nil => false,
	}
}

fn as_bool(lit: Literal) -> Result<bool, EvalError> {
	use self::Literal::*;
	match lit {
		Bool(b) => Ok(b),
		_ => Err(everror(&format!("Expected Boolean, got {:?}", lit)))
	}
}

fn as_num(lit: Literal) -> Result<f64, EvalError> {
	use self::Literal::*;
	match lit {
		Num(n) => Ok(n),
		_ => Err(everror(&format!("Expected number, got {:?}", lit)))
	}
}

fn as_string(lit: Literal) -> Result<String, EvalError> {
	use self::Literal::*;
	match lit {
		Str(s) => Ok(s.to_owned()),
		_ => Err(everror(&format!("Expected String, got {:?}", lit)))
	}
}
