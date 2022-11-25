use std::cmp::Ordering;

use crate::ASTNode;

pub type PatternFunc<N, E> = fn(&[N]) -> Result<N, E>;

#[derive(Clone)]
pub struct Pattern<N, E>
where
	N: ASTNode
{
	name: String,
	elems: Vec<String>,
	func: PatternFunc<N, E>
}

impl<N, E> Pattern<N, E>
where
	N: ASTNode
{
	pub fn new(name: &str, elems: &[&str], func: PatternFunc<N, E>) -> Self {
		Self {
			name: name.to_owned(),
			elems: elems.iter()
				.map(|x| x.to_string())
				.collect(),
			func
		}
	}

	pub fn name(&self) -> &String {
		&self.name
	}

	pub fn elems(&self) -> &Vec<String> {
		&self.elems
	}

	pub fn func(&self) -> PatternFunc<N, E> {
		self.func
	}
}

impl<N, E> Ord for Pattern<N, E>
where
	N: ASTNode
{
	fn cmp(&self, other: &Self) -> Ordering {
		let name_ord = self.name.cmp(other.name());

		match name_ord {
			Ordering::Equal => (),
			_ => return name_ord
		}

		let elems_len_ord = self.elems.len().cmp(&other.elems().len());

		match elems_len_ord {
			Ordering::Equal => (),
			_ => return elems_len_ord
		}

		for (self_elem, other_elem) in self.elems.iter().zip(other.elems()) {
			let elem_ord = self_elem.cmp(other_elem);

			match elem_ord {
				Ordering::Equal => (),
				_ => return elem_ord
			}
		}

		Ordering::Equal
	}
}

impl<N, E> PartialOrd for Pattern<N, E>
where
	N: ASTNode
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl<N, E> PartialEq for Pattern<N, E>
where
	N: ASTNode
{
	fn eq(&self, other: &Self) -> bool {
		let count_eq = self.elems.iter().zip(other.elems()).filter(|(a, b)| a == b).count();

		if self.name == *other.name() && self.elems.len() == count_eq && other.elems().len() == count_eq {
			return true;
		}

		false
	}
}

impl<N, E> Eq for Pattern<N, E>
where
	N: ASTNode
{
	
}