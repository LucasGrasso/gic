use crate::mgu::mgu::{Substitution, Unifiable};
use crate::mgu::substitution::{
	apply_substitution, apply_substitution_to_clause, apply_substitution_to_sub, empty_substitution,
};
use crate::types::ast::{Proposition, Term};
use crate::types::clause::{Clause, Literal};

pub fn is_list(
	sub: &Substitution,
	p: &Proposition,
	goal: &Clause,
) -> Option<Vec<(Clause, Substitution)>> {
	if let Unifiable::Term(Term::FunctionApplication { name, args }) =
		apply_substitution(sub, &Unifiable::Term(p.terms[0].clone()))
	{
		if (name == "cons" && args.len() == 2) || (name == "empty_list" && args.is_empty()) {
			let mut rem = goal.0.clone();
			rem.remove(0);
			return Some(vec![(Clause(rem), sub.clone())]);
		}
	}
	None
}

pub fn list_length(
	goal: &Clause,
	p: &Proposition,
	sub: &Substitution,
) -> Option<(Clause, Substitution)> {
	let mut cur = apply_substitution(sub, &Unifiable::Term(p.terms[0].clone()));
	let mut count = 0;
	while let Unifiable::Term(Term::FunctionApplication { name, args }) = &cur {
		if name == "empty_list" && args.is_empty() {
			break;
		}
		if name == "cons" && args.len() == 2 {
			count += 1;
			cur = Unifiable::Term(args[1].clone());
		} else {
			return None;
		}
	}
	match apply_substitution(sub, &Unifiable::Term(p.terms[1].clone())) {
		Unifiable::Term(Term::Identifier(var)) => {
			let mut rem = goal.0.clone();
			rem.remove(0);
			let mut new_sub = sub.clone();
			new_sub.insert(
				Unifiable::Term(Term::Identifier(var.clone())),
				Unifiable::Term(Term::FunctionApplication {
					name: count.to_string(),
					args: vec![],
				}),
			);
			let mut clause = Clause(rem.clone());
			apply_substitution_to_clause(&new_sub, &mut clause);
			Some((clause, new_sub))
		},
		Unifiable::Term(Term::FunctionApplication { name, args }) if args.is_empty() => {
			if name == count.to_string() {
				let mut rem = goal.0.clone();
				rem.remove(0);
				return Some((Clause(rem), sub.clone()));
			}
			None
		},
		_ => None,
	}
}

pub fn list_elem(
	goal: &Clause,
	p: &Proposition,
	sub: &Substitution,
) -> Option<Vec<(Clause, Substitution)>> {
	let list_uv = apply_substitution(sub, &Unifiable::Term(p.terms[0].clone()));
	match list_uv {
		Unifiable::Term(Term::FunctionApplication { name, args })
			if name == "cons" && args.len() == 2 =>
		{
			let head = args[0].clone();
			let tail = args[1].clone();
			let elem = p.terms[1].clone();

			// R1: elem = head
			let mut temp_sub: std::collections::HashMap<Unifiable, Unifiable> =
				empty_substitution();
			temp_sub.insert(Unifiable::Term(elem.clone()), Unifiable::Term(head.clone()));

			let mut new_sub = sub.clone();
			apply_substitution_to_sub(&temp_sub, &mut new_sub);

			let mut g1 = goal.0.clone();
			g1.remove(0);
			let clause1 = Clause(g1);

			// R2: Elem(tail, elem)
			let mut g2 = goal.0.clone();
			g2.remove(0);
			g2.insert(
				0,
				Literal::Not(Proposition {
					name: "Elem".into(),
					terms: vec![tail.clone(), elem.clone()],
				}),
			);
			let clause2 = Clause(g2);

			return Some(vec![(clause1, new_sub), (clause2, sub.clone())]);
		},
		Unifiable::Term(Term::FunctionApplication { name, args })
			if name == "empty_list" && args.is_empty() =>
		{
			return None;
		},
		_ => None,
	}
}
