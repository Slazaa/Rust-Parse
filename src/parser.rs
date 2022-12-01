use std::fmt::Debug;

use crate::{Pattern, ASTNode, Token};

#[derive(Debug)]
pub enum ParserError<E> {
	InvalidPatternName(String),
	NotMatching(String),
	PatternFunc(E),
	TokenRemaining,
	UnknownElem(String)
}

pub struct Parser<N, E>
where
	N: ASTNode + Clone + Debug,
	E: Clone
{
	token_names: Vec<String>,
	patterns: Vec<Pattern<N, E>>,
	token_remaining: bool
}

impl<N, E> Parser<N, E>
where
	N: ASTNode + Clone + Debug,
	E: Clone
{
	pub fn new(token_names: &[String], patterns: &[Pattern<N, E>]) -> Self {
		let mut patterns = patterns.to_vec();
		patterns.dedup();

		Self {
			token_names: token_names.to_owned(),
			patterns: patterns.to_owned(),
			token_remaining: true
		}
	}

	fn is_elem_token(&self, elem: &str) -> bool {
		self.token_names.contains(&elem.to_owned())
	}

	fn is_elem_node(&self, elem: &str) -> bool {
		self.patterns.iter().map(|x| x.name()).any(|x| x == &elem.to_owned())
	}
	
	fn eval_pattern(&mut self, tokens: &[Token], pattern: &Pattern<N, E>) -> Result<(N, usize), ParserError<E>> {
		let elems = pattern.elems();
		let mut nodes = Vec::new();

		let mut idx = 0;

		for elem in elems {
			if self.is_elem_token(elem) {
				if idx >= tokens.len() {
					return Err(ParserError::NotMatching(pattern.name().to_owned()));
				}

				if &tokens[idx].name != elem {
					return Err(ParserError::NotMatching(pattern.name().to_owned()));
				}

				nodes.push((elem, N::new_token(&tokens[idx])));
				idx += 1;
			} else if self.is_elem_node(elem) {
				let (node, token_used_count) = self.eval_pattern_by_name(&tokens[idx..], elem)?;
				nodes.push((elem, node));
				idx += token_used_count;
			} else {
				return Err(ParserError::UnknownElem(elem.to_owned()));
			}
		}

		if self.token_remaining && idx >= tokens.len() {
			self.token_remaining = false;
		}

		match pattern.func()(&nodes.iter().map(|(_, x)| x).cloned().collect::<Vec<N>>()) {
			Ok(x) => Ok((x, idx)),
			Err(e) => Err(ParserError::PatternFunc(e))
		}
	}

	fn eval_pattern_by_name(&mut self, tokens: &[Token], pattern_name: &str) -> Result<(N, usize), ParserError<E>> {
		let patterns: Vec<Pattern<N, E>> = self.patterns.iter().filter(|x| x.name() == pattern_name).cloned().collect();

		if patterns.is_empty() {
			return Err(ParserError::InvalidPatternName(pattern_name.to_owned()));
		}

		for pattern in &patterns {
			match self.eval_pattern(tokens, pattern) {
				Ok(res) => return Ok(res),
				Err(ParserError::NotMatching(_)) => (),
				Err(e) => return Err(e)
			}
		}

		Err(ParserError::NotMatching(pattern_name.to_owned()))
	}

	pub fn parse(&mut self, tokens: &[Token]) -> Result<N, ParserError<E>> {
		let res = self.eval_pattern_by_name(tokens, "program")?.0;

		if self.token_remaining {
			return Err(ParserError::TokenRemaining);
		}

		Ok(res)
	}
}