use crate::mgu::mgu::{Substitution, Unifiable};
use crate::mgu::substitution::apply_substitution;
use crate::types::ast::{Proposition, Term};
use crate::types::clause::Clause;

pub fn eq_pred(
	sub: &Substitution,
	p: &Proposition,
	goal: &Clause,
) -> Option<Vec<(Clause, Substitution)>> {
	let a = apply_substitution(sub, &Unifiable::Term(p.terms[0].clone()));
	let b = apply_substitution(sub, &Unifiable::Term(p.terms[1].clone()));
	if a == b {
		let mut rem = goal.0.clone();
		rem.remove(0);
		return Some(vec![(Clause(rem), sub.clone())]);
	}
	None
}

pub fn diff_pred(
	sub: &Substitution,
	p: &Proposition,
	goal: &Clause,
) -> Option<Vec<(Clause, Substitution)>> {
	let a = apply_substitution(sub, &Unifiable::Term(p.terms[0].clone()));
	let b = apply_substitution(sub, &Unifiable::Term(p.terms[1].clone()));
	if a != b {
		let mut rem = goal.0.clone();
		rem.remove(0);
		return Some(vec![(Clause(rem), sub.clone())]);
	}
	None
}

pub fn var_pred(
	sub: &Substitution,
	p: &Proposition,
	goal: &Clause,
) -> Option<Vec<(Clause, Substitution)>> {
	if let Unifiable::Term(Term::Identifier(_)) =
		apply_substitution(sub, &Unifiable::Term(p.terms[0].clone()))
	{
		let mut rem = goal.0.clone();
		rem.remove(0);
		return Some(vec![(Clause(rem), sub.clone())]);
	}
	None
}
