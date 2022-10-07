use std::fmt::Debug;

use crate::{LexerStream, Pattern, ASTNode, Position};

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

		    nodes.push((token.name().to_owned(), N::new_token(&token)));
		}

		for ((idx, elem), (tag, node)) in pattern.elems().iter().enumerate().zip(nodes.clone().iter()) {
            if !self.token_names.contains(elem) {
                let mut eval_nodes = nodes[idx..].to_vec();

                let res_node = match self.eval_pattern_by_name(lexer_stream, elem, &mut eval_nodes) {
                    Ok(x) => x,
                    Err(e) => {
                        nodes.append(&mut eval_nodes);
                        return Err(e);
                    }
                };

                nodes.drain(idx..);
                nodes.push((elem.to_owned(), res_node));

                continue;
            } else if elem != tag {
				return Err(("Invalid pattern".to_owned(), self.pos));
			}

			nodes.push((tag.to_owned(), node.to_owned()));
		}

		Ok(pattern.func()(&nodes.iter().map(|(_, x)| x.clone()).collect::<Vec<N>>()))
	}

	pub fn eval_pattern_by_name(&mut self, lexer_stream: &mut LexerStream, pattern_name: &str, nodes: &mut Vec<(String, N)>) -> Result<N, (String, Position)> {
		let patterns: Vec<Pattern<N>> = self.patterns.iter().filter(|x| x.name() == pattern_name).cloned().collect();

        if patterns.is_empty() {
            return Err((format!("Invalid pattern name '{}'", pattern_name), self.pos));
        }

        let mut found_pattern = false;

		for pattern in &patterns {
			if let Ok(node) = self.eval_pattern(lexer_stream, nodes, pattern) {
				*nodes = Vec::new();
				nodes.push((pattern.name().to_owned(), node));
				found_pattern = true;
				break;
			}
		}

        if !found_pattern {
            return Err(("Could not find pattern".to_owned(), self.pos));
        }

        Ok(nodes.first().unwrap().1.clone())
	}
	
	pub fn parse(&mut self, mut lexer_stream: LexerStream) -> Result<N, (String, Position)> {
	/*
		let res = self.eval_pattern_by_name(&mut lexer_stream, "program", &mut Vec::new());

		if lexer_stream.next().is_some() {
			return Err(("Tokens remaining".to_owned(), self.pos));
		}

		res
	*/
		self.eval_pattern_by_name(&mut lexer_stream, "program", &mut Vec::new())
	}
}
