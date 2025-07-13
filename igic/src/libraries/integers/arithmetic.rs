use std::iter;

use crate::mgu::mgu::{Substitution, Unifiable};
use crate::mgu::substitution::{
	apply_substitution, apply_substitution_to_clause, compose_substitutions, empty_substitution,
};
use crate::types::ast::{Proposition, Term};
use crate::types::clause::Clause;

pub fn arithmetic_op_pred<F>(
	goal: &Clause,
	prop: &Proposition,
	sub: &Substitution,
	op: F,
) -> Option<Box<dyn Iterator<Item = (Clause, Substitution)>>>
where
	F: Fn(i64, i64) -> i64,
{
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

	let result = op(a, b);
	let expected_result_term = Unifiable::Term(Term::Number(result));

	match t3 {
		Unifiable::Term(Term::Number(n)) => {
			if n == result {
				let new_goal = Clause(goal.0[1..].to_vec());
				return Some(Box::new(iter::once((new_goal, sub.clone()))));
			}
		},
		Unifiable::Term(Term::Identifier(_)) => {
			let mut temp_sub = empty_substitution();
			temp_sub.insert(t3.clone(), expected_result_term);

			let mut new_sub = sub.clone();
			compose_substitutions(&temp_sub, &mut new_sub);

			let mut new_goal = Clause(goal.0[1..].to_vec());
			apply_substitution_to_clause(&temp_sub, &mut new_goal);
			return Some(Box::new(iter::once((new_goal, new_sub))));
		},
		_ => {},
	}
	None
}
