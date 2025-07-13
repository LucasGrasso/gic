use crate::mgu::mgu::{Substitution, Unifiable};
use crate::mgu::substitution::{
	apply_substitution, apply_substitution_to_clause, compose_substitutions, empty_substitution,
};
use crate::types::ast::{Proposition, Term};
use crate::types::clause::Clause;
use std::iter;

pub fn compare_pred<F>(
	goal: &Clause,
	prop: &Proposition,
	sub: &Substitution,
	cmp: F,
) -> Option<Box<dyn Iterator<Item = (Clause, Substitution)>>>
where
	F: Fn(i64, i64) -> bool + 'static,
{
	let t1 = apply_substitution(sub, &Unifiable::Term(prop.terms[0].clone()));
	let t2 = apply_substitution(sub, &Unifiable::Term(prop.terms[1].clone()));

	let a = match t1 {
		Unifiable::Term(Term::Number(n)) => n,
		_ => return None,
	};
	let b = match t2 {
		Unifiable::Term(Term::Number(n)) => n,
		_ => return None,
	};

	if cmp(a, b) {
		let mut rem = goal.0.clone();
		rem.remove(0);
		Some(Box::new(iter::once((Clause(rem), sub.clone()))))
	} else {
		None
	}
}

pub fn between_pred<'a>(
	goal: &'a Clause,
	prop: &'a Proposition,
	sub: &'a Substitution,
) -> Option<Box<dyn Iterator<Item = (Clause, Substitution)> + 'a>> {
	let t1 = apply_substitution(sub, &Unifiable::Term(prop.terms[0].clone()));
	let t2 = apply_substitution(sub, &Unifiable::Term(prop.terms[1].clone()));
	let t3 = apply_substitution(sub, &Unifiable::Term(prop.terms[2].clone()));

	let a = match t1 {
		Unifiable::Term(Term::Number(n)) => n,
		_ => return None,
	};
	let b = match t2 {
		Unifiable::Term(Term::Number(n)) => n,
		_ => return None,
	};

	match t3 {
		Unifiable::Term(Term::Number(n)) => {
			if a <= n && n <= b {
				let new_goal = Clause(goal.0[1..].to_vec());
				return Some(Box::new(std::iter::once((new_goal, sub.clone()))));
			}
			None
		},
		Unifiable::Term(Term::Identifier(id)) => {
			let iter = (a..=b).map(move |i| {
				let mut temp_sub = empty_substitution();
				temp_sub.insert(
					Unifiable::Term(Term::Identifier(id.clone())),
					Unifiable::Term(Term::Number(i)),
				);

				let mut new_sub = sub.clone();
				compose_substitutions(&temp_sub, &mut new_sub);

				let mut new_goal = Clause(goal.0[1..].to_vec());
				apply_substitution_to_clause(&temp_sub, &mut new_goal);

				(new_goal, new_sub)
			});
			Some(Box::new(iter))
		},
		_ => None,
	}
}
