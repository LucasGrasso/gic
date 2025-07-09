use std::collections::VecDeque;

use rustyline::history::FileHistory;
use rustyline::Editor;

use crate::mgu::mgu::{mgu, Result, Substitution, Unifiable, UnificationEquation};
use crate::mgu::substitution::{
	apply_substitution, apply_substitution_to_clause, apply_substitution_to_sub, empty_substitution,
};
use crate::types::ast::Term;
use crate::types::clause::{Clause, Literal, Progam};

fn unify_literals(l1: &Literal, l2: &Literal) -> Result<Option<Substitution>> {
	match (l1, l2) {
		(Literal::Proposition(p1), Literal::Not(p2))
		| (Literal::Not(p1), Literal::Proposition(p2)) => {
			let eq: UnificationEquation =
				vec![(Unifiable::Prop(p1.clone()), Unifiable::Prop(p2.clone()))];
			mgu(eq).map(Some)
		},
		_ => Ok(None),
	}
}

pub fn sld_resolution(program: &Progam, goal: &Clause, rl: &mut Editor<(), FileHistory>) {
	if goal.is_empty() {
		eprintln!("Goal is empty, no resolution needed.");
		return;
	}
	if !goal.is_goal() {
		eprintln!("Goal is not a valid goal clause.");
		return;
	}
	if program.0.is_empty() {
		eprintln!("Program is empty, no clauses to resolve.");
		return;
	}
	if !program.is_horn() {
		eprintln!("Program is not a Horn clause program, SLD resolution not applicable.");
		return;
	}

	let free_vars = goal.fv();
	let free_var_terms = free_vars
		.iter()
		.map(|var| Unifiable::Term(Term::Identifier(var.clone())))
		.collect::<Vec<_>>();

	let mut stack: VecDeque<(Clause, Substitution)> = VecDeque::new();
	stack.push_back((goal.clone(), empty_substitution()));

	while let Some((current_goal, current_sub)) = stack.pop_front() {
		if current_goal.is_empty() {
			println!("✔ Solution found!");
			let mut bindings = Vec::new();
			for var in &free_var_terms {
				if let Some(value) = current_sub.get(var) {
					bindings.push(format!("{} := {}", var, value));
				}
			}
			println!("{}", bindings.join(", "));
			let readline = rl.readline("Continue? (Y/N) ");
			match readline {
				Ok(input) => {
					if input.trim().eq_ignore_ascii_case("n") {
						return;
					} else {
						continue;
					}
				},
				Err(_) => return, // Exit if there's an error reading input
			}
		}
		if let Some(goal_literal) = current_goal.0.first() {
			if let Some(new_goal) = built_in_preds(&current_goal, goal_literal, &current_sub) {
				stack.push_back((new_goal, current_sub.clone()));
				continue;
			}
			for clause in &program.0 {
				if let Some(literal) = clause.0.first() {
					match unify_literals(goal_literal, literal) {
						Ok(Some(mgu_sub)) => {
							let mut new_sub = current_sub.clone();
							apply_substitution_to_sub(&mgu_sub, &mut new_sub);

							let mut remaining_goal_lits = current_goal.0.clone();
							remaining_goal_lits.remove(0);

							let mut new_goal_lits = remaining_goal_lits;
							new_goal_lits.extend(clause.negatives().into_iter().cloned());

							let mut new_goal = Clause(new_goal_lits);
							apply_substitution_to_clause(&mgu_sub, &mut new_goal);
							stack.push_back((new_goal, new_sub));
						},
						Ok(None) => continue,
						Err(_e) => {
							continue;
						},
					}
				}
			}
		}
	}
	println!("✘ No solution found.");
}

fn built_in_preds(
	current_goal: &Clause,
	goal_literal: &Literal,
	current_sub: &Substitution,
) -> Option<Clause> {
	match goal_literal {
		Literal::Not(p) => {
			match p.name.as_str() {
				"Eq" if p.terms.len() == 2 => {
					let left =
						apply_substitution(&current_sub, &Unifiable::Term(p.terms[0].clone()));
					let right =
						apply_substitution(&current_sub, &Unifiable::Term(p.terms[1].clone()));
					if left == right {
						let mut remaining_goal = current_goal.0.clone();
						remaining_goal.remove(0);
						return Some(Clause(remaining_goal));
					} else {
						return None;
					}
				},
				"Diff" if p.terms.len() == 2 => {
					let left =
						apply_substitution(&current_sub, &Unifiable::Term(p.terms[0].clone()));
					let right =
						apply_substitution(&current_sub, &Unifiable::Term(p.terms[1].clone()));
					if left != right {
						let mut remaining_goal = current_goal.0.clone();
						remaining_goal.remove(0);
						return Some(Clause(remaining_goal));
					} else {
						return None;
					}
				},
				"Var" if p.terms.len() == 1 => {
					let term =
						apply_substitution(current_sub, &Unifiable::Term(p.terms[0].clone()));
					match term {
						Unifiable::Term(Term::Identifier(_)) => {
							let mut remaining_goal = current_goal.0.clone();
							remaining_goal.remove(0);
							return Some(Clause(remaining_goal));
						},
						_ => None,
					}
				},
				"Atom" if p.terms.len() == 1 => {
					let term =
						apply_substitution(current_sub, &Unifiable::Term(p.terms[0].clone()));
					match term {
						Unifiable::Term(Term::FunctionApplication { .. }) => {
							let mut remaining_goal = current_goal.0.clone();
							remaining_goal.remove(0);
							return Some(Clause(remaining_goal));
						},
						_ => None,
					}
				},
				_ => None, // not a built-in, proceed with standard resolution
			}
		},
		_ => None,
	}
}
