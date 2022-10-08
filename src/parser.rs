use std::fmt::Debug;

use crate::{LexerStream, Pattern, ASTNode, Position};

#[derive(Debug)]
pub enum ParserError {
	InvalidPatternName,
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

			// Get new tokens, if the pattern is longer than the current number of token
			while nodes.len() < pattern.elems().len() {
				let token = match lexer_stream.next() {
					Some(node) => {
						match node {
							Ok(x) => x,
							Err((_, pos)) => return Err((ParserError::NotMatching, pos))
						}
					}
					None => return Err((ParserError::NotMatching, self.pos))
				};
	
				nodes.push((token.name().to_owned(), N::new_token(&token)));
			}

			let (tag, _node) = &nodes[idx];

			// Check if the pattern element is a pattern
			// If it is, evaluate the pattern
            if !self.token_names.contains(elem) {
                let mut eval_nodes = nodes[idx..].to_vec();
				nodes.drain(idx..);

                let res_node = match self.eval_pattern_by_name(lexer_stream, elem, &mut eval_nodes) {
                    Ok(x) => x,
                    Err(e) => {
                        nodes.append(&mut eval_nodes);
                        return Err(e);
                    }
                };

				// Replace the last nodes with the new evaluated node
                nodes.push((elem.to_owned(), res_node));
                continue;
            }

			// Else, that means it's a token
			// Check if the pattern element is different from the node tag
			// If it is, that means the nodes don't match the pattern
			else if elem != tag {
				return Err((ParserError::NotMatching, self.pos));
			}
		}

		match pattern.func()(&nodes[..pattern.elems().len()].iter().map(|(_, x)| x).cloned().collect::<Vec<N>>()) {
			Ok(x) => Ok(x),
			Err(e) => Err((ParserError::PatternFunc(e), self.pos))
		}
	}

	pub fn eval_pattern_by_name(&mut self, lexer_stream: &mut LexerStream, pattern_name: &str, nodes: &mut Vec<(String, N)>) -> Result<N, (ParserError, Position)> {
		let patterns: Vec<Pattern<N>> = self.patterns.iter().filter(|x| x.name() == pattern_name).cloned().collect();

		// Check if a pattern matches this name
		// If not, return an error
        if patterns.is_empty() {
            return Err((ParserError::InvalidPatternName, self.pos));
        }

        let mut found_pattern = false;

		for pattern in &patterns {
			match self.eval_pattern(lexer_stream, nodes, pattern) {
				Ok(node) => {
					*nodes = Vec::new();
					nodes.push((pattern.name().to_owned(), node));
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

        Ok(nodes.first().unwrap().1.clone())
	}
	
	pub fn parse(&mut self, mut lexer_stream: LexerStream) -> Result<N, (ParserError, Position)> {
		let res = self.eval_pattern_by_name(&mut lexer_stream, "program", &mut Vec::new());

		if lexer_stream.next().is_some() {
			return Err((ParserError::TokenRemaining, self.pos));
		}

		res
	}
}
