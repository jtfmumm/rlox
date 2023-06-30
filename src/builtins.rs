use crate::callable::Callable;
use crate::interpreter::Interpreter;
use crate::object::Object;

use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct ClockFn {}

impl Callable for ClockFn {
	fn arity(&self) -> usize { 0 }

	fn call(&self, interpreter: &mut Interpreter, args: &Vec<Rc<Object>>) -> Rc<Object> {
	    Rc::new(
	    	Object::Num(
		    	SystemTime::now()
			        .duration_since(UNIX_EPOCH)
			        .unwrap()
			        .as_millis() as f64))
	}
}

#[derive(Debug)]
pub struct StrFn {}

impl Callable for StrFn {
	fn arity(&self) -> usize { 1 }

	fn call(&self, interpreter: &mut Interpreter, args: &Vec<Rc<Object>>) -> Rc<Object> {
	    let arg = &args[0];
	    let res = format!("{}", arg);
	    Rc::new(Object::Str(res))
	}
}
