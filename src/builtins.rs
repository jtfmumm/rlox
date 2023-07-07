use crate::callable::Callable;
use crate::interpreter::Interpreter;
use crate::lox_error::EvalError;
use crate::object::Object;

use rand::Rng;
use std::io;
use std::io::Write;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct ClockFn {}

impl Callable for ClockFn {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _args: &[Rc<Object>],
    ) -> Result<Rc<Object>, EvalError> {
        Ok(Rc::new(Object::Num(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as f64,
        )))
    }
}

#[derive(Debug)]
pub struct StrFn {}

impl Callable for StrFn {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        args: &[Rc<Object>],
    ) -> Result<Rc<Object>, EvalError> {
        let arg = &args[0];
        let res = format!("{}", arg);
        Ok(Rc::new(Object::Str(res)))
    }
}

#[derive(Debug)]
pub struct NumFn {}

impl Callable for NumFn {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        args: &[Rc<Object>],
    ) -> Result<Rc<Object>, EvalError> {
        let arg = &args[0];
        match &format!("{}", arg).parse::<f64>() {
            Ok(n) => Ok(Rc::new(Object::Num(*n))),
            Err(_) => Err(EvalError::new("Expect number."))
        }
    }
}

#[derive(Debug)]
pub struct InputFn {}

impl Callable for InputFn {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        args: &[Rc<Object>],
    ) -> Result<Rc<Object>, EvalError> {
        let mut line = String::new();
        let prompt = &args[0];
        print!("{}", prompt);
        match io::stdout().flush() {
            Ok(_) => {},
            Err(err) => return Err(EvalError::new(&format!("{:?}", err)))
        };
        io::stdin().read_line(&mut line).unwrap();
        line.pop(); // remove \n
        Ok(Rc::new(Object::Str(line)))
    }
}

#[derive(Debug)]
pub struct RandIntFn {}

impl Callable for RandIntFn {
    fn arity(&self) -> usize {
        2
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        args: &[Rc<Object>],
    ) -> Result<Rc<Object>, EvalError> {
        let mut rng = rand::thread_rng();
        let (Object::Num(low), Object::Num(high)) = (&*args[0], &*args[1])
            else { return Err(EvalError::new("Expect numbers.")) };
        let res = rng.gen_range(*low as i32..*high as i32 + 1);
        Ok(Rc::new(Object::Num(res as f64)))
    }
}
