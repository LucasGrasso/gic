use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use crate::types::ast::{Expression, Proposition, Term};
use crate::types::{GicError, Result};

#[derive(Parser)]
#[grammar = "gic.pest"]
pub struct GicParser;

pub fn parse_gic_file(input: &str) -> Result<Vec<Expression>> {
	let pairs = GicParser::parse(Rule::file, input).map_err(GicError::from)?;
	let mut expressions = Vec::new();
	for pair in pairs {
		match pair.as_rule() {
			Rule::clause => {
				let expr = parse_expression_from_pair(pair.into_inner().next().unwrap())?;
				expressions.push(expr);
			},
			Rule::EOI | Rule::WHITESPACE | Rule::COMMENT => continue,
			_ => {
				return Err(GicError::ParseError(format!("Unexpected top-level rule: {:?}", pair)))
			},
		}
	}
	Ok(expressions)
}

fn parse_expression_from_pair(pair: Pair<Rule>) -> Result<Expression> {
	match pair.as_rule() {
		Rule::formula => parse_expression_from_pair(pair.into_inner().next().unwrap()),
		Rule::implies => parse_binary_expression(pair, Rule::impl_op, Expression::Implies),
		Rule::or_expr => parse_binary_expression(pair, Rule::or_op, Expression::Or),
		Rule::and_expr => parse_binary_expression(pair, Rule::and_op, Expression::And),
		Rule::not_expr => {
			let mut inner = pair.into_inner();
			if let Some(not) = inner.next() {
				if not.as_rule() == Rule::not_op {
					let expr = parse_expression_from_pair(inner.next().unwrap())?;
					Ok(Expression::Not(Box::new(expr)))
				} else {
					parse_expression_from_pair(not)
				}
			} else {
				Err(GicError::ParseError("Invalid not_expr".into()))
			}
		},
		Rule::quant => {
			let mut inner = pair.into_inner();
			let quantifier = inner.next().unwrap().as_str();
			let var = inner.next().unwrap().as_str().to_string();
			let expr = parse_expression_from_pair(inner.next().unwrap())?;
			match quantifier {
				"forall" => Ok(Expression::ForAll(var, Box::new(expr))),
				"exists" => Ok(Expression::Exists(var, Box::new(expr))),
				_ => Err(GicError::ParseError("Unknown quantifier".into())),
			}
		},
		Rule::atom_form => parse_atom(pair.into_inner().next().unwrap()),
		_ => Err(GicError::ParseError(format!("Unexpected rule: {:?}", pair))),
	}
}

fn parse_binary_expression(
	pair: Pair<Rule>,
	_op_rule: Rule,
	constructor: fn(Box<Expression>, Box<Expression>) -> Expression,
) -> Result<Expression> {
	let mut inner = pair.into_inner();
	let mut left = parse_expression_from_pair(inner.next().unwrap())?;
	while let Some(_) = inner.next() {
		let right = parse_expression_from_pair(inner.next().unwrap())?;
		left = constructor(Box::new(left), Box::new(right));
	}
	Ok(left)
}

fn parse_atom(pair: Pair<Rule>) -> Result<Expression> {
	match pair.as_rule() {
		Rule::predicate => Ok(Expression::Proposition(parse_proposition(pair)?)),
		Rule::bottom => Ok(Expression::Bottom),
		_ => Err(GicError::ParseError(format!("Unexpected atom: {:?}", pair))),
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
		Rule::var | Rule::cnt => Ok(Term::Identifier(pair.as_str().to_string())),
		Rule::func => {
			let mut inner = pair.into_inner();
			let name = inner.next().unwrap().as_str().to_string();
			let args: Result<Vec<Term>> = inner.map(parse_term).collect();
			Ok(Term::FunctionApplication { name, args: args? })
		},
		_ => Err(GicError::ParseError(format!("Unexpected term: {:?}", pair))),
	}
}
