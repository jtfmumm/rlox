import json
import re

CAMEL_PATTERN = re.compile(r'(?<!^)(?=[A-Z])')

class Field:
	def __init__(self, name, typ):
		self.name = name
		self.type = typ.replace('-', ',')

	def __str__(self):
		return self.name + ": " + self.type

class Expr:
	def __init__(self, name, fields):
		self.name = name
		self.fields = fields

	def field_names(self):
		return list(map(lambda x: x.name, self.fields))

	def __str__(self):
		fieldstrs = ""
		for f in self.fields:
			fieldstrs += str(f) + ', '
		return self.name + ' -> ' + fieldstrs[:-2]

def fieldize(s):
	raw = s.split(' ')
	return Field(raw[2].strip(), raw[1].strip())

def parse_fields(s):
	raw = s.split(',')
	return list(map(fieldize, raw))

def parse_variant(variant):
	parts = variant.split(':')
	name = parts[0].strip()
	fields = parse_fields(parts[1])
	return Expr(name, fields)

def new_fn_name(enum):
	return CAMEL_PATTERN.sub('_', enum).lower()

def gen_source(prelude, enum, def_file, target_file):
	with open('tools/' + def_file, 'r') as f:
		data = json.load(f)[enum]

	variants = list(map(parse_variant, data))

	output = '///////////////////////\n// This file is \n// auto-generated code\n///////////////////////\n'
	output += prelude
	output += """
use std::fmt;
	"""
	output += '\n#[derive(Debug)]\n'
	output += 'pub enum ' + enum + ' {\n'

	for v in variants:
		output += '\t' + v.name + ' { '
		output += ', '.join(map(str, v.fields)) + ' },\n'

	output += '}\n\n'
	output += 'impl ' + enum + ' {\n'

	for v in variants:
		fn_name = new_fn_name(v.name)
		output += '\tpub fn ' + fn_name + '('
		# output += ', '.join(map(str, v.fields)) + ') -> Rc<' + enum + '> {\n'
		# output += '\t\tRc::new(' + enum + '::' + v.name + ' { '
		output += ', '.join(map(str, v.fields)) + ') -> ' + enum + ' {\n'
		output += '\t\t' + enum + '::' + v.name + ' { '
		output += ', '.join(v.field_names()) + ' }\n\t}\n\n'

	output += '}\n\n'

	output += 'impl fmt::Display for ' + enum + "{\n"
	output += '\tfn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {\n'
	output += """
		let s = format!("{:?}", self);
		write!(f, "{}", s)
	}
}
	"""

	with open(target_file, 'w+') as f:
		f.write(output)

expr_prelude = """
use crate::object::Object;
use crate::token::Token;
use std::rc::Rc;
"""

gen_source(expr_prelude, 'Expr', 'ast_def.json', 'src/expr.rs')

stmt_prelude = """
use crate::expr::Expr;
use crate::token::Token;
use std::rc::Rc;
"""

gen_source(stmt_prelude, 'Stmt', 'ast_def.json', 'src/stmt.rs')
