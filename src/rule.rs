use regex::Regex;

#[derive(Clone)]
pub struct Rule {
	name: String,
	pattern: Regex
}

impl Rule {
	pub fn new(name: &str, pattern: &str) -> Result<Self, String> {
		Ok(Self {
			name: name.to_owned(),
			pattern: match Regex::new(pattern) {
				Ok(x) => x,
				Err(_) => return Err("Invalid regex".to_owned())
			}
		})
	}

	pub fn name(&self) -> &String {
		&self.name
	}

	pub fn pattern(&self) -> &Regex {
		&self.pattern
	}
}