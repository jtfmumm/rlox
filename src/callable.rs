use crate::interpreter::Interpreter;
use crate::lox_error::EvalError;
use crate::object::Object;

use std::fmt::Debug;
use std::rc::Rc;

pub trait Callable: Debug {
	fn arity(&self) -> usize;
	fn call(&self, interpreter: &mut Interpreter,
		    args: &Vec<Rc<Object>>) -> Result<Rc<Object>,EvalError>;
}
