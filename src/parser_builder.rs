use crate::Parser;

pub struct ParserBuilder {
	token_names: Vec<String>,
	patterns: Vec<String>
}

impl ParserBuilder {
	pub fn new(token_names: &[&str]) -> Self {
		Self {
			token_names: token_names.iter()
				.map(|x| x.to_string())
				.collect(),
			patterns: Vec::new()
		}
	}

	pub fn add_pattern(&mut self, pattern: &str) {
		self.patterns.push(pattern.to_owned());
	}

	pub fn add_patterns(&mut self, patterns: &[&str]) {
		for pattern in patterns {
			self.add_pattern(pattern);
		}
	}

	pub fn build(&self) -> Parser {
		todo!();
	}
}