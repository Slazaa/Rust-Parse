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

	pub fn is_elem_token(&self, elem: &str) -> bool {
		self.token_names.contains(&elem.to_owned())
	}

	pub fn is_elem_node(&self, elem: &str) -> bool {
		self.patterns.iter().map(|x| x.name()).any(|x| x == &elem.to_owned())
	}

	pub fn eval_pattern(&mut self, lexer_stream: &mut LexerStream, nodes: &mut Vec<(String, N)>, pattern: &Pattern<N>) -> Result<N, (ParserError, Position)> {
		let mut tokens = nodes.clone();
		
		for (idx, elem) in pattern.elems().iter().enumerate() {
			if self.is_elem_token(elem) {
				while nodes.len() <= idx {
					let token = match lexer_stream.next() {
						Some(node) => {
							match node {
								Ok(x) => x,
								Err(e) => return Err(e)
							}
						}
						None => return Err((ParserError::NotMatching, self.pos))
					};

					tokens.push((token.name().to_owned(), N::new_token(&token)));
					nodes.push((token.name().to_owned(), N::new_token(&token)));
				}

				if nodes[idx].0 != *elem {
					return Err((ParserError::NotMatching, self.pos));
				}
			} else if self.is_elem_node(elem) {
				let mut eval_nodes = match nodes.len() > idx {
					false => Vec::new(),
					true => nodes.drain(idx..).collect::<Vec<(String, N)>>()
				};

				let res_node = match self.eval_pattern_by_name(lexer_stream, elem, &mut eval_nodes) {
					Ok(x) => x,
					Err(e) => {
						*nodes = tokens;
						return Err(e);
					}
				};

				nodes.push((elem.to_owned(), res_node));
				nodes.append(&mut eval_nodes);
			} else {
				*nodes = tokens;
				return Err((ParserError::UnknownElem(elem.to_owned()), self.pos));
			}
		}

		match pattern.func()(&nodes[..pattern.elems().len()].iter().map(|(_, x)| x).cloned().collect::<Vec<N>>()) {
			Ok(x) => Ok(x),
			Err(e) => {
				*nodes = tokens;
				Err((ParserError::PatternFunc(e), self.pos))
			}
		}
	}

	pub fn eval_pattern_by_name(&mut self, lexer_stream: &mut LexerStream, pattern_name: &str, nodes: &mut Vec<(String, N)>) -> Result<N, (ParserError, Position)> {
		let patterns: Vec<Pattern<N>> = self.patterns.iter().filter(|x| x.name() == pattern_name).cloned().collect();

		if patterns.is_empty() {
			return Err((ParserError::InvalidPatternName, self.pos));
		}

		for pattern in &patterns {
			match self.eval_pattern(lexer_stream, nodes, pattern) {
				Ok(node) => {
					*nodes = nodes[pattern.elems().len()..].to_vec();
					return Ok(node);
				}
				Err(e) => {
					match e.0 {
						ParserError::NotMatching => (),
						_ => {
							println!("{:?}", e);
							return Err(e)
						}
					}
				}
			}
		}

		Err((ParserError::NotMatching, self.pos))
	}

	pub fn parse(&mut self, mut lexer_stream: LexerStream) -> Result<N, (ParserError, Position)> {
		let mut nodes = Vec::new();

		let res = match self.eval_pattern_by_name(&mut lexer_stream, "program", &mut nodes) {
			Ok(x) => x,
			Err(e) => return Err(e)
		};

		let mut remain_nodes = Vec::new();

		for node in nodes.iter().map(|(_, x)| x) {
			remain_nodes.push(node.clone());
		}

		for token in lexer_stream {
			match token {
				Ok(token) => remain_nodes.push(N::new_token(&token)),
				Err(e) => {
					println!("{:?}", e);
					return Ok(res);
				}
			}
		}

		if !remain_nodes.is_empty() {
			println!("{:#?}", remain_nodes);
			return Err((ParserError::TokenRemaining, self.pos));
		}

		Ok(res)
	}
}