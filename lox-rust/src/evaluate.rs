use crate::cerror::EvalError;
use crate::expr::Expr;
use crate::literal::Literal;
use crate::token::{Token, TokenType};

pub fn evaluate(expr: &Expr) -> Result<Literal, EvalError> {
	use Expr::*;

	match expr {
		Binary { ref left, ref operator, ref right } => {
			let res = eval_binary(left, operator, right);
			match res {
				Ok(lit) => Ok(lit),
				Err(everr) => Err(everr.with_context(operator.clone())),
			}
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
			let res = eval_unary(operator, right);
			match res {
				Ok(lit) => Ok(lit),
				Err(everr) => Err(everr.with_context(operator.clone())),
			}
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
		tt => Err(EvalError::new(&format!("eval_unary: Invalid operator! {:?}", tt)))
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
		Plus => Ok(eval_plus(l, r)?),
		Slash => Ok(Num(as_num(l)? / as_num(r)?)),
		Star => Ok(Num(as_num(l)? * as_num(r)?)),
		EqualEqual => Ok(Bool(is_equal(l, r))),
		BangEqual => Ok(Bool(!is_equal(l, r))),
		Greater => Ok(Bool(as_num(l)? > as_num(r)?)),
		GreaterEqual => Ok(Bool(as_num(l)? >= as_num(r)?)),
		Less => Ok(Bool(as_num(l)? < as_num(r)?)),
		LessEqual => Ok(Bool(as_num(l)? <= as_num(r)?)),
		tt => Err(EvalError::new(&format!("eval_binary: Invalid operator! {:?}", tt)))
	}
}

fn eval_plus(l: Literal, r: Literal) -> Result<Literal, EvalError> {
	use self::Literal::*;
	match l {
		Num(n) => Ok(Num(n + as_num(r)?)),
		Str(s) => Ok(Str(s + &as_str(r)?)),
		_ => Err(EvalError::new(&format!("eval_plus: Tried to add {:?} and {:?}!", l, r)))
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

// In Lox you can compare different types, but
// that returns false.
fn is_equal(l: Literal, r: Literal) -> bool {
	use self::Literal::*;
	match (l, r) {
		(Nil, Nil) => true,
		(Num(n1), Num(n2)) => n1 == n2,
		(Str(s1), Str(s2)) => s1 == s2,
		(Bool(b1), Bool(b2)) => b1 == b2,
		_ => false,
 	}
}

fn as_bool(lit: Literal) -> Result<bool, EvalError> {
	use self::Literal::*;
	match lit {
		Bool(b) => Ok(b),
		_ => Err(EvalError::new(&format!("Expected Boolean, got {:?}", lit)))
	}
}

fn as_num(lit: Literal) -> Result<f64, EvalError> {
	use self::Literal::*;
	match lit {
		Num(n) => Ok(n),
		_ => Err(EvalError::new(&format!("Expected number, got {:?}", lit)))
	}
}

fn as_str(lit: Literal) -> Result<String, EvalError> {
	use self::Literal::*;
	match lit {
		Str(s) => Ok(s.to_owned()),
		_ => Err(EvalError::new(&format!("Expected String, got {:?}", lit)))
	}
}
