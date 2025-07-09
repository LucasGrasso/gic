use super::mgu::{Unifiable, UnificationEquation};
use crate::types::clause::Literal;
use crate::types::{
	ast::{Proposition, Term},
	clause::Clause,
};

use std::collections::HashMap;

pub type Substitution = HashMap<Unifiable, Unifiable>;

pub fn empty_substitution() -> Substitution {
	HashMap::new()
}

pub fn apply_substitution(sub: &Substitution, t: &Unifiable) -> Unifiable {
	match t {
		Unifiable::Term(Term::Identifier(_)) => sub.get(t).cloned().unwrap_or(t.clone()),
		Unifiable::Term(Term::FunctionApplication { name, args }) => {
			let new_args = args
				.iter()
				.map(|a| match apply_substitution(sub, &Unifiable::Term(a.clone())) {
					Unifiable::Term(t) => t,
					_ => panic!("Invalid substitution result"),
				})
				.collect();
			Unifiable::Term(Term::FunctionApplication { name: name.clone(), args: new_args })
		},
		Unifiable::Prop(p) => {
			let new_terms = p
				.terms
				.iter()
				.map(|t| match apply_substitution(sub, &Unifiable::Term(t.clone())) {
					Unifiable::Term(nt) => nt,
					_ => panic!("Invalid substitution result in proposition"),
				})
				.collect();
			Unifiable::Prop(Proposition { name: p.name.clone(), terms: new_terms })
		},
	}
}

pub fn apply_substitution_to_clause(sub: &Substitution, clause: &mut Clause) {
	for literal in clause.0.iter_mut() {
		match literal {
			Literal::Proposition(prop) => {
				let new_terms = prop
					.terms
					.iter()
					.map(|t| match apply_substitution(sub, &Unifiable::Term(t.clone())) {
						Unifiable::Term(nt) => nt,
						_ => panic!("Invalid substitution result in proposition"),
					})
					.collect();
				*literal =
					Literal::Proposition(Proposition { name: prop.name.clone(), terms: new_terms });
			},
			Literal::Not(prop) => {
				let new_terms = prop
					.terms
					.iter()
					.map(|t| match apply_substitution(sub, &Unifiable::Term(t.clone())) {
						Unifiable::Term(nt) => nt,
						_ => panic!("Invalid substitution result in proposition"),
					})
					.collect();
				*literal = Literal::Not(Proposition { name: prop.name.clone(), terms: new_terms });
			},
		}
	}
}

pub fn apply_substitution_to_equation(sub: &Substitution, eq: &mut UnificationEquation) {
	for (t1, t2) in eq.iter_mut() {
		*t1 = apply_substitution(sub, t1);
		*t2 = apply_substitution(sub, t2);
	}
}

pub fn apply_substitution_to_sub(s1: &Substitution, s2: &mut Substitution) {
	let keys: Vec<_> = s2.keys().cloned().collect(); // clone keys to avoid borrow issue
	for key in keys {
		if let Some(val) = s2.get(&key) {
			let new_val = apply_substitution(s1, val);
			s2.insert(key, new_val);
		}
	}
	s2.extend(s1.clone());
}

/* pub fn print_substitution(sub: &Substitution) {
	println!("{{ ");
	for (k, v) in sub.iter() {
		println!("{} -> {}", k, v);
	}
	println!("}}");
}
 */
