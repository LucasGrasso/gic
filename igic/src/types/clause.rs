use crate::types::ast::{Proposition, Term};
use std::fmt;
use std::slice::Iter;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Literal {
	Proposition(Proposition),
	Not(Proposition),
}

impl Literal {
	pub fn is_positive(&self) -> bool {
		matches!(self, Literal::Proposition(_))
	}

	pub fn is_negative(&self) -> bool {
		!self.is_positive()
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

	pub fn positives(&self) -> Vec<&Literal> {
		self.0.iter().filter(|lit| matches!(lit, Literal::Proposition(_))).collect()
	}

	pub fn negatives(&self) -> Vec<&Literal> {
		self.0.iter().filter(|lit| matches!(lit, Literal::Not(_))).collect()
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

	pub fn fv(&self) -> Vec<String> {
		// Returns all free variables in the clause
		let mut free_vars = Vec::new();
		for lit in &self.0 {
			match lit {
				Literal::Proposition(prop) => {
					for term in &prop.terms {
						if let Term::Identifier(ref id) = term {
							if !free_vars.contains(id) {
								free_vars.push(id.clone());
							}
						}
					}
				},
				Literal::Not(prop) => {
					for term in &prop.terms {
						if let Term::Identifier(ref id) = term {
							if !free_vars.contains(id) {
								free_vars.push(id.clone());
							}
						}
					}
				},
			}
		}
		free_vars
	}

	pub fn suffix_vars(&self, suffix: &str) -> Clause {
		let new_lits = self
			.0
			.iter()
			.map(|lit| match lit {
				Literal::Proposition(prop) => {
					let new_terms =
						prop.terms.iter().map(|t| t.append_suffix_to_vars(suffix)).collect();
					Literal::Proposition(Proposition { name: prop.name.clone(), terms: new_terms })
				},
				Literal::Not(prop) => {
					let new_terms =
						prop.terms.iter().map(|t| t.append_suffix_to_vars(suffix)).collect();
					Literal::Not(Proposition { name: prop.name.clone(), terms: new_terms })
				},
			})
			.collect();

		Clause(new_lits)
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
pub struct Program(pub Vec<Clause>);

impl Program {
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

	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl IntoIterator for Program {
	type Item = Clause;
	type IntoIter = std::vec::IntoIter<Clause>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

impl<'a> IntoIterator for &'a Program {
	type Item = &'a Clause;
	type IntoIter = std::slice::Iter<'a, Clause>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.iter()
	}
}

impl fmt::Display for Program {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "{{")?;
		for clause in &self.0 {
			writeln!(f, "  {},", clause)?; // Each clause in braces + comma
		}
		writeln!(f, "}}")
	}
}
