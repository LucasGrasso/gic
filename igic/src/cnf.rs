use crate::types::ast::{Proposition, Term};
use crate::types::clause::Clause;
use crate::types::{Expression, GicError, Result};

pub fn to_cnf(expr: Expression) -> Clause {
	let no_implications = eliminate_implications(expr);
	let nnf = to_nnf(no_implications);
	let prenex = distribute_quantifiers(nnf);
	let mut counter = 0;
	let no_existentials = deskolem(prenex, &[], &mut counter);
	let quantifier_free = remove_universal_quantifiers(no_existentials);
	flatten_cnf(quantifier_free).unwrap()
}

fn flatten_cnf(expr: Expression) -> Result<Clause> {
	match expr {
		Expression::And(a, b) => {
			let mut left = flatten_cnf(*a)?;
			let mut right = flatten_cnf(*b)?;
			left.append(&mut right);
			Ok(left)
		},
		Expression::Or(a, b) => {
			let left = flatten_cnf(*a)?;
			let right = flatten_cnf(*b)?;

			let mut result = vec![];

			for l in &left.0 {
				for r in &right.0 {
					let mut disjunction = l.clone();
					disjunction.extend_from_slice(&r);
					result.push(disjunction);
				}
			}

			Ok(Clause(result))
		},
		e => Ok(Clause(vec![vec![e]])), // cláusula unitaria con literal
	}
}

fn eliminate_implications(expr: Expression) -> Expression {
	match expr {
		Expression::Implies(a, b) => Expression::Or(
			Box::new(Expression::Not(Box::new(eliminate_implications(*a)))),
			Box::new(eliminate_implications(*b)),
		),
		Expression::And(a, b) => Expression::And(
			Box::new(eliminate_implications(*a)),
			Box::new(eliminate_implications(*b)),
		),
		Expression::Or(a, b) => Expression::Or(
			Box::new(eliminate_implications(*a)),
			Box::new(eliminate_implications(*b)),
		),
		Expression::Not(inner) => Expression::Not(Box::new(eliminate_implications(*inner))),
		Expression::Exists(var, inner) => {
			Expression::Exists(var, Box::new(eliminate_implications(*inner)))
		},
		Expression::ForAll(var, inner) => {
			Expression::ForAll(var, Box::new(eliminate_implications(*inner)))
		},
		other => other,
	}
}

fn to_nnf(expr: Expression) -> Expression {
	match expr {
		Expression::Not(inner) => match *inner {
			Expression::And(a, b) => Expression::Or(
				Box::new(to_nnf(Expression::Not(a))),
				Box::new(to_nnf(Expression::Not(b))),
			),
			Expression::Or(a, b) => Expression::And(
				Box::new(to_nnf(Expression::Not(a))),
				Box::new(to_nnf(Expression::Not(b))),
			),
			Expression::Not(e) => to_nnf(*e),
			Expression::ForAll(v, e) => Expression::Exists(v, Box::new(to_nnf(Expression::Not(e)))),
			Expression::Exists(v, e) => Expression::ForAll(v, Box::new(to_nnf(Expression::Not(e)))),
			e => Expression::Not(Box::new(to_nnf(e))),
		},
		Expression::And(a, b) => Expression::And(Box::new(to_nnf(*a)), Box::new(to_nnf(*b))),
		Expression::Or(a, b) => Expression::Or(Box::new(to_nnf(*a)), Box::new(to_nnf(*b))),
		Expression::Exists(v, e) => Expression::Exists(v, Box::new(to_nnf(*e))),
		Expression::ForAll(v, e) => Expression::ForAll(v, Box::new(to_nnf(*e))),
		other => other,
	}
}

fn distribute_quantifiers(expr: Expression) -> Expression {
	match expr {
		Expression::And(a, b) => match (*a, *b) {
			(Expression::ForAll(x, a1), b1) => Expression::ForAll(
				x,
				Box::new(distribute_quantifiers(Expression::And(a1, Box::new(b1)))),
			),
			(a1, Expression::ForAll(x, b1)) => Expression::ForAll(
				x,
				Box::new(distribute_quantifiers(Expression::And(Box::new(a1), b1))),
			),
			(Expression::Exists(x, a1), b1) => Expression::Exists(
				x,
				Box::new(distribute_quantifiers(Expression::And(a1, Box::new(b1)))),
			),
			(a1, Expression::Exists(x, b1)) => Expression::Exists(
				x,
				Box::new(distribute_quantifiers(Expression::And(Box::new(a1), b1))),
			),
			(a2, b2) => Expression::And(
				Box::new(distribute_quantifiers(a2)),
				Box::new(distribute_quantifiers(b2)),
			),
		},
		Expression::Or(a, b) => match (*a, *b) {
			(Expression::ForAll(x, a1), b1) => Expression::ForAll(
				x,
				Box::new(distribute_quantifiers(Expression::Or(a1, Box::new(b1)))),
			),
			(a1, Expression::ForAll(x, b1)) => Expression::ForAll(
				x,
				Box::new(distribute_quantifiers(Expression::Or(Box::new(a1), b1))),
			),
			(Expression::Exists(x, a1), b1) => Expression::Exists(
				x,
				Box::new(distribute_quantifiers(Expression::Or(a1, Box::new(b1)))),
			),
			(a1, Expression::Exists(x, b1)) => Expression::Exists(
				x,
				Box::new(distribute_quantifiers(Expression::Or(Box::new(a1), b1))),
			),
			(a2, b2) => Expression::Or(
				Box::new(distribute_quantifiers(a2)),
				Box::new(distribute_quantifiers(b2)),
			),
		},
		Expression::Exists(x, e) => Expression::Exists(x, Box::new(distribute_quantifiers(*e))),
		Expression::ForAll(x, e) => Expression::ForAll(x, Box::new(distribute_quantifiers(*e))),
		other => other,
	}
}

fn deskolem(expr: Expression, scope: &[String], counter: &mut usize) -> Expression {
	match expr {
		Expression::Exists(var, inner) => {
			let fname = format!("f{}", *counter);
			*counter += 1;
			let args: Vec<Term> = scope.iter().map(|v| Term::Identifier(v.clone())).collect();
			let replacement = Term::FunctionApplication { name: fname, args };
			let new_body = substitute_term(*inner, &var, replacement);
			deskolem(new_body, scope, counter)
		},
		Expression::ForAll(var, inner) => {
			let mut extended = scope.to_vec();
			extended.push(var.clone());
			Expression::ForAll(var, Box::new(deskolem(*inner, &extended, counter)))
		},
		Expression::And(a, b) => Expression::And(
			Box::new(deskolem(*a, scope, counter)),
			Box::new(deskolem(*b, scope, counter)),
		),
		Expression::Or(a, b) => Expression::Or(
			Box::new(deskolem(*a, scope, counter)),
			Box::new(deskolem(*b, scope, counter)),
		),
		Expression::Not(inner) => Expression::Not(Box::new(deskolem(*inner, scope, counter))),
		other => other,
	}
}

fn substitute_term(expr: Expression, var: &str, replacement: Term) -> Expression {
	match expr {
		Expression::Proposition(p) => {
			let new_terms =
				p.terms.into_iter().map(|t| substitute_in_term(t, var, &replacement)).collect();
			Expression::Proposition(Proposition { name: p.name, terms: new_terms })
		},
		Expression::Not(e) => {
			Expression::Not(Box::new(substitute_term(*e, var, replacement.clone())))
		},
		Expression::And(a, b) => Expression::And(
			Box::new(substitute_term(*a, var, replacement.clone())),
			Box::new(substitute_term(*b, var, replacement.clone())),
		),
		Expression::Or(a, b) => Expression::Or(
			Box::new(substitute_term(*a, var, replacement.clone())),
			Box::new(substitute_term(*b, var, replacement.clone())),
		),
		Expression::ForAll(v, e) => {
			Expression::ForAll(v.clone(), Box::new(substitute_term(*e, var, replacement)))
		},
		Expression::Exists(v, e) => {
			Expression::Exists(v.clone(), Box::new(substitute_term(*e, var, replacement)))
		},
		other => other,
	}
}

fn substitute_in_term(term: Term, var: &str, replacement: &Term) -> Term {
	match term {
		Term::Identifier(s) if s == var => replacement.clone(),
		Term::FunctionApplication { name, args } => Term::FunctionApplication {
			name,
			args: args.into_iter().map(|t| substitute_in_term(t, var, replacement)).collect(),
		},
		t => t,
	}
}

fn remove_universal_quantifiers(expr: Expression) -> Expression {
	match expr {
		Expression::ForAll(_, inner) => remove_universal_quantifiers(*inner),
		Expression::And(a, b) => Expression::And(
			Box::new(remove_universal_quantifiers(*a)),
			Box::new(remove_universal_quantifiers(*b)),
		),
		Expression::Or(a, b) => Expression::Or(
			Box::new(remove_universal_quantifiers(*a)),
			Box::new(remove_universal_quantifiers(*b)),
		),
		Expression::Not(e) => Expression::Not(Box::new(remove_universal_quantifiers(*e))),
		other => other,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_to_cnf_pipeline() {
		let expr = Expression::Implies(
			Box::new(Expression::Proposition(Proposition {
				name: "P".into(),
				terms: vec![Term::Identifier("x".into())],
			})),
			Box::new(Expression::Proposition(Proposition {
				name: "Q".into(),
				terms: vec![Term::Identifier("x".into())],
			})),
		);

		let cnf_clauses = to_cnf(expr);
		println!("CNF: {:?}", cnf_clauses);
	}
}
