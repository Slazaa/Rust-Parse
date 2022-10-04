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

	pub fn eval_pattern(&mut self, lexer_stream: &mut LexerStream, pattern: &Pattern<N>) -> Result<N, (String, Position)> {
		todo!();
	}

	pub fn eval_pattern_by_name(&mut self, lexer_stream: &mut LexerStream, pattern_name: &str, tokens: &mut Vec<Token>) -> Result<N, (String, Position)> {
		let patterns: Vec<Pattern<N>> = self.patterns.iter().filter(|x| x.name() == pattern_name).map(|x| x.clone()).collect();
		let mut added_tokens = 0;
		let mut expected_elems = Vec::new();

		'pattern_loop: for pattern in patterns {
			let elems = pattern.elems();

			if elems.is_empty() {
				match lexer_stream.next() {
					Some(token) => {
						let token = token?;
						self.pos = *token.end_pos();
						tokens.push(token);
						added_tokens += 1;
						continue;
					}
					None => return Ok(pattern.func()(&Vec::new()[..]))
				}
			}

			while tokens.len() < elems.len() {
				match lexer_stream.next() {
					Some(token) => {
						let token = token?;
						self.pos = *token.end_pos();
						tokens.push(token);
						added_tokens += 1;
					}
					None => continue 'pattern_loop
				}
			}

			expected_elems.push(elems[0].clone());

			let elems_tokens = &tokens.clone()[tokens.len()-elems.len()..];
			let mut nodes = Vec::new();

			for (elem, token) in elems.iter().zip(elems_tokens.iter()) {
				// Check if elem is a pattern
				if self.patterns.iter().any(|x| x.name() == elem) {
					nodes.push(self.eval_pattern_by_name(lexer_stream, elem, tokens)?);
				}
				// Check if elem is a token
				else if self.token_names.contains(elem) {
					nodes.push(N::token(token))
				}
				// Else invalid elem
				else {
					return Err((format!("Invalid element '{}'", elem), self.pos))
				}
			}

			if elems.len() == nodes.len() {
				// TODO: - Try other patterns
				return Ok(pattern.func()(&nodes[..]));
			}
		}
	
		let mut expected_str = String::new();

		for (i, exp) in expected_elems.iter().enumerate() {
			expected_str.push_str(&format!("'{}'", exp));

			if i + 2 < expected_elems.len() {
				expected_str.push_str(", ");
			} else if i + 1 < expected_elems.len() {
				expected_str.push_str(" or ");
			}
		}

		Err((format!("Failed creating '{}' node, expected {}", pattern_name, expected_str), self.pos))
	}
	
	pub fn parse(&mut self, mut lexer_stream: LexerStream) -> Result<N, (String, Position)> {
		let res = self.eval_pattern_by_name(&mut lexer_stream, "program", &mut Vec::new());

		if lexer_stream.next().is_some() {
			return Err(("Tokens remaining".to_owned(), self.pos));
		}

		res
	}
}