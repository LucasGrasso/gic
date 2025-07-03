use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug)]
pub enum GicError {
	EmptyList,
	NoChildren,
	NumArguments(usize, usize),
	ParseError(String),
	ReadlineError(String),
	WrongType(String, String),
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

impl fmt::Display for GicError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			GicError::EmptyList => write!(f, "The list is empty"),
			GicError::NoChildren => write!(f, "No children found"),
			GicError::NumArguments(expected, found) => {
				write!(f, "Expected {} arguments, found {}", expected, found)
			},
			GicError::ParseError(msg) => write!(f, "Parse error: {}", msg),
			GicError::ReadlineError(msg) => write!(f, "Readline error: {}", msg),
			GicError::WrongType(expected, found) => {
				write!(f, "Expected type {}, found type {}", expected, found)
			},
		}
	}
}
