use regex::Match;

use crate::{Rule, Token, Position, Error, Location};

#[derive(Clone)]
pub struct Lexer {
	rules: Vec<Rule>,
	ignore_rules: Vec<Rule>,
}

impl Lexer {
	pub fn new(rules: Vec<Rule>, ignore_rules: Vec<Rule>) -> Self {
		Self {
			rules,
			ignore_rules,
		}
	}

	pub fn rules(&self) -> &Vec<Rule> {
		&self.rules
	}

	pub fn ignore_rules(&self) -> &Vec<Rule> {
		&self.ignore_rules
	}

	pub fn lex(&self, input: &str) -> LexerStream {
		LexerStream::new(self, input)
	}
}

pub struct LexerStream {
	lexer: Lexer,
	input: String,
	location: Location
}

impl LexerStream {
	pub fn new(lexer: &Lexer, input: &str) -> Self {
		Self {
			lexer: lexer.clone(),
			input: input.to_owned(),
			location: Location::default()
		}
	}

	pub fn update_pos(&mut self, mat: &Match) {
		self.location.end.idx += mat.end();
		self.location.end.line += mat.as_str().matches('\n').count();
		self.location.end.col += match self.input[..mat.start()].rfind('\n') {
			Some(last_nl) => self.location.end.idx - last_nl,
			None => mat.end()
		};
		self.input = self.input[mat.end()..].to_owned();
	}
}

impl Iterator for LexerStream {
	type Item = Result<Token, (Error, Position)>;

	fn next(&mut self) -> Option<Self::Item> {
		self.location.start = self.location.end;

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
					location: self.location
				}));
			}
		}

		Some(Err((Error::InvalidToken, self.location.end)))
	}
}