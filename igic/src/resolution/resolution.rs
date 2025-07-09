use colored::*;
use std::collections::VecDeque;

use rustyline::history::FileHistory;
use rustyline::Editor;

use crate::mgu::mgu::{mgu, Result, Substitution, Unifiable, UnificationEquation};
use crate::mgu::substitution::{
	apply_substitution, apply_substitution_to_clause, apply_substitution_to_sub, empty_substitution,
};
use crate::types::ast::{Proposition, Term};
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
					stack.push_back((new_goal, new_sub));
				}
				continue;
			}
			for clause in &program.0 {
				clause_counter += 1;
				let unique_suffix = format!("_{}", clause_counter);
				let std_clause = clause.suffix_vars(&unique_suffix);
				if let Some(literal) = std_clause.0.first() {
					if let Ok(Some(mgu_sub)) = unify_literals(goal_literal, literal) {
						let mut new_sub = current_sub.clone();
						apply_substitution_to_sub(&mgu_sub, &mut new_sub);

						let mut new_goal_lits = current_goal.0.clone();
						new_goal_lits.remove(0);
						new_goal_lits.extend(std_clause.iter().skip(1).cloned());

						let mut new_goal = Clause(new_goal_lits);
						apply_substitution_to_clause(&mgu_sub, &mut new_goal);
						stack.push_back((new_goal, new_sub));
					}
				}
			}
		}
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
			("Is_list", 1) => {
				if let Unifiable::Term(Term::FunctionApplication { name, args }) =
					apply_substitution(sub, &Unifiable::Term(p.terms[0].clone()))
				{
					if (name == "cons" && args.len() == 2)
						|| (name == "empty_list" && args.is_empty())
					{
						let mut rem = goal.0.clone();
						rem.remove(0);
						return Some(vec![(Clause(rem), sub.clone())]);
					}
				}
			},
			("Length", 2) => {
				return built_in_preds_length(goal, p, sub).map(|cl| vec![cl]);
			},
			("Elem", 2) => {
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
						temp_sub
							.insert(Unifiable::Term(elem.clone()), Unifiable::Term(head.clone()));

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
					_ => {},
				}
			},
			_ => {},
		}
	}
	None
}

fn built_in_preds_length(
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
