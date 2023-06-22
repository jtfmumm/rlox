import json

class Field:
	def __init__(self, name, typ):
		self.name = name
		self.type = typ

	def __str__(self):
		t = self.type
		if t == 'Any':
			t = 'Rc<dyn ' + t + '>'
		elif t == 'Expr':
			t = 'Rc<' + t + '>'

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

output = 'pub enum Expr {\n'

for e in exprs:
	output += '\t' + e.name + ' { '
	output += ', '.join(map(str, e.fields)) + ' },\n'

output += '}\n\n'
output += 'impl Expr {\n'

for e in exprs:
	output += '\tfn ' + e.name.lower() + '('
	output += ', '.join(map(str, e.fields)) + ') -> Rc<Expr> {\n'
	output += '\t\tRc::new(Expr::' + e.name + ' { '
	output += ', '.join(e.field_names()) + ' })\n\t}\n\n'

output += '}'

print(output)
