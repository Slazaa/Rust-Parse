use std::{env, fs};

use parse::{Token, ASTNode};

#[derive(Debug, Copy, Clone)]
struct Expr {
	pub value: f64
}

#[derive(Debug, Clone)]
enum Node {
	Token(Token),
	// ----------
	Expr(Expr),
	Program(Option<Expr>)
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
		Node::Expr(x) => x,
		_ => return Err("Invalid node".to_owned())
	};

	let op = match &nodes[1] {
		Node::Token(x) => x,
		_ => return Err("Invalid node".to_owned())
	};

	let right = match &nodes[2] {
		Node::Expr(x) => x,
		_ => return Err("Invalid node".to_owned())
	};

	let value = match op.name().as_str() {
		"PLUS" => left.value + right.value,
		_ => return Err("Invalid operator".to_owned())
	};

	Ok(Node::Expr(Expr { value }))
}

fn program(nodes: &[Node]) -> Result<Node, String> {
    if nodes.is_empty() {
        return Ok(Node::Program(None));
    }

	match nodes[0] {
		Node::Expr(expr) =>Ok(Node::Program(Some(expr))),
		_ => Err("Invalid node!".to_owned())
	}
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

	let mut lexer_builder = parse::LexerBuilder::new();
	lexer_builder.ignore_rule(r"(^[ \t]+)").unwrap();
	
	if let Err(e) = lexer_builder.add_rules(&[
		("ID", r"(^[a-zA-Z_][a-zA-Z0-9_]*)"),
		("NL", r"(^[\r\n]+)"),
		("NUM", r"(^\d+(\.\d+)?)"),
		("PLUS", r"(^+)")
	]) {
		println!("{}", e);
		return;
	}

	let lexer = lexer_builder.build();
	let mut parser_builder = parse::ParserBuilder::<Node>::new(&lexer.rules().iter().map(|x| x.name().as_str()).collect::<Vec<&str>>());
	
	if let Err(e) = parser_builder.add_patterns(&[
		("expr", "NUM", expr_num),
		("expr", "expr PLUS expr", expr_op),
		("program", "expr", program),
		("program", "", program)
	]) {
		println!("{}", e);
		return;
	}
	
    let mut parser = parser_builder.build();

	let ast = match parser.parse(lexer.lex(&input)) {
		Ok(x) => x,
		Err((e, pos)) => {
			println!("{} at {}", e, pos);
			return;
		}
	};

	println!("{:#?}", ast);
}
