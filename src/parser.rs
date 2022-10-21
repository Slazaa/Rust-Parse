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
/*
	pub fn eval_pattern(&mut self, lexer_stream: &mut LexerStream, nodes: &mut Vec<(String, N)>, pattern: &Pattern<N>) -> Result<N, (ParserError, Position)> {
		if pattern.elems().is_empty() {
			println!("{} {:?}", pattern.name(), nodes.iter().map(|(x, _)| x).collect::<Vec<&String>>());
		}

		for (idx, elem) in pattern.elems().iter().enumerate() {
			// Check if the pattern element is valid
			if !self.token_names.contains(elem) && !self.patterns.iter().any(|x| x.name() == elem) {
				return Err((ParserError::UnknownElem(elem.to_owned()), self.pos));
			}

			println!("{} {} {:?}", pattern.name(), elem, nodes.iter().map(|(x, _)| x).collect::<Vec<&String>>());
			
			// Check if the pattern element is a pattern
			// If it is, evaluate the pattern
			if !self.token_names.contains(elem) {
				let mut eval_nodes = if nodes.len() > idx {
					nodes[idx..].to_vec()
				} else {
					vec![]
				};

				let mut node_used_count = 0;

				println!("BEF {} {} {:?}", pattern.name(), elem, nodes.iter().map(|(x, _)| x).collect::<Vec<&String>>());

				if !eval_nodes.is_empty() {
					nodes.drain(idx..);
				}

				let res_node = match self.eval_pattern_by_name(lexer_stream, elem, &mut eval_nodes, Some(&mut node_used_count)) {
					Ok(x) => x,
					Err(e) => {
						nodes.append(&mut eval_nodes);
						return Err(e)
					}
				};

				println!("AFT {} {} {:?}", pattern.name(), elem, nodes.iter().map(|(x, _)| x).collect::<Vec<&String>>());

				// Replace the last nodes with the new evaluated node
				nodes.push((elem.to_owned(), res_node));
				nodes.append(&mut eval_nodes);

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
*/
    pub fn is_elem_token(&self, elem: &str) -> bool {
        self.token_names.contains(&elem.to_owned())
    }

    pub fn is_elem_node(&self, elem: &str) -> bool {
        self.patterns.iter().map(|x| x.name()).any(|x| x == &elem.to_owned())
    }

	pub fn eval_pattern(&mut self, lexer_stream: &mut LexerStream, tokens: &mut Vec<(String, N)>, nodes: &mut Vec<(String, N)>, pattern: &Pattern<N>) -> Result<N, (ParserError, Position)> {
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
				println!("BEF {} {} {:?}", pattern.name(), elem, nodes.iter().map(|(x, _)| x).collect::<Vec<&String>>());

                let mut eval_tokens = Vec::new();
                let mut eval_nodes = match nodes.len() > idx {
                    false => Vec::new(),
                    true => nodes.drain(idx..).collect::<Vec<(String, N)>>()
                };

                let res_node = match self.eval_pattern_by_name(lexer_stream, elem, &mut eval_tokens, &mut eval_nodes) {
                    Ok(x) => x,
                    Err(e) => {
                        nodes.append(&mut eval_tokens);
                        return Err(e);
                    }
                };

                nodes.push((elem.to_owned(), res_node));
                nodes.append(&mut eval_nodes);

				println!("AFT {} {} {:?}", pattern.name(), elem, nodes.iter().map(|(x, _)| x).collect::<Vec<&String>>());
            } else {
				return Err((ParserError::UnknownElem(elem.to_owned()), self.pos));
            }
		}

		match pattern.func()(&nodes[..pattern.elems().len()].iter().map(|(_, x)| x).cloned().collect::<Vec<N>>()) {
			Ok(x) => Ok(x),
			Err(e) => Err((ParserError::PatternFunc(e), self.pos))
		}
	}

	pub fn eval_pattern_by_name(&mut self, lexer_stream: &mut LexerStream, pattern_name: &str, tokens: &mut Vec<(String, N)>, nodes: &mut Vec<(String, N)>) -> Result<N, (ParserError, Position)> {
		let patterns: Vec<Pattern<N>> = self.patterns.iter().filter(|x| x.name() == pattern_name).cloned().collect();

		if patterns.is_empty() {
			return Err((ParserError::InvalidPatternName, self.pos));
		}

		let mut res_node = None;

		for pattern in &patterns {
			match self.eval_pattern(lexer_stream, tokens, nodes, pattern) {
				Ok(node) => {
					// Keep the nodes that were not used during the evaluation
					*nodes = nodes[pattern.elems().len()..].to_vec();
					res_node = Some(node);

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

		match res_node {
			Some(x) => Ok(x),
			None => Err((ParserError::NotMatching, self.pos)) 
		}
	}

	pub fn parse(&mut self, mut lexer_stream: LexerStream) -> Result<N, (ParserError, Position)> {
		let mut nodes = Vec::new();
		let mut tokens = Vec::new();

		let res = match self.eval_pattern_by_name(&mut lexer_stream, "program", &mut tokens, &mut nodes) {
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