use std::fmt::{self, Debug};

use crate::{Pattern, ASTNode, Position, Token};

#[derive(Debug)]
pub enum Error<E> {
	FileNotFound(String),
	InvalidPatternName(String),
	InvalidToken,
	NotMatching(String),
	TokenRemaining,
	UnknownElem(String),
	PatternFunc(E)
}

impl<E> fmt::Display for Error<E>
where
	E: fmt::Display
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", match self {
			Self::FileNotFound(filename) => format!("File not found '{}'", filename),
			Self::InvalidPatternName(name) => format!("Invalid pattern name '{}'", name),
			Self::InvalidToken => "Invalid token".to_owned(),
			Self::NotMatching(pattern_name) => format!("Could not find match for pattern '{}'", pattern_name),
			Self::TokenRemaining => "Unused token remaining".to_owned(),
			Self::UnknownElem(elem) => format!("Unknown element '{}'", elem),
			Self::PatternFunc(err) => format!("{}", err)
		})
	}
}

pub struct Parser<N, E>
where
	N: ASTNode + Clone + Debug,
	E: Clone
{
	token_names: Vec<String>,
	patterns: Vec<Pattern<N, E>>,
	pos: Position
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
			pos: Position::default()
		}
	}

	fn is_elem_token(&self, elem: &str) -> bool {
		self.token_names.contains(&elem.to_owned())
	}

	fn is_elem_node(&self, elem: &str) -> bool {
		self.patterns.iter().map(|x| x.name()).any(|x| x == &elem.to_owned())
	}
	/*
	fn eval_pattern(&mut self, lexer_stream: &mut LexerStream<E>, pattern: &Pattern<N, E>, mut tokens: Vec<(String, N)>) -> (Result<N, (Error<E>, Position)>, Vec<(String, N)>) {
		let mut nodes = tokens.clone();

		for (idx, elem) in pattern.elems().iter().enumerate() {
			if self.is_elem_token(elem) {
				while nodes.len() <= idx {
					let token = match lexer_stream.next() {
						Some(Ok(x)) => x,
						Some(Err(e)) => return (Err(e), tokens),
						None => return (Err((Error::NotMatching(pattern.name().to_owned()), self.pos.to_owned())), tokens)
					};

					nodes.push((token.name.to_owned(), N::new_token(&token)));
					tokens.push((token.name.to_owned(), N::new_token(&token)));
				}

				if nodes[idx].0 != *elem {
					return (Err((Error::NotMatching(pattern.name().to_owned()), self.pos.to_owned())), tokens);
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
				return (Err((Error::UnknownElem(elem.to_owned()), self.pos.to_owned())), tokens);
			}
		}

		match pattern.func()(&nodes[..pattern.elems().len()].iter().map(|(_, x)| x).cloned().collect::<Vec<N>>()) {
			Ok(x) => (Ok(x), nodes[pattern.elems().len()..].to_vec()),
			Err(e) => (Err((Error::PatternFunc(e), self.pos.to_owned())), tokens)
		}
	}

	fn eval_pattern_by_name(&mut self, lexer_stream: &mut LexerStream<E>, pattern_name: &str, mut tokens: Vec<(String, N)>) -> (Result<N, (Error<E>, Position)>, Vec<(String, N)>) {
		let patterns: Vec<Pattern<N, E>> = self.patterns.iter().filter(|x| x.name() == pattern_name).cloned().collect();

		if patterns.is_empty() {
			return (Err((Error::InvalidPatternName(pattern_name.to_owned()), self.pos.to_owned())), Vec::new());
		}

		for pattern in &patterns {
			match self.eval_pattern(lexer_stream, pattern, tokens.to_owned()) {
				(Ok(node), rem_tokens) => return (Ok(node), rem_tokens),
				(Err((Error::NotMatching(_), _)), rem_tokens) => tokens = rem_tokens,
				(Err(e), rem_tokens) => return (Err(e), rem_tokens)
			}
		}

		(Err((Error::NotMatching(pattern_name.to_owned()), self.pos.to_owned())), tokens)
	}

	pub fn parse(&mut self, mut lexer_stream: LexerStream<E>) -> Result<N, (Error<E>, Position)> {
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
			return Err((Error::TokenRemaining, self.pos.to_owned()));
		}

		Ok(res_node)
	}
	*/
	fn eval_pattern(&mut self, tokens: &[Token], pattern: &Pattern<N, E>) -> Result<N, Error<E>> {
		let mut nodes = Vec::new();
		let mut curr_idx = 0;

		for (idx, elem) in pattern.elems().iter().enumerate() {
			if self.is_elem_token(elem) {
				while nodes.len() <= idx {
					let token_name = tokens[curr_idx].name.to_owned();

					if token_name != *elem {
						return Err(Error::NotMatching(pattern.name().to_owned()));
					}

					nodes.push((token_name, N::new_token(&tokens[curr_idx])));
					curr_idx += 1;
				}
			} else if self.is_elem_node(elem) {
				let res = match self.eval_pattern_by_name(&tokens[curr_idx..], elem) {
					Ok(x) => x,
					Err(e) => return Err(e)
				};

				nodes.push((elem.to_owned(), res));
				curr_idx += 1;
			} else {
				return Err(Error::UnknownElem(elem.to_owned()));
			}

			if curr_idx >= tokens.len() {
				return Err(Error::NotMatching(pattern.name().to_owned()));
			}
		}

		match pattern.func()(&nodes[..pattern.elems().len()].iter().map(|(_, x)| x).cloned().collect::<Vec<N>>()) {
			Ok(x) => Ok(x),
			Err(e) => Err(Error::PatternFunc(e))
		}
	}

	fn eval_pattern_by_name(&mut self, tokens: &[Token], pattern_name: &str) -> Result<N, Error<E>> {
		let patterns: Vec<Pattern<N, E>> = self.patterns.iter().filter(|x| x.name() == pattern_name).cloned().collect();

		if patterns.is_empty() {
			return Err((Error::InvalidPatternName(pattern_name.to_owned()), self.pos.to_owned()));
		}

		for pattern in &patterns {
			match self.eval_pattern(tokens, pattern) {
				Ok(node) => return Ok(node),
				Err((Error::NotMatching(_), _)) => (),
				Err(e) => return Err(e)
			}
		}

		Err(Error::NotMatching(pattern_name.to_owned()))
	}

	pub fn parse(&mut self, tokens: &[Token]) -> Result<N, Error<E>> {
		self.eval_pattern_by_name(tokens, "program")
	}
}