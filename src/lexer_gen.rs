use crate::{Rule, Lexer};

pub struct LexerGen {
	rules: Vec<Rule>,
	ignore_rules: Vec<Rule>
}

impl LexerGen {
	pub fn new() -> Self {
		Self {
			rules: Vec::new(),
			ignore_rules: Vec::new()
		}
	}

	pub fn add(&mut self, name: &str, pattern: &str) -> Result<(), ()> {
		self.rules.push(Rule::new(name, pattern)?);
		Ok(())
	}

	pub fn add_vec(&mut self, rules: &[(&str, &str)]) -> Result<(), String> {
		for (name, pattern) in rules {
			if self.add(name, pattern).is_err() {
				return Err(format!("Invalid regex '{}'", pattern))
			}
		}

		Ok(())
	}

	pub fn ignore(&mut self, pattern: &str) -> Result<(), ()> {
		self.ignore_rules.push(Rule::new("", pattern)?);
		Ok(())
	}

	pub fn ignore_vec(&mut self, rules: &[&str]) -> Result<(), String> {
		for pattern in rules {
			if self.ignore(pattern).is_err() {
				return Err(format!("Invalid regex '{}'", pattern))
			}
		}

		Ok(())
	}

	pub fn build(&self) -> Lexer {
		Lexer::new(self.rules.clone(), self.ignore_rules.clone())
	}
}