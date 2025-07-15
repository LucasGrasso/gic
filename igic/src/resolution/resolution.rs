use colored::*;
use std::collections::VecDeque;

use rustyline::history::FileHistory;
use rustyline::Editor;

use crate::libraries::built_in_preds;

use crate::mgu::mgu::{mgu, Result, Substitution, Unifiable, UnificationEquation};
use crate::mgu::substitution::{
	apply_substitution_to_clause, compose_substitutions, empty_substitution,
};

use crate::types::ast::Term;
use crate::types::clause::{Clause, Literal, Program};

pub fn sld_resolution(program: &Program, goal: &Clause, rl: &mut Editor<(), FileHistory>) {
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
	let mut clause_counter = 1;
	stack.push_back((goal.clone(), empty_substitution()));

	while let Some((current_goal, current_sub)) = stack.pop_back() {
		if let Some(goal_literal) = current_goal.0.first() {
			if let Some(branches) = built_in_preds(&current_goal, goal_literal, &current_sub) {
				for (new_goal, new_sub) in branches {
					if let Some(new_goal_literal) = new_goal.0.first() {
						sld_backtrack(
							&new_goal,
							new_goal_literal,
							&new_sub,
							program,
							&mut clause_counter,
							&mut stack,
							&free_var_terms,
							rl,
						);
					} else {
						print_solutions(&free_var_terms, &new_sub);
						match continue_prompt(rl) {
							true => continue,
							false => return,
						}
					}
				}
				continue;
			}
			sld_backtrack(
				&current_goal,
				goal_literal,
				&current_sub,
				program,
				&mut clause_counter,
				&mut stack,
				&free_var_terms,
				rl,
			);
		}
	}
	println!("{}", "false.".red());
}

fn sld_backtrack(
	current_goal: &Clause,
	current_goal_literal: &Literal,
	current_sub: &Substitution,
	program: &Program,
	clause_counter: &mut i32,
	stack: &mut VecDeque<(Clause, Substitution)>,
	free_vars_terms: &[Unifiable],
	rl: &mut Editor<(), FileHistory>,
) -> () {
	for clause in &program.0 {
		*clause_counter += 1;
		let unique_suffix = format!("_{}", clause_counter);
		let std_clause = clause.suffix_vars(&unique_suffix);
		if let Some(literal) = std_clause.0.first() {
			if let Ok(Some(mgu_sub)) = unify_literals(current_goal_literal, literal) {
				let mut new_sub = current_sub.clone();
				compose_substitutions(&mgu_sub, &mut new_sub);

				let mut new_goal_lits = std_clause.0[1..].to_vec();
				new_goal_lits.extend(current_goal.iter().skip(1).cloned());

				let mut new_goal = Clause(new_goal_lits);
				if new_goal.is_empty() {
					print_solutions(&free_vars_terms, &new_sub);
					match continue_prompt(rl) {
						true => continue,
						false => return,
					}
				} else {
					apply_substitution_to_clause(&mgu_sub, &mut new_goal);
					stack.push_back((new_goal, new_sub));
				}
			}
		}
	}
}

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

fn print_solutions(free_vars: &[Unifiable], sub: &Substitution) {
	let mut bindings = Vec::new();
	for var in free_vars {
		if let Some(value) = sub.get(var) {
			bindings.push(format!("{} := {}", var, value));
		}
	}
	if bindings.is_empty() {
		println!("{}", "true.".green());
	} else {
		println!("{}", bindings.join(", "));
	}
}

fn continue_prompt(rl: &mut Editor<(), FileHistory>) -> bool {
	let readline = rl.readline("Continue? (Y/N) ");
	match readline {
		Ok(input) => input.trim().eq_ignore_ascii_case("y"),
		Err(_) => false, // Exit if there's an error reading input
	}
}
