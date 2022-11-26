use std::{fs, marker::PhantomData};

use regex::Match;

use crate::{Rule, Token, Position, Error, Loc};

#[derive(Clone)]
pub struct Lexer {
	rules: Vec<Rule>,
	ignore_rules: Vec<Rule>
}

impl Lexer {
	pub fn new(rules: Vec<Rule>, ignore_rules: Vec<Rule>) -> Self {
		Self {
			rules,
			ignore_rules
		}
	}

	pub fn rules(&self) -> &Vec<Rule> {
		&self.rules
	}

	pub fn ignore_rules(&self) -> &Vec<Rule> {
		&self.ignore_rules
	}

	pub fn lex<E>(&self, input: &str) -> LexerStream<E> {
		LexerStream::new(self, input, None)
	}

	pub fn lex_from_file<E>(&self, filename: &str) -> Result<LexerStream<E>, Error<E>> {
		let input = match fs::read_to_string(filename) {
			Ok(x) => x,
			Err(_) => return Err(Error::FileNotFound(filename.to_owned()))
		};

		Ok(LexerStream::new(self, &input, Some(filename.to_owned())))
	}
}

pub struct LexerStream<E> {
	lexer: Lexer,
	input: String,
	loc: Loc,
	phantom: PhantomData<E>
}

impl<E> LexerStream<E> {
	pub fn new(lexer: &Lexer, input: &str, filename: Option<String>) -> Self {
		Self {
			lexer: lexer.clone(),
			input: input.to_owned(),
			loc: Loc {
				filename,
				..Default::default()
			},
			phantom: PhantomData
		}
	}

	pub fn update_pos(&mut self, mat: &Match) {
		self.loc.end.idx += mat.end();
		self.loc.end.line += mat.as_str().matches('\n').count();

		self.loc.end.col = if self.input[..mat.start()].rfind('\n').is_some() {
			1
		} else {
			self.loc.end.col + mat.end()
		};
		
		self.input = self.input[mat.end()..].to_owned();
	}
}

impl<E> Iterator for LexerStream<E> {
	type Item = Result<Token, (Error<E>, Position)>;

	fn next(&mut self) -> Option<Self::Item> {
		self.loc.start = self.loc.end.to_owned();

		loop {
			if self.input.is_empty() {
				return None;
			}

			let mut found_mat = false;

			for rule in self.lexer.ignore_rules() {
				if let Some(mat) = rule.pattern().find(&self.input.clone()) {
					found_mat = true;
					self.update_pos(&mat);
					break;
				}
			}

			if !found_mat {
				break;
			}
		}

		for rule in self.lexer.rules() {
			if let Some(mat) = rule.pattern().find(&self.input.clone()) {
				let rule_name = rule.name().clone();
				self.update_pos(&mat);

				return Some(Ok(Token {
					name: rule_name,
					symbol: mat.as_str().to_owned(),
					loc: self.loc.to_owned()
				}));
			}
		}

		Some(Err((Error::InvalidToken, self.loc.end.to_owned())))
	}
}