use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use pratt::{Affix, Associativity, PrattParser, Precedence};

use crate::types::ast::{Expression, Proposition, Term};
use crate::types::{GicError, Result};

#[derive(Parser)]
#[grammar = "gic.pest"]
pub struct TokenParser;

pub fn parse_formula(input: &str) -> Result<Expression> {
	let mut pairs = TokenParser::parse(Rule::expr, input).map_err(GicError::from)?;

	let expr_pair = pairs
		.next()
		.ok_or_else(|| GicError::ParseError("No expression found".to_string()))?;

	let mut expr_children = expr_pair
		.into_inner()
		.filter(|p| !matches!(p.as_rule(), Rule::WHITESPACE | Rule::COMMENT));

	let mut pratt = crate::parser::GicParser;
	pratt
		.parse(&mut expr_children)
		.map_err(|e| GicError::ParseError(format!("Pratt parser error: {:?}", e)))
}

pub fn parse_gic_file(input: &str) -> Result<Vec<Expression>> {
	let mut pairs = TokenParser::parse(Rule::file, input).map_err(GicError::from)?;

	let mut expressions = Vec::new();

	let file_pair = pairs.next().unwrap(); // Rule::file

	for clause in file_pair.into_inner() {
		if clause.as_rule() != Rule::clause {
			continue;
		}

		let formula_pair = clause.into_inner().next().unwrap();
		let expr_pair = formula_pair.into_inner().next().unwrap(); // expr

		let mut expr_children = expr_pair
			.into_inner()
			.filter(|p| !matches!(p.as_rule(), Rule::WHITESPACE | Rule::COMMENT));

		let mut pratt = crate::parser::GicParser;
		let parsed_expr = pratt
			.parse(&mut expr_children)
			.map_err(|e| GicError::ParseError(format!("Pratt parser error: {:?}", e)))?;

		expressions.push(parsed_expr);
	}

	Ok(expressions)
}

struct GicParser;
impl<'i, I> PrattParser<I> for GicParser
where
	I: Iterator<Item = Pair<'i, Rule>>,
{
	type Error = GicError;
	type Input = Pair<'i, Rule>;
	type Output = Expression;

	fn query(&mut self, pair: &Self::Input) -> Result<Affix> {
		match pair.as_rule() {
			Rule::not_op => Ok(Affix::Prefix(Precedence(2))),
			Rule::quantifier_expr => {
				let s = pair.as_str().trim();
				// quantifier_expr has format: "<quant> <var>."
				let mut parts = s.split_whitespace();
				let quant = parts.next().unwrap_or("");
				match quant {
					"forall" | "∀" => Ok(Affix::Prefix(Precedence(3))),
					"exists" | "∃" => Ok(Affix::Prefix(Precedence(4))),
					_ => Err(GicError::SemanticError(format!("Unknown quantifier: {}", quant))),
				}
			},
			Rule::and_op => Ok(Affix::Infix(Precedence(5), Associativity::Left)),
			Rule::or_op => Ok(Affix::Infix(Precedence(6), Associativity::Left)),
			Rule::impl_op => Ok(Affix::Infix(Precedence(7), Associativity::Right)),
			_ => Ok(Affix::Nilfix),
		}
	}

	fn primary(&mut self, pair: Pair<Rule>) -> Result<Expression> {
		match pair.as_rule() {
			Rule::predicate => Ok(Expression::Proposition(parse_proposition(pair)?)),
			Rule::bottom => Ok(Expression::Bottom),
			Rule::group => {
				let inner_expr = pair.into_inner().next().unwrap(); // Rule::expr
				self.parse(&mut inner_expr.into_inner())
					.map_err(|e| GicError::ParseError(format!("Pratt parser error: {:?}", e)))
			},
			Rule::expr | Rule::pratt_expr => self
				.parse(&mut pair.into_inner())
				.map_err(|e| GicError::ParseError(format!("Pratt parser error: {:?}", e))),
			_ => Err(GicError::SemanticError(format!(
				"Unexpected rule in primary: {:?}",
				pair.as_rule()
			))),
		}
	}

	fn prefix(&mut self, pair: Pair<Rule>, rhs: Expression) -> Result<Expression> {
		match pair.as_rule() {
			Rule::not_op => Ok(Expression::Not(Box::new(rhs))),
			Rule::quantifier_expr => {
				// Manually parse inner parts from the string
				let s = pair.as_str();
				let s = s.trim_end_matches('.'); // remove the trailing dot
				let (quant, var) = s.split_once(' ').ok_or_else(|| {
					GicError::SemanticError(format!("Malformed quantifier: {}", s))
				})?;
				let var = var.trim().to_string();
				match quant {
					"forall" | "∀" => Ok(Expression::ForAll(var, Box::new(rhs))),
					"exists" | "∃" => Ok(Expression::Exists(var, Box::new(rhs))),
					_ => Err(GicError::SemanticError(format!("Unknown quantifier: {}", quant))),
				}
			},
			_ => Err(GicError::SemanticError(format!(
				"Unexpected rule in prefix: {:?}",
				pair.as_rule()
			))),
		}
	}

	fn infix(&mut self, lhs: Expression, op: Pair<Rule>, rhs: Expression) -> Result<Expression> {
		match op.as_rule() {
			Rule::and_op => Ok(Expression::And(Box::new(lhs), Box::new(rhs))),
			Rule::or_op => Ok(Expression::Or(Box::new(lhs), Box::new(rhs))),
			Rule::impl_op => Ok(Expression::Implies(Box::new(lhs), Box::new(rhs))),
			_ => Err(GicError::SemanticError(format!(
				"Unexpected rule in infix: {:?}",
				op.as_rule()
			))),
		}
	}

	fn postfix(
		&mut self,
		lhs: Self::Output,
		op: Self::Input,
	) -> std::result::Result<Self::Output, Self::Error> {
		let _ = op;
		let _ = lhs;
		Ok(Expression::Bottom)
	}
}

fn parse_proposition(pair: Pair<Rule>) -> Result<Proposition> {
	let mut inner = pair.into_inner();
	let name = inner.next().unwrap().as_str().to_string();
	let terms: Result<Vec<Term>> = inner.map(parse_term).collect();
	Ok(Proposition { name, terms: terms? })
}

fn parse_term(pair: Pair<Rule>) -> Result<Term> {
	match pair.as_rule() {
		Rule::term => parse_term(pair.into_inner().next().unwrap()),

		Rule::var => Ok(Term::Identifier(pair.as_str().to_string())),

		Rule::cnt => {
			Ok(Term::FunctionApplication { name: pair.as_str().to_string(), args: vec![] })
		},

		Rule::func => {
			let mut inner = pair.into_inner();
			let name = inner.next().unwrap().as_str().to_string();
			let args: Result<Vec<Term>> = inner.map(parse_term).collect();
			Ok(Term::FunctionApplication { name, args: args? })
		},

		Rule::list => parse_term(pair.into_inner().next().unwrap()),

		Rule::list_empty => {
			Ok(Term::FunctionApplication { name: "empty_list".to_string(), args: vec![] })
		},

		Rule::list_plain => {
			let inner = pair.into_inner();
			let items: Vec<Term> = inner.map(parse_term).collect::<Result<_>>()?;
			let list = items.into_iter().rfold(
				Term::FunctionApplication { name: "empty_list".to_string(), args: vec![] },
				|tail, head| Term::FunctionApplication {
					name: "cons".to_string(),
					args: vec![head, tail],
				},
			);
			Ok(list)
		},

		Rule::list_cons => {
			let mut inner = pair.into_inner();
			let head = parse_term(inner.next().unwrap())?;
			let tail = parse_term(inner.next().unwrap())?;
			Ok(Term::FunctionApplication { name: "cons".to_string(), args: vec![head, tail] })
		},

		Rule::number => {
			let num_str = pair.as_str();
			num_str
				.parse::<i64>()
				.map(Term::Number)
				.map_err(|_| GicError::SemanticError(format!("Invalid number: {}", num_str)))
		},

		_ => Err(GicError::SemanticError(format!("Unexpected rule in term: {:?}", pair.as_rule()))),
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn parse_list(input: &str) -> Term {
		// We cheat by parsing a dummy predicate around the list,
		// e.g. "p(<list>)", then extracting the sole term.
		let wrapped = format!("p({});", input);
		let mut exprs = parse_gic_file(&wrapped).unwrap();
		if let Expression::Proposition(prop) = exprs.pop().unwrap() {
			prop.terms.into_iter().next().unwrap()
		} else {
			panic!("Expected proposition");
		}
	}

	#[test]
	fn test_empty_list() {
		assert_eq!(
			parse_list("[]"),
			Term::FunctionApplication { name: "empty_list".into(), args: vec![] }
		);
	}

	#[test]
	fn test_plain_list() {
		// [a,b,c] → cons(a, cons(b, cons(c, empty_list())))
		let t = parse_list("[a,b,c]");
		let expected = Term::FunctionApplication {
			name: "cons".into(),
			args: vec![
				Term::Identifier("a".into()),
				Term::FunctionApplication {
					name: "cons".into(),
					args: vec![
						Term::Identifier("b".into()),
						Term::FunctionApplication {
							name: "cons".into(),
							args: vec![
								Term::Identifier("c".into()),
								Term::FunctionApplication {
									name: "empty_list".into(),
									args: vec![],
								},
							],
						},
					],
				},
			],
		};
		assert_eq!(t, expected);
	}

	#[test]
	fn test_list_cons_with_list_tail() {
		// [a|[b,c]] → cons(a, cons(b, cons(c, empty_list())))
		let t = parse_list("[a|[b,c]]");
		// it should flatten into the same as above
		assert_eq!(format!("{}", t), "[a, b, c]");
	}

	#[test]
	fn test_list_cons_with_var_tail() {
		// [H|T] → cons(H, T)
		let t = parse_list("[H|T]");
		assert_eq!(
			t,
			Term::FunctionApplication {
				name: "cons".into(),
				args: vec![Term::Identifier("H".into()), Term::Identifier("T".into())],
			}
		);
	}
}
