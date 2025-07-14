use crate::types::ast::Expression;
use crate::types::ast::Term;

#[derive(Debug, Clone)]
pub struct SkolemContext {
	clause_id: usize,
}

impl SkolemContext {
	pub fn new() -> Self {
		Self { clause_id: 1 }
	}

	pub fn set_clause_id(&mut self, id: usize) {
		self.clause_id = id;
	}

	pub fn next_name(&mut self, var: &str) -> String {
		let name = format!("_{}_{}", var, self.clause_id);
		name
	}

	pub fn deskolem(&mut self, expr: Expression) -> Expression {
		deskolem(expr, &mut vec![], self)
	}
}

fn deskolem(expr: Expression, scope: &mut Vec<String>, ctx: &mut SkolemContext) -> Expression {
	match expr {
		Expression::Exists(var, inner) => {
			let var = var.trim_end_matches('.').to_string();

			let mut seen = std::collections::HashSet::new();
			let mut args: Vec<Term> = vec![];
			for v in scope.iter() {
				if seen.insert(v) {
					args.push(Term::Identifier(v.clone()));
				}
			}

			let name = ctx.next_name(&var);
			let skolem_term: Term = if scope.len() == 0 {
				Term::Identifier(name.clone())
			} else {
				Term::FunctionApplication { name, args }
			};

			let substituted = substitute_var(*inner, &var, &skolem_term);
			deskolem(substituted, scope, ctx)
		},

		Expression::ForAll(var, inner) => {
			let var = var.trim_end_matches('.').to_string();
			scope.push(var.clone());
			let res = Expression::ForAll(var.clone(), Box::new(deskolem(*inner, scope, ctx)));
			scope.pop();
			res
		},

		Expression::And(a, b) => {
			Expression::And(Box::new(deskolem(*a, scope, ctx)), Box::new(deskolem(*b, scope, ctx)))
		},

		Expression::Or(a, b) => {
			Expression::Or(Box::new(deskolem(*a, scope, ctx)), Box::new(deskolem(*b, scope, ctx)))
		},

		Expression::Not(inner) => Expression::Not(Box::new(deskolem(*inner, scope, ctx))),
		other => other,
	}
}

fn substitute_var(expr: Expression, var: &str, replacement: &Term) -> Expression {
	match expr {
		Expression::Proposition(mut p) => {
			p.terms = p.terms.into_iter().map(|t| substitute_term(t, var, replacement)).collect();
			Expression::Proposition(p)
		},

		Expression::Not(inner) => {
			Expression::Not(Box::new(substitute_var(*inner, var, replacement)))
		},
		Expression::And(a, b) => Expression::And(
			Box::new(substitute_var(*a, var, replacement)),
			Box::new(substitute_var(*b, var, replacement)),
		),
		Expression::Or(a, b) => Expression::Or(
			Box::new(substitute_var(*a, var, replacement)),
			Box::new(substitute_var(*b, var, replacement)),
		),
		Expression::ForAll(v, e) if v == var => Expression::ForAll(v, e),
		Expression::Exists(v, e) if v == var => Expression::Exists(v, e),
		Expression::ForAll(v, e) => {
			Expression::ForAll(v.clone(), Box::new(substitute_var(*e, var, replacement)))
		},
		Expression::Exists(v, e) => {
			Expression::Exists(v.clone(), Box::new(substitute_var(*e, var, replacement)))
		},
		other => other,
	}
}

fn substitute_term(term: Term, var: &str, replacement: &Term) -> Term {
	match term {
		Term::Identifier(id) if id == var => replacement.clone(),
		Term::FunctionApplication { name, args } => Term::FunctionApplication {
			name,
			args: args.into_iter().map(|t| substitute_term(t, var, replacement)).collect(),
		},
		other => other,
	}
}
