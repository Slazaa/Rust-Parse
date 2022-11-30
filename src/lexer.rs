use std::fs;

use regex::Match;

use crate::{Rule, Token, Position, Loc};

pub enum LexerError {
	FileNotFound(String),
	InvalidToken(Position)
}

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

	pub fn lex<E>(&self, input: &str) -> Result<Vec<Token>, LexerError> {
		let mut res = Vec::new();
		let lexer_stream = LexerStream::new(self, input, None);

		for token in lexer_stream {
			res.push(token?);
		}

		Ok(res)
	}

	pub fn lex_from_file<E>(&self, filename: &str) -> Result<Vec<Token>, LexerError> {
		let input = match fs::read_to_string(filename) {
			Ok(x) => x,
			Err(_) => return Err(LexerError::FileNotFound(filename.to_owned()))
		};

		let mut res = Vec::new();
		let lexer_stream = LexerStream::new(self, &input, Some(filename.to_owned()));

		for token in lexer_stream {
			res.push(token?);
		}

		Ok(res)
	}
}

pub struct LexerStream {
	lexer: Lexer,
	input: String,
	loc: Loc
}

impl LexerStream {
	pub fn new(lexer: &Lexer, input: &str, filename: Option<String>) -> Self {
		Self {
			lexer: lexer.clone(),
			input: input.to_owned(),
			loc: Loc {
				filename,
				..Default::default()
			}
		}
	}

	pub fn update_pos(&mut self, mat: &Match) {
		self.loc.end.idx += mat.end();
		self.loc.end.line += mat.as_str().matches('\n').count();

		let tabs_col = mat.as_str().matches('\t').count() * 4;

		self.loc.end.col = match mat.as_str().rfind('\n') {
			Some(last_nl) => mat.end() - last_nl,
			None => self.loc.end.col + mat.end()
		} + tabs_col;
		
		self.input = self.input[mat.end()..].to_owned();
	}
}

impl Iterator for LexerStream {
	type Item = Result<Token, LexerError>;

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

		Some(Err(LexerError::InvalidToken(self.loc.end.to_owned())))
	}
}