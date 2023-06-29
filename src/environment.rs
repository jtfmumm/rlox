use crate::cerror::EvalError;
use crate::object::Object;
use crate::token::{Token, TokenType};

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Environment {
	outer: Option<Rc<RefCell<Environment>>>,
	env: HashMap<String, Object>
}

impl Environment {
	pub fn new() -> Self {
		Environment { outer: None, env: HashMap::new() }
	}

	fn from_outer(outer: Rc<RefCell<Environment>>) -> Self {
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

	pub fn declare(&mut self, id: Token, value: Object) -> Result<(), EvalError>  {
		let name = match id.ttype {
			TokenType::Identifier(name) => name.clone(),
			_ => return Err(EvalError::new(&format!("Expect variable, got '{}'.", id.clone())))
		};
		self.env.insert(name, value);
		Ok(())
	}

	pub fn lookup(&self, id: Token) -> Result<Object, EvalError> {
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

				// None => Err(EvalError::new_with_context(id.clone(), &id.to_string(), "Undefined variable."))
			}
		}
	}

	pub fn assign(&mut self, id: Token, value: Object) -> Result<(), EvalError> {
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
				// None => Err(EvalError::new_with_context(id.clone(), &id.to_string(), "Undefined variable."))
			}
		}
	}
}
