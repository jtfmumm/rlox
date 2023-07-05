use crate::interpreter::Interpreter;
use crate::lox_error::EvalError;
use crate::object::Object;

use std::fmt::Debug;
use std::fmt;
use std::rc::Rc;

pub trait Callable {
	fn arity(&self) -> usize;
	fn call(&self, interpreter: &mut Interpreter,
		    args: &[Rc<Object>]) -> Result<Rc<Object>,EvalError>;
	fn debug(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
    	write!(f, "<native fn>")
	}
}

impl Debug for dyn Callable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
    	self.debug(f)
    }
}
