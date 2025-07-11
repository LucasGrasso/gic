use colored::*;
use std::collections::VecDeque;

use rustyline::history::FileHistory;
use rustyline::Editor;

use crate::builtins::integers::arithmetic::{arithmetic_builtin, compare_builtin};
use crate::builtins::lists::lists_builtins::{is_list, list_elem, list_length};
use crate::mgu::mgu::{mgu, Result, Substitution, Unifiable, UnificationEquation};
use crate::mgu::substitution::{
	apply_substitution, apply_substitution_to_clause, apply_substitution_to_sub,
	empty_substitution, print_substitution,
};
use crate::types::ast::Term;
use crate::types::clause::{Clause, Literal, Progam};

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
	let mut clause_counter = 1;
	stack.push_back((goal.clone(), empty_substitution()));

	while let Some((current_goal, current_sub)) = stack.pop_back() {
		if current_goal.is_empty() {
			let mut bindings = Vec::new();
			for var in &free_var_terms {
				if let Some(value) = current_sub.get(var) {
					bindings.push(format!("{} := {}", var, value));
				}
			}
			if bindings.is_empty() {
				println!("{}", "true.".green());
			} else {
				println!("{}", bindings.join(", "));
			}
			if stack.is_empty() {
				return;
			}
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
			if let Some(branches) = built_in_preds(&current_goal, goal_literal, &current_sub) {
				for (new_goal, new_sub) in branches {
					println!("Resolving built-in predicate: {}", goal_literal);
					println!("New goal: {}", new_goal);
					stack.push_back((new_goal, new_sub));
				}
				continue;
			}
			println!("Resolving: {}", current_goal);
			println!("Current substitution: ");
			print_substitution(&current_sub);
			for clause in &program.0 {
				clause_counter += 1;
				let unique_suffix = format!("_{}", clause_counter);
				let std_clause = clause.suffix_vars(&unique_suffix);
				if let Some(literal) = std_clause.0.first() {
					if let Ok(Some(mgu_sub)) = unify_literals(goal_literal, literal) {
						println!("Unifying with clause: {}", std_clause);
						let mut new_sub = current_sub.clone();
						apply_substitution_to_sub(&mgu_sub, &mut new_sub);

						let mut new_goal_lits = current_goal.0.clone();
						new_goal_lits.remove(0);
						new_goal_lits.extend(std_clause.iter().skip(1).cloned());

						let mut new_goal = Clause(new_goal_lits);
						apply_substitution_to_clause(&mgu_sub, &mut new_goal);
						println!("New goal after unification: {}", new_goal);
						stack.push_back((new_goal, new_sub));
					}
				}
			}
		}
		println!("")
	}
	println!("{}", "false.".red());
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

fn built_in_preds(
	goal: &Clause,
	lit: &Literal,
	sub: &Substitution,
) -> Option<Vec<(Clause, Substitution)>> {
	if let Literal::Not(p) = lit {
		match (p.name.as_str(), p.terms.len()) {
			("Eq", 2) => {
				let a = apply_substitution(sub, &Unifiable::Term(p.terms[0].clone()));
				let b = apply_substitution(sub, &Unifiable::Term(p.terms[1].clone()));
				if a == b {
					let mut rem = goal.0.clone();
					rem.remove(0);
					return Some(vec![(Clause(rem), sub.clone())]);
				}
			},
			("Diff", 2) => {
				let a = apply_substitution(sub, &Unifiable::Term(p.terms[0].clone()));
				let b = apply_substitution(sub, &Unifiable::Term(p.terms[1].clone()));
				if a != b {
					let mut rem = goal.0.clone();
					rem.remove(0);
					return Some(vec![(Clause(rem), sub.clone())]);
				}
			},
			("Var", 1) => {
				if let Unifiable::Term(Term::Identifier(_)) =
					apply_substitution(sub, &Unifiable::Term(p.terms[0].clone()))
				{
					let mut rem = goal.0.clone();
					rem.remove(0);
					return Some(vec![(Clause(rem), sub.clone())]);
				}
			},
			("Add", 3) => return arithmetic_builtin(goal, p, sub, |a, b| a + b),
			("Sub", 3) => return arithmetic_builtin(goal, p, sub, |a, b| a - b),
			("Mul", 3) => return arithmetic_builtin(goal, p, sub, |a, b| a * b),
			("Div", 3) => {
				return arithmetic_builtin(goal, p, sub, |a, b| {
					if b == 0 {
						panic!("Division by zero");
					}
					a / b
				});
			},
			("Mod", 3) => return arithmetic_builtin(goal, p, sub, |a, b| a % b),
			("Lt", 2) => return compare_builtin(goal, p, sub, |a, b| a < b),
			("Lt_eq", 2) => return compare_builtin(goal, p, sub, |a, b| a <= b),
			("Gt", 2) => return compare_builtin(goal, p, sub, |a, b| a > b),
			("Gt_eq", 2) => return compare_builtin(goal, p, sub, |a, b| a >= b),
			("Eq_int", 2) => return compare_builtin(goal, p, sub, |a, b| a == b),
			("Diff_int", 2) => return compare_builtin(goal, p, sub, |a, b| a != b),
			("Is_list", 1) => return is_list(sub, p, goal),
			("Length", 2) => {
				return list_length(goal, p, sub).map(|cl| vec![cl]);
			},
			("Elem", 2) => return list_elem(goal, p, sub),
			_ => return None,
		}
	}
	None
}
