use std::fmt::Debug;

use crate::{Parser, Pattern, ASTNode, PatternFunc};

pub struct ParserBuilder<N, E>
where
	N: ASTNode + Clone + Debug,
	E: Clone
{
	token_names: Vec<String>,
	patterns: Vec<Pattern<N, E>>
}

impl<N, E> ParserBuilder<N, E>
where
	N: ASTNode + Clone + Debug,
	E: Clone
{
	pub fn new(token_names: &[&str]) -> Self {
		Self {
			token_names: token_names.iter()
				.map(|x| x.to_string())
				.collect(),
			patterns: Vec::new()
		}
	}

	pub fn add_pattern(&mut self, name: &str, pattern: &str, func: PatternFunc<N, E>) -> Result<(), String> {
		if self.token_names.contains(&name.to_owned()) {
			return Err("Pattern name already a token".to_owned())
		}

		self.patterns.push(Pattern::new(name, &pattern.split_whitespace().collect::<Vec<&str>>(), func));

		Ok(())
	}

	pub fn add_patterns(&mut self, patterns: &[(&str, &str, PatternFunc<N, E>)]) -> Result<(), String> {
		for (name, pattern, func) in patterns {
			self.add_pattern(name, pattern, *func)?;
		}

		Ok(())
	}

	pub fn build(&self) -> Parser<N, E> {
		Parser::new(&self.token_names, &self.patterns)
	}
}
