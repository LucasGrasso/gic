use rustyline::history::FileHistory;
use rustyline::Editor;

use crate::mgu::mgu::{mgu, Result, Substitution, Unifiable, UnificationEquation};
use crate::mgu::substitution::{
	self, apply_substitution, apply_substitution_to_clause, apply_substitution_to_sub,
	empty_substitution,
};
use crate::types::clause::{self, Clause, Literal, Progam};
use std::collections::VecDeque;

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

pub fn sld_resolution(progam: &Progam, goal: &Clause, rl: &mut Editor<(), FileHistory>) {
	if goal.is_empty() {
		eprintln!("Goal is empty, no resolution needed.");
		return;
	}
	if !goal.is_goal() {
		eprintln!("Goal is not a valid goal clause.");
		return;
	}
	if progam.0.is_empty() {
		eprintln!("Program is empty, no clauses to resolve.");
		return;
	}
	if !progam.is_horn() {
		eprintln!("Program is not a Horn clause program, SLD resolution not applicable.");
		return;
	}

	let mut queue: VecDeque<(Clause, Substitution)> = VecDeque::new();
	queue.push_back((goal.clone(), empty_substitution()));

	while let Some((current_goal, current_sub)) = queue.pop_front() {
		if current_goal.is_empty() {
			println!("✔ Solution found with substitution: {:?} \n", current_sub);
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

		for goal_literal in &current_goal.0 {
			for clause in &progam.0 {
				for literal in &clause.0 {
					match unify_literals(goal_literal, literal) {
						Ok(Some(mgu_sub)) => {
							let mut new_sub = current_sub.clone();
							apply_substitution_to_sub(&mgu_sub, &mut new_sub);

							let mut new_goal = Clause::new();
							new_goal.0.extend(current_goal.0.iter().skip(1).cloned());
							new_goal.0.extend(clause.0.iter().cloned());
							apply_substitution_to_clause(&new_sub, &mut new_goal);
							println!("new_goal: {:?}", new_goal);
							queue.push_back((new_goal, new_sub));
						},
						Ok(None) => continue,
						Err(e) => {
							continue;
						},
					}
				}
			}
		}
	}

	println!("✘ No solution found.");
}
