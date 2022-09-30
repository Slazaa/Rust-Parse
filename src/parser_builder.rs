use crate::Parser;

pub struct ParserBuilder {
	token_names: Vec<String>
}

impl ParserBuilder {
	pub fn new(token_names: &[&str]) -> Self {
		Self {
			token_names: token_names.iter()
				.map(|x| x.to_string())
				.collect()
		}
	}

	pub fn build(&self) -> Parser {
		todo!();
	}
}