use crate::types::ast::Proposition;
use std::fmt;
use std::slice::Iter;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Literal {
	Proposition(Proposition),
	Not(Proposition),
}

impl Literal {}

impl fmt::Display for Literal {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Literal::Proposition(prop) => {
				// Asumiendo que Proposition tiene name y terms
				write!(
					f,
					"{}({})",
					prop.name,
					prop.terms.iter().map(|t| format!("{}", t)).collect::<Vec<String>>().join(", ")
				)
			},
			Literal::Not(prop) => {
				write!(
					f,
					"¬{}({})",
					prop.name,
					prop.terms.iter().map(|t| format!("{}", t)).collect::<Vec<String>>().join(", ")
				)
			},
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause(pub Vec<Literal>);

impl Clause {
	pub fn new() -> Self {
		Clause(vec![])
	}

	pub fn from_literals(lits: Vec<Literal>) -> Self {
		Clause(lits)
	}

	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	pub fn is_goal(&self) -> bool {
		!(self.0.iter().any(|lit| matches!(lit, Literal::Proposition(_))))
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn append(&mut self, other: &mut Clause) {
		self.0.append(&mut other.0);
	}

	pub fn iter(&self) -> impl Iterator<Item = &Literal> {
		self.0.iter()
	}
}

impl IntoIterator for Clause {
	type Item = Literal;
	type IntoIter = std::vec::IntoIter<Literal>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

impl<'a> IntoIterator for &'a Clause {
	type Item = &'a Literal;
	type IntoIter = Iter<'a, Literal>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.iter()
	}
}

impl fmt::Display for Clause {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{{")?;
		// Join literals with commas, no `∨` operator inside clause
		write!(
			f,
			"{}",
			self.0.iter().map(|lit| format!("{}", lit)).collect::<Vec<String>>().join(", ")
		)?;
		write!(f, "}}")
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Progam(pub Vec<Clause>);

impl Progam {
	pub fn get_clause(&self, index: usize) -> Option<&Clause> {
		self.0.get(index)
	}

	pub fn append(&mut self, other: &mut Self) {
		self.0.append(&mut other.0);
	}

	pub fn is_horn(&self) -> bool {
		self.0.iter().all(|clause| {
			let positive_count =
				clause.iter().filter(|lit| matches!(lit, Literal::Proposition(_))).count();
			positive_count <= 1 // At most one positive literal
		})
	}
}

impl IntoIterator for Progam {
	type Item = Clause;
	type IntoIter = std::vec::IntoIter<Clause>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

impl<'a> IntoIterator for &'a Progam {
	type Item = &'a Clause;
	type IntoIter = std::slice::Iter<'a, Clause>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.iter()
	}
}

impl fmt::Display for Progam {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "{{")?;
		for clause in &self.0 {
			writeln!(f, "  {},", clause)?; // Each clause in braces + comma
		}
		writeln!(f, "}}")
	}
}
