use crate::{Rule, Lexer};

pub struct LexerGen<'a> {
	rules: Vec<Rule<'a>>,
	ignore_rules: Vec<Rule<'a>>
}

impl<'a> LexerGen<'a> {
	pub fn new() -> Self {
		Self {
			rules: Vec::new(),
			ignore_rules: Vec::new()
		}
	}

	pub fn add(&mut self, name: &'a str, pattern: &'a str) -> Result<(), ()> {
		self.rules.push(Rule::new(name, pattern)?);
		Ok(())
	}

	pub fn add_vec(&mut self, rules: &[(&'a str, &'a str)]) -> Result<(), String> {
		for (name, pattern) in rules {
			if self.add(name, pattern).is_err() {
				return Err(format!("Invalid regex '{}'", pattern))
			}
		}

		Ok(())
	}

	pub fn ignore(&mut self, pattern: &'a str) -> Result<(), ()> {
		self.ignore_rules.push(Rule::new("", pattern)?);
		Ok(())
	}

	pub fn ignore_vec(&mut self, rules: &[&'a str]) -> Result<(), String> {
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