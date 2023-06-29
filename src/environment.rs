use crate::cerror::EvalError;
use crate::object::Object;

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

	pub fn declare(&mut self, name: &str, value: Object) -> Result<(), EvalError>  {
		self.env.insert(name.to_string(), value);
		Ok(())
	}

	pub fn lookup(&self, name: &str) -> Result<Object, EvalError> {
		if self.env.contains_key(name) {
			Ok(self.env.get(name).unwrap().clone())
		} else {
			match self.outer {
				Some(ref outer_env) => {
					outer_env.borrow().lookup(&name)
				},
				None => Err(EvalError::new(&format!("Undefined variable '{}'.", name.clone())))
			}
		}
	}

	pub fn assign(&mut self, name: &str, value: Object) -> Result<(), EvalError> {
		if self.env.contains_key(name) {
			self.env.insert(name.to_string(), value);
			Ok(())
		} else {
			match self.outer.take() {
				Some(outer_env) => {
					let res = outer_env.borrow_mut().assign(&name, value);
					self.outer = Some(outer_env);
					res
				},
				None => Err(EvalError::new(&format!("Undefined variable '{}'.", name.clone())))
			}
		}
	}
}
