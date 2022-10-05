use crate::{LexerStream, Pattern, ASTNode, Token, Position};

pub struct Parser<N>
where
	N: ASTNode + Clone
{
	token_names: Vec<String>,
	patterns: Vec<Pattern<N>>,
	pos: Position
}

impl<N> Parser<N>
where
	N: ASTNode + Clone
{
	pub fn new(token_names: &[String], patterns: &[Pattern<N>]) -> Self {
		let mut patterns = patterns.to_vec();
		patterns.sort();
		patterns.dedup();

		Self {
			token_names: token_names.to_owned(),
			patterns: patterns.to_owned(),
			pos: Position::new(0, 1, 1)
		}
	}

	pub fn eval_pattern(&mut self, lexer_stream: &mut LexerStream, nodes: &mut Vec<(String, N)>, pattern: &Pattern<N>) -> Result<N, (String, Position)> {
		if nodes.len() < pattern.elems().len() {
			let token = match lexer_stream.next() {
				Some(node) => {
					match node {
						Ok(x) => x,
						Err(e) => return Err(e)
					}
				}
				None => return Err(("Not enough token".to_owned(), self.pos))
			};

			nodes.push((pattern.name().to_owned(), N::new_token(&token)));
		}

		let mut pattern_nodes = Vec::new();

		for (elem, (tag, node)) in pattern.elems().iter().zip(nodes.iter()) {
			if elem != tag {
				return Err(("Invalid pattern".to_owned(), self.pos));
			}

			pattern_nodes.push(node.to_owned());
		}

		Ok(pattern.func()(&pattern_nodes))
	}

	pub fn eval_pattern_by_name(&mut self, lexer_stream: &mut LexerStream, pattern_name: &str, tokens: &mut Vec<Token>) -> Result<N, (String, Position)> {
		let patterns: Vec<Pattern<N>> = self.patterns.iter().filter(|x| x.name() == pattern_name).map(|x| x.clone()).collect();
		let mut nodes: Vec<(String, N)> = Vec::new();
		let mut found_new_pattern = true;

		while found_new_pattern {
			found_new_pattern = false;

			for pattern in &patterns {
				if let Ok(node) = self.eval_pattern(lexer_stream, &mut nodes, pattern) {
					nodes = Vec::new();
					nodes.push((pattern.name().to_owned(), node));
					found_new_pattern = true;
					break;
				}
			}
		}

		match nodes.first() {
			Some((_, node)) => Ok(node.clone()),
			None => Err(("Could not find pattern".to_owned(), self.pos))
		}
	}
	
	pub fn parse(&mut self, mut lexer_stream: LexerStream) -> Result<N, (String, Position)> {
		let res = self.eval_pattern_by_name(&mut lexer_stream, "program", &mut Vec::new());

		if lexer_stream.next().is_some() {
			return Err(("Tokens remaining".to_owned(), self.pos));
		}

		res
	}
}