use crate::lox_error::EvalError;
use crate::object::Object;
use crate::token::{Token, TokenType};

use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::rc::Rc;

#[derive(Debug)]
pub struct Environment {
	env: HashMap<String, Rc<Object>>,
	outer: Option<Rc<RefCell<Environment>>>
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

	pub fn lookup(&self, id: Token, depth: u32) -> Result<Rc<Object>, EvalError> {
		let name = match id.ttype {
			TokenType::Identifier(ref name) => name.clone(),
			_ => return Err(EvalError::new_with_context(id.clone(), &id.to_string(), "Expect variable."))
		};
		if depth > 0 {
			return self.outer.as_ref()
				.expect("Expect variable in environment.")
				.borrow()
				.lookup(id, depth - 1)
		}
		match self.env.get(&name) {
			Some(n) => Ok(n.clone()),
			_ => Err(EvalError::new(&format!("Undefined variable '{}'.", name))
						.with_context(id.clone(), &id.to_string()))
		}
	}

	pub fn assign(&mut self, id: Token, value: Rc<Object>) -> Result<(), EvalError> {
		let name = match id.ttype {
			TokenType::Identifier(ref name) => name.clone(),
			_ => return Err(EvalError::new_with_context(id.clone(), &id.to_string(), "Expect variable."))
		};
		if let Entry::Occupied(mut e) = self.env.entry(name.clone()) {
			e.insert(value);
			Ok(())
		} else {
			match self.outer.take() {
				Some(outer_env) => {
					let res = outer_env.borrow_mut().assign(id, value);
					self.outer = Some(outer_env);
					res
				},
				None => Err(
					EvalError::new(&format!("Undefined variable '{}'.", name))
						.with_context(id.clone(), &id.to_string()))
			}
		}
	}
}
