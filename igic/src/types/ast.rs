use std::fmt;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum Term {
	Identifier(String),
	FunctionApplication { name: String, args: Vec<Term> },
}

impl Term {
	pub fn append_suffix_to_vars(&self, suffix: &str) -> Term {
		let new_term = match self {
			Term::Identifier(id) => Term::Identifier(format!("{}{}", id, suffix)),
			Term::FunctionApplication { name, args } => {
				let new_args: Vec<Term> =
					args.iter().map(|arg| arg.append_suffix_to_vars(suffix)).collect();
				Term::FunctionApplication { name: name.clone(), args: new_args }
			},
		};
		new_term
	}
}

impl fmt::Display for Term {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Term::Identifier(id) => write!(f, "{}", id),

			Term::FunctionApplication { name, args } => {
				// Detención de empty_list y cons
				if name == "empty_list" && args.is_empty() {
					return write!(f, "[]");
				}

				if name == "cons" && args.len() == 2 {
					// Intentamos reconstruir [X|XS] sintácticamente
					let head = &args[0];
					let tail = &args[1];

					// Si el tail también es una lista, vamos a aplanarla
					let mut elements = vec![format!("{}", head)];
					let mut current_tail = tail;

					while let Term::FunctionApplication { name, args } = current_tail {
						if name == "cons" && args.len() == 2 {
							elements.push(format!("{}", args[0]));
							current_tail = &args[1];
						} else if name == "empty_list" && args.is_empty() {
							// lista completa
							return write!(f, "[{}]", elements.join(", "));
						} else {
							// forma [X|Tail]
							return write!(f, "[{}|{}]", elements.join(", "), current_tail);
						}
					}
					// Llegamos a algo que no es lista
					write!(f, "[{}|{}]", elements.join(", "), current_tail)
				} else {
					// Default: name(arg1, arg2, ...)
					let args_str: Vec<String> = args.iter().map(|arg| format!("{}", arg)).collect();
					write!(f, "{}({})", name, args_str.join(", "))
				}
			},
		}
	}
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct Proposition {
	pub name: String,
	pub terms: Vec<Term>,
}

impl fmt::Display for Proposition {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let terms_str: Vec<String> = self
			.terms
			.iter()
			.map(|term| format!("{}", term)) // Recursively call fmt::Display for each term
			.collect();
		write!(f, "{}({})", self.name, terms_str.join(", "))
	}
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

impl fmt::Display for Expression {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Expression::Proposition(prop) => {
				write!(f, "{}", prop)
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
