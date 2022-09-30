use std::{env, fs};

use parse::Token;

static TOKEN_NAMES: [&str; 3] = [
	"ID", "NL", "NUM"
];

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

	let mut parser_builder = parse::ParserBuilder::new(&TOKEN_NAMES);
	parser_builder.add_patterns(&[
		"expr : NUM",
		"program : expr"
	]);

	let parser = parser_builder.build();
	let lexer = lexer_builder.build();

	let _ast = parser.parse(lexer.lex(&input));
}