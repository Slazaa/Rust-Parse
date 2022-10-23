use std::fmt::Debug;

use crate::{LexerStream, Pattern, ASTNode, Position};

#[derive(Debug)]
pub enum Error {
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

	fn is_elem_token(&self, elem: &str) -> bool {
		self.token_names.contains(&elem.to_owned())
	}

	fn is_elem_node(&self, elem: &str) -> bool {
		self.patterns.iter().map(|x| x.name()).any(|x| x == &elem.to_owned())
	}

	fn eval_pattern(&mut self, lexer_stream: &mut LexerStream, pattern: &Pattern<N>, mut tokens: Vec<(String, N)>) -> (Result<N, (Error, Position)>, Vec<(String, N)>) {
		let mut nodes = tokens.clone();

		for (idx, elem) in pattern.elems().iter().enumerate() {
			if self.is_elem_token(elem) {
				while nodes.len() <= idx {
					let token = match lexer_stream.next() {
						Some(Ok(x)) => x,
						Some(Err(e)) => return (Err(e), tokens),
						None => return (Err((Error::NotMatching, self.pos)), tokens)
					};

					nodes.push((token.name().to_owned(), N::new_token(&token)));
					tokens.push((token.name().to_owned(), N::new_token(&token)));
				}

				if nodes[idx].0 != *elem {
					return (Err((Error::NotMatching, self.pos)), tokens);
				}
			} else if self.is_elem_node(elem) {
				let eval_tokens = match nodes.len() > idx {
					false => Vec::new(),
					true => nodes.drain(idx..).collect()
				};

				let (res_node, mut rem_tokens) = match self.eval_pattern_by_name(lexer_stream, elem, eval_tokens) {
					(Ok(x), tokens) => (x, tokens),
					(Err(e), rem_tokens) => return (Err(e), rem_tokens)
				};

				nodes.push((elem.to_owned(), res_node));
				nodes.append(&mut rem_tokens);
			} else {
				return (Err((Error::UnknownElem(elem.to_owned()), self.pos)), tokens);
			}
		}

		match pattern.func()(&nodes[..pattern.elems().len()].iter().map(|(_, x)| x).cloned().collect::<Vec<N>>()) {
			Ok(x) => (Ok(x), nodes[pattern.elems().len()..].to_vec()),
			Err(e) => (Err((Error::PatternFunc(e), self.pos)), tokens)
		}
	}

	fn eval_pattern_by_name(&mut self, lexer_stream: &mut LexerStream, pattern_name: &str, mut tokens: Vec<(String, N)>) -> (Result<N, (Error, Position)>, Vec<(String, N)>) {
		let patterns: Vec<Pattern<N>> = self.patterns.iter().filter(|x| x.name() == pattern_name).cloned().collect();

		if patterns.is_empty() {
			return (Err((Error::InvalidPatternName, self.pos)), Vec::new());
		}

		for pattern in &patterns {
			match self.eval_pattern(lexer_stream, pattern, tokens.to_owned()) {
				(Ok(node), rem_tokens) => return (Ok(node), rem_tokens),
				(Err((Error::NotMatching, _)), rem_tokens) => tokens = rem_tokens,
				(Err(e), rem_tokens) => return (Err(e), rem_tokens)
			}
		}

		(Err((Error::NotMatching, self.pos)), tokens)
	}

	pub fn parse(&mut self, mut lexer_stream: LexerStream) -> Result<N, (Error, Position)> {
		let (res_node, mut rem_tokens) = match self.eval_pattern_by_name(&mut lexer_stream, "program", Vec::new()) {
			(Ok(node), rem_tokens) => (node, rem_tokens.iter().map(|(_, token)| token.to_owned()).collect::<Vec<N>>()),
			(Err(e), _) => return Err(e)
		};

		for token in lexer_stream {
			match token {
				Ok(token) => rem_tokens.push(N::new_token(&token)),
				Err(e) => return Err(e)
			}
		}

		if !rem_tokens.is_empty() {
			println!("{:#?}", rem_tokens);
			return Err((Error::TokenRemaining, self.pos));
		}

		Ok(res_node)
	}
}