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

		Self {
			token_names: token_names.to_owned(),
			patterns: patterns.to_owned(),
			pos: Position::new(0, 1, 1)
		}
	}

	pub fn eval_token(&self, token: &Token, token_name: &str) -> N {


		todo!();
	}

	pub fn eval_pattern(&mut self, lexer_stream: &mut LexerStream, pattern_name: &str, tokens: &mut Vec<Token>) -> Result<N, (String, Position)> {
		let patterns: Vec<&Pattern<N>> = self.patterns.iter().filter(|x| x.name() == pattern_name).collect();
		let mut added_tokens = 0;
		let mut expected = Vec::new();

		'pattern_loop: for pattern in patterns {
			let elems = pattern.elems();

			if let Some(elem) = elems.last() {
				if !expected.contains(elem) {
					expected.push(elem.to_owned());
				}
			}

			while tokens.len() < elems.len() {
				match lexer_stream.next() {
					Some(token) => {
						let token = token?;
						self.pos = *token.start_pos();
						tokens.push(token);
					}
					None => continue 'pattern_loop
				}
			}

			for elem in elems {
				// Check if elem is a pattern
				if self.patterns.iter().any(|x| x.name() == pattern_name) {
					//self.eval_pattern(lexer_stream, elem, tokens)
				}
			}
		}

		let mut expected_str = String::new();

		for (i, exp) in expected.iter().enumerate() {
			expected_str.push_str(&format!("'{}'", exp));

			if i + 2 < expected.len() {
				expected_str.push_str(", ");
			} else if i + 1 < expected.len() {
				expected_str.push_str(" or ");
			}
		}

		Err((format!("Failed creating '{}' node, expected {}", pattern_name, expected_str), self.pos))
	}
	
	pub fn parse(&mut self, mut lexer_stream: LexerStream) -> Result<N, (String, Position)> {
		self.eval_pattern(&mut lexer_stream, "program", &mut Vec::new())
	}
}