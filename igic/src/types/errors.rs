use crate::parser::Rule;
use pest::iterators::Pair;
use pratt::PrattError;
use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug)]
pub enum GicError {
	EmptyList,
	NoChildren,
	NumArguments(usize, usize),
	ParseError(String),
	SemanticError(String),
	ReadlineError(String),
	WrongType(String, String),
	ClauseError(String),
}

impl<T> From<pest::error::Error<T>> for GicError
where
	T: Debug + Ord + Copy + Hash,
{
	fn from(error: pest::error::Error<T>) -> Self {
		GicError::ParseError(format!("{}", error))
	}
}

impl From<std::io::Error> for GicError {
	fn from(error: std::io::Error) -> Self {
		GicError::ParseError(error.to_string())
	}
}

impl From<PrattError<Pair<'_, Rule>, GicError>> for GicError {
	fn from(error: PrattError<Pair<'_, Rule>, GicError>) -> Self {
		GicError::ParseError(format!("{}", error))
	}
}

impl fmt::Display for GicError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			GicError::EmptyList => write!(f, "The list is empty"),
			GicError::NoChildren => write!(f, "No children found"),
			GicError::NumArguments(expected, found) => {
				write!(f, "Expected {} arguments, found {}", expected, found)
			},
			GicError::ParseError(msg) => write!(f, "Parse error: {}", msg),
			GicError::SemanticError(msg) => write!(f, "Semantic error: {}", msg),
			GicError::ReadlineError(msg) => write!(f, "Readline error: {}", msg),
			GicError::WrongType(expected, found) => {
				write!(f, "Expected type {}, found type {}", expected, found)
			},
			GicError::ClauseError(msg) => write!(f, "Clause error: {}", msg),
		}
	}
}
