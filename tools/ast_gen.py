# import json

# class Field:
# 	def __init__(self, name, typ):
# 		self.name = name
# 		self.type = typ

# 	def __str__(self):
# 		t = self.type
# 		if t == 'Any':
# 			t = 'Rc<dyn ' + t + '>'
# 		elif t == 'Expr':
# 			t = 'Rc<' + t + '>'

# 		return self.name + ": " + t

# class Expr:
# 	def __init__(self, name, fields):
# 		self.name = name
# 		self.fields = fields

# 	def field_names(self):
# 		return list(map(lambda x: x.name, self.fields))

# 	def __str__(self):
# 		fieldstrs = ""
# 		for f in self.fields:
# 			fieldstrs += str(f) + ', '
# 		return self.name + ' -> ' + fieldstrs[:-2]

# def fieldize(s):
# 	raw = s.split(' ')
# 	return Field(raw[2].strip(), raw[1].strip())

# def parse_fields(s):
# 	raw = s.split(',')
# 	return list(map(fieldize, raw))

# def parse_expr(expr):
# 	parts = expr.split(':')
# 	name = parts[0].strip()
# 	fields = parse_fields(parts[1])
# 	return Expr(name, fields)

# with open('tools/ast_def.json', 'r') as f:
# 	data = json.load(f)

# exprs = list(map(parse_expr, data))

# output = '///////////////////////\n// This file is \n// auto-generated code\n///////////////////////\n'
# output += """
# use crate::literal::Literal;
# use crate::token::Token;
# use std::rc::Rc;

# """
# output += 'pub enum Expr {\n'

# for e in exprs:
# 	output += '\t' + e.name + ' { '
# 	output += ', '.join(map(str, e.fields)) + ' },\n'

# output += '}\n\n'
# output += 'impl Expr {\n'

# for e in exprs:
# 	output += '\tpub fn ' + e.name.lower() + '('
# 	output += ', '.join(map(str, e.fields)) + ') -> Rc<Expr> {\n'
# 	output += '\t\tRc::new(Expr::' + e.name + ' { '
# 	output += ', '.join(e.field_names()) + ' })\n\t}\n\n'

# output += """\t// Visitor methods
# 	fn parens(left: String, right: String) -> String {
# 		format!("({:} {:})", left, right)
# 	}

# 	pub fn to_string(&self) -> String {
# 		use Expr::*;

# 		match *self {
# 			Binary { ref left, ref operator, ref right } => {
# 				operator.to_string() + " " + &Expr::parens(left.to_string(), right.to_string())
# 			},
# 			Grouping { ref expression } => expression.to_string(),
# 			Literal { ref value } => value.to_string(),
# 			Unary { ref operator, ref right } => {
# 				Expr::parens(operator.to_string(), right.to_string())
# 			},
# 		}
# 	}
# """

# output += '}'

# with open('src/SOMETHING!', 'w+') as f:
# 	f.write(output)

# # print(output)
