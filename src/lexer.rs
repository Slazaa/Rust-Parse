use std::fs;

use regex::Match;

use crate::{Rule, Token, Position, Error, Location};

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

	pub fn lex(&self, input: &str) -> LexerStream {
		LexerStream::new(self, input, None)
	}

	pub fn lex_from_file(&self, filename: &str) -> Result<LexerStream, Error> {
		let input = match fs::read_to_string(filename) {
			Ok(x) => x,
			Err(_) => return Err(Error::FileNotFound)
		};

		Ok(LexerStream::new(self, &input, Some(filename.to_owned())))
	}
}

pub struct LexerStream {
	lexer: Lexer,
	input: String,
	loc: Location
}

impl LexerStream {
	pub fn new(lexer: &Lexer, input: &str, filename: Option<String>) -> Self {
		Self {
			lexer: lexer.clone(),
			input: input.to_owned(),
			loc: Location {
				filename,
				..Default::default()
			}
		}
	}

	pub fn update_pos(&mut self, mat: &Match) {
		self.loc.end.idx += mat.end();
		self.loc.end.line += mat.as_str().matches('\n').count();
		self.loc.end.col += match self.input[..mat.start()].rfind('\n') {
			Some(last_nl) => self.loc.end.idx - last_nl,
			None => mat.end()
		};
		self.input = self.input[mat.end()..].to_owned();
	}
}

impl Iterator for LexerStream {
	type Item = Result<Token, (Error, Position)>;

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

				println!("{:#?}", self.loc);
				
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