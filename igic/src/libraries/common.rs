use crate::mgu::mgu::{Substitution, Unifiable};
use crate::mgu::substitution::apply_substitution;
use crate::types::ast::{Proposition, Term};
use crate::types::clause::Clause;

use std::iter;

pub fn eq_pred(
	sub: &Substitution,
	p: &Proposition,
	goal: &Clause,
) -> Option<Box<dyn Iterator<Item = (Clause, Substitution)>>> {
	let a = apply_substitution(sub, &Unifiable::Term(p.terms[0].clone()));
	let b = apply_substitution(sub, &Unifiable::Term(p.terms[1].clone()));
	if a == b {
		let mut rem = goal.0.clone();
		rem.remove(0);
		Some(Box::new(iter::once((Clause(rem), sub.clone()))))
	} else {
		None
	}
}

pub fn diff_pred(
	sub: &Substitution,
	p: &Proposition,
	goal: &Clause,
) -> Option<Box<dyn Iterator<Item = (Clause, Substitution)>>> {
	let a = apply_substitution(sub, &Unifiable::Term(p.terms[0].clone()));
	let b = apply_substitution(sub, &Unifiable::Term(p.terms[1].clone()));
	if a != b {
		let mut rem = goal.0.clone();
		rem.remove(0);
		Some(Box::new(iter::once((Clause(rem), sub.clone()))))
	} else {
		None
	}
}

pub fn var_pred(
	sub: &Substitution,
	p: &Proposition,
	goal: &Clause,
) -> Option<Box<dyn Iterator<Item = (Clause, Substitution)>>> {
	match apply_substitution(sub, &Unifiable::Term(p.terms[0].clone())) {
		Unifiable::Term(Term::Identifier(_)) => {
			let mut rem = goal.0.clone();
			rem.remove(0);
			Some(Box::new(iter::once((Clause(rem), sub.clone()))))
		},
		_ => None,
	}
}
