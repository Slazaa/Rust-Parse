use std::{env, fs};

use parse::*;

#[derive(Debug, Clone)]
pub enum Item {
	Func(Func),
	FuncProto(FuncProto),
	VarDecl(VarDecl)
}

#[derive(Debug, Clone)]
pub struct Func {
	pub id: String,
	pub stmts: Stmts
}

#[derive(Debug, Clone)]
pub struct FuncProto {
	pub id: String
}

#[derive(Debug, Clone)]
pub struct VarDecl {
	pub id: String,
	pub expr: Option<Expr>
}

#[derive(Debug, Clone)]
pub enum Stmt {
	Expr(Expr),
	Item(Item)
}

#[derive(Debug, Clone)]
pub struct Stmts {
	pub stmts: Vec<Stmt>
}

#[derive(Debug, Clone)]
pub enum Expr {
	Literal(Literal),
	BinExpr()
}

#[derive(Debug, Clone)]
pub enum LiteralKind {
	Char,
	Float,
	Int,
	Str
}

#[derive(Debug, Clone)]
pub struct Literal {
	pub kind: LiteralKind,
	pub value: String
}

#[derive(Debug, Clone)]
pub struct BinExpr {
	pub left: Expr,
	pub op: String,
	pub right: Expr
}

#[derive(Debug, Clone)]
pub enum Node {
	Token(Token),
	// ----------
	NewLine,
	OptNewLine,
	Item(Item),
	Func(Func),
	FuncProto(FuncProto),
	Stmt(Stmt),
	Stmts(Stmts),
	Literal(Literal),
	Expr(Expr),
	Program(Option<Stmts>),
	VarDecl(VarDecl)
}

impl ASTNode for Node {
	fn new_token(token: &Token) -> Self {
		Self::Token(token.to_owned())
	}

	fn token(&self) -> Result<&Token, String> {
		match self {
			Self::Token(token) => Ok(token),
			_ => Err("Node is not a token".to_owned())
		}
	}

	fn is_token(&self) -> bool {
		matches!(self, Self::Token(_))
	}
}

fn expr_literal(nodes: &[Node]) -> Result<Node, String> {
	Ok(match &nodes[0] {
		Node::Literal(x) => Node::Expr(Expr::Literal(x.to_owned())),
		_ => return Err(format!("Invalid node '{:?}' in 'func'", nodes[0]))
	})
}

fn func(nodes: &[Node]) -> Result<Node, String> {
	let id = match &nodes[1] {
		Node::Token(token) if token.name() == "ID" => token.symbol().to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'func'", nodes[1]))
	};

	let stmts = match &nodes[3] {
		Node::Stmts(x) => x.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'func'", nodes[4]))
	};

	Ok(Node::Func(Func { id, stmts }))
}

fn func_proto(nodes: &[Node]) -> Result<Node, String> {
	let id = match &nodes[1] {
		Node::Token(token) if token.name() == "ID" => token.symbol().to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'func'", nodes[1]))
	};

	Ok(Node::FuncProto(FuncProto { id }))
}

fn item(nodes: &[Node]) -> Result<Node, String> {
	match &nodes[0] {
		Node::Func(x) => Ok(Node::Item(Item::Func(x.to_owned()))),
		Node::FuncProto(x) => Ok(Node::Item(Item::FuncProto(x.to_owned()))),
		Node::VarDecl(x) => Ok(Node::Item(Item::VarDecl(x.to_owned()))),
		_ => Err(format!("Invalid node '{:?}' in 'item'", nodes[0]))
	}
}

fn literal_int(nodes: &[Node]) -> Result<Node, String> {
	Ok(Node::Literal(Literal {
		kind: LiteralKind::Int,
		value: nodes[0].token().unwrap().symbol().to_owned()
	}))
}

fn literal_float(nodes: &[Node]) -> Result<Node, String> {
	Ok(Node::Literal(Literal {
		kind: LiteralKind::Float,
		value: nodes[0].token().unwrap().symbol().to_owned()
	}))
}

fn literal_str(nodes: &[Node]) -> Result<Node, String> {
	Ok(Node::Literal(Literal {
		kind: LiteralKind::Str,
		value: nodes[0].token().unwrap().symbol().to_owned()
	}))
}

fn program(nodes: &[Node]) -> Result<Node, String> {
	if nodes.is_empty() {
		return Ok(Node::Program(None));
	}

	match &nodes[0] {
		Node::Stmts(x) => Ok(Node::Program(Some(x.to_owned()))),
		_ => Err(format!("Invalid node '{:?}' in 'program'", nodes[0]))
	}
}

fn stmt(nodes: &[Node]) -> Result<Node, String> {
	Ok(Node::Stmt(match nodes[0].to_owned() {
		Node::Expr(x) => Stmt::Expr(x),
		Node::Item(x) => Stmt::Item(x),
		_ => return Err(format!("Invalid node '{:?}' in 'stmt'", nodes[0]))
	}))
}

fn stmts(nodes: &[Node]) -> Result<Node, String> {
	if nodes.is_empty() {
		return Ok(Node::Stmts(Stmts { stmts: vec![] }));
	}

	let node_stmt = match &nodes[0] {
		Node::Stmt(x) => x.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'stmts'", nodes[0]))
	};

	let node_stmts = match nodes.get(1) {
		Some(Node::Stmts(x)) => x.stmts.clone(),
		_ => Vec::new()
	};

	let mut stmts_vec = vec![node_stmt];
	stmts_vec.extend(node_stmts);

	Ok(Node::Stmts(Stmts { stmts: stmts_vec }))
}

fn var_decl(nodes: &[Node]) -> Result<Node, String> {
	let id = match &nodes[1] {
		Node::Token(token) if token.name() == "ID" => token.symbol().to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'var'", nodes[1]))
	};

	let expr = match nodes.get(3) {
		Some(Node::Expr(expr)) => Some(expr.to_owned()),
		Some(_) => return Err(format!("Invalid node '{:?}' in 'var'", nodes[3])),
		None => None
	};

	Ok(Node::VarDecl(VarDecl { id, expr }))
}

fn main() {
	let args: Vec<String> = env::args().skip(1).collect();

	if args.is_empty() {
		println!("An input file is needed");
		return;
	}

	if args.len() > 1 {
		println!("Too much arguments were given");
		return;
	}

	let input = match fs::read_to_string(&args[0]) {
		Ok(x) => x,
		Err(_) => {
			println!("Invalid filename '{}'", &args[0]);
			return;
		}
	};

	let mut lexer_builder = LexerBuilder::new();

	lexer_builder.ignore_rules(&[
		r"(^[ \r\n\t]+)"
	]).unwrap();

	lexer_builder.add_rules(&[
		// Keywords
		("FUNC",  r"(^func)"),
		("VAR",   r"(^var)"),

		// Operators
		("PLUS",  r"(^\+)"),
		("MINUS", r"(^-)"),
		("MULT",  r"(^\*)"),
		("DIV",   r"(^/)"),
		("EQ",    r"(^=)"),

		// Identifier / Literal
		("BOOL",  r"(^(false|true))"),
		("ID",    r"(^[a-zA-Z_][a-zA-Z0-9_]*)"),
		("FLOAT", r"(^\d+\.\d+)"),
		("INT",   r"(^\d+)"),
		("STR",   r#"(^".*")"#),

		// Misc
		("COL",   r"(^:)"),
		("LCBR",  r"(^\{)"),
		("LPAR",  r"(^\()"),
		("RCBR",  r"(^\})"),
		("RPAR",  r"(^\))"),
		("SEMI",  r"(^;)")
	]).unwrap();

	let lexer = lexer_builder.build();
/*
	for token in lexer.lex(&input) {
		match token {
			Ok(token) => println!("{:#?}", token),
			Err(e) => {
				println!("{:?}", e);
				break;
			}
		}
	}
*/
	let mut parser_builder = parse::ParserBuilder::<Node>::new(&lexer.rules().iter().map(|x| x.name().as_str()).collect::<Vec<&str>>());

	parser_builder.add_patterns(&[
		("expr",       "literal", expr_literal),
		("func",       "FUNC ID LCBR stmts RCBR", func),
		("func_proto", "FUNC ID", func_proto),
		("item",       "func", item),
		("item",       "func_proto", item),
		("item",       "var_decl", item),
		("literal",    "INT", literal_int),
		("literal",    "FLOAT", literal_float),
		("literal",    "STR", literal_str),
		("program",    "stmts", program),
		("program",    "", program),
		("stmt",       "item", stmt),
		("stmts",      "stmt stmts", stmts),
		("stmts",      "stmt", stmts),
		("stmts",      "", stmts),
		("var_decl",   "VAR ID SEMI", var_decl),
		("var_decl",   "VAR ID EQ expr SEMI", var_decl)
	]).unwrap();
	
	let mut parser = parser_builder.build();

	let ast = match parser.parse(lexer.lex(&input)) {
		Ok(x) => x,
		Err((e, pos)) => {
			println!("{:?} at {}", e, pos);
			return;
		}
	};

	println!("{:#?}", ast);
}