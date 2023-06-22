///////////////////////
// This file is 
// auto-generated code
///////////////////////
pub enum Expr {
	Binary { left: Rc<Expr>, operator: Token, right: Rc<Expr> },
	Grouping { expression: Rc<Expr> },
	Literal { value: Rc<dyn Any> },
	Unary { operator: Token, right: Rc<Expr> },
}

impl Expr {
	fn binary(left: Rc<Expr>, operator: Token, right: Rc<Expr>) -> Rc<Expr> {
		Rc::new(Expr::Binary { left, operator, right })
	}

	fn grouping(expression: Rc<Expr>) -> Rc<Expr> {
		Rc::new(Expr::Grouping { expression })
	}

	fn literal(value: Rc<dyn Any>) -> Rc<Expr> {
		Rc::new(Expr::Literal { value })
	}

	fn unary(operator: Token, right: Rc<Expr>) -> Rc<Expr> {
		Rc::new(Expr::Unary { operator, right })
	}

}