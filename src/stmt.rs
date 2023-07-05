use crate::expr::Expr;
use crate::token::Token;
use std::rc::Rc;

use std::fmt;

#[derive(Debug)]
pub enum Stmt {
    Block {
        stmts: Box<Vec<Stmt>>,
    },
    Expr {
        expr: Expr,
    },
    For {
        init: Option<Box<Stmt>>,
        condition: Option<Expr>,
        inc: Option<Expr>,
        block: Box<Stmt>,
    },
    Fun {
        name: Token,
        params: Vec<Token>,
        body: Rc<Vec<Stmt>>,
        depth: Option<u32>,
    },
    If {
        conditionals: Vec<(Expr, Box<Stmt>)>,
        else_block: Option<Box<Stmt>>,
    },
    Print {
        expr: Expr,
    },
    Return {
        expr: Expr,
    },
    VarDecl {
        variable: Expr,
        value: Expr,
    },
    While {
        condition: Expr,
        block: Box<Stmt>,
    },
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", s)
    }
}
