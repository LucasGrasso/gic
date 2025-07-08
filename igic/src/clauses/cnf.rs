use super::skolem::SkolemContext;
use crate::types::ast::Expression;
use crate::types::clause::{self, Clause, Literal, Progam};
use crate::types::{GicError, Result}; // adjust path as needed

pub struct Clausifier {
	clause_id: usize,
	ctx: SkolemContext,
	program: Progam,
}

impl Clausifier {
	pub fn new() -> Self {
		Clausifier { clause_id: 1, ctx: SkolemContext::new(), program: Progam(Vec::new()) }
	}

	pub fn add_to_progam(&mut self, expr: Expression) -> Result<()> {
		let progamified_clause = self.clausify(expr)?;
		let clause = progamified_clause
			.0
			.first()
			.ok_or_else(|| GicError::ClauseError("No clauses generated".to_string()))?;
		self.program.0.push(clause.clone());
		Ok(())
	}

	pub fn clausify(&mut self, expr: Expression) -> Result<Progam> {
		self.ctx.set_clause_id(self.clause_id);
		self.clause_id += 1;

		let no_implications = eliminate_implications(expr);
		let nnf = to_nnf(no_implications);
		let prenex = distribute_quantifiers(nnf);

		let no_existentials = self.ctx.deskolem(prenex);
		let quantifier_free = remove_universal_quantifiers(no_existentials);
		let cnf = flatten_cnf(quantifier_free)?;

		Ok(cnf) // Return the first clause from the CNF
	}

	pub fn get_program(&self) -> &Progam {
		&self.program
	}
}

fn expr_to_literal(expr: Expression) -> Result<Literal> {
	match expr {
		Expression::Proposition(p) => Ok(Literal::Proposition(p)),
		Expression::Not(inner) => match *inner {
			Expression::Proposition(p) => Ok(Literal::Not(p)),
			_ => Err(GicError::ClauseError(format!("Not applied to non-proposition: {}", inner))),
		},
		_ => Err(GicError::ClauseError(format!("Expression is not a literal: {}", expr))),
	}
}

pub fn flatten_cnf(expr: Expression) -> Result<Progam> {
	match expr {
		Expression::And(a, b) => {
			let mut left_clauses = flatten_cnf(*a)?;
			let mut right_clauses = flatten_cnf(*b)?;
			left_clauses.append(&mut right_clauses);
			Ok(left_clauses)
		},

		Expression::Or(a, b) => {
			let left = flatten_cnf(*a)?;
			let right = flatten_cnf(*b)?;
			let mut result = Vec::new();

			// Distribute literals from both sides
			for Clause(lits_l) in &left {
				for Clause(lits_r) in &right {
					let mut disj = lits_l.clone();
					disj.extend(lits_r.clone());
					result.push(Clause(disj));
				}
			}

			Ok(Progam(result))
		},

		leaf => {
			let lit = expr_to_literal(leaf)?;
			Ok(Progam(vec![Clause(vec![lit])]))
		},
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
	use std::vec;

	use super::*;
	use crate::types::ast::{Expression, Proposition, Term};

	#[test]
	fn test_cnf_forall_impl() {
		let mut clausifier = Clausifier::new();
		let expr = Expression::ForAll(
			"X".to_string(),
			Box::new(Expression::Implies(
				Box::new(Expression::Proposition(Proposition {
					name: "P".to_string(),
					terms: vec![Term::Identifier("X".to_string())],
				})),
				Box::new(Expression::Proposition(Proposition {
					name: "Q".to_string(),
					terms: vec![Term::Identifier("X".to_string())],
				})),
			)),
		);

		let clause = clausifier.clausify(expr).unwrap();

		let expected_progam = Progam(vec![Clause(vec![
			Literal::Not(Proposition {
				name: "P".to_string(),
				terms: vec![Term::Identifier("X".to_string())],
			}),
			Literal::Proposition(Proposition {
				name: "Q".to_string(),
				terms: vec![Term::Identifier("X".to_string())],
			}),
		])]);
		assert_eq!(clause, expected_progam);
	}

	#[test]
	fn test_cnf_and() {
		let mut clausifier = Clausifier::new();
		let expr = Expression::And(
			Box::new(Expression::Proposition(Proposition {
				name: "P".to_string(),
				terms: vec![Term::Identifier("X".to_string())],
			})),
			Box::new(Expression::Proposition(Proposition {
				name: "Q".to_string(),
				terms: vec![Term::Identifier("Y".to_string())],
			})),
		);

		let program = clausifier.clausify(expr).unwrap();
		let expected_progam = Progam(vec![
			Clause(vec![Literal::Proposition(Proposition {
				name: "P".to_string(),
				terms: vec![Term::Identifier("X".to_string())],
			})]),
			Clause(vec![Literal::Proposition(Proposition {
				name: "Q".to_string(),
				terms: vec![Term::Identifier("Y".to_string())],
			})]),
		]);
		assert_eq!(program, expected_progam);
	}

	#[test]
	fn test_cnf_deskolem() {
		let mut clausifier = Clausifier::new();
		let expr = Expression::ForAll(
			"X".to_string(),
			Box::new(Expression::Exists(
				"Y".to_string(),
				Box::new(Expression::Proposition(Proposition {
					name: "R".to_string(),
					terms: vec![
						Term::Identifier("X".to_string()),
						Term::Identifier("Y".to_string()),
					],
				})),
			)),
		);

		let program = clausifier.clausify(expr).unwrap();

		let expected_progam = Progam(vec![Clause(vec![Literal::Proposition(Proposition {
			name: "R".to_string(),
			terms: vec![
				Term::Identifier("X".to_string()),
				Term::FunctionApplication {
					name: "_Y_1".to_string(),
					args: vec![Term::Identifier("X".to_string())],
				},
			],
		})])]);

		assert_eq!(program, expected_progam);
	}
}
