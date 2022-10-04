use std::{env, fs};

use parse::{Token, ASTNode};

static TOKEN_NAMES: [&str; 3] = [
	"ID", "NL", "NUM"
];

#[derive(Debug, Copy, Clone)]
struct Expr {
	pub value: f64
}

#[derive(Debug, Clone)]
enum Node {
	Expr(Expr),
	Program(Option<Expr>),
	Token(Token)
}

impl ASTNode for Node {
	fn new_token(token: &Token) -> Self {
		Self::Token(token.to_owned())
	}

	fn is_token(&self) -> bool {
		match self {
			Self::Token(_) => true,
			_ => false
		}
	}

	fn token(&self) -> Result<&Token, String> {
		match self {
			Self::Token(token) => Ok(token),
			_ => Err("Node is not a token".to_owned())
		}
	}
}

fn expr_num(nodes: &[Node]) -> Node {
	Node::Expr(Expr { value: nodes[0].token().unwrap().symbol().parse::<f64>().unwrap() })
}

fn program_empty(_nodes: &[Node]) -> Node {
	Node::Program(None)
}

fn program(nodes: &[Node]) -> Node {
	match nodes[0] {
		Node::Expr(expr) => Node::Program(Some(expr)),
		_ => panic!("Invalid node")
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
		("NUM", r"(^\d+(\.\d+)?)")
	]) {
		println!("{}", e);
		return;
	}

	let mut parser_builder = parse::ParserBuilder::<Node>::new(&TOKEN_NAMES);
	
	if let Err(e) = parser_builder.add_patterns(&[
		("expr", "NUM", expr_num),
		("program", "", program_empty),
		("program", "expr", program)
	]) {
		println!("{}", e);
		return;
	}

	let mut parser = parser_builder.build();
	let lexer = lexer_builder.build();

	let ast = match parser.parse(lexer.lex(&input)) {
		Ok(x) => x,
		Err((e, pos)) => {
			println!("{} at {}", e, pos);
			return;
		}
	};

	println!("{:#?}", ast);
}