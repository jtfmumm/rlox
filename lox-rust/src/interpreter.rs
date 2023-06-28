use crate::cerror::EvalError;
use crate::environment::Environment;
use crate::expr::Expr;
use crate::object::{Object, stringify_cli_result};
use crate::stmt::Stmt;
use crate::token::{Token, TokenType};

use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
	is_repl: bool,
	env: Rc<RefCell<Environment>>
}

impl Interpreter {
	pub fn new(is_repl: bool) -> Self {
		Interpreter { is_repl, env: Rc::new(RefCell::new(Environment::new())) }
	}

	pub fn interpret(&mut self, stmts: Vec<Rc<Stmt>>) -> Result<(),()> {
		let mut hit_error = false;
		for stmt in stmts {
			match self.execute(stmt) {
				Ok(lit) => {
					if self.is_repl { println!("val: {}", stringify_cli_result(&lit)); }
				},
				Err(_) => {
					println!("\n\x1b[1;31merror\x1b[0m: could not interpret due to previous error");
					hit_error = true;
				},
			}
		}
		if hit_error { Ok(()) } else { Err(()) }
	}

	fn execute(&mut self, stmt: Rc<Stmt>) -> Result<Object, EvalError> {
		use Stmt::*;
		match &*stmt {
			VarDeclStmt { variable, value } => {
				match &*variable.clone() {
					Expr::Variable { name } => {
						let val = self.evaluate(&value)?;
						self.env.borrow_mut().declare(name, val)?;
						Ok(Object::Nil)
					},
					_ => Err(EvalError::new("Expect declaration to declare variable."))
				}
			},
			AssignStmt { variable, value } => {
				match &*variable.clone() {
					Expr::Variable { name } => {
						let val = self.evaluate(&value)?;
						self.env.borrow_mut().assign(name, val)?;
						Ok(Object::Nil)
					},
					_ => Err(EvalError::new("Expect assignment to assign to variable."))
				}
			},
			ExprStmt { expr } => {
				self.evaluate(&expr)
			},
			PrintStmt { expr } => {
				let obj = self.evaluate(&expr)?;
				println!("{}", stringify_cli_result(&obj));
				Ok(Object::Nil)
			}
		}
	}

	pub fn evaluate(&mut self, expr: &Expr) -> Result<Object, EvalError> {
		use Expr::*;

		match expr {
			Binary { ref left, ref operator, ref right } => {
				match self.eval_binary(left, operator, right) {
					Ok(exp) => Ok(exp),
					Err(everr) => Err(everr.with_context(operator.clone(), &expr.to_string())),
				}
			},
			Grouping { ref expression } => {
				self.eval_grouping(expression)
			},
			Literal { ref value } => {
				use self::Object::*;
				Ok(match value {
					Nil => Nil.clone(),
					Bool(b) => Bool(*b),
					Num(n) => Num(*n),
					Str(s) => Str(s.clone()),
					// TODO: This shouldn't happen
					// Variable { name } => Variable { name: name.clone() },
				})
			},
			Variable { ref name } => {
				Ok(self.env.borrow_mut().lookup(name)?)
			},
			Unary { ref operator, ref right } => {
				match self.eval_unary(operator, right) {
					Ok(exp) => Ok(exp),
					Err(everr) => Err(everr.with_context(operator.clone(), &expr.to_string())),
				}
			},
		}
	}

	pub fn eval_grouping(&mut self, expr: &Expr) -> Result<Object, EvalError> {
		self.evaluate(expr)
	}

	pub fn eval_unary(&mut self, op: &Token, right: &Expr) -> Result<Object, EvalError> {
		let r = self.evaluate(right)?;

		use TokenType::*;
		use self::Object::*;
		match &op.ttype {
			Minus => Ok(Num(-as_num(r)?)),
			Bang => Ok(Bool(!(is_truthy(r)))),
			tt => Err(EvalError::new(&format!("eval_unary: Invalid operator! {:?}", tt)))
		}
	}

	pub fn eval_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<Object, EvalError> {
		let l = self.evaluate(left)?;
		let r = self.evaluate(right)?;

		use TokenType::*;
		use self::Object::*;
		match &operator.ttype {
			Minus => Ok(Num(as_num(l)? - as_num(r)?)),
			// TODO: Handle concat for Strings! "3" + "rd"
			Plus => Ok(eval_plus(l, r)?),
			Slash => Ok(eval_div(l, r)?),
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
}

fn eval_plus(l: Object, r: Object) -> Result<Object, EvalError> {
	use self::Object::*;
	match l {
		Num(n) => Ok(Num(n + as_num(r)?)),
		Str(s) => Ok(Str(s + &as_str(r)?)),
		// TODO: Fix variable case
		_ => Err(EvalError::new(&format!("eval_plus: Tried to add {:?} and {:?}!", l, r)))
	}
}

fn eval_div(l: Object, r: Object) -> Result<Object, EvalError> {
	let divisor = as_num(r)?;
	if divisor == 0.0 {
		return Err(EvalError::new(&format!("eval_div: Tried to divide by 0!")))
	}
	let res = as_num(l)? / divisor;
	Ok(Object::Num(res))
}

fn is_truthy(obj: Object) -> bool {
	use self::Object::*;
	match obj {
		Bool(b) => b,
		Num(_) | Str(_) => true,
		Nil => false,
		// // TODO: Fix self!
		// Variable { .. } => false
	}
}

// In Lox you can compare different types, but
// that returns false.
fn is_equal(l: Object, r: Object) -> bool {
	use self::Object::*;
	match (l, r) {
		(Nil, Nil) => true,
		(Num(n1), Num(n2)) => n1 == n2,
		(Str(s1), Str(s2)) => s1 == s2,
		(Bool(b1), Bool(b2)) => b1 == b2,
		// TODO: Fix variable case!
		_ => false,
 	}
}

fn as_bool(obj: Object) -> Result<bool, EvalError> {
	use self::Object::*;
	match obj {
		Bool(b) => Ok(b),
		_ => Err(EvalError::new(&format!("Expected Boolean, got {:?}", obj)))
	}
}

fn as_num(obj: Object) -> Result<f64, EvalError> {
	use self::Object::*;
	match obj {
		Num(n) => Ok(n),
		_ => Err(EvalError::new(&format!("Expected number, got {:?}", obj)))
	}
}

fn as_str(obj: Object) -> Result<String, EvalError> {
	use self::Object::*;
	match obj {
		Str(s) => Ok(s.to_owned()),
		_ => Err(EvalError::new(&format!("Expected String, got {:?}", obj)))
	}
}
