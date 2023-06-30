use crate::builtins::{ClockFn, StrFn};
use crate::lox_error::{EvalError, LoxError};
use crate::environment::Environment;
use crate::expr::Expr;
use crate::object::{Object, stringify_cli_result};
use crate::stmt::Stmt;
use crate::token::{Token, TokenType};

use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
	is_repl: bool,
	global_env: Rc<RefCell<Environment>>,
	local_env: Rc<RefCell<Environment>>
}

impl Interpreter {
	pub fn new() -> Self {
		let global_env = Rc::new(RefCell::new(Environment::new()));
		let local_env = Rc::new(RefCell::new(Environment::from_outer(global_env.clone())));
		global_env.borrow_mut().declare("clock", Rc::new(Object::Fun(Rc::new(ClockFn {}))));
		global_env.borrow_mut().declare("str", Rc::new(Object::Fun(Rc::new(StrFn {}))));

		Interpreter { is_repl: false, global_env, local_env }
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
					err.report();
					hit_error = true;
				}
			}
		}
		if hit_error { Err(LoxError::Runtime) } else { Ok(()) }
	}

	fn execute(&mut self, stmt: Rc<Stmt>) -> Result<Rc<Object>, EvalError> {
		use Stmt::*;
		match &*stmt {
			BlockStmt { stmts } => {
				self.local_env = Environment::add_scope(self.local_env.clone());
				match self.interpret(stmts.to_vec()) {
					Ok(()) => {
						self.local_env = self.local_env.clone().borrow().remove_scope()?;
						Ok(Rc::new(Object::Nil))
					},
					Err(_) => Err(EvalError::new("Failed while evaluating block."))
				}
			},
			ExprStmt { expr } => {
				self.evaluate(&expr)
			},
			ForStmt { init, condition, inc, block } => {
				self.local_env = Environment::add_scope(self.local_env.clone());
				if let Some(stmt) = &*init.clone() { self.execute(stmt.clone())?; }
				let cond = if let Some(ref exp) = &*condition.clone() {
					exp.clone()
				} else {
					Expr::literal(Rc::new(Object::Bool(true)))
				};
				while is_truthy(&self.evaluate(&cond)?) {
					self.execute(block.clone())?;
					if let Some(expr) = &*inc.clone() { self.evaluate(&*expr.clone())?; }
				}
				self.local_env = self.local_env.clone().borrow().remove_scope()?;
				Ok(Rc::new(Object::Nil))
			},
			FunStmt { name, params, body } => {

				Err(EvalError::new("Not implemented yet!"))
			},
			IfStmt { conditionals, else_block } => {
				for (c, blk) in conditionals.iter() {
					if is_truthy(&self.evaluate(c)?) {
						return self.execute(blk.clone())
					}
				}
				if let Some(blk) = &*else_block.clone() {
					self.execute(blk.clone())
				} else {
					Ok(Rc::new(Object::Nil))
				}
			},
			PrintStmt { expr } => {
				let obj = self.evaluate(&expr)?;
				println!("{}", stringify_cli_result(&obj));
				Ok(Rc::new(Object::Nil))
			},
			VarDeclStmt { variable, value } => {
				match &*variable.clone() {
					Expr::Variable { name } => {
						let val = self.evaluate(&value)?;
						match name.ttype {
							TokenType::Identifier(ref nm) => {
								self.local_env.borrow_mut().declare(nm, val.clone());
							},
							_ => unreachable!()
						}
						Ok(Rc::new(Object::Nil))
					},
					_ => Err(EvalError::new("Expect declaration to declare variable."))
				}
			},
			WhileStmt { condition, block } => {
				while is_truthy(&self.evaluate(condition)?) {
					self.execute(block.clone())?;
				}
				Ok(Rc::new(Object::Nil))
			},
		}
	}

	pub fn evaluate(&mut self, expr: &Expr) -> Result<Rc<Object>, EvalError> {
		use Expr::*;

		match expr {
			Assign { variable, value } => {
				match &*variable.clone() {
					Expr::Variable { name } => {
						let val = self.evaluate(&value)?;
						self.local_env.borrow_mut().assign(name.clone(), val.clone())?;
						Ok(val)
					},
					_ => Err(EvalError::new("Invalid assignment target."))
				}
			},
			Binary { ref left, ref operator, ref right } => {
				match self.eval_binary(left, operator, right) {
					Ok(exp) => Ok(exp),
					Err(everr) => Err(everr.with_context(operator.clone(), &expr.to_string())),
				}
			},
			Call { ref callee, ref paren, ref args } => {
				self.eval_call(callee, paren, args)
			},
			Grouping { ref expression } => {
				self.eval_grouping(expression)
			},
			Literal { ref value } => {
				use self::Object::*;
				Ok(Rc::new(match &*value.clone() {
					Nil => Nil,
					Bool(b) => Bool(*b),
					Num(n) => Num(*n),
					Str(s) => Str(s.clone()),
					Fun(f) => Fun(f.clone()),
				}))
			},
			Logic { ref left, ref operator, ref right } => {
				match self.eval_logic(left, operator, right) {
					Ok(exp) => Ok(exp),
					Err(everr) => Err(everr.with_context(operator.clone(), &expr.to_string())),
				}
			},
			Unary { ref operator, ref right } => {
				match self.eval_unary(operator, right) {
					Ok(exp) => Ok(exp),
					Err(everr) => Err(everr.with_context(operator.clone(), &expr.to_string())),
				}
			},
			Variable { ref name } => {
				Ok(self.local_env.borrow_mut().lookup(name.clone())?)
			},
		}
	}

	pub fn eval_grouping(&mut self, expr: &Expr) -> Result<Rc<Object>, EvalError> {
		self.evaluate(expr)
	}

	pub fn eval_call(&mut self, callee: &Expr, paren: &Token, args: &Vec<Rc<Expr>>) -> Result<Rc<Object>, EvalError> {
		match &*self.evaluate(callee)?.clone() {
			Object::Fun(f) => {
				if args.len() != f.arity() {
					return Err(EvalError::new_with_context(paren.clone(), &callee.to_string(),
						&format!("Expected {} arguments but got {}.", f.arity(), args.len())))
				}
				let mut obj_args = Vec::new();
				for arg in args.iter() {
					obj_args.push(self.evaluate(arg)?);
				}
				Ok(f.call(self, &obj_args))
			},
			_ => Err(EvalError::new_with_context(paren.clone(), &callee.to_string(),
				"Can only call functions and classes."))
		}
	}

	pub fn eval_unary(&mut self, op: &Token, right: &Expr) -> Result<Rc<Object>, EvalError> {
		let r = self.evaluate(right)?;

		use TokenType::*;
		use self::Object::*;
		Ok(Rc::new(match &op.ttype {
			Bang => Bool(!(is_truthy(&r))),
			Minus => {
				match &*r.clone() {
					Num(n) => Num(-n),
					_ => return Err(EvalError::new("Operand must be a number."))
				}
			}
			tt => return Err(EvalError::new(&format!("eval_unary: Invalid operator! {:?}", tt)))
		}))
	}

	pub fn eval_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<Rc<Object>, EvalError> {
		let l = self.evaluate(left)?;
		let r = self.evaluate(right)?;

		use TokenType::*;
		use self::Object::*;
		Ok(Rc::new(match &operator.ttype {
			BangEqual => Bool(!is_equal(l, r)),
			EqualEqual => Bool(is_equal(l, r)),
			Greater => Bool(as_num(l)? > as_num(r)?),
			GreaterEqual => Bool(as_num(l)? >= as_num(r)?),
			Less => Bool(as_num(l)? < as_num(r)?),
			LessEqual => Bool(as_num(l)? <= as_num(r)?),
			Minus => Num(as_num(l)? - as_num(r)?),
			Plus => eval_plus(l, r)?,
			Slash => eval_div(l, r)?,
			Star => Num(as_num(l)? * as_num(r)?),
			tt => return Err(EvalError::new(&format!("eval_binary: Invalid operator! {:?}", tt)))
		}))
	}

	pub fn eval_logic(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<Rc<Object>, EvalError> {
		debug_assert!(operator.ttype == TokenType::And || operator.ttype == TokenType::Or);
		let l = self.evaluate(left)?;

		if operator.ttype == TokenType::And {
			return Ok(if !is_truthy(&l) { l } else { self.evaluate(right)? })
		} else {
			return Ok(if !is_truthy(&l) { self.evaluate(right)? } else { l })
		}
	}
}

fn eval_plus(l: Rc<Object>, r: Rc<Object>) -> Result<Object, EvalError> {
	use self::Object::*;
	let err = Err(EvalError::new("Operands must be two numbers or two strings."));
	match &*l {
		Num(n) => {
			let n2 = as_num(r);
			if n2.is_err() { return err }
			Ok(Num(n + n2.unwrap()))
		},
		Str(s) => {
			let s2 = &as_str(r);
			if s2.is_err() { return err }
			Ok(Str(s.to_string() + s2.as_ref().unwrap()))
		},
		_ => err
	}
}

fn eval_div(l: Rc<Object>, r: Rc<Object>) -> Result<Object, EvalError> {
	let divisor = as_num(r)?;
	if divisor == 0.0  {
		return Err(EvalError::new(&format!("Tried to divide by 0!")))
	}
	let res = as_num(l)? / divisor;
	Ok(Object::Num(res))
}

fn is_truthy(obj: &Rc<Object>) -> bool {
	use self::Object::*;
	match &*obj.clone() {
		Bool(b) => *b,
		Num(_) | Str(_) | Fun(_) => true,
		Nil => false,
	}
}

// In Lox you can compare different types, but
// that returns false.
fn is_equal(l: Rc<Object>, r: Rc<Object>) -> bool {
	use self::Object::*;
	match (&*l, &*r) {
		(Bool(b1), Bool(b2)) => b1 == b2,
		(Num(n1), Num(n2)) => n1 == n2,
		(Str(s1), Str(s2)) => s1 == s2,
		(Nil, Nil) => true,
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

fn as_num(obj: Rc<Object>) -> Result<f64, EvalError> {
	use self::Object::*;
	match &*obj {
		Num(n) => Ok(*n),
		_ => Err(EvalError::new("Operands must be numbers."))
	}
}

fn as_str(obj: Rc<Object>) -> Result<String, EvalError> {
	use self::Object::*;
	match &*obj {
		Str(s) => Ok(s.to_owned()),
		_ => Err(EvalError::new("Operands must be strings."))
	}
}
