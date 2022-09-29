use regex::Regex;

#[derive(Clone)]
pub struct Rule<'a> {
	name: &'a str,
	pattern: Regex
}

impl<'a> Rule<'a> {
	pub fn new(name: &'a str, pattern: &str) -> Result<Self, ()> {
		Ok(Self {
			name,
			pattern: match Regex::new(pattern) {
				Ok(x) => x,
				Err(_) => return Err(())
			}
		})
	}

	pub fn name(&self) -> &'a str {
		self.name
	}

	pub fn pattern(&self) -> &Regex {
		&self.pattern
	}
}