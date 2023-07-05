use crate::object::Object;
use crate::token::Token;
use std::rc::Rc;

use std::fmt;

#[derive(Debug)]
pub enum Expr {
    Assign {
        variable: Box<Expr>,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Rc<Expr>,
        paren: Token,
        args: Rc<Vec<Expr>>,
    },
    Grouping {
        expr: Box<Expr>,
    },
    Literal {
        value: Object,
    },
    Logic {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
        depth: Option<u32>,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", s)
    }
}
