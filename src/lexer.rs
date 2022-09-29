use regex::Match;

use crate::{Rule, Token, Position};

pub struct Lexer<'a> {
	rules: Vec<Rule<'a>>,
	ignore_rules: Vec<Rule<'a>>,
}

impl<'a> Lexer<'a> {
	pub fn new(rules: Vec<Rule<'a>>, ignore_rules: Vec<Rule<'a>>) -> Self {
		Self {
			rules,
			ignore_rules,
		}
	}

	pub fn rules(&self) -> &Vec<Rule<'a>> {
		&self.rules
	}

	pub fn ignore_rules(&self) -> &Vec<Rule<'a>> {
		&self.ignore_rules
	}

	pub fn lex(&self, input: &'a str) -> LexerStream {
		LexerStream::new(&self, input)
	}
}

pub struct LexerStream<'a> {
	lexer: &'a Lexer<'a>,
	input: &'a str,
	pos: Position,
	start_pos: Position
}

impl<'a> LexerStream<'a> {
	pub fn new(lexer: &'a Lexer<'a>, input: &'a str) -> Self {
		Self {
			lexer,
			input,
			pos: Position::new(0, 1, 1),
			start_pos: Position::new(0, 1, 1)
		}
	}

	pub fn update_pos(&mut self, mat: &Match) {
		*self.pos.idx_mut() += mat.end();
		*self.pos.line_mut() += self.input.matches("\n").count();
		*self.pos.col_mut() += match self.input.rfind("\n") {
			Some(last_nl) => mat.start() - last_nl,
			None => mat.end()
		};
		self.input = &self.input[mat.end()..];
	}
}

impl<'a> Iterator for LexerStream<'a> {
	type Item = Result<Token<'a>, (String, Position)>;

	fn next(&mut self) -> Option<Self::Item> {
		self.start_pos = self.pos.clone();

		loop {
			if self.input.is_empty() {
				return None;
			}

			let mut found_mat = false;

			for rule in self.lexer.ignore_rules() {
				if let Some(mat) = rule.pattern().find(self.input) {
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
			if let Some(mat) = rule.pattern().find(self.input) {
				self.update_pos(&mat);
				return Some(Ok(Token::new(rule.name(), mat.as_str(), &self.start_pos, &self.pos)));
			}
		}

		Some(Err(("Invalid syntax".to_owned(), self.pos)))
	}
}