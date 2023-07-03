use crate::callable::Callable;
use crate::environment::Environment;
use crate::interpreter::Interpreter;
use crate::lox_error::EvalError;
use crate::object::Object;
use crate::stmt::Stmt;
use crate::token::{Token, TokenType};

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

pub struct Function {
	name: Token,
	params: Rc<Vec<Token>>,
	body: Rc<Stmt>,
	closure: Rc<RefCell<Environment>>
}

impl Function {
	pub fn new(name: Token, params: Rc<Vec<Token>>,
		       body: Rc<Stmt>, env: Rc<RefCell<Environment>>) -> Self {
		let closure = env;
		Function { name, params, body, closure }
	}
}

impl Callable for Function {
	fn arity(&self) -> usize { self.params.len() }

	fn call(&self, interpreter: &mut Interpreter,
		    args: &Vec<Rc<Object>>) -> Result<Rc<Object>,EvalError> {
		let scope = Rc::new(RefCell::new(Environment::from_outer(self.closure.clone())));
		// This should be enforced before call() is called
		debug_assert!(self.params.len() == args.len());
		self.params.iter()
			.zip(args)
			.for_each(|(p, a)| match p.ttype.clone() {
				TokenType::Identifier(name) => scope.borrow_mut().declare(&name, a.clone()),
				_ => unreachable!()
			}
		);
		match interpreter.execute_with_env(self.body.clone(), scope.clone()) {
			Ok(obj) => Ok(obj),
			Err(EvalError::Fail(msg)) => Err(EvalError::new(&msg)),
			Err(EvalError::Return(obj)) => Ok(obj),
		}
	}

	fn debug(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
    	write!(f, "{}", &format!("<fn {}>", self.name))
	}
}


impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
    	write!(f, "{}", &format!("<fn {}>", self.name))
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
    	write!(f, "{}", &format!("<fn {}>", self.name))
    }
}
