use crate::types::ast::Proposition;
use std::fmt;
use std::slice::Iter;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Literal {
	Proposition(Proposition),
	Not(Proposition),
}

impl Literal {
	pub fn to_proposition(&self) -> Proposition {
		match self {
			Literal::Proposition(p) | Literal::Not(p) => p.clone(),
		}
	}
}

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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Progam(pub Vec<Clause>);

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

impl Progam {
	pub fn append(&mut self, other: &mut Self) {
		self.0.append(&mut other.0);
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
