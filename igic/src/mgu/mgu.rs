use super::substitution::{
	apply_substitution, apply_substitution_to_equation, apply_substitution_to_sub,
	empty_substitution,
};
use crate::types::ast::{Proposition, Term};
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Unifiable {
	Term(Term),
	Prop(Proposition),
}

impl fmt::Display for Unifiable {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Unifiable::Term(term) => write!(f, "{}", term),
			Unifiable::Prop(prop) => write!(f, "{}", prop),
		}
	}
}

pub type UnifiablePair = (Unifiable, Unifiable);

pub type UnificationEquation = Vec<UnifiablePair>;

pub type Substitution = HashMap<Unifiable, Unifiable>;

#[derive(Debug)]
pub enum MguError {
	Clash(String),
	UnificationError(String),
	OccurCheck(String),
}

pub type Result<T> = std::result::Result<T, MguError>;

fn is_var(term: &Unifiable) -> bool {
	match term {
		Unifiable::Term(Term::Identifier(_)) => true,
		_ => false,
	}
}

pub fn mgu(mut equations: UnificationEquation) -> Result<Substitution> {
	let mut sub = empty_substitution();

	while let Some(pair) = equations.pop() {
		match decompose(&pair) {
			Ok(decomposed) => {
				equations.extend(decomposed);
			},
			Err(_e) => match delete(&pair) {
				Ok(new_sub) => {
					apply_substitution_to_sub(&new_sub, &mut sub);
					apply_substitution_to_equation(&sub, &mut equations);
				},
				Err(err) => {
					return Err(err);
				},
			},
		}
	}

	Ok(sub)
}

fn decompose(pair: &UnifiablePair) -> Result<Vec<UnifiablePair>> {
	match pair {
		(
			Unifiable::Term(Term::FunctionApplication { name: n1, args: a1 }),
			Unifiable::Term(Term::FunctionApplication { name: n2, args: a2 }),
		) if n1 == n2 && a1.len() == a2.len() => Ok(a1
			.into_iter()
			.zip(a2.into_iter())
			.map(|(l, r)| (Unifiable::Term(l.clone()), Unifiable::Term(r.clone())))
			.collect()),
		(Unifiable::Prop(p1), Unifiable::Prop(p2))
			if p1.name == p2.name && p1.terms.len() == p2.terms.len() =>
		{
			Ok(p1
				.clone()
				.terms
				.into_iter()
				.zip(p2.clone().terms.into_iter())
				.map(|(l, r)| (Unifiable::Term(l), Unifiable::Term(r)))
				.collect())
		},
		_ => Err(MguError::Clash(format!("Cannot decompose: {:?} and {:?}", pair.0, pair.1))),
	}
}

fn delete(pair: &UnifiablePair) -> Result<Substitution> {
	match pair {
		(Unifiable::Term(Term::Identifier(ref id)), Unifiable::Term(Term::Identifier(ref id1))) => {
			if id == id1 {
				return Ok(empty_substitution());
			}
			let mut sub = empty_substitution();
			sub.insert(
				Unifiable::Term(Term::Identifier(id.clone())),
				Unifiable::Term(Term::Identifier(id1.clone())),
			);
			Ok(sub)
		},
		(
			Unifiable::Term(Term::Identifier(ref id)),
			Unifiable::Term(Term::FunctionApplication { name, args }),
		) => {
			let var = Term::Identifier(id.clone());
			let term = Term::FunctionApplication { name: name.to_string(), args: args.to_vec() };
			if occurs_check(&var, &term) {
				return Err(MguError::OccurCheck(format!("{} occurs in {}", var, term)));
			}
			let mut sub = empty_substitution();
			sub.insert(Unifiable::Term(var), Unifiable::Term(term));
			return Ok(sub);
		},
		(
			Unifiable::Term(Term::FunctionApplication { name, args }),
			Unifiable::Term(Term::Identifier(ref id)),
		) => {
			// swap case
			let var = Term::Identifier(id.clone());
			let term = Term::FunctionApplication { name: name.to_string(), args: args.to_vec() };
			if occurs_check(&var, &term) {
				return Err(MguError::OccurCheck(format!("{} occurs in {}", var, term)));
			}
			let mut sub = empty_substitution();
			sub.insert(Unifiable::Term(var), Unifiable::Term(term));
			return Ok(sub);
		},
		_ => Err(MguError::UnificationError(format!(
			"Cannot delete terms: {:?} and {:?}",
			pair.0, pair.1
		))),
	}
}

fn occurs_check(var: &Term, term: &Term) -> bool {
	match term {
		Term::Identifier(_) => term == var,
		Term::FunctionApplication { args, .. } => args.iter().any(|arg| occurs_check(var, arg)),
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_mgu_decompose() {
		let clause1 = Unifiable::Prop(Proposition {
			name: "P".to_string(),
			terms: vec![Term::Identifier("X".to_string())],
		});
		let clause1_clone = clause1.clone();
		let clause2 = Unifiable::Prop(Proposition {
			name: "P".to_string(),
			terms: vec![Term::FunctionApplication { name: "0".to_string(), args: vec![] }],
		});
		let clause2_clone = clause2.clone();
		let equations = vec![(clause1, clause2)];
		let result = mgu(equations);
		let clause1_sub = apply_substitution(&result.as_ref().unwrap(), &clause1_clone);
		assert_eq!(clause1_sub, clause2_clone);
	}

	#[test]
	fn test_mgu_err1() {
		let clause1 = Unifiable::Term(Term::Identifier("X".to_string()));
		let clause2 = Unifiable::Prop(Proposition {
			name: "P".to_string(),
			terms: vec![Term::FunctionApplication { name: "0".to_string(), args: vec![] }],
		});
		let equations = vec![(clause1, clause2)];
		let result = mgu(equations);
		assert!(result.is_err());
	}

	#[test]
	fn test_mgu_nested_decomp_delete() {
		let clause1 = Unifiable::Term(Term::FunctionApplication {
			name: "f".to_string(),
			args: vec![Term::Identifier("X".to_string())],
		});
		let clause1_clone = clause1.clone();
		let clause2 = Unifiable::Term(Term::FunctionApplication {
			name: "f".to_string(),
			args: vec![Term::FunctionApplication {
				name: "g".to_string(),
				args: vec![Term::Identifier("Y".to_string())],
			}],
		});
		let equations = vec![(clause1, clause2)];
		let result = mgu(equations);
		// check that x is substituted with g(Y)
		let clause1_sub = apply_substitution(&result.as_ref().unwrap(), &clause1_clone);
		let expected = Unifiable::Term(Term::FunctionApplication {
			name: "f".to_string(),
			args: vec![Term::FunctionApplication {
				name: "g".to_string(),
				args: vec![Term::Identifier("Y".to_string())],
			}],
		});
		assert_eq!(clause1_sub, expected);
	}

	#[test]
	fn test_mgu_swap() {
		let clause1 = Unifiable::Term(Term::Identifier("X".to_string()));
		let clause2 = Unifiable::Term(Term::FunctionApplication {
			name: "f".to_string(),
			args: vec![Term::Identifier("Y".to_string())],
		});
		let equations = vec![(clause2.clone(), clause1.clone())];
		let result = mgu(equations);
		assert!(result.is_ok());
		let sub = result.unwrap();
		let clause1_sub = apply_substitution(&sub, &clause1);
		assert_eq!(clause1_sub, clause2);
	}

	#[test]
	fn test_trivial_mgu() {
		let clause1 = Unifiable::Term(Term::Identifier("X".to_string()));
		let clause2 = Unifiable::Term(Term::Identifier("X".to_string()));
		let clause3 =
			Unifiable::Term(Term::FunctionApplication { name: "a".to_string(), args: vec![] });
		let clause4 =
			Unifiable::Term(Term::FunctionApplication { name: "a".to_string(), args: vec![] });

		let equations = vec![(clause1, clause2), (clause3, clause4)];

		let result = mgu(equations);

		assert!(result.is_ok());
		let sub = result.unwrap();
		assert!(sub.is_empty(), "Expected empty substitution for trivial MGU");
	}
}
