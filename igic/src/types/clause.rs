use super::ast::Expression;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Clause(pub Vec<Vec<Expression>>);

impl IntoIterator for Clause {
	type Item = Vec<Expression>;
	type IntoIter = std::vec::IntoIter<Vec<Expression>>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

impl<'a> IntoIterator for &'a Clause {
	type Item = &'a Vec<Expression>;
	type IntoIter = std::slice::Iter<'a, Vec<Expression>>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.iter()
	}
}

impl Clause {
	pub fn new() -> Self {
		Clause(vec![])
	}

	pub fn add_clause(&mut self, clause: Vec<Expression>) {
		self.0.push(clause);
	}

	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn append(&mut self, other: &mut Self) {
		self.0.append(&mut other.0);
	}

	pub fn iter(&self) -> impl Iterator<Item = &Vec<Expression>> {
		self.0.iter()
	}
}

impl fmt::Display for Clause {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{{")?;
		for (i, clause) in self.0.iter().enumerate() {
			if i > 0 {
				write!(f, ", ")?;
			}
			write!(
				f,
				"{{{}}}",
				clause.iter().map(|e| format!("{}", e)).collect::<Vec<_>>().join(" ∨ ")
			)?;
		}
		write!(f, "}}")
	}
}
