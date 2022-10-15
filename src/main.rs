use std::{env, fs};

use parse::*;

#[derive(Debug, Clone)]
pub enum Item {
	Func(Func),
	FuncProto(FuncProto)
}

#[derive(Debug, Clone)]
pub struct Label {
	pub id: String,
	pub item: Item
}

#[derive(Debug, Clone)]
pub struct Func {
	pub stmts: Stmts
}

#[derive(Debug, Clone)]
pub struct FuncProto;

#[derive(Debug, Clone)]
pub enum Stmt {
	Expr(Expr),
	Label(Label)
}

#[derive(Debug, Clone)]
pub struct Stmts {
	pub stmts: Vec<Stmt>
}

#[derive(Debug, Clone)]
pub struct Expr {
	pub value: f64
}

#[derive(Debug, Clone)]
pub enum Node {
	Token(Token),
	// ----------
	NewLine,
	OptNewLine(bool),
	Item(Item),
	Label(Label),
	Func(Func),
	FuncProto(FuncProto),
	Stmt(Stmt),
	Stmts(Stmts),
	Expr(Expr),
	Program(Option<Stmts>)
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

fn expr_num(nodes: &[Node]) -> Result<Node, String> {
	Ok(Node::Expr(Expr { value: nodes[0].token().unwrap().symbol().parse::<f64>().unwrap() }))
}

fn expr_op(nodes: &[Node]) -> Result<Node, String> {
	let left = match &nodes[0] {
		Node::Token(x) if x.name() == "NUM" => x,
		_ => return Err(format!("Invalid node '{:?}' in 'expr_op'", nodes[0]))
	};

	let op = match &nodes[1] {
		Node::Token(x) => x,
		_ => return Err(format!("Invalid node '{:?}' in 'expr_op'", nodes[1]))
	};

	let right = match &nodes[2] {
		Node::Expr(x) => x,
		_ => return Err(format!("Invalid node '{:?}' in 'expr_op'", nodes[2]))
	};

	let value = match op.name().as_str() {
		"MINUS" => left.symbol().parse::<f64>().unwrap() - right.value,
		"PLUS" => left.symbol().parse::<f64>().unwrap() + right.value,
		"MULT" => left.symbol().parse::<f64>().unwrap() * right.value,
		"DIV" => left.symbol().parse::<f64>().unwrap() / right.value,
		_ => return Err(format!("Invalid operator '{}' in expr_op", op.name()))
	};

	Ok(Node::Expr(Expr { value }))
}

fn func(nodes: &[Node]) -> Result<Node, String> {
	let stmts = match &nodes[4] {
		Node::Stmts(x) => x.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'func'", nodes[4]))
	};

	Ok(Node::Func(Func { stmts }))
}

fn func_proto(_nodes: &[Node]) -> Result<Node, String> {
	Ok(Node::FuncProto(FuncProto))
}

fn item(nodes: &[Node]) -> Result<Node, String> {
	match &nodes[0] {
		Node::Func(x) => Ok(Node::Item(Item::Func(x.to_owned()))),
		Node::FuncProto(x) => Ok(Node::Item(Item::FuncProto(x.to_owned()))),
		_ => Err(format!("Invalid node '{:?}' in 'item'", nodes[0]))
	}
}

fn label(nodes: &[Node]) -> Result<Node, String> {
	let id = match &nodes[0] {
		Node::Token(x) if x.name() == "ID" => x.symbol().to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'label'", nodes[0]))
	};

	let item = match &nodes[2] {
		Node::Item(x) => x.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'label'", nodes[2]))
	};

	Ok(Node::Label(Label { id, item }))
}

fn opt_new_line(nodes: &[Node]) -> Result<Node, String> {
	if nodes.is_empty() {
		return Ok(Node::OptNewLine(false))
	}

	match &nodes[0] {
		Node::Token(x) if x.name() == "NL" => Ok(Node::OptNewLine(true)),
		_ => Err(format!("Invalid node '{:?}' in 'opt_new_line'", nodes[0]))
	}
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
	match &nodes[0] {
		Node::Expr(x) => Ok(Node::Stmt(Stmt::Expr(x.to_owned()))),
		Node::Label(x) => Ok(Node::Stmt(Stmt::Label(x.to_owned()))),
		_ => Err(format!("Invalid node '{:?}' in 'stmt'", nodes[0]))
	}
}

fn stmts(nodes: &[Node]) -> Result<Node, String> {
	if nodes.is_empty() {
		return Ok(Node::Stmts(Stmts { stmts: vec![] }));
	}

	let node_stmt = match &nodes[0] {
		Node::Stmt(x) => x.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'stmts'", nodes[0]))
	};
/*
	if let Some(Node::Stmts(node_stmts)) = nodes.get(1) {
	    let mut stmt_vec = vec![node_stmt];
	    stmt_vec.extend(node_stmts.stmts.clone());
        return Ok(Node::Stmts(Stmts { stmts: stmt_vec }))
	}
*/
	Ok(Node::Stmts(Stmts { stmts: vec![node_stmt] }))
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

	lexer_builder.ignore_rule(r"(^[ \t]+)").unwrap();
	lexer_builder.add_rules(&[
		("COL",   r"(^[:])"),
		("DIV",   r"(^[/])"),
		("FUNC",  r"(^func)"),
		("ID",    r"(^[a-zA-Z_][a-zA-Z0-9_]*)"),
		("LCBR",  r"(^[{])"),
		("LPAR",  r"(^[(])"),
		("MINUS", r"(^[-])"),
		("MULT",  r"(^[*])"),
		("NL",    r"(^[\r\n]+)"),
		("NUM",   r"(^\d+(\.\d+)?)"),
		("PLUS",  r"(^[+])"),
		("RCBR",  r"(^[}])"),
		("RPAR",  r"(^[)])")
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
		("expr",       "NUM PLUS expr", expr_op),
		("expr",       "NUM MINUS expr", expr_op),
		("expr",       "NUM MULT expr", expr_op),
		("expr",       "NUM DIV expr", expr_op),
		("expr",       "NUM", expr_num),
		("func",       "FUNC opt_nl LCBR opt_nl stmts opt_nl RCBR", func),
		("func_proto", "FUNC", func_proto),
		("item",       "func", item),
		("item",       "func_proto", item),
		("label",      "opt_nl ID COL item opt_nl", label),
		("opt_nl",     "NL", opt_new_line),
		("opt_nl",     "", opt_new_line),
		("program",    "stmts", program),
		("program",    "", program),
		("stmt",       "expr", stmt),
		("stmt",       "label", stmt),
		("stmts",      "stmt stmts", stmts),
		("stmts",      "stmt", stmts),
		("stmts",      "", stmts),
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
