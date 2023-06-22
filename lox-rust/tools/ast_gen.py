import json

class Field:
	def __init__(self, name, typ):
		self.name = name
		self.type = typ

	def __str__(self):
		t = self.type
		if t == 'Expr' or t == 'Any':
			t = 'Box<dyn ' + t + '>'
		return self.name + ": " + t

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

def parse_expr(expr):
	parts = expr.split(':')
	name = parts[0].strip()
	fields = parse_fields(parts[1])
	return Expr(name, fields)

with open('tools/ast_def.json', 'r') as f:
	data = json.load(f)

exprs = list(map(parse_expr, data))

output = 'pub trait Expr {}\n\n'

for e in exprs:
	output += 'pub struct ' + e.name + ' {\n'
	for f in e.fields:
		output += '\t' + str(f) + ',\n'
	output += '}\n\n'
	output += 'impl ' + e.name + ' {\n'
	output += '\tfn new('
	output += ', '.join(map(str, e.fields)) + ') -> Self {\n'
	output += '\t\t' + e.name + ' { '
	output += ', '.join(e.field_names()) + ' }\n\t}\n}\n\n'
	output += 'impl Expr for ' + e.name + ' {}\n\n'

print(output)
