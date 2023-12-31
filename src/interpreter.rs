use crate::builtins::*;
use crate::environment::Environment;
use crate::expr;
use crate::expr::Expr;
use crate::function::Function;
use crate::lox_error::{EvalError, LoxError};
use crate::object::{stringify_cli_result, Object};
use crate::stmt::Stmt;
use crate::token::{Token, TokenType};

use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
    is_repl: bool,
    global_env: Rc<RefCell<Environment>>,
    local_env: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut global_env = Environment::new();
        global_env.declare("clock", Rc::new(Object::Fun(Rc::new(ClockFn {}))));
        global_env.declare("input", Rc::new(Object::Fun(Rc::new(InputFn {}))));
        global_env.declare("num", Rc::new(Object::Fun(Rc::new(NumFn {}))));
        global_env.declare("rand_int", Rc::new(Object::Fun(Rc::new(RandIntFn {}))));
        global_env.declare("str", Rc::new(Object::Fun(Rc::new(StrFn {}))));

        Interpreter {
            is_repl: false,
            global_env: Rc::new(RefCell::new(global_env)),
            local_env: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<Rc<Object>, LoxError> {
        let mut hit_error = false;
        let mut last_result = Rc::new(Object::Nil);
        for stmt in stmts {
            match self.execute(&stmt) {
                Ok(obj) => last_result = obj,
                Err(err) => {
                    err.report();
                    hit_error = true;
                }
            }
        }
        if hit_error {
            Err(LoxError::Runtime)
        } else {
            Ok(last_result)
        }
    }

    pub fn execute_with_env(
        &mut self,
        stmts: &[Stmt],
        env: Rc<RefCell<Environment>>,
    ) -> Result<Rc<Object>, EvalError> {
        let prev_env = self.local_env.clone();
        self.local_env = env;
        let res = self.execute_block_with_current_scope(stmts);
        self.local_env = prev_env;
        res
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<Rc<Object>, EvalError> {
        use Stmt::*;
        match stmt {
            Block { stmts } => self.execute_block(stmts),
            Expr { expr } => self.evaluate(expr),
            For {
                init,
                condition,
                inc,
                block,
            } => {
                self.local_env = Environment::add_scope(self.local_env.clone());
                if let Some(ref stmt) = init {
                    self.execute(stmt)?;
                }
                let tr = expr::Expr::Literal {
                    value: Object::Bool(true),
                };
                while is_truthy(&self.evaluate(condition.as_ref().unwrap_or(&tr))?) {
                    self.execute(block)?;
                    if let Some(expr) = inc {
                        self.evaluate(expr)?;
                    }
                }
                self.local_env = self.local_env.clone().borrow().remove_scope()?;
                Ok(Rc::new(Object::Nil))
            }
            Fun {
                name,
                params,
                body,
                depth,
            } => {
                let f = Rc::new(Function::new(
                    name.clone(),
                    params.clone(),
                    body.clone(),
                    self.local_env.clone(),
                ));
                let fobj = Rc::new(Object::Fun(f));
                if depth.is_some() {
                    self.local_env
                        .borrow_mut()
                        .declare(&name.lexeme, fobj.clone());
                } else {
                    self.global_env
                        .borrow_mut()
                        .declare(&name.lexeme, fobj.clone());
                }

                Ok(fobj)
            }
            If {
                conditionals,
                else_block,
            } => {
                for (c, blk) in conditionals.iter() {
                    if is_truthy(&self.evaluate(c)?) {
                        return self.execute(blk);
                    }
                }
                if let Some(blk) = else_block {
                    self.execute(blk)
                } else {
                    Ok(Rc::new(Object::Nil))
                }
            }
            Print { expr } => {
                let obj = self.evaluate(expr)?;
                println!("{}", stringify_cli_result(&obj));
                Ok(Rc::new(Object::Nil))
            }
            Return { expr } => Err(EvalError::new_return(self.evaluate(expr)?)),
            VarDecl { variable, value } => {
                let val = self.evaluate(value)?;
                let (name, depth) = name_and_depth_for(variable)?;
                if depth.is_some() {
                    self.local_env.borrow_mut().declare(&name.lexeme, val);
                } else {
                    self.global_env.borrow_mut().declare(&name.lexeme, val);
                }
                Ok(Rc::new(Object::Nil))
            }
            While { condition, block } => {
                while is_truthy(&self.evaluate(condition)?) {
                    self.execute(block)?;
                }
                Ok(Rc::new(Object::Nil))
            }
        }
    }

    fn execute_block(&mut self, stmts: &[Stmt]) -> Result<Rc<Object>, EvalError> {
        self._execute_block(stmts, false)
    }

    fn execute_block_with_current_scope(
        &mut self,
        stmts: &[Stmt],
    ) -> Result<Rc<Object>, EvalError> {
        self._execute_block(stmts, true)
    }

    fn _execute_block(
        &mut self,
        stmts: &[Stmt],
        use_current_scope: bool,
    ) -> Result<Rc<Object>, EvalError> {
        let mut last_error = None;
        let mut last_res = Rc::new(Object::Nil);
        if !use_current_scope {
            self.local_env = Environment::add_scope(self.local_env.clone());
        }
        for stmt in stmts.iter() {
            match self.execute(stmt) {
                Ok(obj) => {
                    last_res = obj.clone();
                    if self.is_repl {
                        println!("val: {}", stringify_cli_result(&obj));
                    }
                }
                Err(EvalError::Return(obj)) => return Err(EvalError::Return(obj)),
                Err(err) => {
                    err.report();
                    last_error = Some(err);
                }
            }
        }
        if !use_current_scope {
            self.local_env = self.local_env.clone().borrow().remove_scope()?;
        }
        if let Some(err) = last_error {
            Err(err)
        } else {
            Ok(last_res)
        }
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<Rc<Object>, EvalError> {
        use Expr::*;
        match expr {
            Assign { variable, value } => {
                let val = self.evaluate(value)?;
                let (name, depth) = name_and_depth_for(variable)?;
                if depth.is_some() {
                    self.local_env.borrow_mut().assign(name, val.clone())?;
                } else {
                    self.global_env.borrow_mut().assign(name, val.clone())?;
                }
                Ok(val)
            }
            Binary {
                ref left,
                ref operator,
                ref right,
            } => match self.eval_binary(left, operator, right) {
                Ok(exp) => Ok(exp),
                Err(everr) => Err(everr.with_context(operator.clone(), &expr.to_string())),
            },
            Call {
                ref callee,
                ref paren,
                ref args,
            } => self.eval_call(callee, paren, args),
            Grouping { ref expr } => self.eval_grouping(expr),
            Literal { ref value } => {
                use self::Object::*;
                Ok(Rc::new(match value {
                    Nil => Nil,
                    Bool(b) => Bool(*b),
                    Num(n) => Num(*n),
                    Str(s) => Str(s.clone()),
                    Fun(f) => Fun(f.clone()),
                }))
            }
            Logic {
                ref left,
                ref operator,
                ref right,
            } => match self.eval_logic(left, operator, right) {
                Ok(exp) => Ok(exp),
                Err(everr) => Err(everr.with_context(operator.clone(), &expr.to_string())),
            },
            Unary {
                ref operator,
                ref right,
            } => match self.eval_unary(operator, right) {
                Ok(exp) => Ok(exp),
                Err(everr) => Err(everr.with_context(operator.clone(), &expr.to_string())),
            },
            Variable {
                ref name,
                ref depth,
            } => Ok(if let Some(d) = depth {
                self.local_env.borrow_mut().lookup(name.clone(), *d)?
            } else {
                self.global_env.borrow_mut().lookup(name.clone(), 0)?
            }),
        }
    }

    pub fn eval_grouping(&mut self, expr: &Expr) -> Result<Rc<Object>, EvalError> {
        self.evaluate(expr)
    }

    pub fn eval_call(
        &mut self,
        callee: &Expr,
        paren: &Token,
        args: &Rc<Vec<Expr>>,
    ) -> Result<Rc<Object>, EvalError> {
        match &*self.evaluate(callee)? {
            Object::Fun(f) => {
                if args.len() != f.arity() {
                    return Err(EvalError::new(&format!(
                        "Expected {} arguments but got {}.",
                        f.arity(),
                        args.len()
                    ))
                    .with_context(paren.clone(), &callee.to_string()));
                }
                let mut obj_args = Vec::new();
                for arg in args.iter() {
                    obj_args.push(self.evaluate(arg)?);
                }
                f.call(self, &obj_args)
            }
            _ => Err(EvalError::new("Can only call functions and classes.")
                .with_context(paren.clone(), &callee.to_string())),
        }
    }

    pub fn eval_unary(&mut self, op: &Token, right: &Expr) -> Result<Rc<Object>, EvalError> {
        let r = self.evaluate(right)?;

        use self::Object::*;
        use TokenType::*;
        Ok(Rc::new(match &op.ttype {
            Bang => Bool(!(is_truthy(&r))),
            Minus => match &*r {
                Num(n) => Num(-n),
                _ => return Err(EvalError::new("Operand must be a number.")),
            },
            tt => {
                return Err(EvalError::new(&format!(
                    "eval_unary: Invalid operator! {:?}",
                    tt
                )))
            }
        }))
    }

    pub fn eval_binary(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Rc<Object>, EvalError> {
        let l = self.evaluate(left)?;
        let r = self.evaluate(right)?;

        use self::Object::*;
        use TokenType::*;
        Ok(Rc::new(match &operator.ttype {
            BangEqual => Bool(!is_equal(l, r)),
            EqualEqual => Bool(is_equal(l, r)),
            Greater => Bool(as_num(l)? > as_num(r)?),
            GreaterEqual => Bool(as_num(l)? >= as_num(r)?),
            Less => Bool(as_num(l)? < as_num(r)?),
            LessEqual => Bool(as_num(l)? <= as_num(r)?),
            Minus => Num(as_num(l)? - as_num(r)?),
            Plus => eval_plus(l, r)?,
            Slash => eval_div(l, r)?,
            Star => Num(as_num(l)? * as_num(r)?),
            tt => {
                return Err(EvalError::new(&format!(
                    "eval_binary: Invalid operator! {:?}",
                    tt
                )))
            }
        }))
    }

    pub fn eval_logic(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Rc<Object>, EvalError> {
        debug_assert!(operator.ttype == TokenType::And || operator.ttype == TokenType::Or);
        let l = self.evaluate(left)?;

        Ok(if operator.ttype == TokenType::And {
            if !is_truthy(&l) {
                l
            } else {
                self.evaluate(right)?
            }
        } else if !is_truthy(&l) {
            self.evaluate(right)?
        } else {
            l
        })
    }
}

fn eval_plus(l: Rc<Object>, r: Rc<Object>) -> Result<Object, EvalError> {
    use self::Object::*;
    let err = Err(EvalError::new(
        "Operands must be two numbers or two strings.",
    ));
    match &*l {
        Num(n) => {
            let n2 = as_num(r);
            if n2.is_err() {
                return err;
            }
            Ok(Num(n + n2.unwrap()))
        }
        Str(s) => {
            let s2 = &as_str(r);
            if s2.is_err() {
                return err;
            }
            Ok(Str(s.to_string() + s2.as_ref().unwrap()))
        }
        _ => err,
    }
}

fn eval_div(l: Rc<Object>, r: Rc<Object>) -> Result<Object, EvalError> {
    let divisor = as_num(r)?;
    if divisor == 0.0 {
        return Err(EvalError::new("Tried to divide by 0!"));
    }
    let res = as_num(l)? / divisor;
    Ok(Object::Num(res))
}

fn name_and_depth_for(variable: &Expr) -> Result<(Token, Option<u32>), EvalError> {
    match variable {
        Expr::Variable { name, depth } => Ok((name.clone(), *depth)),
        _ => Err(EvalError::new("Invalid assignment target.")),
    }
}

fn is_truthy(obj: &Rc<Object>) -> bool {
    use self::Object::*;
    match &*obj.clone() {
        Bool(b) => *b,
        Num(_) | Str(_) | Fun(_) => true,
        Nil => false,
    }
}

// In Lox you can compare different types, but
// that returns false.
fn is_equal(l: Rc<Object>, r: Rc<Object>) -> bool {
    use self::Object::*;
    match (&*l, &*r) {
        (Bool(b1), Bool(b2)) => b1 == b2,
        (Num(n1), Num(n2)) => n1 == n2,
        (Str(s1), Str(s2)) => s1 == s2,
        (Nil, Nil) => true,
        _ => false,
    }
}

fn as_num(obj: Rc<Object>) -> Result<f64, EvalError> {
    use self::Object::*;
    match &*obj {
        Num(n) => Ok(*n),
        _ => Err(EvalError::new("Operands must be numbers.")),
    }
}

fn as_str(obj: Rc<Object>) -> Result<String, EvalError> {
    use self::Object::*;
    match &*obj {
        Str(s) => Ok(s.to_owned()),
        _ => Err(EvalError::new("Operands must be strings.")),
    }
}
