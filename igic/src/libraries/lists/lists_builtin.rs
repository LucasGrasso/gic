use crate::mgu::mgu::{Substitution, Unifiable};
use crate::mgu::substitution::{
	apply_substitution, apply_substitution_to_clause, compose_substitutions, empty_substitution,
};
use crate::types::ast::{Proposition, Term};
use crate::types::clause::Clause;

pub fn is_list_pred(
	goal: &Clause,
	p: &Proposition,
	sub: &Substitution,
) -> Option<Box<dyn Iterator<Item = (Clause, Substitution)>>> {
	let term = apply_substitution(sub, &Unifiable::Term(p.terms[0].clone()));
	if let Unifiable::Term(t) = term {
		if is_proper_list(&t) {
			let mut rem = goal.0.clone();
			rem.remove(0);
			return Some(Box::new(std::iter::once((Clause(rem), sub.clone()))));
		}
	}
	None
}

fn is_proper_list(term: &Term) -> bool {
	match term {
		Term::FunctionApplication { name, args } => {
			if name == "empty_list" && args.is_empty() {
				true
			} else if name == "cons" && args.len() == 2 {
				is_proper_list(&args[1])
			} else {
				false
			}
		},
		_ => false,
	}
}

pub fn length_pred<'a>(
	goal: &'a Clause,
	prop: &'a Proposition,
	sub: &'a Substitution,
) -> Option<Box<dyn Iterator<Item = (Clause, Substitution)> + 'a>> {
	let list_term = apply_substitution(sub, &Unifiable::Term(prop.terms[0].clone()));
	let len_term = apply_substitution(sub, &Unifiable::Term(prop.terms[1].clone()));

	match (list_term, len_term) {
		// Case: both list and length are instanciated
		(
			Unifiable::Term(Term::FunctionApplication { name, args }),
			Unifiable::Term(Term::Number(n)),
		) => {
			let l_term = Unifiable::Term(Term::FunctionApplication { name, args });
			if let Some(true_length) = get_length_of_list(&l_term) {
				if true_length == n as usize {
					let new_goal = Clause(goal.0[1..].to_vec());
					let new_sub = sub.clone();
					return Some(Box::new(std::iter::once((new_goal, new_sub))));
				}
			}
			None
		},
		// Case: lists is instanciated, length is free
		(
			Unifiable::Term(Term::FunctionApplication { name, args }),
			Unifiable::Term(Term::Identifier(_)),
		) => {
			let l_term = Unifiable::Term(Term::FunctionApplication { name, args });
			if let Some(true_length) = get_length_of_list(&l_term) {
				let mut temp_sub = empty_substitution();
				temp_sub.insert(
					Unifiable::Term(prop.terms[1].clone()),
					Unifiable::Term(Term::Number(true_length as i64)),
				);

				let mut new_goal = Clause(goal.0[1..].to_vec());
				let mut new_sub = sub.clone();
				compose_substitutions(&temp_sub, &mut new_sub);
				apply_substitution_to_clause(&temp_sub, &mut new_goal);

				return Some(Box::new(std::iter::once((new_goal, new_sub))));
			}
			None
		},
		// Case: length is a concrete number, we generate that many empty cons cells
		(Unifiable::Term(Term::Identifier(_)), Unifiable::Term(Term::Number(n))) => {
			let list = generate_list_of_length(n as usize);
			let mut temp_sub = empty_substitution();
			temp_sub.insert(Unifiable::Term(prop.terms[0].clone()), Unifiable::Term(list));

			let mut new_goal = Clause(goal.0[1..].to_vec());
			let mut new_sub = sub.clone();
			compose_substitutions(&temp_sub, &mut new_sub);
			apply_substitution_to_clause(&temp_sub, &mut new_goal);

			Some(Box::new(std::iter::once((new_goal, new_sub))))
		},
		(Unifiable::Term(Term::Identifier(_)), Unifiable::Term(Term::Identifier(_))) => {
			Some(Box::new((0..).map(move |n| {
				let list = generate_list_of_length(n as usize);

				let mut temp_sub = empty_substitution();
				temp_sub.insert(Unifiable::Term(prop.terms[0].clone()), Unifiable::Term(list));
				temp_sub.insert(
					Unifiable::Term(prop.terms[1].clone()),
					Unifiable::Term(Term::Number(n)),
				);

				let mut new_goal = Clause(goal.0[1..].to_vec());
				let mut new_sub = sub.clone();

				compose_substitutions(&temp_sub, &mut new_sub);
				apply_substitution_to_clause(&temp_sub, &mut new_goal);

				(new_goal, new_sub)
			})))
		},
		_ => None,
	}
}

fn get_length_of_list(term: &Unifiable) -> Option<usize> {
	match term {
		Unifiable::Term(Term::FunctionApplication { name, args }) => {
			if name == "empty_list" && args.is_empty() {
				Some(0)
			} else if name == "cons" && args.len() == 2 {
				let tail = &args[1];
				let unifiable_tail = Unifiable::Term(tail.clone());
				get_length_of_list(&unifiable_tail).map(|len| len + 1)
			} else {
				None
			}
		},
		_ => None,
	}
}

fn generate_list_of_length(n: usize) -> Term {
	let mut list = Term::FunctionApplication { name: "empty_list".to_string(), args: vec![] };
	for i in (0..n).rev() {
		list = Term::FunctionApplication {
			name: "cons".to_string(),
			args: vec![Term::Identifier(format!("E{}", i)), list],
		};
	}
	list
}
