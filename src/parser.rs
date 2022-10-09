use std::fmt::Debug;

use crate::{LexerStream, Pattern, ASTNode, Position};

#[derive(Debug)]
pub enum ParserError {
	InvalidPatternName,
	InvalidToken,
	NotMatching,
	TokenRemaining,
	UnknownElem(String),
	PatternFunc(String)
}

pub struct Parser<N>
where
	N: ASTNode + Clone + Debug
{
	token_names: Vec<String>,
	patterns: Vec<Pattern<N>>,
	pos: Position
}

impl<N> Parser<N>
where
	N: ASTNode + Clone + Debug
{
	pub fn new(token_names: &[String], patterns: &[Pattern<N>]) -> Self {
		let mut patterns = patterns.to_vec();
		patterns.dedup();

		Self {
			token_names: token_names.to_owned(),
			patterns: patterns.to_owned(),
			pos: Position::new(0, 1, 1)
		}
	}

	pub fn eval_pattern(&mut self, lexer_stream: &mut LexerStream, nodes: &mut Vec<(String, N)>, pattern: &Pattern<N>) -> Result<N, (ParserError, Position)> {
		for (idx, elem) in pattern.elems().iter().enumerate() {
			// Check if the pattern element is valid
			if !self.token_names.contains(elem) && !self.patterns.iter().map(|x| x.name()).collect::<Vec<&String>>().contains(&elem) {
				return Err((ParserError::UnknownElem(elem.to_owned()), self.pos));
			}
			
			// Check if the pattern element is a pattern
			// If it is, evaluate the pattern
			if !self.token_names.contains(elem) {
				let mut was_eval_nodes = false;
				let mut eval_nodes = if nodes.len() > idx {
					was_eval_nodes = true;
					nodes[idx..].to_vec()
				} else {
					vec![]
				};

				let mut node_used_count = 0;

				let res_node = match self.eval_pattern_by_name(lexer_stream, elem, &mut eval_nodes, Some(&mut node_used_count)) {
					Ok(x) => x,
					Err(e) => {
						if eval_nodes.len() >= node_used_count {
							nodes.append(&mut eval_nodes[node_used_count..].to_owned());
						}

						return Err(e)
					}
				};

				// Replace the last nodes with the new evaluated node
				if was_eval_nodes {
					nodes.drain(idx..idx+node_used_count);
					nodes.insert(idx, (elem.to_owned(), res_node));
				} else {
					nodes.push((elem.to_owned(), res_node));
				}

				if eval_nodes.len() >= node_used_count {
					nodes.append(&mut eval_nodes[node_used_count..].to_owned());
				}

				continue;
			}

			// Get new token, if the pattern is longer than the current number of token
			if nodes.len() <= idx {
				let token = match lexer_stream.next() {
					Some(node) => {
						match node {
							Ok(x) => x,
							Err(e) => return Err(e)
						}
					}
					None => return Err((ParserError::NotMatching, self.pos))
				};

				nodes.push((token.name().to_owned(), N::new_token(&token)));
			}

			let (tag, _node) = &nodes[idx];

			// Else, that means it's a token
			// Check if the pattern element is different from the node tag
			// If it is, that means the nodes don't match the pattern
			if elem != tag {
				return Err((ParserError::NotMatching, self.pos));
			}
		}

		match pattern.func()(&nodes[..pattern.elems().len()].iter().map(|(_, x)| x).cloned().collect::<Vec<N>>()) {
			Ok(x) => Ok(x),
			Err(e) => Err((ParserError::PatternFunc(e), self.pos))
		}
	}

	pub fn eval_pattern_by_name(&mut self, lexer_stream: &mut LexerStream, pattern_name: &str, nodes: &mut Vec<(String, N)>, node_used_count: Option<&mut usize>) -> Result<N, (ParserError, Position)> {
		let patterns: Vec<Pattern<N>> = self.patterns.iter().filter(|x| x.name() == pattern_name).cloned().collect();

		// Check if a pattern matches this name
		// If not, return an error
		if patterns.is_empty() {
			return Err((ParserError::InvalidPatternName, self.pos));
		}

		let mut found_pattern = false;
		let mut res_node = None;

		for pattern in &patterns {
			match self.eval_pattern(lexer_stream, nodes, pattern) {
				Ok(node) => {
					*nodes = nodes[pattern.elems().len()..].to_vec();
					res_node = Some(node);
					
					if let Some(node_used_count) = node_used_count {
						*node_used_count = pattern.elems().len();
					}

					found_pattern = true;
					break;
				}
				Err(e) => {
					match e.0 {
						ParserError::NotMatching => (),
						_ => return Err(e)
					}
				}
			}
		}

		if !found_pattern {
			return Err((ParserError::NotMatching, self.pos));
		}

		match res_node {
			Some(x) => Ok(x),
			None => panic!()
		}
	}
	
	pub fn parse(&mut self, mut lexer_stream: LexerStream) -> Result<N, (ParserError, Position)> {
		let mut nodes = Vec::new();
		let res = match self.eval_pattern_by_name(&mut lexer_stream, "program", &mut nodes, None) {
			Ok(x) => x,
			Err(e) => return Err(e)
		};

		if nodes.len() > 1 {
			println!("{:#?}", nodes);
			return Err((ParserError::TokenRemaining, self.pos));
		}

		Ok(res)
	}
}
