use crate::{Rule, Lexer};

pub struct LexerBuilder {
	rules: Vec<Rule>,
	ignore_rules: Vec<Rule>
}

impl LexerBuilder {
	pub fn new() -> Self {
		Self {
			rules: Vec::new(),
			ignore_rules: Vec::new()
		}
	}

	pub fn add_rule(&mut self, name: &str, pattern: &str) -> Result<(), ()> {
		self.rules.push(Rule::new(name, pattern)?);
		Ok(())
	}

	pub fn add_rules(&mut self, rules: &[(&str, &str)]) -> Result<(), String> {
		for (name, pattern) in rules {
			if self.add_rule(name, pattern).is_err() {
				return Err(format!("Invalid regex '{}'", pattern))
			}
		}

		Ok(())
	}

	pub fn ignore_rule(&mut self, pattern: &str) -> Result<(), ()> {
		self.ignore_rules.push(Rule::new("", pattern)?);
		Ok(())
	}

	pub fn ignore_rules(&mut self, rules: &[&str]) -> Result<(), String> {
		for pattern in rules {
			if self.ignore_rule(pattern).is_err() {
				return Err(format!("Invalid regex '{}'", pattern))
			}
		}

		Ok(())
	}

	pub fn build(&self) -> Lexer {
		Lexer::new(self.rules.clone(), self.ignore_rules.clone())
	}
}