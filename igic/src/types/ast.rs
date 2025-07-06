use std::fmt;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum Term {
	Identifier(String),
	FunctionApplication { name: String, args: Vec<Term> },
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct Proposition {
	pub name: String,
	pub terms: Vec<Term>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
	Proposition(Proposition),
	Bottom,
	And(Box<Expression>, Box<Expression>),
	Or(Box<Expression>, Box<Expression>),
	Implies(Box<Expression>, Box<Expression>),
	Not(Box<Expression>),
	Exists(String, Box<Expression>),
	ForAll(String, Box<Expression>),
}

impl fmt::Display for Term {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Term::Identifier(id) => write!(f, "{}", id),
			Term::FunctionApplication { name, args } => {
				// Format: name(arg1, arg2, ...)
				let args_str: Vec<String> = args
					.iter()
					.map(|arg_term| format!("{}", arg_term)) // Recursively call fmt::Display for each argument
					.collect();
				write!(f, "{}({})", name, args_str.join(", "))
			},
		}
	}
}

impl fmt::Display for Expression {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Expression::Proposition(prop) => {
				write!(
					f,
					"{}({})",
					prop.name,
					prop.terms
						.iter()
						.map(|t| match t {
							Term::Identifier(id) => id.clone(),
							Term::FunctionApplication { name, args } => {
								format!(
									"{}({})",
									name,
									args.iter()
										.map(|a| format!("{}", a))
										.collect::<Vec<String>>()
										.join(", ")
								)
							},
						})
						.collect::<Vec<String>>()
						.join(", ")
				)
			},
			Expression::Bottom => write!(f, "⊥"),
			Expression::And(left, right) => write!(f, "({} ∧ {})", left, right),
			Expression::Or(left, right) => write!(f, "({} ∨ {})", left, right),
			Expression::Implies(left, right) => write!(f, "({} => {})", left, right),
			Expression::Not(expr) => write!(f, "¬{}", expr),
			Expression::Exists(var, expr) => write!(f, "∃{}: {}", var, expr),
			Expression::ForAll(var, expr) => write!(f, "∀{}: {}", var, expr),
		}
	}
}
