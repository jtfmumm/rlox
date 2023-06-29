use crate::cerror::{EvalError, LoxError};
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
	pub fn new() -> Self {
		Interpreter { is_repl: false, env: Rc::new(RefCell::new(Environment::new())) }
	}

	pub fn repl(&mut self) {
		self.is_repl = true;
	}

	pub fn interpret(&mut self, stmts: Vec<Rc<Stmt>>) -> Result<(),LoxError> {
		let mut hit_error = false;
		for stmt in stmts {
			match self.execute(stmt) {
				Ok(lit) => {
					if self.is_repl {
						println!("val: {}", stringify_cli_result(&lit));
					}
				},
				Err(err) => {
					match err {
						EvalError::WithoutContext { msg } => {
							eprintln!("{}", msg);
						},
						EvalError::WithContext { .. } => {}
					}
					hit_error = true // println!("\n\x1b[1;31merror\x1b[0m: could not interpret due to previous error");
				}
			}
		}
		if hit_error { Err(LoxError::Runtime) } else { Ok(()) }
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
					_ => Err(EvalError::new("Invalid assignment target."))
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
			BlockStmt { stmts } => {
				self.env = Environment::add_scope(self.env.clone());
				match self.interpret(stmts.to_vec()) {
					Ok(()) => {
						self.env = self.env.clone().borrow().remove_scope()?;
						Ok(Object::Nil)
					},
					Err(_) => Err(EvalError::new("Failed while evaluating block."))
				}
			},
			WhileStmt { condition, block } => {
				while is_truthy(&self.evaluate(condition)?) {
					self.execute(block.clone())?;
				}
				Ok(Object::Nil)
			},
			// ForStmt { init, condition, inc, block } => {

			// 	// while is_truthy(&self.evaluate(condition)?) {
			// 	// 	self.execute(block.clone())?;
			// 	// }
			// 	// Ok(Object::Nil)
			// },
			IfStmt { conditionals, else_block } => {
				for (c, blk) in conditionals.iter() {
					if is_truthy(&self.evaluate(c)?) {
						return self.execute(blk.clone())
					}
				}
				if let Some(blk) = &*else_block.clone() {
					self.execute(blk.clone())
				} else {
					Ok(Object::Nil)
				}
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
			Logic { ref left, ref operator, ref right } => {
				match self.eval_logic(left, operator, right) {
					Ok(exp) => Ok(exp),
					Err(everr) => Err(everr.with_context(operator.clone(), &expr.to_string())),
				}
			},
			Grouping { ref expression } => {
				self.eval_grouping(expression)
			},
			Block { ref expression } => {
				self.eval_block(expression)
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

	pub fn eval_block(&mut self, expr: &Expr) -> Result<Object, EvalError> {
		// TODO: It seems this code will never run. I'll stick this here for now.
		assert!(false);
		self.env = Environment::add_scope(self.env.clone());
		let obj = self.evaluate(expr);
		self.env = self.env.clone().borrow().remove_scope()?;
		obj
	}

	pub fn eval_unary(&mut self, op: &Token, right: &Expr) -> Result<Object, EvalError> {
		let r = self.evaluate(right)?;

		use TokenType::*;
		use self::Object::*;
		match &op.ttype {
			Minus => Ok(Num(-as_num(r)?)),
			Bang => Ok(Bool(!(is_truthy(&r)))),
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

	pub fn eval_logic(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<Object, EvalError> {
		debug_assert!(operator.ttype == TokenType::And || operator.ttype == TokenType::Or);
		let l = self.evaluate(left)?;

		if operator.ttype == TokenType::And {
			return Ok(if !is_truthy(&l) { l } else { self.evaluate(right)? })
		} else {
			return Ok(if !is_truthy(&l) { self.evaluate(right)? } else { l })
		}
	}
}

fn eval_plus(l: Object, r: Object) -> Result<Object, EvalError> {
	use self::Object::*;
	let err = Err(EvalError::new("Operands must be two numbers or two strings."));
	match l {
		Num(n) => {
			let n2 = as_num(r);
			if n2.is_err() { return err }
			Ok(Num(n + n2.unwrap()))
		},
		Str(s) => {
			let s2 = &as_str(r);
			if s2.is_err() { return err }
			Ok(Str(s + &s2.as_ref().unwrap()))
		},
		_ => err
	}
}

fn eval_div(l: Object, r: Object) -> Result<Object, EvalError> {
	let divisor = as_num(r)?;
	if divisor == 0.0  {
		return Err(EvalError::new(&format!("Tried to divide by 0!")))
	}
	let res = as_num(l)? / divisor;
	Ok(Object::Num(res))
}

fn is_truthy(obj: &Object) -> bool {
	use self::Object::*;
	match obj {
		Bool(b) => *b,
		Num(_) | Str(_) => true,
		Nil => false,
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
		_ => false,
 	}
}

// fn as_bool(obj: Object) -> Result<bool, EvalError> {
// 	use self::Object::*;
// 	match obj {
// 		Bool(b) => Ok(b),
// 		_ => Err(EvalError::new("Operands must be Booleans"))
// 	}
// }

fn as_num(obj: Object) -> Result<f64, EvalError> {
	use self::Object::*;
	match obj {
		Num(n) => Ok(n),
		_ => Err(EvalError::new("Operands must be numbers."))
	}
}

fn as_str(obj: Object) -> Result<String, EvalError> {
	use self::Object::*;
	match obj {
		Str(s) => Ok(s.to_owned()),
		_ => Err(EvalError::new("Operands must be strings."))
	}
}
