use super::mgu::{mgu, Result, Substitution, Unifiable, UnificationEquation};
use super::substitution::{
	self, apply_substitution, apply_substitution_to_sub, empty_substitution,
};
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

pub fn resolvent(c1: &Clause, c2: &Clause) -> Option<(Clause, Substitution)> {
	for l1 in &c1.0 {
		for l2 in &c2.0 {
			if let Ok(Some(sub)) = unify_literals(l1, l2) {
				let mut new_lits = Vec::new();
				for l in &c1.0 {
					if l != l1 {
						new_lits.push(
							match apply_substitution(&sub, &Unifiable::Prop(l.to_proposition())) {
								Unifiable::Prop(p) => Literal::Proposition(p),
								_ => panic!("Expected proposition after substitution"),
							},
						);
					}
				}
				for l in &c2.0 {
					if l != l2 {
						new_lits.push(
							match apply_substitution(&sub, &Unifiable::Prop(l.to_proposition())) {
								Unifiable::Prop(p) => Literal::Proposition(p),
								_ => panic!("Expected proposition after substitution"),
							},
						);
					}
				}
				return Some((Clause::from_literals(new_lits), sub));
			}
		}
	}
	None
}

pub fn sld_resolution(mut prog: Progam, mut goal: Clause) -> bool {
	// Ensure the goal is a single clause
	println!("Initial goal: {:?}", goal);
	while !goal.is_empty() {
		let mut resolved = false;
		for clause in &prog.0 {
			if let Some((resolvent, _sub)) = resolvent(clause, &goal) {
				println!("Resolving with clause: {:?}", clause);
				println!("New goal after resolution: {:?}", resolvent);
				goal = resolvent;
				resolved = true;
				break;
			}
		}
		if !resolved {
			return false;
		}
	}
	true
}
