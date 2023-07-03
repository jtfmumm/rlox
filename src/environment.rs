use crate::lox_error::EvalError;
use crate::object::Object;
use crate::token::{Token, TokenType};

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// TODO: Remove once testing is over.
use rand;
use rand::Rng;

#[derive(Debug)]
pub struct Environment {
	id: u16,
	env: HashMap<String, Rc<Object>>,
	outer: Option<Rc<RefCell<Environment>>>
}

impl Environment {
	pub fn new() -> Self {
   		let mut rng = rand::thread_rng();
   		let id: u16 = rng.gen();
		Environment { id, outer: None, env: HashMap::new() }
	}

	pub fn from_outer(outer: Rc<RefCell<Environment>>) -> Self {
   		let mut rng = rand::thread_rng();
   		let id: u16 = rng.gen();
		Environment { id, outer: Some(outer), env: HashMap::new() }
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
		// println!("DECLARE ***{:?}:{:?}***", name, value);
		// println!("!@Declaring {:?},{:?}", name, value);
		self.env.insert(name.to_string(), value);
	}

	// pub fn lookup_with_backup_env(&self, id: Token,
	// 							  depth: Rc<Option<i32>>,
	// 							  env: Rc<RefCell<Environment>>
	// 							  ) -> Result<Rc<Object>, EvalError> {


	// 	match self.lookup(id.clone(), depth) {
	// 		Ok(res) => Ok(res),
	// 		Err(_) => env.borrow_mut().lookup(id)
	// 	}
	// }

	pub fn lookup(&self, id: Token, depth: u32) -> Result<Rc<Object>, EvalError> {
		// println!("LLLLLookup: {:?}", id);
		let name = match id.ttype {
			TokenType::Identifier(ref name) => name.clone(),
			_ => return Err(EvalError::new_with_context(id.clone(), &id.to_string(), "Expect variable."))
		};
		// println!("!@Looking up {:?},{:?}", id, depth);
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
		// Ok(self.env.get(&name).expect(&format!("Undefined variable '{}'.", name)).clone())

		// if self.env.contains_key(&name) {
		// 	let v = Ok(env.get(&name).unwrap().clone());
		// 	// println!("{:?}", self);
		// 	// println!("--FOUND INNER {:?}", v);
		// 	v
		// } else {
		// 	match self.outer {
		// 		Some(ref outer_env) => {
		// 			let v = outer_env.borrow().lookup(id);
		// 			// println!("{:?}", self);
		// 			// println!("--FOUND OUTER {:?}", v);
		// 			v
		// 		},
		// 		None => Err(
		// 			EvalError::new(&format!("Undefined variable '{}'.", name.to_string()))
		// 				.with_context(id.clone(), &id.to_string()))
		// 	}
		// }
	}

	pub fn assign(&mut self, id: Token, value: Rc<Object>) -> Result<(), EvalError> {
		let name = match id.ttype {
			TokenType::Identifier(ref name) => name.clone(),
			_ => return Err(EvalError::new_with_context(id.clone(), &id.to_string(), "Expect variable."))
		};
		// println!("!@Assigning {:?},{:?},{:?}", id, value, self);
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
