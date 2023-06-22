

def lcs(ws):
	ws = ws.replace("\n", ",|,")
	ws = ws.split(",")
	new_ws = ""
	for w in ws:
		new_ws += lc(w)
	return new_ws

def lc(w):
	if w == "|":
		return "\n"
	print(w)
	w = w.strip()
	if w == "":
		return ""
	ws = w.split("_")
	new_w = ""
	for w in ws:
		new_w += w.capitalize()
	return new_w + ", "


text = """
  // Single-character tokens.
  LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
  COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

  // One or two character tokens.
  BANG, BANG_EQUAL,
  EQUAL, EQUAL_EQUAL,
  GREATER, GREATER_EQUAL,
  LESS, LESS_EQUAL,

  // Literals.
  IDENTIFIER, STRING, NUMBER,

  // Keywords.
  AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
  PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

  EOF,

  // Temporary Error one
  ERROR
"""

ntext = lcs(text).replace("|", "\n")

print(ntext)
