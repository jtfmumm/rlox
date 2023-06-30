use crate::lox_error::EvalError;
use crate::object::Object;
use crate::token::{Token, TokenType};

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Environment {
	outer: Option<Rc<RefCell<Environment>>>,
	env: HashMap<String, Rc<Object>>
}

impl Environment {
	pub fn new() -> Self {
		Environment { outer: None, env: HashMap::new() }
	}

	pub fn from_outer(outer: Rc<RefCell<Environment>>) -> Self {
		Environment { outer: Some(outer), env: HashMap::new() }
	}

	pub fn add_scope(outer: Rc<RefCell<Environment>>) -> Rc<RefCell<Environment>> {
		Rc::new(RefCell::new(Environment::from_outer(outer)))
	}

	pub fn remove_scope(&self) -> Result<Rc<RefCell<Environment>>,EvalError> {
	    match &self.outer {
			Some(env) => Ok(env.clone()),
			None => Err(EvalError::new("Tried to remove non-existent scope from environment!"))
		}
	}

	pub fn declare(&mut self, name: &str, value: Rc<Object>)  {
		self.env.insert(name.to_string(), value);
	}

	pub fn lookup_with_backup_env(&self, id: Token, env: Rc<RefCell<Environment>>) -> Result<Rc<Object>, EvalError> {
		match self.lookup(id.clone()) {
			Ok(res) => Ok(res),
			Err(_) => env.borrow_mut().lookup(id)
		}
	}

	pub fn lookup(&self, id: Token) -> Result<Rc<Object>, EvalError> {
		let name = match id.ttype {
			TokenType::Identifier(ref name) => name.clone(),
			_ => return Err(EvalError::new_with_context(id.clone(), &id.to_string(), "Expect variable."))
		};
		if self.env.contains_key(&name) {
			Ok(self.env.get(&name).unwrap().clone())
		} else {
			match self.outer {
				Some(ref outer_env) => {
					outer_env.borrow().lookup(id)
				},
				None => Err(
					EvalError::new(&format!("Undefined variable '{}'.", name.to_string()))
						.with_context(id.clone(), &id.to_string()))
			}
		}
	}

	pub fn assign(&mut self, id: Token, value: Rc<Object>) -> Result<(), EvalError> {
		let name = match id.ttype {
			TokenType::Identifier(ref name) => name.clone(),
			_ => return Err(EvalError::new_with_context(id.clone(), &id.to_string(), "Expect variable."))
		};
		if self.env.contains_key(&name) {
			self.env.insert(name.to_string(), value);
			Ok(())
		} else {
			match self.outer.take() {
				Some(outer_env) => {
					let res = outer_env.borrow_mut().assign(id, value);
					self.outer = Some(outer_env);
					res
				},
				None => Err(
					EvalError::new(&format!("Undefined variable '{}'.", name.to_string()))
						.with_context(id.clone(), &id.to_string()))
			}
		}
	}
}
